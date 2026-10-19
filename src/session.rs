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
const FAST: Duration = Duration::from_secs(3);

/// Direction of the questions.
#[derive(Clone, Copy, ValueEnum)]
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

/// Runs a review session on the due cards (plus look-alikes) and saves after each answer.
/// With `ahead`, every unlocked card is asked once, like Anki's "Review ahead".
pub fn run(
    cards: &mut [Card],
    script: Script,
    today: NaiveDate,
    mode: Mode,
    ahead: bool,
) -> Summary {
    let table = script.table();
    let mut summary = Summary::default();
    if !ahead && !cards.iter().any(|c| c.is_due(today)) {
        println!("Rien à réviser pour l'instant.");
        println!("Pour réviser quand même : « kana revision --avance ».");
        summary.next_review = cards.iter().map(|c| c.next_study).min();
        return summary;
    }

    // kana → index in `cards`, to find look-alikes quickly
    let index: HashMap<&str, usize> = cards
        .iter()
        .enumerate()
        .map(|(i, c)| (table[c.id].kana, i))
        .collect();
    let mut asked: HashSet<usize> = HashSet::new();
    let mut extras: HashSet<usize> = HashSet::new(); // look-alikes added to this session

    loop {
        let pool: Vec<usize> = (0..cards.len())
            .filter(|&i| {
                cards[i].is_due(today) || extras.contains(&i) || (ahead && !asked.contains(&i))
            })
            .collect();
        if pool.is_empty() {
            println!("Session terminée !");
            break;
        }

        let x = pool[rand::rng().random_range(0..pool.len())];
        let to_guess = &table[cards[x].id];
        let Some(outcome) = ask(to_guess, mode) else {
            break;
        };

        summary.asked += 1;
        let before = cards[x].stability();
        if outcome != Outcome::Wrong {
            summary.correct += 1;
        }
        // A look-alike asked early keeps its schedule when right. In ahead mode
        // every answer counts: FSRS gives an early success only a small gain.
        if outcome == Outcome::Wrong || cards[x].is_due(today) || ahead {
            cards[x].answer(outcome, today);
        }
        if outcome != Outcome::Wrong && cards[x].stability() > before {
            summary.strengthened += 1;
        }

        asked.insert(x);
        extras.remove(&x);
        for other in kana::look_alikes(to_guess.kana) {
            if let Some(&i) = index.get(other)
                && !asked.contains(&i)
            {
                extras.insert(i);
            }
        }

        if let Err(e) = storage::save(script, cards) {
            println!("Erreur de sauvegarde : {e}");
        }
    }

    summary.next_review = cards.iter().map(|c| c.next_study).min();
    summary
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
    if let Some(mnemonic) = k.mnemonic {
        println!("  {} {}", "Astuce :".yellow(), mnemonic.italic());
    }
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
