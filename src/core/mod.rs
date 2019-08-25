//! This is the core module.

/// card.rs has value and suit.
mod card;
/// Re-export Card, Value, and Suit
pub use self::card::{Card, Suit, Value};


/// Code related to cards in hands.
mod hand;
/// Everything in there should be public.
pub use self::hand::*;

/// We want to be able to iterate over five card hands.
mod card_iter;
/// Make that functionality public.
pub use self::card_iter::*;

mod deck;
pub use self::deck::{Deck};

/// 5 Card hand ranking code.
mod rank;
/// Export the trait and the results.
pub use self::rank::{Rank, Rankable};

pub mod errors;