# rust_blackjack
Blackjack game and simulator, used as an opportunity to learn rust.

If run without any arguments, a text-based game of Blackjack starts with simulated betting.

If `RUNS` is provided, the program will run `RUNS` number of simulated games, in parallel and report the final
earnings. The human player uses an optimized strategy using a probability table.

## Usage
```sh
Usage: rust_blackjack [RUNS]

Arguments:
  [RUNS]  Number of simulations to run. A negative value will start a human-playable game [default: -1]

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```

## Screenshots

```
The current bet is $1. New bet (enter to skip)? $2

########## Game #1    ##########

Dealer
  <DOWN CARD>
  3 of Spades

Player 1 (14) | $98
  8 of Clubs
  6 of Clubs

Bet: $2 | (H)it | (S)tay | (Q)uit > h
Dealer
  <DOWN CARD>
  3 of Spades

Player 1 (22) | $98
  8 of Clubs
  6 of Clubs
  8 of Diamonds

Bust!
+++++ Dealer's Turn +++++
Dealer (13) | $-1
  King of Hearts
  3 of Spades

Dealer (15) | $-1
  King of Hearts
  3 of Spades
  2 of Clubs

Dealer (22) | $-1
  King of Hearts
  3 of Spades
  2 of Clubs
  7 of Diamonds

Player 1 (22) | $98
  8 of Clubs
  6 of Clubs
  8 of Diamonds

----- Loser!  -----
Credits: $98 | Play again? (Y)es | (N)o >
```
