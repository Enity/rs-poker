use rs_poker::core::{Deck, Hand, Rank, Rankable};
use std::convert::TryFrom;

fn main() {
    let mut deck = Deck::default();

    let mut hand = Hand::from(deck.get_random_cards(5).unwrap());
    let mut finished = false;
    let mut i = 0;
    while !finished {
        match hand.rank() {
            Rank::StraightFlush(n) => {
                println!("FINDED in {} {}", i, hand);
                finished = true;
            },
            _ => {
                i += 1;
                deck = Deck::default();
                hand = Hand::from(deck.get_random_cards(5).unwrap());
            }
        }
    }
}