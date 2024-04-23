//!
//! File:           deck.rs
//! Description:    Describes a deck of cards
//!

use rand::seq::SliceRandom;
use rand::thread_rng;
use rstest::{fixture, rstest};
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

#[fixture]
pub fn deck_fixture() -> Deck {
    Deck::new()
}

/// Validates that the deck decreases in size when cards are dealt.
#[rstest]
fn display_cards(mut deck_fixture: Deck) {
    deck_fixture.deal();
    assert_eq!(deck_fixture.cards.len(), 51)
}

/// Dealing past the size of the deck should return `None`
#[rstest]
fn deal_empty_deck(mut deck_fixture: Deck) {
    for _ in 0..52 {
        assert!(deck_fixture.deal().is_some());
    }
    assert!(deck_fixture.deal().is_none());
}
