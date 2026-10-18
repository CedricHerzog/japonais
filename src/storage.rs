use std::fs::{self, File};
use std::io::{self, Write};

use crate::card::Card;
use std::env;

/// Returns an empty deck on first launch (no file yet).
pub fn load() -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let path = env::home_dir().ok_or("dossier introuvable")?;
    let content = match fs::read_to_string(path.join(".kana/progression.json")) {
        Ok(content) => content,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e.into()),
    };
    let cards: Vec<Card> = serde_json::from_str(&content)?;
    Ok(cards)
}

pub fn save(cards: &[Card]) -> Result<(), Box<dyn std::error::Error>> {
    let path = env::home_dir().ok_or("dossier introuvable")?;

    fs::create_dir_all(path.join(".kana"))?;
    let card_content = serde_json::to_string_pretty(cards)?;
    let mut f = File::create(path.join(".kana/progression.json"))?;
    f.write_all(card_content.as_bytes())?;

    Ok(())
}
