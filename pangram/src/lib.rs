use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<HashSet<_>>()
        .len()
        == 26
}
