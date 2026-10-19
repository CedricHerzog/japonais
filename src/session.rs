use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io;
use std::time::{Duration, Instant};

use chrono::NaiveDate;
use clap::ValueEnum;
use colored::Colorize;
use rand::RngExt;

use crate::card::{Card, Outcome};
use crate::kana::{self, Kana, Script};
use crate::storage;

/// A right answer typed faster than this is rated Easy instead of Good.
pub const FAST: Duration = Duration::from_secs(3);

/// Direction of the questions.
#[derive(Clone, Copy, PartialEq, ValueEnum)]
pub enum Mode {
    /// Kana → romaji : tu tapes la réponse
    Normal,
    /// Romaji → kana : tu l'écris sur papier et tu t'auto-évalues
    Inverse,
}

#[derive(Default)]
pub struct Summary {
    pub asked: usize,
    pub correct: usize,
    pub strengthened: usize,
    pub next_review: Option<NaiveDate>,
}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {}/{}", "Score :".bold(), self.correct, self.asked)?;
        writeln!(f, "Cartes renforcées : {}", self.strengthened)?;
        match self.next_review {
            Some(date) => write!(f, "Prochaine révision : {}", date.format("%d/%m/%Y")),
            None => write!(f, "Prochaine révision : aucune"),
        }
    }
}

/// The rules of a review session, shared by the terminal and the GUI:
/// which card comes next, and what an answer does.
pub struct Session {
    ahead: bool,
    kanas: Vec<&'static str>, // kana of each card, by index in `cards`
    index: HashMap<&'static str, usize>, // and the reverse, to find look-alikes
    asked: HashSet<usize>,
    extras: HashSet<usize>, // look-alikes added to this session
    pub summary: Summary,
}

impl Session {
    /// With `ahead`, every unlocked card is asked once, like Anki's "Review ahead".
    pub fn new(cards: &[Card], script: Script, ahead: bool) -> Session {
        let kanas: Vec<&'static str> = cards.iter().map(|c| script.table()[c.id].kana).collect();
        Session {
            ahead,
            index: kanas.iter().enumerate().map(|(i, &k)| (k, i)).collect(),
            kanas,
            asked: HashSet::new(),
            extras: HashSet::new(),
            summary: Summary::default(),
        }
    }

    /// A random card to ask next (an index in `cards`), or None when the session is over.
    pub fn next(&self, cards: &[Card], today: NaiveDate) -> Option<usize> {
        let pool: Vec<usize> = (0..cards.len())
            .filter(|&i| {
                cards[i].is_due(today)
                    || self.extras.contains(&i)
                    || (self.ahead && !self.asked.contains(&i))
            })
            .collect();
        if pool.is_empty() {
            return None;
        }
        Some(pool[rand::rng().random_range(0..pool.len())])
    }

    /// Records the answer to card `x`, and queues its unlocked look-alikes.
    pub fn record(&mut self, cards: &mut [Card], x: usize, outcome: Outcome, today: NaiveDate) {
        self.summary.asked += 1;
        let before = cards[x].stability();
        if outcome != Outcome::Wrong {
            self.summary.correct += 1;
        }
        // A look-alike asked early keeps its schedule when right. In ahead mode
        // every answer counts: FSRS gives an early success only a small gain.
        if outcome == Outcome::Wrong || cards[x].is_due(today) || self.ahead {
            cards[x].answer(outcome, today);
        }
        if outcome != Outcome::Wrong && cards[x].stability() > before {
            self.summary.strengthened += 1;
        }

        self.asked.insert(x);
        self.extras.remove(&x);
        for other in kana::look_alikes(self.kanas[x]) {
            if let Some(&i) = self.index.get(other)
                && !self.asked.contains(&i)
            {
                self.extras.insert(i);
            }
        }
    }

    pub fn finish(mut self, cards: &[Card]) -> Summary {
        self.summary.next_review = cards.iter().map(|c| c.next_study).min();
        self.summary
    }
}

