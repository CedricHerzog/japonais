use chrono::{Days, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::fsrs::{self, Memory, Rating};
#[cfg(test)]
use crate::kana::HIRAGANA;
use crate::kana::Kana;

/// A group is mastered when all its cards reach this stability (in days).
/// Above FSRS's first-Good value (3.2), so a group needs either a second
/// successful review or a fast first answer, as box 3 did with Leitner.
const UNLOCK_STABILITY: f64 = 5.0;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Outcome {
    Fast, // right and quick: rated Easy
    Correct,
    Wrong,
}

impl From<Outcome> for Rating {
    fn from(outcome: Outcome) -> Rating {
        match outcome {
            Outcome::Fast => Rating::Easy,
            Outcome::Correct => Rating::Good,
            Outcome::Wrong => Rating::Again,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub id: usize,              // index into its script's table
    pub memory: Option<Memory>, // None until the first review
    pub last_review: Option<NaiveDate>,
    pub next_study: NaiveDate,
}

impl Card {
    pub fn new(id: usize, today: NaiveDate) -> Card {
        Card {
            id,
            memory: None,
            last_review: None,
            next_study: today,
        }
    }

    pub fn is_due(&self, today: NaiveDate) -> bool {
        self.next_study <= today
    }

    /// Stability in days, 0 for a card never reviewed.
    pub fn stability(&self) -> f64 {
        self.memory.map_or(0.0, |m| m.stability)
    }

    pub fn answer(&mut self, outcome: Outcome, today: NaiveDate) {
        let rating = Rating::from(outcome);
        let memory = match (self.memory, self.last_review) {
            (Some(m), Some(last)) => {
                let elapsed = (today - last).num_days().max(0) as f64;
                m.next(rating, elapsed)
            }
            _ => Memory::initial(rating),
        };
        self.memory = Some(memory);
        self.last_review = Some(today);
        // A wrong answer stays due, so it comes back in this session.
        self.next_study = match outcome {
            Outcome::Wrong => today,
            _ => today + Days::new(fsrs::interval_days(memory.stability)),
        };
    }
}

/// Counts cards by stability: how many days they stay 90% remembered.
pub fn stability_buckets(cards: &[Card]) -> [(&'static str, usize); 5] {
    let bounds = [
        ("< 3 j", 0.0, 3.0),
        ("3–7 j", 3.0, 7.0),
        ("7–21 j", 7.0, 21.0),
        ("21 j +", 21.0, f64::INFINITY),
    ];
    let count = |min: f64, max: f64| {
        cards
            .iter()
            .filter(|c| {
                c.memory
                    .is_some_and(|m| min <= m.stability && m.stability < max)
            })
            .count()
    };
    let new = cards.iter().filter(|c| c.memory.is_none()).count();
    let [a, b, c, d] = bounds.map(|(label, min, max)| (label, count(min, max)));
    [("Nouveaux", new), a, b, c, d]
}

/// Adds the next group's cards if every card of the current group is stable enough.
/// Returns the unlocked group number, if any.
pub fn unlock_next_group(cards: &mut Vec<Card>, table: &[Kana], today: NaiveDate) -> Option<u8> {
    let current = cards.iter().map(|c| table[c.id].group).max().unwrap_or(0);

    let mastered = cards
        .iter()
        .filter(|c| table[c.id].group == current)
        .all(|c| c.stability() >= UNLOCK_STABILITY);
    if !mastered {
        return None;
    }

    let next = current + 1;
    let new_cards: Vec<Card> = table
        .iter()
        .enumerate()
        .filter(|&(_, k)| k.group == next)
        .map(|(id, _)| Card::new(id, today))
        .collect();
    if new_cards.is_empty() {
        return None; // last group already unlocked
    }

    cards.extend(new_cards);
    Some(next)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()
    }

    fn with_stability(card: &mut Card, stability: f64) {
        card.memory = Some(Memory {
            stability,
            difficulty: 5.0,
        });
    }

    #[test]
    fn correct_on_new_card_is_due_in_3_days() {
        let mut card = Card::new(0, day());
        card.answer(Outcome::Correct, day());
        assert_eq!(card.next_study, day() + Days::new(3));
        assert_eq!(card.last_review, Some(day()));
    }

    #[test]
    fn fast_on_new_card_waits_longer() {
        let mut card = Card::new(0, day());
        card.answer(Outcome::Fast, day());
        assert_eq!(card.next_study, day() + Days::new(16));
    }

    #[test]
    fn wrong_stays_due_and_then_recovers() {
        let mut card = Card::new(0, day());
        card.answer(Outcome::Wrong, day());
        assert!(card.is_due(day()));
        card.answer(Outcome::Correct, day());
        assert_eq!(card.next_study, day() + Days::new(1));
    }

    #[test]
    fn reviews_on_time_grow_the_interval() {
        let mut card = Card::new(0, day());
        card.answer(Outcome::Correct, day());
        let first = card.next_study;
        card.answer(Outcome::Correct, first);
        assert!(card.next_study - first > first - day());
    }

    #[test]
    fn early_review_gains_less_than_on_time() {
        let mut early = Card::new(0, day());
        early.answer(Outcome::Correct, day());
        let mut on_time = Card::new(0, day());
        on_time.answer(Outcome::Correct, day());
        let due = on_time.next_study;

        early.answer(Outcome::Correct, day() + Days::new(1));
        on_time.answer(Outcome::Correct, due);
        assert!(early.stability() < on_time.stability());
    }

    #[test]
    fn empty_deck_unlocks_group_1() {
        let mut cards = Vec::new();
        assert_eq!(unlock_next_group(&mut cards, HIRAGANA, day()), Some(1));
        assert_eq!(cards.len(), 5);
    }

    #[test]
    fn group_not_mastered_unlocks_nothing() {
        let mut cards = Vec::new();
        unlock_next_group(&mut cards, HIRAGANA, day());
        for card in cards.iter_mut() {
            with_stability(card, UNLOCK_STABILITY);
        }
        with_stability(&mut cards[0], 3.0);
        assert_eq!(unlock_next_group(&mut cards, HIRAGANA, day()), None);
        assert_eq!(cards.len(), 5);
    }

    #[test]
    fn one_correct_answer_is_not_enough_to_unlock() {
        let mut cards = Vec::new();
        unlock_next_group(&mut cards, HIRAGANA, day());
        for card in cards.iter_mut() {
            card.answer(Outcome::Correct, day());
        }
        assert_eq!(unlock_next_group(&mut cards, HIRAGANA, day()), None);
    }

    #[test]
    fn mastered_group_unlocks_next() {
        let mut cards = Vec::new();
        unlock_next_group(&mut cards, HIRAGANA, day());
        for card in cards.iter_mut() {
            with_stability(card, UNLOCK_STABILITY);
        }
        assert_eq!(unlock_next_group(&mut cards, HIRAGANA, day()), Some(2));
        assert_eq!(cards.len(), 10);
    }

    #[test]
    fn nothing_after_last_group() {
        let mut cards: Vec<Card> = (0..HIRAGANA.len()).map(|id| Card::new(id, day())).collect();
        for card in cards.iter_mut() {
            with_stability(card, 30.0);
        }
        assert_eq!(unlock_next_group(&mut cards, HIRAGANA, day()), None);
    }
}
