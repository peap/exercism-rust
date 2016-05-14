use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> usize {
    dna.chars().filter(|&c| c == nucleotide).count()
}

pub fn nucleotide_counts(dna: &str) -> HashMap<char, usize> {
    let mut counter = HashMap::new();
    for nucleotide in NUCLEOTIDES.iter() {
        counter.insert(nucleotide.clone(), count(*nucleotide, dna));
    }
    counter
}
