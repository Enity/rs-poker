use std::fmt;
use std::error;

#[derive(Debug, Clone, PartialEq)]
pub struct StrParseErr;

impl fmt::Display for StrParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing error. Got invalid str")
    }
}

impl error::Error for StrParseErr {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeckIsEmptyErr;

impl fmt::Display for DeckIsEmptyErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No more cards in the deck")
    }
}

impl error::Error for DeckIsEmptyErr {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}