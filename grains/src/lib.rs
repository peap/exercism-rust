const TWO: u64 = 2;

pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    TWO.pow(s - 1)
}

pub fn total() -> u64 {
    (1..65).fold(0, |acc, s| acc + square(s))
}
