use std::convert::From;
use std::fmt;

/// 1 -> I
/// 5 -> V
/// 10 -> X
/// 50 -> L
/// 100 -> C
/// 500 -> D
/// 1000 -> M

///
///  1 => I
///  2 => II
///  3 => III
///  4 => IV
///  5 => V
///  6 => VI
///  7 => VII
///  8 => VIII
///  9 => IX
/// 10 => X
/// 20 => XX
/// 40 => XL
/// 50 => L
/// 100 => C
/// 500 => D
/// 1000 => M
///

#[derive(Debug)]
pub struct Roman {
    ordinal: u32,
    roman_numeral: String,
}

impl From<u32> for Roman {
    fn from(int: u32) -> Roman {
        Roman::new(int)
    }
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.roman_numeral)
    }
}

impl Roman {
    fn new(int: u32) -> Roman {
        let mut builder = vec![];
        let mut remainder = int.clone();
        while remainder > 0 {
            if remainder >= 1000 {
                let count = remainder / 1000;
                for _ in 0..count {
                    builder.push("M");   
                }
                remainder -= count * 1000;
            } else if remainder >= 100 {
                let count = remainder / 100;
                let hundreds = match count {
                    1 => "C",
                    2 => "CC",
                    3 => "CCC",
                    4 => "CD",
                    5 => "D",
                    6 => "DC",
                    7 => "DCC",
                    8 => "DCCC",
                    9 => "CM",
                    10 => "M",
                    _ => "?",
                };
                builder.push(hundreds);
                remainder -= count * 100;
            } else if remainder >= 10 {
                let count = remainder / 10;
                let tens = match count {
                    1 => "X",
                    2 => "XX",
                    3 => "XXX",
                    4 => "XL",
                    5 => "L",
                    6 => "LX",
                    7 => "LXX",
                    8 => "LXXX",
                    9 => "XC",
                    10 => "C",
                    _ => "x",
                };
                builder.push(tens);
                remainder -= count * 10;
            } else {
                let ones = match remainder {
                    1 => "I",
                    2 => "II",
                    3 => "III",
                    4 => "IV",
                    5 => "V",
                    6 => "VI",
                    7 => "VII",
                    8 => "VIII",
                    9 => "IX",
                    _ => "x",
                };
                builder.push(ones);
                remainder -= remainder;
            }
        }
        let roman: String = builder.iter().cloned().collect();
        Roman { ordinal: int, roman_numeral: roman }
    }
}
