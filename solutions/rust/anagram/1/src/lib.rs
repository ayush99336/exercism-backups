use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    // Normalize the string for case-insensitive comparison:
    // - Lowercase (Unicode-aware via std::string::ToString::to_lowercase)
    // - Collect chars and sort them so anagrams share the same canonical form.
    fn canonical(s: &str) -> Vec<char> {
        let lowered = s.to_lowercase();
        let mut chars: Vec<char> = lowered.chars().collect();
        chars.sort_unstable();
        chars
    }

    let word_canon = canonical(word);

    let mut results: HashSet<&'a str> = HashSet::new();
    for &candidate in possible_anagrams.iter() {
        // Skip candidates that are identical to `word` in a case-insensitive sense.
        // (This mirrors the typical anagram definition used by exercise tracks.)
        if candidate.to_lowercase() == word.to_lowercase() {
            continue;
        }

        if canonical(candidate) == word_canon {
            results.insert(candidate);
        }
    }

    results
}
