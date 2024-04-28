//!
//! File:           main.rs
//! Description:    CLI interface for this project
//!
use std::io::{self, Write};
use std::{process, thread, time};

use clap::Parser;
use rayon::prelude::*;

use crate::types::deck::Deck;
use crate::types::hand::{
    Hand, Outcome, Strategy, DEALER_INFINITE_CREDITS, DEFAULT_BET_VALUE, HUMAN_DEFAULT_CREDITS,
    NO_BET_VALUE,
};
use crate::types::stats::{RunStats, TotalRunStats};

pub mod data;
pub mod types;

const DEFAULT_MAX_GAMES_PER_RUN: usize = 50;

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "A virtual BlackJack text-based game and simulator with betting."
)]
struct CliArgs {
    /// Number of simulations to run. A negative value will start a human-playable game.
    #[arg(default_value_t=-1)]
    runs: isize,
}

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

/// Initialize a game between a player and a dealer
fn init_game(player: &mut Hand, dealer: &mut Hand, deck: &mut Deck) {
    for _ in 0..2 {
        player.hit(deck);
        dealer.hit(deck);
    }
}

/// Resets a game, providing a new deck of cards to work with
fn reset_game(player: &mut Hand, dealer: &mut Hand) -> Deck {
    player.clear_hand();
    dealer.clear_hand();
    Deck::new()
}

/// Plays a game with the dealer at most `max_games` number of times. Bails early if the player runs out of money.
/// This simulates a single "session" of a player sitting down to play a game.
/// TODO: Add Monte Carlo and other betting strats
/// TODO: Add support for a physical game by re-using the Deck to some degree.
fn run_automated_match(max_games: usize) -> RunStats {
    let mut deck = Deck::new();
    let mut dealer = Hand::new("Dealer", Strategy::Dealer, DEALER_INFINITE_CREDITS);
    let mut player = Hand::new(
        "Auto Player",
        Strategy::ProbabilityTable,
        HUMAN_DEFAULT_CREDITS,
    );

    let mut stats = RunStats::new();

    for _ in 0..max_games {
        init_game(&mut player, &mut dealer, &mut deck);

        let bet = DEFAULT_BET_VALUE;
        player.sub_credits(bet);

        // Player control
        let final_bet: isize;
        loop {
            let (stop, new_bet) = player.play_once(&mut deck, bet, dealer.get_up_card_rank());
            if stop {
                final_bet = new_bet;
                break;
            }
        }

        // Dealer control
        loop {
            let (stop, _) = dealer.play_once(&mut deck, NO_BET_VALUE, dealer.get_up_card_rank());
            if stop {
                break;
            }
        }

        let match_outcome = Hand::determine_outcome(&player, &dealer);
        match match_outcome {
            Outcome::Win => {
                player.add_credits(final_bet * 2);
            }
            Outcome::Loss => (),
            Outcome::Push => {
                player.add_credits(final_bet);
            }
        }
        stats.record_match_end(match_outcome);

        // Broke players can't play
        if player.get_credits() <= 0 {
            break;
        }

        // According to the internet, digital Blackjack machines reset the deck every game instance.
        deck = reset_game(&mut player, &mut dealer);
    }

    stats.record_credits(player.get_credits());
    stats
}

/// Runs a single player text-based game or runs a parallelized simulation.
fn main() {
    let args = CliArgs::parse();

    if args.runs > 0 {
        let mut total_stats = TotalRunStats::new(HUMAN_DEFAULT_CREDITS);
        // Each game is run in a parallel using rayon's `map()` functionality.
        let results: Vec<RunStats> = (0..args.runs)
            .into_par_iter()
            .map(|_| run_automated_match(DEFAULT_MAX_GAMES_PER_RUN))
            .collect();
        for stats in results {
            total_stats.add_run(stats);
        }
        println!("{}", total_stats);
        process::exit(0);
    }

    let mut deck = Deck::new();
    let mut dealer = Hand::new("Dealer", Strategy::Dealer, DEALER_INFINITE_CREDITS);
    let mut human = Hand::new("Player 1", Strategy::Human, HUMAN_DEFAULT_CREDITS);

    // Current bet tracks bets between games for easier user interaction.
    let mut cur_bet: isize = DEFAULT_BET_VALUE;

    let mut game_cntr = 1;
    loop {
        // Deal initial cards
        init_game(&mut human, &mut dealer, &mut deck);

        // Bet must occur before cards are shown
        cur_bet = bet_menu(cur_bet, human.get_credits());
        human.sub_credits(cur_bet);

        println!("\n########## Game #{:<4} ##########\n", game_cntr);

        // Final bet is used in betting calculations as it accounts for a player doubling down.
        let final_bet;
        loop {
            println!("{}", dealer);
            println!("{}", human);
            let (stop, new_bet) = human.play_once(&mut deck, cur_bet, dealer.get_up_card_rank());
            if stop {
                final_bet = new_bet;
                break;
            }
        }
        println!("+++++ Dealer's Turn +++++");
        loop {
            thread::sleep(time::Duration::from_secs(1));
            dealer.show_hand();
            println!("{}", dealer);
            let (stop, _) = dealer.play_once(&mut deck, NO_BET_VALUE, dealer.get_up_card_rank());
            if stop {
                break;
            }
        }
        // Reprint the human's hand at the end to visualize the final result.
        println!("{}", human);

        // Determine the outcome and adjust the player's credits.
        match Hand::determine_outcome(&human, &dealer) {
            Outcome::Win => {
                human.add_credits(final_bet * 2);
                println!("----- Winner! -----");
            }
            Outcome::Loss => println!("----- Loser!  -----"),
            Outcome::Push => {
                human.add_credits(final_bet);
                println!("-----  Push.  -----");
            }
        }

        play_again_menu(human.get_credits());
        // If we've gotten to this point, the user has NOT quit, so we must
        // reset for the next round.
        deck = reset_game(&mut human, &mut dealer);
        game_cntr += 1;
    }
}
