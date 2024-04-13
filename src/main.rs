//!
//! File:           main.rs
//! Description:    CLI interface for this project
//!
use std::io::{self, Write};
use std::process;

use crate::types::deck::Deck;
use crate::types::hand::{
    Hand, Strategy, DEALER_INFINITE_CREDITS, HUMAN_DEFAULT_CREDITS, NO_BET_VALUE,
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

/// Runs a single player text-based game or runs a parallelized simulation.
fn main() {
    // TODO: Parse CLI args to spin up a game or simulation

    let mut deck = Deck::new();
    let mut dealer = Hand::new("Dealer", Strategy::Dealer, DEALER_INFINITE_CREDITS);
    let mut human = Hand::new("Player 1", Strategy::Human, HUMAN_DEFAULT_CREDITS);

    let mut players = Vec::from([&mut human, &mut dealer]);

    for _ in 0..2 {
        for player in players.iter_mut() {
            player.hit(&mut deck)
        }
    }

    let mut cur_bet: isize = 1;
    loop {
        // Bet must occur before cards are shown
        cur_bet = bet_menu(cur_bet, human.get_credits());

        // TODO add a game cntr when replays are added
        println!("\n---------- Game # ----------\n");

        // Player action loop
        loop {
            println!("{}", dealer);
            println!("{}", human);

            // End early if user ran out of money
            if human.get_credits() <= 0 {
                println!("You're out of money! Good say, sir!");
                return;
            }

            let mut action = String::new();

            // TODO: conditionally show double down based on total and if there's enough credits.
            print!(
                "Bet: ${} | (H)it | (D)ouble Down | (S)tay | (Q)uit > ",
                cur_bet
            );
            let _ = io::stdout().flush();
            io::stdin()
                .read_line(&mut action)
                .expect("Failed to read user input");

            match action.trim().to_lowercase().as_str() {
                "h" | "hit" => human.hit(&mut deck),
                // TODO optionally enable
                "d" | "double" | "double down" | "neil breen" => {
                    // TODO Fix: Double down carries over to next round
                    cur_bet = human.double_down(&mut deck, cur_bet);
                    break;
                }
                "s" | "stay" | "stand" => break,
                "q" | "quit" => return,
                _ => continue,
            }
        }
        // TODO wrap-up game on stay with dealer logic

        // TODO play again?
    }
}
