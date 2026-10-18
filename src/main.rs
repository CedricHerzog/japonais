use chrono::NaiveDate;
use clap::{Parser, Subcommand};

use crate::card::Card;
use crate::session::Mode;

mod card;
mod kana;
mod session;
mod storage;

/// Apprends les hiragana dans ton terminal.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
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

    match cli
        .command
        .unwrap_or(Command::Revision { mode: Mode::Normal })
    {
        Command::Revision { mode } => {
            let Some(mut cards) = load() else { return };
            if let Some(group) = card::unlock_next_group(&mut cards, today) {
                let new_kana: Vec<&str> = kana::HIRAGANA
                    .iter()
                    .filter(|k| k.group == group)
                    .map(|k| k.kana)
                    .collect();
                println!("Nouveau groupe débloqué : {}", new_kana.join(" "));
            }
            let summary = session::run(&mut cards, today, mode);
            println!("\n{summary}");
        }
        Command::Stats => {
            let Some(cards) = load() else { return };
            stats(&cards, today);
        }
        Command::Reset => reset(),
    }
}

/// Loads the deck, or prints why it can't (so a corrupt file is never overwritten).
fn load() -> Option<Vec<Card>> {
    match storage::load() {
        Ok(cards) => Some(cards),
        Err(e) => {
            println!("Impossible de charger la progression : {e}");
            println!("(« kana reset » repart de zéro.)");
            None
        }
    }
}

fn stats(cards: &[Card], today: NaiveDate) {
    if cards.is_empty() {
        println!("Aucune progression. Lance « kana revision » pour commencer.");
        return;
    }
    println!("Kana débloqués : {}/{}", cards.len(), kana::HIRAGANA.len());
    for b in 1..=5 {
        let n = cards.iter().filter(|c| c.box_number == b).count();
        println!("Boîte {b} : {:<3} {}", n, "█".repeat(n));
    }
    let due = cards.iter().filter(|c| c.is_due(today)).count();
    println!("À réviser aujourd'hui : {due}");
}

fn reset() {
    println!("Effacer toute la progression ? (o/n)");
    if session::read_line().as_deref() != Some("o") {
        println!("Annulé.");
        return;
    }
    match storage::reset() {
        Ok(()) => println!("Progression effacée."),
        Err(e) => println!("Erreur : {e}"),
    }
}
