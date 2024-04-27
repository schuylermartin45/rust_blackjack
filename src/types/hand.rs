//!
//! File:           hand.rs
//! Description:    Describes a hand of cards (either a dealer or player)
//!
use rstest::rstest;
use std::io::{self, Write};
use std::{fmt, process, usize};

use crate::data::probability_table::{get_action, Action};
use crate::types::card::{Card, Rank, Suit, MAX_BLACKJACK};
use crate::types::deck::Deck;

/// Represents the dealer's "infinite" money pile
pub const DEALER_INFINITE_CREDITS: isize = -1;
/// Default number of credits a human starts with
pub const HUMAN_DEFAULT_CREDITS: isize = 100;
/// Default bet size
pub const DEFAULT_BET_VALUE: isize = 1;
/// Used for the dealer who does not "bet" and to initialize hands.
pub const NO_BET_VALUE: isize = 0;
/// Dealer's do not deal to themselves past this value
pub const DEALER_HAND_THRESHOLD: usize = 17;
/// The maximum number of cards one could have before going bust.
pub const MAX_HAND_CARD_COUNT: usize = 11;

/// The dealer's first card is the down card
pub const DOWN_CARD_IDX: usize = 0;
/// The dealer's second card is the up card.
pub const UP_CARD_IDX: usize = 1;

/// Minimum value allowed for doubling down (virtual BlackJack rules)
pub const DD_MIN: usize = 9;
/// Maximum value allowed for doubling down (virtual BlackJack rules)
pub const DD_MAX: usize = 11;

/// Describes the player role/strategy
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Strategy {
    Dealer,
    Human,
    ProbabilityTable,
}

/// Describes the final result of a round (from the player's perspective).
#[derive(Debug, Eq, PartialEq)]
pub enum Outcome {
    Win,
    Loss,
    Push, // Tie
}

/// Describes the value of a hand (handles Ace value options)
pub struct HandValue {
    lo_sum: usize,
    hi_sum: usize,
}
impl fmt::Display for HandValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Don't show the split score if it is redundant or the upper bound is a bust.
        if self.lo_sum == self.hi_sum || self.hi_sum > MAX_BLACKJACK {
            return write!(f, "{}", self.lo_sum);
        }
        write!(f, "{}/{}", self.lo_sum, self.hi_sum)
    }
}

/// Describes a player or dealer's hand
#[derive(Clone)]
pub struct Hand {
    name: String,
    cards: Vec<Card>,
    strategy: Strategy,
    /// Bets should not go negative. Dealers have "infinite" credits.
    credits: isize,
    /// Flag used by the dealer to render the face-down card.
    show_dealer_hand: bool,
}
impl Hand {
    /// Constructs a hand with the first two dealt cards.
    pub fn new(name: &str, strategy: Strategy, credits: isize) -> Self {
        let hand = Hand {
            name: String::from(name),
            cards: Vec::with_capacity(MAX_HAND_CARD_COUNT),
            strategy: strategy,
            credits: credits,
            show_dealer_hand: false,
        };
        hand
    }

    /// Constructs a Hand from a list of cards. Used in unit testing.
    pub fn from_vector(name: &str, strategy: Strategy, vector: Vec<Card>) -> Self {
        let hand = Hand {
            name: String::from(name),
            cards: vector,
            strategy: strategy,
            credits: HUMAN_DEFAULT_CREDITS,
            show_dealer_hand: false,
        };
        hand
    }

