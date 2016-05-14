fn is_anagram_of(word: &str, possible_anagram: &str) -> bool {
    if word.len() != possible_anagram.len() {
        return false;
    }
    if word == possible_anagram {
        return false;
    }
    if word.to_lowercase() == possible_anagram.to_lowercase() {
        return false;
    }
    let mut word_chars: Vec<char> = word.chars().collect();
    for letter in possible_anagram.chars() {
        let compare_fn = |c: &char| {
            if let Some(letter_lower) = letter.to_lowercase().next() {
                if let Some(c_lower) = c.to_lowercase().next() {
                    return letter_lower == c_lower
                }
            }
            letter == *c
        };
        match word_chars.iter().position(compare_fn) {
            Some(index) => {
                word_chars.remove(index);
            }
            None => return false,
        }
    }
    word_chars.len() == 0
}

pub fn anagrams_for<'a>(word: &'a str, inputs: &[&'a str]) -> Vec<&'a str> {
    inputs.into_iter().filter(|input| is_anagram_of(word, input)).cloned().collect()
}
