use std::mem;
use std::cmp;
use std::fmt;
use super::errors::{StrParseErr};
use std::convert::{TryFrom};

/// Card rank or value.
/// This is basically the face value - 2
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Value {
    /// 2
    Two = 0,
    /// 3
    Three = 1,
    /// 4
    Four = 2,
    /// 5
    Five = 3,
    /// 6
    Six = 4,
    /// 7
    Seven = 5,
    /// 8
    Eight = 6,
    /// 9
    Nine = 7,
    /// T
    Ten = 8,
    /// J
    Jack = 9,
    /// Q
    Queen = 10,
    /// K
    King = 11,
    /// A
    Ace = 12,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Value::*;
        match *self {
            Two => write!(f, "2"),
            Three => write!(f, "3"),
            Four => write!(f, "4"),
            Five => write!(f, "5"),
            Six => write!(f, "6"),
            Seven => write!(f, "7"),
            Eight => write!(f, "8"),
            Nine => write!(f, "9"),
            Ten => write!(f, "T"),
            Jack => write!(f, "J"),
            Queen => write!(f, "Q"),
            King => write!(f, "K"),
            Ace => write!(f, "A"),
        }
    }
}

impl TryFrom<char> for Value {
    type Error = StrParseErr;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Value::*;
        match c {
            '2' => Ok(Two),
            '3' => Ok(Three),
            '4' => Ok(Four),
            '5' => Ok(Five),
            '6' => Ok(Six),
            '7' => Ok(Seven),
            '8' => Ok(Eight),
            '9' => Ok(Nine),
            'T' => Ok(Ten),
            'J' => Ok(Jack),
            'Q' => Ok(Queen),
            'K' => Ok(King),
            'A' => Ok(Ace),
            _ => Err(StrParseErr),
        }
    }
}

impl From<u8> for Value {
    fn from(v: u8) -> Self {
         unsafe { mem::transmute(cmp::min(v, Value::Ace as u8)) }
    }
}

impl Into<char> for Value {
    fn into(self) -> char {
        use Value::*;
        match self {
            Ace => 'A',
            King => 'K',
            Queen => 'Q',
            Jack => 'J',
            Ten => 'T',
            Nine => '9',
            Eight => '8',
            Seven => '7',
            Six => '6',
            Five => '5',
            Four => '4',
            Three => '3',
            Two => '2',
        }
    }
}

/// Constant of all the values.
/// This is what `Value::values()` returns
const VALUES: [Value; 13] = [
    Value::Two,
    Value::Three,
    Value::Four,
    Value::Five,
    Value::Six,
    Value::Seven,
    Value::Eight,
    Value::Nine,
    Value::Ten,
    Value::Jack,
    Value::Queen,
    Value::King,
    Value::Ace,
];

impl Value {
    /// Take a u32 and convert it to a value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_poker::core::Value;
    /// assert_eq!(Value::Four, Value::from_u8(Value::Four as u8));
    /// ```
    pub fn from_u8(v: u8) -> Value {
        unsafe { mem::transmute(cmp::min(v, Value::Ace as u8)) }
    }
    /// Get all of the `Value`'s that are possible.
    /// This is used to iterate through all possible
    /// values when creating a new deck, or
    /// generating all possible starting hands.
    pub fn values() -> [Value; 13] {
        VALUES
    }

    /// How card ranks seperate the two values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_poker::core::Value;
    /// assert_eq!(1, Value::Ace.gap(Value::King));
    /// ```
    pub fn gap(self, other: Value) -> u8 {
        let min = cmp::min(self as u8, other as u8);
        let max = cmp::max(self as u8, other as u8);
        max - min
    }
}

