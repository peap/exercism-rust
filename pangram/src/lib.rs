use std::collections::HashSet;

const A: u8 = 'a' as u8;
const Z: u8 = 'z' as u8;

pub fn is_pangram<'a>(sentence: &'a str) -> bool {
    let mut expected = HashSet::new();
    let mut found = HashSet::new();
    for c in A..(Z + 1) {
        expected.insert(c as char);
    }
    for chr in sentence.chars() {
        if let Some(c) = chr.to_lowercase().next() {
            if c as u8 >= A && c as u8 <= Z {
                found.insert(c);
            }
        };
    }
    found == expected
}
