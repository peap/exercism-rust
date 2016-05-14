#[derive(Debug, PartialEq)]
pub struct DeoxyribonucleicAcid {
    sequence: String,
}

#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid {
    sequence: String,
}

impl DeoxyribonucleicAcid {
    pub fn new<S: Into<String>>(sequence: S) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { sequence: sequence.into() }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        let transcribed: String = self.sequence.chars().map(transcriber).collect();
        RibonucleicAcid::new(transcribed)
    }
}

impl RibonucleicAcid {
    pub fn new<S: Into<String>>(sequence: S) -> RibonucleicAcid {
        RibonucleicAcid { sequence: sequence.into() }
    }
}

fn transcriber(c: char) -> char {
    match c {
        'G' => 'C',
        'C' => 'G',
        'T' => 'A',
        'A' => 'U',
        _   => ' ',
    }
}
