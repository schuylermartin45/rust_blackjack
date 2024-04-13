//!
//! File:           deck.rs
//! Description:    Describes a deck of cards
//!

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

use crate::types::card::Card;
use crate::types::card::Rank;
use crate::types::card::Suit;

const SIZE_OF_DECK: usize = 52;

/// Represents a virtual deck of cards
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Constructs a new deck, containing all 52 cards, shuffled
    pub fn new() -> Self {
        let mut deck = Deck {
            cards: Vec::with_capacity(SIZE_OF_DECK),
        };

        for s in Suit::iter() {
            for r in Rank::iter() {
                deck.cards.push(Card { suit: *s, rank: *r });
            }
        }
        deck.shuffle();

        deck
    }

    /// Randomly shuffles cards in a deck. According to the internet, most
    /// digital variants of card games shuffle on each hand.
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    /// Deals 1 card
    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in self.cards.iter() {
            writeln!(f, "{}", card).expect("I/O Error");
        }
        Ok(())
    }
}
