use std::collections::HashMap;

pub type Pairs = Vec<(&'static str, &'static str)>;

pub struct CodonInfo {
    pairs: Pairs,
}

impl CodonInfo {
    pub fn new(pairs: Pairs) -> CodonInfo {
        CodonInfo { pairs: pairs }
    }

    pub fn name_for(&self, codon: &'static str) -> Result<&'static str, &'static str> {
        // let codon_map = HashMap::new();
        // self.pairs.map(|name| ... );  // todo 
        Ok("idk")
    }
}

pub fn parse(pairs: Pairs) -> CodonInfo {
    CodonInfo::new(pairs)
}
