use std::convert::From;
use std::fmt;

#[derive(Debug)]
pub struct Roman {
    ordinal: i32,
    roman_numeral: String,
}

impl From<i32> for Roman {
    fn from(int: i32) -> Roman {
        Roman::new(int)
    }
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.roman_numeral)
    }
}

impl Roman {
    fn new(int: i32) -> Roman {
        Roman { ordinal: int, roman_numeral: "I".to_string() }
    }
}
