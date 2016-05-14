use std::slice::Iter;

pub type Score = u8;  // u8 generates one warning due to a test, but still compiles 

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Allergen {
    Eggs         = 0b00000001,
    Peanuts      = 0b00000010,
    Shellfish    = 0b00000100,
    Strawberries = 0b00001000,
    Tomatoes     = 0b00010000,
    Chocolate    = 0b00100000,
    Pollen       = 0b01000000,
    Cats         = 0b10000000,
}

// Couldn't find a built-in way to iterate through all the Allergen values,
// nor cast a Score to an Allergen.
//
// see http://stackoverflow.com/a/21376984
impl Allergen {
    /// Iterator over all Allergen values.
    pub fn iterator() -> Iter<'static, Allergen> {
        use Allergen::*;
        static ALLERGENS: [Allergen; 8] = [
            Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats
        ];
        ALLERGENS.into_iter()
    }
}

#[derive(Debug)]
pub struct Allergies {
    score: Score,
}

impl Allergies {
    pub fn new(score: Score) -> Allergies {
        Allergies { score: score }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies = vec!();
        for allergen in Allergen::iterator() {
            if self.is_allergic_to(&allergen) {
                allergies.push(allergen.clone())
            }
        }
        allergies
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        *allergen as Score & self.score > 0
    }
}
