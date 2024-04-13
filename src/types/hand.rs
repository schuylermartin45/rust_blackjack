//!
//! File:           hand.rs
//! Description:    Describes a hand of cards (either a dealer or player)
//!

use std::{fmt, usize};

use crate::types::card::{Card, Rank, MAX_BLACKJACK};
use crate::types::deck::Deck;

/// Represents the dealer's "infinite" money pile
pub const DEALER_INFINITE_CREDITS: isize = -1;
/// Default number of credits a human starts with
pub const HUMAN_DEFAULT_CREDITS: isize = 100;
/// Used for the dealer who does not "bet" and to initialize hands.
pub const NO_BET_VALUE: isize = 0;

/// Describes how
pub enum Strategy {
    Dealer,
    Human,
    ProbabilityTable,
}

/// Describes the value of a hand (handles Ace value options)
pub struct HandValue {
    lo_sum: usize,
    hi_sum: usize,
}
impl fmt::Display for HandValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.lo_sum == self.hi_sum {
            return write!(f, "{}", self.lo_sum);
        }
        write!(f, "{}/{}", self.lo_sum, self.hi_sum)
    }
}

/// Describes a player or dealer's hand
pub struct Hand {
    name: String,
    cards: Vec<Card>,
    strategy: Strategy,
    /// Bets should not go negative. Dealers have "infinite" credits.
    credits: isize,
}
impl Hand {
    /// Constructs a hand with the first two dealt cards.
    pub fn new(name: &str, strategy: Strategy, credits: isize) -> Self {
        let hand = Hand {
            name: String::from(name),
            cards: Vec::new(),
            strategy: strategy,
            credits: credits,
        };
        hand
    }

    /// Returns the value of the hand as a tuple of options.
    /// The first value is the "low" sum, all Aces as 1.
    /// The second value is the "high sum", with 1 Ace as 11.
    pub fn value(&self) -> HandValue {
        let mut lo_sum = 0;
        let mut hi_sum = 0;
        for card in self.cards.iter() {
            if matches!(card.rank, Rank::Ace) {
                lo_sum += 1;
                if hi_sum + 11 > MAX_BLACKJACK {
                    hi_sum += 1;
                } else {
                    hi_sum += 11;
                }
                continue;
            }

            lo_sum += card.rank.value();
            hi_sum += card.rank.value();
        }

        HandValue { lo_sum, hi_sum }
    }

    /// Deals a card to the hand, with a bet
    pub fn hit(&mut self, deck: &mut Deck, bet: isize) {
        let card = deck.deal();
        match card {
            None => panic!("Deck ran out of cards!"),
            Some(c) => self.cards.push(c),
        };
        // TODO handle betting
        if bet == NO_BET_VALUE {
            return;
        }
    }

    pub fn stay(&self) {}

    pub fn double_down(&self, card: Card) -> usize {
        0
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.strategy {
            // TODO handle the final reveal.
            Strategy::Dealer => {
                writeln!(f, "{}", self.name).expect("I/O Error");
                for (i, card) in self.cards.iter().enumerate() {
                    // The second card is the "down" card.
                    if i == 1 {
                        writeln!(f, "  <DOWN CARD>").expect("I/O Error");
                        continue;
                    }
                    writeln!(f, "  {}", card).expect("I/O Error");
                }
                Ok(())
            }
            _ => {
                writeln!(f, "{} ({}):", self.name, self.value()).expect("I/O Error");
                for card in self.cards.iter() {
                    writeln!(f, "  {}", card).expect("I/O Error");
                }
                Ok(())
            }
        }
    }
}
