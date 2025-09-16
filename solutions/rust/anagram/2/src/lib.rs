use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    /// Given a string that is already lowercased, return its canonical sorted
    /// `Vec<char>` form.
    fn sorted_chars_from_lowered(s: &str) -> Vec<char> {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        chars
    }

    // Lowercase the target word once (Unicode-aware).
    let word_lower = word.to_lowercase();
    let word_canon = sorted_chars_from_lowered(&word_lower);

    let mut results: HashSet<&'a str> = HashSet::new();
    for &candidate in possible_anagrams.iter() {
        // Lowercase the candidate once for comparisons.
        let candidate_lower = candidate.to_lowercase();

        // Skip candidates that are identical to the target in a case-insensitive way.
        if candidate_lower == word_lower {
            continue;
        }

        // If the canonical (sorted) forms match, it's an anagram.
        if sorted_chars_from_lowered(&candidate_lower) == word_canon {
            results.insert(candidate);
        }
    }

    results
}
