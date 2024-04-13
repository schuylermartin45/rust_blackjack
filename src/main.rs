//!
//! File:           main.rs
//! Description:    CLI interface for this project
//!

use crate::types::deck::Deck;
use crate::types::hand::{
    Hand, Strategy, DEALER_INFINITE_CREDITS, HUMAN_DEFAULT_CREDITS, NO_BET_VALUE,
};

pub mod types;

/// Runs a single player text-based game or runs a parallelized simulation.
fn main() {
    // TODO: Parse CLI args to spin up a game or simulation

    let mut deck = Deck::new();
    let mut dealer = Hand::new("Dealer", Strategy::Dealer, DEALER_INFINITE_CREDITS);
    let mut human = Hand::new("Player 1", Strategy::Human, HUMAN_DEFAULT_CREDITS);

    let mut players = Vec::from([&mut human, &mut dealer]);

    for _ in 0..2 {
        for player in players.iter_mut() {
            player.hit(&mut deck, NO_BET_VALUE)
        }
    }

    println!("{}", dealer);
    println!("{}", human);
}
