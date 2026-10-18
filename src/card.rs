use chrono::{Days, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::kana::HIRAGANA;

pub enum Outcome {
    Correct,
    Wrong,
}

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub id: usize,      // index into HIRAGANA
    pub box_number: u8, // 1 to 5
    pub next_study: NaiveDate,
}

impl Card {
    pub fn new(id: usize, today: NaiveDate) -> Card {
        Card {
            id,
            box_number: 1,
            next_study: today,
        }
    }

    pub fn is_due(&self, today: NaiveDate) -> bool {
        self.next_study <= today
    }

    pub fn answer(&mut self, outcome: Outcome, today: NaiveDate) {
        match outcome {
            Outcome::Correct => {
                if self.box_number < 5 {
                    self.box_number += 1;
                }
            }
            Outcome::Wrong => self.box_number = 1,
        }
        let days = match self.box_number {
            1 => 0,
            2 => 1,
            3 => 3,
            4 => 7,
            _ => 21,
        };
        self.next_study = today + Days::new(days);
    }
}

/// Adds the next group's cards if every card of the current group is in box 3+.
/// Returns the unlocked group number, if any.
pub fn unlock_next_group(cards: &mut Vec<Card>, today: NaiveDate) -> Option<u8> {
    let current = cards
        .iter()
        .map(|c| HIRAGANA[c.id].group)
        .max()
        .unwrap_or(0);

    let mastered = cards
        .iter()
        .filter(|c| HIRAGANA[c.id].group == current)
        .all(|c| c.box_number >= 3);
    if !mastered {
        return None;
    }

    let next = current + 1;
    let new_cards: Vec<Card> = HIRAGANA
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

    #[test]
    fn correct_moves_up_one_box() {
        let mut card = Card::new(0, day());
        card.answer(Outcome::Correct, day());
        assert_eq!(card.box_number, 2);
        assert_eq!(card.next_study, day() + Days::new(1));
    }

    #[test]
    fn wrong_goes_back_to_box_1_and_stays_due() {
        let mut card = Card::new(0, day());
        card.box_number = 4;
        card.answer(Outcome::Wrong, day());
        assert_eq!(card.box_number, 1);
        assert!(card.is_due(day()));
    }

    #[test]
    fn box_never_exceeds_5() {
        let mut card = Card::new(0, day());
        for _ in 0..10 {
            card.answer(Outcome::Correct, day());
        }
        assert_eq!(card.box_number, 5);
    }

    #[test]
    fn empty_deck_unlocks_group_1() {
        let mut cards = Vec::new();
        assert_eq!(unlock_next_group(&mut cards, day()), Some(1));
        assert_eq!(cards.len(), 5);
    }

    #[test]
    fn group_not_mastered_unlocks_nothing() {
        let mut cards = Vec::new();
        unlock_next_group(&mut cards, day());
        for card in cards.iter_mut() {
            card.box_number = 3;
        }
        cards[0].box_number = 2;
        assert_eq!(unlock_next_group(&mut cards, day()), None);
        assert_eq!(cards.len(), 5);
    }

    #[test]
    fn mastered_group_unlocks_next() {
        let mut cards = Vec::new();
        unlock_next_group(&mut cards, day());
        for card in cards.iter_mut() {
            card.box_number = 3;
        }
        assert_eq!(unlock_next_group(&mut cards, day()), Some(2));
        assert_eq!(cards.len(), 10);
    }

    #[test]
    fn nothing_after_last_group() {
        let mut cards: Vec<Card> = (0..HIRAGANA.len()).map(|id| Card::new(id, day())).collect();
        for card in cards.iter_mut() {
            card.box_number = 5;
        }
        assert_eq!(unlock_next_group(&mut cards, day()), None);
    }
}
