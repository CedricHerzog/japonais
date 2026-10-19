use std::fs;
use std::io;
use std::path::PathBuf;

use chrono::{Days, NaiveDate};
use serde::Deserialize;

use crate::card::Card;
use crate::fsrs::Memory;
use crate::kana::Script;
use std::env;

/// A card as found in the file: the current format, or the Leitner one from before FSRS.
#[derive(Deserialize)]
#[serde(untagged)]
enum SavedCard {
    // Tried first: a current card would also match a missing-fields Leitner card otherwise.
    Leitner {
        id: usize,
        box_number: u8,
        next_study: NaiveDate,
    },
    Current(Card),
}

impl From<SavedCard> for Card {
    fn from(saved: SavedCard) -> Card {
        match saved {
            SavedCard::Current(card) => card,
            SavedCard::Leitner {
                id,
                box_number,
                next_study,
            } => {
                // A Leitner box's interval is a fair guess of the card's stability.
                let days = match box_number {
                    0 | 1 => return Card::new(id, next_study),
                    2 => 1,
                    3 => 3,
                    4 => 7,
                    _ => 21,
                };
                Card {
                    id,
                    memory: Some(Memory {
                        stability: days as f64,
                        difficulty: 5.0,
                    }),
                    last_review: next_study.checked_sub_days(Days::new(days)),
                    next_study,
                }
            }
        }
    }
}

fn file_path(script: Script) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = env::home_dir().ok_or("dossier introuvable")?;
    // Hiragana keeps the original file name so existing progress still loads.
    let name = match script {
        Script::Hiragana => "progression.json",
        Script::Katakana => "katakana.json",
    };
    Ok(home.join(".kana").join(name))
}

/// Returns an empty deck on first launch (no file yet).
pub fn load(script: Script) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let content = match fs::read_to_string(file_path(script)?) {
        Ok(content) => content,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e.into()),
    };
    let saved: Vec<SavedCard> = serde_json::from_str(&content)?;
    Ok(saved.into_iter().map(Card::from).collect())
}

pub fn save(script: Script, cards: &[Card]) -> Result<(), Box<dyn std::error::Error>> {
    let path = file_path(script)?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(path, serde_json::to_string_pretty(cards)?)?;
    Ok(())
}

/// Deletes the save file. Doing it when there is no file is not an error.
pub fn reset(script: Script) -> Result<(), Box<dyn std::error::Error>> {
    match fs::remove_file(file_path(script)?) {
        Err(e) if e.kind() != io::ErrorKind::NotFound => Err(e.into()),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leitner_cards_are_converted() {
        let json = r#"[
            {"id": 0, "box_number": 1, "next_study": "2026-01-01"},
            {"id": 1, "box_number": 4, "next_study": "2026-01-10"}
        ]"#;
        let saved: Vec<SavedCard> = serde_json::from_str(json).unwrap();
        let cards: Vec<Card> = saved.into_iter().map(Card::from).collect();
        assert!(cards[0].memory.is_none());
        assert_eq!(cards[1].stability(), 7.0);
        assert_eq!(cards[1].last_review, NaiveDate::from_ymd_opt(2026, 1, 3));
        assert_eq!(
            cards[1].next_study,
            NaiveDate::from_ymd_opt(2026, 1, 10).unwrap()
        );
    }

    #[test]
    fn current_cards_round_trip() {
        let mut card = Card::new(3, NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
        card.answer(crate::card::Outcome::Correct, card.next_study);
        let json = serde_json::to_string(&[card]).unwrap();
        let saved: Vec<SavedCard> = serde_json::from_str(&json).unwrap();
        let back = Card::from(saved.into_iter().next().unwrap());
        assert_eq!(back.id, 3);
        assert!(back.memory.is_some() && back.last_review.is_some());
    }
}
