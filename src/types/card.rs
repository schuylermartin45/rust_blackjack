//!
//! File:           card.rs
//! Description:    Describes a card
//!

use std::fmt;
use std::slice::Iter;

pub const MAX_BLACKJACK: usize = 21;

/// Enumeration representing the "type" of a card
#[derive(Clone, Copy)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl Suit {
    /// Iterator for traversing all available Suits
    pub fn iter() -> Iter<'static, Suit> {
        static SUITS: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs];
        SUITS.iter()
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match *self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Spades => "Spades",
            Suit::Clubs => "Clubs",
        };
        write!(f, "{}", str)
    }
}

/// Enumeration representing the "value" of a card
#[derive(Clone, Copy)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    /// Returns the numeric value associated to a Rank
    pub fn value(&self) -> usize {
        match *self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ace => 11,
        }
    }

    /// Iterator for traversing all available Ranks
    pub fn iter() -> Iter<'static, Rank> {
        static RANKS: [Rank; 13] = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];
        RANKS.iter()
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match *self {
            Rank::Two
            | Rank::Three
            | Rank::Four
            | Rank::Five
            | Rank::Six
            | Rank::Seven
            | Rank::Eight
            | Rank::Nine
            | Rank::Ten => self.value().to_string(),
            Rank::Jack => String::from("Jack"),
            Rank::Queen => String::from("Queen"),
            Rank::King => String::from("King"),
            Rank::Ace => String::from("Ace"),
        };
        write!(f, "{}", str)
    }
}

/// Represents a card in a deck
#[derive(Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}
