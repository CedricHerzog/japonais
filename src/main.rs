use std::collections::{HashMap, HashSet};
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
        Err(e) => {
            // Don't start (and overwrite) if the save file exists but can't be read.
            println!("Impossible de charger la progression : {e}");
            return;
        }
    };

    if let Some(group) = card::unlock_next_group(&mut cards, today) {
        let new_kana: Vec<&str> = kana::HIRAGANA
            .iter()
            .filter(|k| k.group == group)
            .map(|k| k.kana)
            .collect();
        println!("Nouveau groupe débloqué : {}", new_kana.join(" "));
    }

    // kana → index in `cards`, to find look-alikes quickly
    let index: HashMap<&str, usize> = cards
        .iter()
        .enumerate()
        .map(|(i, c)| (kana::HIRAGANA[c.id].kana, i))
        .collect();
    let mut asked: HashSet<usize> = HashSet::new();
    let mut extras: HashSet<usize> = HashSet::new(); // look-alikes added to this session

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
                    // A look-alike asked early keeps its schedule when right.
                    if cards[x].is_due(today) {
                        cards[x].answer(Outcome::Correct, today);
                    }
                } else {
                    println!("Raté, c'était {}... :-(", to_guess.romaji[0]);
                    cards[x].answer(Outcome::Wrong, today);
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
