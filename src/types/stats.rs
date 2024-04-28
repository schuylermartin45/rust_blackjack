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

/// Aggregates all simulated runs into one data set
pub struct TotalRunStats {
    starting_credits: isize,
    num_runs: usize,
    num_games: usize,
    wins: usize,
    losses: usize,
    pushes: usize,
    total_credits: isize,
    num_walk_away_with_more: usize,
}

impl TotalRunStats {
    pub fn new(starting_credits: isize) -> Self {
        TotalRunStats {
            starting_credits: starting_credits,
            num_runs: 0,
            num_games: 0,
            wins: 0,
            losses: 0,
            pushes: 0,
            total_credits: 0,
            num_walk_away_with_more: 0,
        }
    }

    /// Adds the statistics for 1 simulated run
    pub fn add_run(&mut self, run: RunStats) {
        self.num_runs += 1;
        self.num_games += run.num_games;
        self.wins += run.wins;
        self.losses += run.losses;
        self.pushes += run.pushes;
        self.total_credits += run.remaining_credits;
        if run.remaining_credits > self.starting_credits {
            self.num_walk_away_with_more += 1;
        }
    }
}

impl fmt::Display for TotalRunStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Calc derived stats
        let win_percent = 100f64 * (self.wins as f64 / self.num_games as f64);
        let loss_percent = 100f64 * (self.losses as f64 / self.num_games as f64);
        let push_percent = 100f64 * (self.pushes as f64 / self.num_games as f64);
        let avg_credits = self.total_credits as f64 / self.num_runs as f64;

        // Display stats
        writeln!(
            f,
            "Total Runs: {} | Total Games: {} | W/L/P %: {:.2}%/{:.2}%/{:.2}%",
            self.num_runs, self.num_games, win_percent, loss_percent, push_percent,
        )
        .expect("I/O Error");
        writeln!(
            f,
            "Avg ending amount: ${:.2} | Walking away with winnings: {} times",
            avg_credits, self.num_walk_away_with_more,
        )
        .expect("I/O Error");
        Ok(())
    }
}
