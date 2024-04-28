//!
//! File:           stats.rs
//! Description:    Data structures for gathering statistics
//!
//!
//!

use std::fmt;

use crate::types::hand::Outcome;

/// Data to track per player "run" (how long a player sits at the table)
pub struct RunStats {
    num_games: usize,
    wins: usize,
    losses: usize,
    pushes: usize,
    remaining_credits: isize,
}

impl RunStats {
    pub fn new() -> Self {
        RunStats {
            num_games: 0,
            wins: 0,
            losses: 0,
            pushes: 0,
            remaining_credits: 0,
        }
    }

    /// Records stats when a game (single match) ends
    pub fn record_match_end(&mut self, outcome: Outcome) {
        self.num_games += 1;
        match outcome {
            Outcome::Win => self.wins += 1,
            Outcome::Loss => self.losses += 1,
            Outcome::Push => self.pushes += 1,
        }
    }

    /// Record the final credit count
    pub fn record_credits(&mut self, credits: isize) {
        self.remaining_credits = credits;
    }
}

impl fmt::Display for RunStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Games: {} | W/L/P: {}/{}/{} | Credits: ${}",
            self.num_games, self.wins, self.losses, self.pushes, self.remaining_credits
        )
    }
}
