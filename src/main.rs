//!
//! File:           main.rs
//! Description:    CLI interface for this project
//!
use std::io::{self, Write};
use std::{process, thread, time};

use crate::types::deck::Deck;
use crate::types::hand::{
    Hand, Outcome, Strategy, DEALER_INFINITE_CREDITS, DEFAULT_BET_VALUE, HUMAN_DEFAULT_CREDITS,
    NO_BET_VALUE,
};

pub mod types;

/// Runs an interactive sub-menu for controlling bets. Checks against the current credit count.
fn bet_menu(cur_bet: isize, cur_credits: isize) -> isize {
    loop {
        print!(
            "The current bet is ${}. New bet (enter to skip)? $",
            cur_bet
        );
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        // Quit the game from this sub-menu or set the old bet as the current.
        match input.trim().to_lowercase().as_str() {
            "q" | "quit" => process::exit(0),
            "" => return cur_bet,
            _ => (),
        }

        let bet: isize = match input.trim().parse() {
            Ok(b) => b,
            Err(_) => continue,
        };

        match bet {
            b if b > 0 && b <= cur_credits => return bet,
            _ => println!("Invalid bet. Try again."),
        }
    }
}

/// Menu to continue or stop the game. Quits program if the user says no.
fn play_again_menu(human_credits: isize) {
    loop {
        print!("Credits: ${} | Play again? (Y)es | (N)o > ", human_credits);
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        // Quit the game from this sub-menu or set the old bet as the current.
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return,
            "n" | "no" | "q" | "quit" => {
                println!("Cashed out: ${}", human_credits);
                process::exit(0);
            }
            _ => (),
        }
    }
}

/// Runs a single player text-based game or runs a parallelized simulation.
fn main() {
    // TODO: Parse CLI args to spin up a game or simulation

    let mut deck = Deck::new();
    let mut dealer = Hand::new("Dealer", Strategy::Dealer, DEALER_INFINITE_CREDITS);
    let mut human = Hand::new("Player 1", Strategy::Human, HUMAN_DEFAULT_CREDITS);

    // Current bet tracks bets between games for easier user interaction.
    let mut cur_bet: isize = DEFAULT_BET_VALUE;

    let mut game_cntr = 1;
    loop {
        // Deal initial cards
        for _ in 0..2 {
            human.hit(&mut deck);
            dealer.hit(&mut deck);
        }

        // Bet must occur before cards are shown
        cur_bet = bet_menu(cur_bet, human.get_credits());
        human.sub_credits(cur_bet);

        println!("\n---------- Game #{:<4} ----------\n", game_cntr);

        // Final bet is used in betting calculations as it accounts for a player doubling down.
        let mut final_bet = cur_bet;
        loop {
            println!("{}", dealer);
            println!("{}", human);
            let (stop, new_bet) = human.play_once(&mut deck, cur_bet);
            if stop {
                final_bet = new_bet;
                break;
            }
        }
        loop {
            thread::sleep(time::Duration::from_secs(1));
            dealer.show_hand();
            println!("{}", dealer);
            println!("{}", human);
            let (stop, _) = dealer.play_once(&mut deck, NO_BET_VALUE);
            if stop {
                break;
            }
        }

        // TODO determine winner
        let outcome = Outcome::Push;

        match outcome {
            Outcome::Win => human.add_credits(final_bet * 2),
            Outcome::Loss => (),
            Outcome::Push => human.add_credits(final_bet),
        }

        play_again_menu(human.get_credits());
        // If we've gotten to this point, the user has NOT quit, so we must
        // reset for the next round.
        deck = Deck::new();
        human.clear_hand();
        dealer.clear_hand();
        game_cntr += 1;
    }
}
