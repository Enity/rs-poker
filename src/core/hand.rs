use super::errors::StrParseErr;
use crate::core::card::*;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt;
use std::ops::Index;
use std::ops::{RangeFrom, RangeFull, RangeTo};
use std::slice::Iter;

/// Struct to hold cards.
///
/// This doesn't have the ability to easily check if a card is
/// in the hand. So do that before adding/removing a card.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Hand {
    /// Where all the cards are placed un-ordered.
    cards: Vec<Card>,
}

impl Hand {
    /// Create the default empty hand.
    pub fn default() -> Hand {
        Hand {
            cards: Vec::with_capacity(5),
        }
    }
    /// Create the hand with specific hand.
    pub fn new_with_cards(cards: Vec<Card>) -> Hand {
        Hand { cards }
    }

    /// Add card at to the hand.
    /// No verification is done at all.
    pub fn push(&mut self, c: Card) {
        self.cards.push(c);
    }
    /// Truncate the hand to the given number of cards.
    pub fn truncate(&mut self, len: usize) {
        self.cards.truncate(len)
    }
    /// How many cards are in this hand so far ?
    pub fn len(&self) -> usize {
        self.cards.len()
    }
    /// Are there any cards at all ?
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
    /// Create an iter on the cards.
    pub fn iter(&self) -> Iter<Card> {
        self.cards.iter()
    }
}

/// Allow indexing into the hand.
impl Index<usize> for Hand {
    type Output = Card;
    fn index(&self, index: usize) -> &Card {
        &self.cards[index]
    }
}

/// Allow the index to get refernce to every card.
impl Index<RangeFull> for Hand {
    type Output = [Card];
    fn index(&self, range: RangeFull) -> &[Card] {
        &self.cards[range]
    }
}

impl Index<RangeTo<usize>> for Hand {
    type Output = [Card];
    fn index(&self, index: RangeTo<usize>) -> &[Card] {
        &self.cards[index]
    }
}

impl Index<RangeFrom<usize>> for Hand {
    type Output = [Card];
    fn index(&self, index: RangeFrom<usize>) -> &[Card] {
        &self.cards[index]
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let represent = self
            .cards
            .iter()
            .map(|c| format!("{}", c))
            .collect::<String>();
        write!(f, "{}", represent)
    }
}

impl TryFrom<&str> for Hand {
    type Error = StrParseErr;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Ok(Hand{
                cards: vec![]
            })
        }
        if s.len() < 2 || s.len() % 2 != 0 {
            return Err(StrParseErr);
        }
        let groups = s
            .as_bytes()
            .chunks(2)
            .map(|s| unsafe { std::str::from_utf8_unchecked(s) })
            .collect::<Vec<_>>();

        let mut cards: HashSet<Card> = HashSet::with_capacity(7);
        for g in groups.into_iter() {
            let card = Card::try_from(g)?;
            if !cards.insert(card) {
                return Err(StrParseErr);
            }
        }

        Ok(Hand {
            cards: cards.into_iter().collect(),
        })
    }
}

impl From<Vec<Card>> for Hand {
    fn from(cards: Vec<Card>) -> Self {
        Self { cards }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::card::Card;

    #[test]
    fn test_add_card() {
        let mut h = Hand::default();
        let c = Card {
            value: Value::Three,
            suit: Suit::Spade,
        };
        h.push(c);
        // Make sure that the card was added to the vec.
        //
        // This will also test that has len works
        assert_eq!(1, h.len());
    }

    #[test]
    fn test_index() {
        let mut h = Hand::default();
        h.push(Card {
            value: Value::Four,
            suit: Suit::Spade,
        });
        // Make sure the card is there
        assert_eq!(
            Card {
                value: Value::Four,
                suit: Suit::Spade,
            },
            h[0]
        );
    }
    #[test]
    fn test_parse_error() {
        assert!(Hand::try_from("BAD").is_err());
        assert!(Hand::try_from("Adx").is_err());
    }

    #[test]
    fn test_parse_one_hand() {
        let h = Hand::try_from("Ad").unwrap();
        assert_eq!(1, h.len())
    }
    #[test]
    fn test_parse_empty() {
        let h = Hand::try_from("").unwrap();
        assert!(h.is_empty());
    }
}
