use std::collections::HashMap;

const LETTER_SCORES: [(u32, &'static str); 7] = [
    (1, "AEIOULNRST"),
    (2, "DG"),
    (3, "BCMP"),
    (4, "FHVWY"),
    (5, "K"),
    (8, "JX"),
    (10, "QZ"),
];

fn get_letter_scores() -> HashMap<char, u32> {
    let mut scores = HashMap::new();
    for &(score, letters) in LETTER_SCORES.iter() {
        for letter in letters.chars() {
            scores.insert(letter, score);
        }
    }
    scores
}

pub fn score(word: &str) -> u32 {
    let scores = get_letter_scores();
    word.chars().fold(0, |acc, chr| {
        let upper: char = chr.to_uppercase().next().unwrap();
        acc + scores.get(&upper).unwrap_or(&0)
    })
}
