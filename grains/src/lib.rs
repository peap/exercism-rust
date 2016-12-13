const TWO: u64 = 2;
const MIN: u32 = 1;
const MAX: u32 = 64;

pub fn square(s: u32) -> u64 {
    if s < MIN || s > MAX {
        panic!("Square must be between 1 and 64");
    }
    TWO.pow(s - 1)
}

pub fn total() -> u64 {
    (MIN..MAX + 1).fold(0, |acc, s| acc + square(s))
}
