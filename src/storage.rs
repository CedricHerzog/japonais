use std::fs;
use std::io;
use std::path::PathBuf;

use crate::card::Card;
use crate::kana::Script;
use std::env;

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
    let cards: Vec<Card> = serde_json::from_str(&content)?;
    Ok(cards)
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
