//! FSRS-5 scheduler (Free Spaced Repetition Scheduler), with its default weights.
//! Each card has a memory state: stability (days until recall drops to 90%)
//! and difficulty (1 to 10). See https://github.com/open-spaced-repetition/fsrs4anki/wiki

use serde::{Deserialize, Serialize};

/// FSRS-5 default weights, fitted on millions of Anki reviews.
const W: [f64; 19] = [
    0.40255, 1.18385, 3.173, 15.69105, 7.1949, 0.5345, 1.4604, 0.0046, 1.54575, 0.1192, 1.01925,
    1.9395, 0.11, 0.29605, 2.2698, 0.2315, 2.9898, 0.51655, 0.6621,
];
const DECAY: f64 = -0.5;
const FACTOR: f64 = 19.0 / 81.0; // makes R = 0.9 when elapsed days = stability
const DESIRED_RETENTION: f64 = 0.9;
const MAX_INTERVAL: u64 = 365;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Rating {
    Again = 1,
    #[allow(dead_code)] // part of FSRS, but the app has no "hard" answer
    Hard,
    Good,
    Easy,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Memory {
    pub stability: f64,  // in days
    pub difficulty: f64, // 1 (easy) to 10 (hard)
}

impl Memory {
    /// Memory state after the very first review.
    pub fn initial(rating: Rating) -> Memory {
        Memory {
            stability: W[rating as usize - 1],
            difficulty: initial_difficulty(rating),
        }
    }

    /// Probability of recalling the card after `elapsed_days`.
    pub fn retrievability(&self, elapsed_days: f64) -> f64 {
        (1.0 + FACTOR * elapsed_days / self.stability).powf(DECAY)
    }

    /// Memory state after a review `elapsed_days` after the previous one.
    pub fn next(self, rating: Rating, elapsed_days: f64) -> Memory {
        let g = rating as u8 as f64;
        let (s, d) = (self.stability, self.difficulty);

        let stability = if elapsed_days == 0.0 {
            // Same-day review: short-term effect only.
            s * (W[17] * (g - 3.0 + W[18])).exp()
        } else {
            let r = self.retrievability(elapsed_days);
            if rating == Rating::Again {
                let lapse = W[11]
                    * d.powf(-W[12])
                    * ((s + 1.0).powf(W[13]) - 1.0)
                    * (W[14] * (1.0 - r)).exp();
                lapse.min(s)
            } else {
                let bonus = match rating {
                    Rating::Hard => W[15],
                    Rating::Easy => W[16],
                    _ => 1.0,
                };
                s * (1.0
                    + W[8].exp()
                        * (11.0 - d)
                        * s.powf(-W[9])
                        * ((W[10] * (1.0 - r)).exp() - 1.0)
                        * bonus)
            }
        };

        // Harder answers raise difficulty, damped near 10, then pulled back toward the default.
        let damped = d - W[6] * (g - 3.0) * (10.0 - d) / 9.0;
        let difficulty = W[7] * initial_difficulty(Rating::Easy) + (1.0 - W[7]) * damped;

        Memory {
            stability,
            difficulty: difficulty.clamp(1.0, 10.0),
        }
    }
}

fn initial_difficulty(rating: Rating) -> f64 {
    let g = rating as u8 as f64;
    (W[4] - (W[5] * (g - 1.0)).exp() + 1.0).clamp(1.0, 10.0)
}

/// Days to wait so recall probability drops to DESIRED_RETENTION.
pub fn interval_days(stability: f64) -> u64 {
    let days = stability / FACTOR * (DESIRED_RETENTION.powf(1.0 / DECAY) - 1.0);
    (days.round() as u64).clamp(1, MAX_INTERVAL)
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATINGS: [Rating; 4] = [Rating::Again, Rating::Hard, Rating::Good, Rating::Easy];

    #[test]
    fn initial_stability_comes_from_weights() {
        assert_eq!(Memory::initial(Rating::Good).stability, W[2]);
        assert_eq!(Memory::initial(Rating::Easy).stability, W[3]);
        assert!(
            Memory::initial(Rating::Again).difficulty > Memory::initial(Rating::Easy).difficulty
        );
    }

    #[test]
    fn interval_equals_stability_at_90_percent() {
        assert_eq!(interval_days(3.173), 3);
        assert_eq!(interval_days(15.69), 16);
        assert_eq!(interval_days(0.4), 1); // never less than a day
        assert_eq!(interval_days(10_000.0), MAX_INTERVAL);
    }

    #[test]
    fn recall_is_90_percent_after_stability_days() {
        let m = Memory::initial(Rating::Good);
        assert!((m.retrievability(m.stability) - 0.9).abs() < 1e-9);
    }

    #[test]
    fn success_raises_stability_and_lapse_lowers_it() {
        let m = Memory::initial(Rating::Good);
        assert!(m.next(Rating::Good, 3.0).stability > m.stability);
        assert!(m.next(Rating::Easy, 3.0).stability > m.next(Rating::Good, 3.0).stability);
        assert!(m.next(Rating::Again, 3.0).stability < m.stability);
    }

    #[test]
    fn same_day_review_has_a_small_effect() {
        let m = Memory::initial(Rating::Again);
        let good = m.next(Rating::Good, 0.0).stability;
        assert!(good > m.stability && good < 2.0 * m.stability);
        assert!(m.next(Rating::Again, 0.0).stability < m.stability);
    }

    #[test]
    fn difficulty_stays_between_1_and_10() {
        for rating in RATINGS {
            let mut m = Memory::initial(rating);
            for _ in 0..50 {
                m = m.next(rating, 1.0);
                assert!((1.0..=10.0).contains(&m.difficulty));
            }
        }
    }
}
