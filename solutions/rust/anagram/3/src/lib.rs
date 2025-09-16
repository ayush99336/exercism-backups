use std::collections::HashSet;


pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut results: HashSet<&'a str> = HashSet::new();
    let lowered_word = word.to_lowercase();
    let mut word_sorted: Vec<char> = lowered_word.chars().collect();
    word_sorted.sort_unstable();

    for &candidate in possible_anagrams {
        let candidate_lower = candidate.to_lowercase();
        if candidate_lower == lowered_word {
            continue;
        }
        let mut candidate_sorted: Vec<char> = candidate_lower.chars().collect();
        candidate_sorted.sort_unstable();
        if (candidate_sorted == word_sorted) {
            results.insert(candidate);
        }
    }

    results
}

