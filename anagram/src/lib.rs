use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut results = HashSet::new();

    let word_lower = word.to_lowercase();
    let mut word_sorted: Vec<char> = word_lower.chars().collect();
    word_sorted.sort_unstable();

    for possible in possible_anagrams {
        let possible_lower = possible.to_lowercase();
        let mut possible_sorted: Vec<char> = possible_lower.chars().collect();
        possible_sorted.sort_unstable();

        if possible_sorted == word_sorted && possible_lower != word_lower {
            results.insert(*possible);
        }
    }

    results
}
