use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io;

use chrono::NaiveDate;
use clap::ValueEnum;
use rand::RngExt;

use crate::card::{Card, Outcome};
use crate::kana::{self, Kana};
use crate::storage;

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
    pub moved_up: usize,
    pub next_review: Option<NaiveDate>,
}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Score : {}/{}", self.correct, self.asked)?;
        writeln!(f, "Cartes montées d'une boîte : {}", self.moved_up)?;
        match self.next_review {
            Some(date) => write!(f, "Prochaine révision : {}", date.format("%d/%m/%Y")),
            None => write!(f, "Prochaine révision : aucune"),
        }
    }
}

/// Runs a review session on the due cards (plus look-alikes) and saves after each answer.
pub fn run(cards: &mut [Card], today: NaiveDate, mode: Mode) -> Summary {
    // kana → index in `cards`, to find look-alikes quickly
    let index: HashMap<&str, usize> = cards
        .iter()
        .enumerate()
        .map(|(i, c)| (kana::HIRAGANA[c.id].kana, i))
        .collect();
    let mut asked: HashSet<usize> = HashSet::new();
    let mut extras: HashSet<usize> = HashSet::new(); // look-alikes added to this session
    let mut summary = Summary::default();

    loop {
        let pool: Vec<usize> = (0..cards.len())
            .filter(|&i| cards[i].is_due(today) || extras.contains(&i))
            .collect();
        if pool.is_empty() {
            println!("Session terminée !");
            break;
        }

        let x = pool[rand::rng().random_range(0..pool.len())];
        let to_guess = &kana::HIRAGANA[cards[x].id];
        let Some(correct) = ask(to_guess, mode) else {
            break;
        };

        summary.asked += 1;
        let before = cards[x].box_number;
        if correct {
            summary.correct += 1;
            // A look-alike asked early keeps its schedule when right.
            if cards[x].is_due(today) {
                cards[x].answer(Outcome::Correct, today);
            }
        } else {
            cards[x].answer(Outcome::Wrong, today);
        }
        if cards[x].box_number > before {
            summary.moved_up += 1;
        }

        asked.insert(x);
        extras.remove(&x);
        for other in kana::look_alikes(to_guess.kana) {
            if let Some(&i) = index.get(other) {
                if !asked.contains(&i) {
                    extras.insert(i);
                }
            }
        }

        if let Err(e) = storage::save(cards) {
            println!("Erreur de sauvegarde : {e}");
        }
    }

    summary.next_review = cards.iter().map(|c| c.next_study).min();
    summary
}

/// Asks one question. Returns whether the answer was right, or None if the user quits.
fn ask(k: &Kana, mode: Mode) -> Option<bool> {
    match mode {
        Mode::Normal => {
            println!("Quel est le romaji de {} ?", k.kana);
            let answer = read_line()?;
            let correct = k.romaji.contains(&answer.as_str());
            if correct {
                println!("Correct ! :-)");
            } else {
                println!("Raté, c'était {}... :-(", k.romaji[0]);
            }
            Some(correct)
        }
        Mode::Inverse => {
            println!(
                "Écris le kana de « {} », puis appuie sur Entrée.",
                k.romaji[0]
            );
            read_line()?;
            println!("Réponse : {}. C'était juste ? (o/n)", k.kana);
            loop {
                match read_line()?.as_str() {
                    "o" => return Some(true),
                    "n" => return Some(false),
                    _ => println!("Réponds par o ou n."),
                }
            }
        }
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
