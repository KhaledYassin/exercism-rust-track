use regex::Regex;
use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut word_count = HashMap::new();
    let re = Regex::new(r"[^a-zA-Z0-9']+").unwrap();
    let cleaned_phrase = re.replace_all(words, " ");

    for word in cleaned_phrase.split_whitespace() {
        let mut lower_case_word = word.to_ascii_lowercase();

        // Check if a word is inside single quotes or double quotes
        if (lower_case_word.starts_with('\'') && lower_case_word.ends_with('\''))
            || (lower_case_word.starts_with('\"') && lower_case_word.ends_with('\"'))
        {
            // Get the word within quotes
            lower_case_word = lower_case_word[1..lower_case_word.len() - 1].to_string()
        }

        *word_count.entry(lower_case_word).or_insert(0) += 1;
    }

    word_count
}
