use std::fs;
use std::io;
use std::path::PathBuf;

use crate::card::Card;
use std::env;

fn file_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = env::home_dir().ok_or("dossier introuvable")?;
    Ok(home.join(".kana").join("progression.json"))
}

/// Returns an empty deck on first launch (no file yet).
pub fn load() -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let content = match fs::read_to_string(file_path()?) {
        Ok(content) => content,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e.into()),
    };
    let cards: Vec<Card> = serde_json::from_str(&content)?;
    Ok(cards)
}

pub fn save(cards: &[Card]) -> Result<(), Box<dyn std::error::Error>> {
    let path = file_path()?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(path, serde_json::to_string_pretty(cards)?)?;
    Ok(())
}

/// Deletes the save file. Doing it when there is no file is not an error.
pub fn reset() -> Result<(), Box<dyn std::error::Error>> {
    match fs::remove_file(file_path()?) {
        Err(e) if e.kind() != io::ErrorKind::NotFound => Err(e.into()),
        _ => Ok(()),
    }
}
