use super::errors::{DeckIsEmptyErr};
use super::card::{Card, Value, Suit};
use rand::prelude::*;

pub struct Deck {
    cards: Vec<Card>,
    rand: rand::rngs::ThreadRng
}

impl Deck {
    pub fn get_random_card(&mut self) -> Result<Card, DeckIsEmptyErr> {
        let current_cards_len = self.cards.len();
        if current_cards_len == 0 {
            return Err(DeckIsEmptyErr)
        }
        let index: usize = self.rand.gen_range(0, current_cards_len);
        let card = self.cards.swap_remove(index);
        Ok(card)
    }

    pub fn get_random_cards(&mut self, amount: usize) -> Result<Vec<Card>, DeckIsEmptyErr> {
        if amount > 52 {
            return Err(DeckIsEmptyErr)
        }
        let mut cards = Vec::with_capacity(amount);
        for _ in 0..amount {
            let card = self.get_random_card()?;
            cards.push(card);
        }
        Ok(cards)
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck {
            cards: get_all_cards().to_vec(),
            rand: rand::thread_rng(),
        }
    }
}

const fn get_all_cards() -> [Card; 52] {
    use Suit::*;
    use Value::*;
    [
        Card{value: Two, suit: Heart},Card{value: Three, suit: Heart},Card{value: Four, suit: Heart},
        Card{value: Five, suit: Heart},Card{value: Six, suit: Heart},Card{value: Seven, suit: Heart},
        Card{value: Eight, suit: Heart},Card{value: Nine, suit: Heart},Card{value: Ten, suit: Heart},
        Card{value: Jack, suit: Heart},Card{value: Queen, suit: Heart},Card{value: King, suit: Heart},
        Card{value: Ace, suit: Heart},
        Card{value: Two, suit: Spade},Card{value: Three, suit: Spade},Card{value: Four, suit: Spade},
        Card{value: Five, suit: Spade},Card{value: Six, suit: Spade},Card{value: Seven, suit: Spade},
        Card{value: Eight, suit: Spade},Card{value: Nine, suit: Spade},Card{value: Ten, suit: Spade},
        Card{value: Jack, suit: Spade},Card{value: Queen, suit: Spade},Card{value: King, suit: Spade},
        Card{value: Ace, suit: Spade},
        Card{value: Two, suit: Club},Card{value: Three, suit: Club},Card{value: Four, suit: Club},
        Card{value: Five, suit: Club},Card{value: Six, suit: Club},Card{value: Seven, suit: Club},
        Card{value: Eight, suit: Club},Card{value: Nine, suit: Club},Card{value: Ten, suit: Club},
        Card{value: Jack, suit: Club},Card{value: Queen, suit: Club},Card{value: King, suit: Club},
        Card{value: Ace, suit: Club},
        Card{value: Two, suit: Diamond},Card{value: Three, suit: Diamond},Card{value: Four, suit: Diamond},
        Card{value: Five, suit: Diamond},Card{value: Six, suit: Diamond},Card{value: Seven, suit: Diamond},
        Card{value: Eight, suit: Diamond},Card{value: Nine, suit: Diamond},Card{value: Ten, suit: Diamond},
        Card{value: Jack, suit: Diamond},Card{value: Queen, suit: Diamond},Card{value: King, suit: Diamond},
        Card{value: Ace, suit: Diamond},
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deck_return_err_if_emtpy() {
        let mut deck = Deck::default();
        for _ in 0..51 {
            let _ = deck.get_random_card();
        }
        assert_ne!(deck.get_random_card(), Err(DeckIsEmptyErr));
        assert_eq!(deck.get_random_card(), Err(DeckIsEmptyErr));
    }

}
