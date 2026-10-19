use chrono::NaiveDate;
use clap::{Parser, Subcommand};

use crate::card::Card;
use crate::kana::Script;
use crate::session::Mode;

mod card;
mod fsrs;
mod kana;
mod session;
mod storage;

/// Apprends les hiragana et les katakana dans ton terminal.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// Syllabaire étudié
    #[arg(long, global = true, value_enum, default_value_t = Script::Hiragana)]
    script: Script,
}

#[derive(Subcommand)]
enum Command {
    /// Lance une session de révision (commande par défaut)
    Revision {
        /// Sens des questions
        #[arg(long, value_enum, default_value_t = Mode::Normal)]
        mode: Mode,
    },
    /// Affiche ta progression
    Stats,
    /// Efface toute ta progression
    Reset,
}

fn main() {
    let cli = Cli::parse();
    let today = chrono::Local::now().date_naive();
    let script = cli.script;

    match cli
        .command
        .unwrap_or(Command::Revision { mode: Mode::Normal })
    {
        Command::Revision { mode } => {
            let Some(mut cards) = load(script) else {
                return;
            };
            if let Some(group) = card::unlock_next_group(&mut cards, script.table(), today) {
                let new_kana: Vec<&str> = script
                    .table()
                    .iter()
                    .filter(|k| k.group == group)
                    .map(|k| k.kana)
                    .collect();
                println!("Nouveau groupe débloqué : {}", new_kana.join(" "));
            }
            let summary = session::run(&mut cards, script, today, mode);
            println!("\n{summary}");
        }
        Command::Stats => {
            let Some(cards) = load(script) else { return };
            stats(&cards, script, today);
        }
        Command::Reset => reset(script),
    }
}

/// Loads the deck, or prints why it can't (so a corrupt file is never overwritten).
fn load(script: Script) -> Option<Vec<Card>> {
    match storage::load(script) {
        Ok(cards) => Some(cards),
        Err(e) => {
            println!("Impossible de charger la progression : {e}");
            println!("(« kana reset » repart de zéro.)");
            None
        }
    }
}

fn stats(cards: &[Card], script: Script, today: NaiveDate) {
    if cards.is_empty() {
        println!(
            "Aucune progression en {0}. Lance « kana revision --script {0} » pour commencer.",
            script.name()
        );
        return;
    }
    println!("Kana débloqués : {}/{}", cards.len(), script.table().len());
    // Cards grouped by stability: how many days they stay 90% remembered.
    let buckets = [
        ("Nouveaux", -1.0, 0.0),
        ("< 3 j", 0.0, 3.0),
        ("3–7 j", 3.0, 7.0),
        ("7–21 j", 7.0, 21.0),
        ("21 j +", 21.0, f64::INFINITY),
    ];
    println!("Mémorisation (stabilité) :");
    for (label, min, max) in buckets {
        let n = cards
            .iter()
            .filter(|c| match c.memory {
                None => min < 0.0,
                Some(m) => min <= m.stability && m.stability < max,
            })
            .count();
        println!("  {label:<9}: {n:<3} {}", "█".repeat(n));
    }
    let due = cards.iter().filter(|c| c.is_due(today)).count();
    println!("À réviser aujourd'hui : {due}");
}

fn reset(script: Script) {
    println!("Effacer la progression des {} ? (o/n)", script.name());
    if session::read_line().as_deref() != Some("o") {
        println!("Annulé.");
        return;
    }
    match storage::reset(script) {
        Ok(()) => println!("Progression effacée."),
        Err(e) => println!("Erreur : {e}"),
    }
}
