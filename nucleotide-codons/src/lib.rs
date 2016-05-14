pub type Pairs = Vec<(&'static str, &'static str)>;

pub struct CodonInfo {
    pairs: Pairs,
}

impl CodonInfo {
    pub fn new(pairs: Pairs) -> CodonInfo {
        CodonInfo { pairs: pairs }
    }

    pub fn name_for(&self, codon: &'static str) -> Result<&'static str, &'static str> {
        // like in the example: https://github.com/exercism/xrust/blob/master/exercises/nucleotide-codons/example.rs
        let decoded: String = codon.chars().map(|letter| {
            match letter {
                'A' | 'W' | 'M' | 'R' | 'D' | 'H' | 'V' | 'N' => 'A',
                'C' | 'S' | 'Y' | 'B' => 'C',
                'G' | 'K' => 'G',
                'T' => 'T',
                _ => ' ',
            }
        }).collect();
        for &(codon, name) in self.pairs.iter() {
            if codon == decoded {
                return Ok(name);
            }
        }
        Err("no match")
    }
}

pub fn parse(pairs: Pairs) -> CodonInfo {
    CodonInfo::new(pairs)
}
