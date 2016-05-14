pub fn hamming_distance(s1: &str, s2: &str) -> Result<i32, &'static str> {
    if s1.len() == s2.len() {
        let count_different_chars = |acc, (c1, c2)| acc + if c1 != c2 { 1 } else { 0 };
        Ok(s1.chars().zip(s2.chars()).fold(0, count_different_chars))
    } else {
        Err("inputs of different length")
    }
}
