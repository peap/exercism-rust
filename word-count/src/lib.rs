use std::collections::HashMap;

pub fn mapper(c: char) -> char {
    if c.is_alphabetic() || c.is_numeric() || c.is_whitespace() {
        c
    } else {
        ' '
    }
}

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut counter = HashMap::new();
    let sanitized: String = s.chars().map(mapper).collect();
    for word in sanitized.split_whitespace() {
        let key = word.to_lowercase();
        let count = counter.entry(key).or_insert(0);
        *count += 1;
    }
    counter
}
