//!
//! File:           hand.rs
//! Description:    Describes a hand of cards (either a dealer or player)
//!

use crate::types::deck::Deck;
use crate::types::card::{Card, Rank, MAX_BLACKJACK};

/// Describes how
enum Strategy {
    Dealer,
    Human,
    ProbabilityTable,
}

/// Describes a player or dealer's hand
struct Hand {
    cards: Vec<Card>,
    strategy: Strategy,
    /// Bets should not go negative
    credits: i32,
}

impl Hand {
    /// Returns the value of the hand as a tuple of options.
    /// The first value is the "low" sum, all Aces as 1.
    /// The second value is the "high sum", with 1 Ace as 11.
    fn value(&self) -> (usize, usize) {
        let mut lo_sum = 0;
        let mut hi_sum = 0;
        for card in self.cards {
            if matches!(card.rank, Rank::Ace) {
                lo_sum += 1;
                if hi_sum + 11 > MAX_BLACKJACK {
                    hi_sum += 1;
                }
                else {
                    hi_sum += 11;
                }
                continue;
            }

            lo_sum += card.rank.value();
            hi_sum += card.rank.value();
        }

        (lo_sum, hi_sum)
    }

    fn bet(&self) -> i32 {

    }

    fn hit(&self, deck: &mut Deck) {

    }

    fn stay(&self) {

    }

    fn double_down(&self, card: Card) -> usize {

    }
}