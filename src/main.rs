use std::io;

use rand::RngExt;

use crate::card::{Card, Outcome};

mod card;
mod kana;
mod storage;

fn main() {
    let today = chrono::Local::now().date_naive();
    let mut cards: Vec<Card> = match storage::load() {
        Ok(cards) => cards,
        Err(_) => kana::HIRAGANA
            .iter()
            .enumerate()
            .filter(|&(_, k)| k.group == 1)
            .map(|(id, _)| Card::new(id, today))
            .collect(),
    };

    loop {
        let due: Vec<usize> = (0..cards.len())
            .filter(|&i| cards[i].is_due(today))
            .collect();
        if due.is_empty() {
            println!("Session terminée !");
            break;
        }

        let x = due[rand::rng().random_range(0..due.len())];
        let to_guess = &kana::HIRAGANA[cards[x].id];
        println!("Quel est le romaji de {} ?", to_guess.kana);

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                let answer = input.trim();
                if answer == "q" {
                    break;
                }

                if to_guess.romaji.contains(&answer) {
                    println!("Correct ! :-)");
                    cards[x].answer(Outcome::Correct, today);
                } else {
                    println!("Raté, c'était {}... :-(", to_guess.romaji[0]);
                    cards[x].answer(Outcome::Wrong, today);
                }

                if let Err(e) = storage::save(&cards) {
                    println!("Erreur de sauvegarde : {e}");
                }
            }
            Err(error) => {
                println!("Erreur : {error}");
                break;
            }
        }
    }
}