    /// Determines the outcome of a game based on the player's hand and the dealer's hand.
    pub fn determine_outcome(player: &Hand, dealer: &Hand) -> Outcome {
        let player_val = player.final_value();
        let dealer_val = dealer.final_value();

        // If the player busts, the dealer automatically wins.
        if player_val > MAX_BLACKJACK {
            return Outcome::Loss;
        }
        // If the dealer busts and you don't (checked above), you win
        if dealer_val > MAX_BLACKJACK {
            return Outcome::Win;
        }
        // If there's a tie, it's a "push"
        if player_val == dealer_val {
            return Outcome::Push;
        }

        // The closest to BlackJack has the lowest diff. The diff must be positive at this point
        // as we have already checked for bust scenarios.
        let player_diff = MAX_BLACKJACK - player_val;
        let dealer_diff = MAX_BLACKJACK - dealer_val;
        if player_diff > dealer_diff {
            return Outcome::Loss;
        }
        Outcome::Win
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

    /// Returns the "final" value of the hand when the round is complete.
    pub fn final_value(&self) -> usize {
        let val = self.value();
        if val.hi_sum <= MAX_BLACKJACK {
            return val.hi_sum;
        }
        val.lo_sum
    }

    /// Inspect the number of credits a player has.
    pub fn get_credits(&self) -> isize {
        self.credits
    }

    /// Adds credits for the user.
    pub fn add_credits(&mut self, to_add: isize) {
        self.credits += to_add;
    }

    /// Subtracts credits from the user.
    pub fn sub_credits(&mut self, to_sub: isize) {
        self.credits -= to_sub;
    }

    /// Deals a card to the hand.
    pub fn hit(&mut self, deck: &mut Deck) {
        let card = deck.deal();
        match card {
            None => panic!("Deck ran out of cards!"),
            Some(c) => self.cards.push(c),
        };
    }

    /// Returns true if doubling down is currently allowed
    pub fn can_double_down(&self, bet: isize) -> bool {
        // You can't double down if you don't have sufficient credits
        if self.credits < bet {
            return false;
        }
        // The exact rules aren't publicized and probably aren't consistent from BlackJack machine to machine or casino
        // to casino.
        let val = self.value().lo_sum;
        val >= DD_MIN && val <= DD_MAX
    }

    /// A double down is a single hit that doubles the bet. Returns the new bet.
    pub fn double_down(&mut self, deck: &mut Deck, bet: isize) -> isize {
        self.sub_credits(bet);
        self.hit(deck);
        2 * bet
    }

    /// Returns the rank of the up card. Can only be used on the dealer.
    pub fn get_up_card_rank(&self) -> Rank {
        if self.strategy != Strategy::Dealer {
            panic!("There is no `up-card` for non-dealer players.")
        }
        self.cards[UP_CARD_IDX].rank
    }

    /// Shows the dealer's full hand when rendered.
    pub fn show_hand(&mut self) {
        self.show_dealer_hand = true;
    }

    /// Clears the hand the player currently has. Does not reset credits or other state.
    pub fn clear_hand(&mut self) {
        self.cards.clear();
        // Reset the dealer's rendering flag
        self.show_dealer_hand = false;
    }

    /// Dealer simulation. Returns true if the dealer stops.
    fn play_dealer(&mut self, deck: &mut Deck) -> bool {
        // Optionally print game moves. Add some delay for human readability.
        let hand_val = self.value();
        // Dealer met the threshold, bust, or got BlackJack
        if hand_val.lo_sum >= DEALER_HAND_THRESHOLD {
            return true;
        }
        // Dealer met the threshold by counting the 1st Ace as 11 without busting.
        if hand_val.hi_sum < MAX_BLACKJACK && hand_val.hi_sum >= DEALER_HAND_THRESHOLD {
            return true;
        }
        self.hit(deck);
        false
    }

    /// Perfect use of the probability table simulation. Returns true if the player quit.
    fn play_probability_table(
        &mut self,
        deck: &mut Deck,
        bet: isize,
        up_card: Rank,
    ) -> (bool, isize) {
        match get_action(self.final_value(), up_card) {
            Action::Hit => self.hit(deck),
            Action::DoubleDown => {
                // Can't double down if there are insufficient funds
                if self.can_double_down(bet) {
                    return (true, self.double_down(deck, bet));
                }
                self.hit(deck)
            }
            Action::Stand => return (true, bet),
        }
        (false, bet)
    }

    /// UI for human playable games. Returns true if the player quit.
    fn play_human(&mut self, deck: &mut Deck, bet: isize) -> (bool, isize) {
        // End early if user ran out of money
        if self.get_credits() <= 0 {
            println!("You're out of money! Good say, sir!");
            return (true, bet);
        }
        // Auto-terminates on BlackJack and bust
        {
            let cur_val = self.value();
            if cur_val.lo_sum == MAX_BLACKJACK || cur_val.hi_sum == MAX_BLACKJACK {
                println!("BlackJack!");
                return (true, bet);
            }
            if cur_val.lo_sum > MAX_BLACKJACK {
                println!("Bust!");
                return (true, bet);
            }
        }

        let mut action = String::new();

        // Conditionally enable double down based on total and if there's enough credits.
        if self.can_double_down(bet) {
            print!("Bet: ${} | (H)it | (D)ouble Down | (S)tay | (Q)uit > ", bet);
        } else {
            print!("Bet: ${} | (H)it | (S)tay | (Q)uit > ", bet);
        }
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read user input");

        match action.trim().to_lowercase().as_str() {
            "h" | "hit" => self.hit(deck),
            "d" | "double" | "double down" | "neil breen" if self.can_double_down(bet) => {
                println!("Double down! (Neil would be proud)");
                return (true, self.double_down(deck, bet));
            }
            "s" | "stay" | "stand" => return (true, bet),
            "q" | "quit" => process::exit(0),
            _ => (),
        }
        (false, bet)
    }

    /// Executes 1 play action based on strategy. Returns true if the player stops.
    pub fn play_once(&mut self, deck: &mut Deck, bet: isize, up_card: Rank) -> (bool, isize) {
        match self.strategy {
            Strategy::Dealer => (self.play_dealer(deck), NO_BET_VALUE),
            Strategy::ProbabilityTable => self.play_probability_table(deck, bet, up_card),
            Strategy::Human => self.play_human(deck, bet),
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.strategy {
            Strategy::Dealer if !self.show_dealer_hand => {
                writeln!(f, "{}", self.name).expect("I/O Error");
                for (i, card) in self.cards.iter().enumerate() {
                    if i == DOWN_CARD_IDX {
                        writeln!(f, "  <DOWN CARD>").expect("I/O Error");
                        continue;
                    }
                    writeln!(f, "  {}", card).expect("I/O Error");
                }
                Ok(())
            }
            _ => {
                writeln!(f, "{} ({}) | ${}", self.name, self.value(), self.credits)
                    .expect("I/O Error");
                for card in self.cards.iter() {
                    writeln!(f, "  {}", card).expect("I/O Error");
                }
                Ok(())
            }
        }
    }
}

/// Basic game simulator that validates expected outcomes of hands that could be dealt.
#[rstest]
// Basic win/loss scenarios
#[case(
    vec![Card{suit: Suit::Clubs, rank: Rank::Jack}, Card{suit: Suit::Clubs, rank: Rank::Eight}],
    vec![Card{suit: Suit::Diamonds, rank: Rank::Jack}, Card{suit: Suit::Diamonds, rank: Rank::Seven}],
    Outcome::Win
)]
#[case(
    vec![Card{suit: Suit::Diamonds, rank: Rank::Jack}, Card{suit: Suit::Diamonds, rank: Rank::Seven}],
    vec![Card{suit: Suit::Clubs, rank: Rank::Jack}, Card{suit: Suit::Clubs, rank: Rank::Eight}],
    Outcome::Loss
)]
// Push scenarios
#[case(
    vec![Card{suit: Suit::Clubs, rank: Rank::Jack}],
    vec![Card{suit: Suit::Diamonds, rank: Rank::King}],
    Outcome::Push
)]
#[case(
    vec![Card{suit: Suit::Clubs, rank: Rank::Jack}, Card{suit: Suit::Clubs, rank: Rank::Ace}],
    vec![Card{suit: Suit::Diamonds, rank: Rank::Jack}, Card{suit: Suit::Diamonds, rank: Rank::Ace}],
    Outcome::Push
)]
// 21 with a sum of multiple cards
#[case(
    vec![Card{suit: Suit::Clubs, rank: Rank::Jack}, Card{suit: Suit::Clubs, rank: Rank::Six}, Card{suit: Suit::Clubs, rank: Rank::Five}],
    vec![Card{suit: Suit::Diamonds, rank: Rank::Jack}, Card{suit: Suit::Diamonds, rank: Rank::King}],
    Outcome::Win
)]
#[case(
    vec![Card{suit: Suit::Diamonds, rank: Rank::Jack}, Card{suit: Suit::Diamonds, rank: Rank::King}],
    vec![Card{suit: Suit::Clubs, rank: Rank::Jack}, Card{suit: Suit::Clubs, rank: Rank::Six}, Card{suit: Suit::Clubs, rank: Rank::Five}],
    Outcome::Loss
)]
// 21 with an Ace
#[case(
    vec![Card{suit: Suit::Clubs, rank: Rank::Jack}, Card{suit: Suit::Clubs, rank: Rank::Ace}],
    vec![Card{suit: Suit::Diamonds, rank: Rank::Jack}, Card{suit: Suit::Diamonds, rank: Rank::King}],
    Outcome::Win
)]
#[case(
    vec![Card{suit: Suit::Diamonds, rank: Rank::Jack}, Card{suit: Suit::Diamonds, rank: Rank::King}],
    vec![Card{suit: Suit::Clubs, rank: Rank::Jack}, Card{suit: Suit::Clubs, rank: Rank::Ace}],
    Outcome::Loss
)]
#[case(
    vec![Card{suit: Suit::Clubs, rank: Rank::Jack}, Card{suit: Suit::Clubs, rank: Rank::Ace}],
    vec![Card{suit: Suit::Diamonds, rank: Rank::Jack}, Card{suit: Suit::Diamonds, rank: Rank::Ace}],
    Outcome::Push
)]
// Bust scenarios
#[case(
    vec![Card{suit: Suit::Clubs, rank: Rank::Jack}, Card{suit: Suit::Clubs, rank: Rank::Three}],
    vec![Card{suit: Suit::Diamonds, rank: Rank::Jack}, Card{suit: Suit::Diamonds, rank: Rank::King}, Card{suit: Suit::Diamonds, rank: Rank::Two}],
    Outcome::Win
)]
#[case(
    vec![Card{suit: Suit::Diamonds, rank: Rank::Jack}, Card{suit: Suit::Diamonds, rank: Rank::King}, Card{suit: Suit::Diamonds, rank: Rank::Two}],
    vec![Card{suit: Suit::Clubs, rank: Rank::Jack}, Card{suit: Suit::Clubs, rank: Rank::Three}],
    Outcome::Loss
)]
#[case(
    vec![Card{suit: Suit::Clubs, rank: Rank::Jack}, Card{suit: Suit::Clubs, rank: Rank::King}, Card{suit: Suit::Clubs, rank: Rank::Two}],
    vec![Card{suit: Suit::Diamonds, rank: Rank::Jack}, Card{suit: Suit::Diamonds, rank: Rank::King}, Card{suit: Suit::Diamonds, rank: Rank::Two}],
    Outcome::Loss
)]
fn check_outcome(
    #[case] player_cards: Vec<Card>,
    #[case] dealer_cards: Vec<Card>,
    #[case] expected: Outcome,
) {
    let player = Hand::from_vector("player", Strategy::ProbabilityTable, player_cards);
    let dealer = Hand::from_vector("dealer", Strategy::Dealer, dealer_cards);
    assert_eq!(Hand::determine_outcome(&player, &dealer), expected)
}
