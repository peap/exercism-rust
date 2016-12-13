use std::convert::From;
use std::fmt;

const NADA: char = 0 as char;

const NUMERALS: [(u32, char, char); 13] = [
    (1000, 'M', NADA),
    (900, 'C', 'M'),
    (500, 'D', NADA),
    (400, 'C', 'D'),
    (100, 'C', NADA),
    (90, 'X', 'C'),
    (50, 'L', NADA),
    (40, 'X', 'L'),
    (10, 'X', NADA),
    (9, 'I', 'X'),
    (5, 'V', NADA),
    (4, 'I', 'V'),
    (1, 'I', NADA),
];

fn divmod(num: u32, base: u32) -> (u32, u32) {
    (num / base, num % base)
}

#[derive(Debug)]
pub struct Roman {
    ordinal: u32,
    numeral: String,
}

impl From<u32> for Roman {
    fn from(int: u32) -> Roman {
        Roman::new(int)
    }
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.numeral)
    }
}

impl Roman {
    fn new(int: u32) -> Roman {
        let mut builder = String::new();
        let mut quotient;
        let mut remainder = int.clone();
        for &(base, letter1, letter2) in NUMERALS.iter() {
            let (q, r) = divmod(remainder, base);
            quotient = q;
            remainder = r;
            if quotient > 0 {
                if letter2 == NADA {
                    for _ in 0..quotient {
                        builder.push(letter1);
                    }
                } else {
                    builder.push(letter1);
                    builder.push(letter2);
                }
            }
        }
        Roman {
            ordinal: int,
            numeral: builder,
        }
    }
}
