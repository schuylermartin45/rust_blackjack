use crate::types::card::Rank;

/// Player actions
pub enum Action {
    Hit,
    Stand,
    DoubleDown,
}

/// Determines which move an "optimized" player should make.
/// Based on this strategy: https://m.media-amazon.com/images/I/816DFf5i0EL._SL1500_.jpg
pub fn get_action(val: usize, up_card: Rank) -> Action {
    if val <= 8 {
        return Action::Hit;
    }

    if val == 9 {
        return match up_card {
            Rank::Two => Action::Hit,
            Rank::Seven => Action::Hit,
            Rank::Eight => Action::Hit,
            Rank::Nine => Action::Hit,
            Rank::Ten => Action::Hit,
            Rank::Jack => Action::Hit,
            Rank::Queen => Action::Hit,
            Rank::King => Action::Hit,
            Rank::Ace => Action::Hit,
            _ => Action::DoubleDown,
        };
    }

    if val == 10 {
        return match up_card {
            Rank::Ten => Action::Hit,
            Rank::Jack => Action::Hit,
            Rank::Queen => Action::Hit,
            Rank::King => Action::Hit,
            Rank::Ace => Action::Hit,
            _ => Action::DoubleDown,
        };
    }

    if val == 11 {
        return Action::DoubleDown;
    }

    if val >= 17 {
        return Action::Stand;
    }

    // Between [13, 16]
    match up_card {
        Rank::Two => Action::Stand,
        Rank::Three => Action::Stand,
        Rank::Four => Action::Stand,
        Rank::Five => Action::Stand,
        Rank::Six => Action::Stand,
        Rank::Seven => Action::Hit,
        Rank::Eight => Action::Hit,
        Rank::Nine => Action::Hit,
        Rank::Ten => Action::Hit,
        Rank::Jack => Action::Hit,
        Rank::Queen => Action::Hit,
        Rank::King => Action::Hit,
        Rank::Ace => Action::Hit,
    }
}