/// Enum for the four different suits.
/// While this has support for ordering it's not
/// sensical. The sorting is only there to allow sorting cards.
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Suit {
    /// Spades
    Spade = 0,
    /// Clubs
    Club = 1,
    /// Hearts
    Heart = 2,
    /// Diamonds
    Diamond = 3,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Suit::*;
        match *self {
            Heart => write!(f, "♥"),
            Diamond => write!(f, "♦"),
            Spade => write!(f, "♠"),
            Club => write!(f, "♣"),
        }
    }
}

impl TryFrom<char> for Suit {
    type Error = StrParseErr;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Suit::*;
        match c {
            'h' => Ok(Heart),
            's' => Ok(Spade),
            'c' => Ok(Club),
            'd' => Ok(Diamond),
            _ => Err(StrParseErr),
        }
    }
}

/// All of the `Suit`'s. This is what `Suit::suits()` returns.
const SUITS: [Suit; 4] = [Suit::Spade, Suit::Club, Suit::Heart, Suit::Diamond];

/// Impl of Suit
///
/// This is just here to provide a list of all `Suit`'s.
impl Suit {
    /// Provide all the Suit's that there are.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rs_poker::core::Suit;
    /// let suits = Suit::suits();
    /// assert_eq!(4, suits.len());
    /// ```
    pub fn suits() -> [Suit; 4] {
        SUITS
    }
}

/// The main struct of this library.
/// This is a carrier for Suit and Value combined.
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub struct Card {
    /// The face value of this card.
    pub value: Value,
    /// The suit of this card.
    pub suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|{}_{}|", self.value, self.suit)
    }
}

impl TryFrom<&str> for Card {
    type Error = StrParseErr;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 2 {
            return Err(StrParseErr);
        }
        let mut chars = s.chars();
        let value = Value::try_from(chars.next().unwrap())?;
        let suit = Suit::try_from(chars.next().unwrap())?;
        Ok(Card{ suit, value })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_constructor() {
        let c = Card {
            value: Value::Three,
            suit: Suit::Spade,
        };
        assert_eq!(Suit::Spade, c.suit);
        assert_eq!(Value::Three, c.value);
    }

    #[test]
    fn test_compare() {
        let c1 = Card {
            value: Value::Three,
            suit: Suit::Spade,
        };
        let c2 = Card {
            value: Value::Four,
            suit: Suit::Spade,
        };
        let c3 = Card {
            value: Value::Four,
            suit: Suit::Club,
        };

        // Make sure that the values are ordered
        assert!(c1 < c2);
        assert!(c2 > c1);
        // Make sure that suit is used.
        assert!(c3 > c2);
    }

    #[test]
    fn test_value_cmp() {
        assert!(Value::Two < Value::Ace);
        assert!(Value::King < Value::Ace);
        assert_eq!(Value::Two, Value::Two);
    }

    #[test]
    fn test_from_u8() {
        assert_eq!(Value::Two, Value::from_u8(0));
        assert_eq!(Value::Ace, Value::from_u8(12));
    }

    #[test]
    fn test_size_card() {
        // Card should be really small. Hopefully just two u8's
        assert!(mem::size_of::<Card>() <= 2);
    }

    #[test]
    fn test_size_suit() {
        // One byte for Suit
        assert!(mem::size_of::<Suit>() <= 1);
    }

    #[test]
    fn test_size_value() {
        // One byte for Value
        assert!(mem::size_of::<Value>() <= 1);
    }

    #[test]
    fn test_gap() {
        // test on gap
        assert!(1 == Value::Ace.gap(Value::King));
        // test no gap at the high end
        assert!(0 == Value::Ace.gap(Value::Ace));
        // test no gap at the low end
        assert!(0 == Value::Two.gap(Value::Two));
        // Test one gap at the low end
        assert!(1 == Value::Two.gap(Value::Three));
        // test that ordering doesn't matter
        assert!(1 == Value::Three.gap(Value::Two));
        // Test things that are far apart
        assert!(12 == Value::Ace.gap(Value::Two));
        assert!(12 == Value::Two.gap(Value::Ace));
    }
}