/// Runs a review session in the terminal and saves after each answer.
pub fn run(
    cards: &mut [Card],
    script: Script,
    today: NaiveDate,
    mode: Mode,
    ahead: bool,
) -> Summary {
    let mut session = Session::new(cards, script, ahead);
    if session.next(cards, today).is_none() {
        println!("Rien à réviser pour l'instant.");
        println!("Pour réviser quand même : « kana revision --avance ».");
        return session.finish(cards);
    }

    while let Some(x) = session.next(cards, today) {
        let Some(outcome) = ask(&script.table()[cards[x].id], mode) else {
            return session.finish(cards);
        };
        session.record(cards, x, outcome, today);
        if let Err(e) = storage::save(script, cards) {
            println!("Erreur de sauvegarde : {e}");
        }
    }
    println!("Session terminée !");
    session.finish(cards)
}

/// Asks one question. Returns the outcome, or None if the user quits.
fn ask(k: &Kana, mode: Mode) -> Option<Outcome> {
    match mode {
        Mode::Normal => {
            println!("Quel est le romaji de {} ?", k.kana.bold());
            let start = Instant::now();
            let answer = read_line()?;
            let elapsed = start.elapsed();

            if !k.romaji.contains(&answer.as_str()) {
                println!("{} c'était {}", "Raté,".red().bold(), k.romaji[0].bold());
                show_mnemonic(k);
                return Some(Outcome::Wrong);
            }
            let seconds = format!("({:.1} s)", elapsed.as_secs_f32()).dimmed();
            if elapsed < FAST {
                println!("{} {seconds}", "Correct, et rapide !".green().bold());
                Some(Outcome::Fast)
            } else {
                println!("{} {seconds}", "Correct !".green());
                Some(Outcome::Correct)
            }
        }
        // No speed bonus here: the time includes writing on paper.
        Mode::Inverse => {
            println!(
                "Écris le kana de « {} », puis appuie sur Entrée.",
                k.romaji[0].bold()
            );
            read_line()?;
            println!("Réponse : {}. C'était juste ? (o/n)", k.kana.bold());
            loop {
                match read_line()?.as_str() {
                    "o" => return Some(Outcome::Correct),
                    "n" => {
                        show_mnemonic(k);
                        return Some(Outcome::Wrong);
                    }
                    _ => println!("Réponds par o ou n."),
                }
            }
        }
    }
}

fn show_mnemonic(k: &Kana) {
    println!("  {} {}", "Astuce :".yellow(), k.mnemonic.italic());
}

/// Reads one trimmed, lowercase line. None on end of input, read error, or "q".
pub fn read_line() -> Option<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) | Err(_) => None,
        Ok(_) => {
            let line = input.trim().to_lowercase();
            if line == "q" { None } else { Some(line) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::unlock_next_group;

    fn day() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()
    }

    /// Group 1, every card answered right today (so nothing is due).
    fn learned_deck() -> Vec<Card> {
        let mut cards = Vec::new();
        unlock_next_group(&mut cards, Script::Hiragana.table(), day());
        for card in cards.iter_mut() {
            card.answer(Outcome::Correct, day());
        }
        cards
    }

    #[test]
    fn nothing_due_means_no_question() {
        let cards = learned_deck();
        let session = Session::new(&cards, Script::Hiragana, false);
        assert_eq!(session.next(&cards, day()), None);
    }

    #[test]
    fn ahead_asks_every_card_once() {
        let mut cards = learned_deck();
        let mut session = Session::new(&cards, Script::Hiragana, true);
        let mut seen = HashSet::new();
        while let Some(x) = session.next(&cards, day()) {
            assert!(seen.insert(x), "card asked twice");
            session.record(&mut cards, x, Outcome::Correct, day());
        }
        assert_eq!(seen.len(), cards.len());
    }

    #[test]
    fn wrong_answer_comes_back() {
        let mut cards = learned_deck();
        let mut session = Session::new(&cards, Script::Hiragana, false);
        session.record(&mut cards, 0, Outcome::Wrong, day());
        assert_eq!(session.next(&cards, day()), Some(0));
        assert_eq!(session.summary.correct, 0);
    }
}
