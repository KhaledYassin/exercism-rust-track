use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_lowercase = word.to_lowercase();
    let word_product: u128 = word_lowercase.chars().map(|c| c as u128).product();
    let result: HashSet<&'a str> = possible_anagrams
        .iter()
        .filter(|anagram| {
            let anagram_product: u128 = anagram.to_lowercase().chars().map(|c| c as u128).product();
            anagram_product == word_product
        })
        .filter(|anagram| !anagram.to_lowercase().eq(&word_lowercase))
        .cloned()
        .collect();

    result
}
