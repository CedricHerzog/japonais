use chrono::{Days, NaiveDate};
use serde::{Deserialize, Serialize};

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
