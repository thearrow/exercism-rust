pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let scores = [
            (Allergen::Cats, 128),
            (Allergen::Pollen, 64),
            (Allergen::Chocolate, 32),
            (Allergen::Tomatoes, 16),
            (Allergen::Strawberries, 8),
            (Allergen::Shellfish, 4),
            (Allergen::Peanuts, 2),
            (Allergen::Eggs, 1),
        ];

        let mut tmp_score = self.score;
        while tmp_score > 255 {
            tmp_score -= 256;
        }
        let mut results: Vec<Allergen> = Vec::new();
        for (allergen, score) in scores {
            let test = tmp_score as i32 - score as i32;
            if test < 0 {
                continue;
            }
            if test >= 0 {
                results.push(allergen);
                tmp_score -= score;
            }
        }
        results
    }
}
