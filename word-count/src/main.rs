use regex::Regex;
use std::collections::HashMap;

fn count_words(s: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    let re = Regex::new(r"[^a-zA-Z0-9']+").unwrap();
    let tokens: Vec<&str> = s.split(|c| c == ' ' || c == '"' || c == '\'').collect();
    let mut inside_quotes = false;
    for token in tokens {
        if token.is_empty() {
            continue;
        }
        if token == "\"" {
            inside_quotes = !inside_quotes;
            continue;
        }
        let cleaned_token = re.replace_all(token, "").to_string().to_lowercase();
        if !cleaned_token.is_empty() && !inside_quotes {
            *counts.entry(cleaned_token).or_insert(0) += 1;
        }
    }
    counts
}

fn main() {
    let s = "The quick 'brown fox' jumps over the 'lazy dog's' tail. 'Hello, world!' she exclaimed. 12345";
    let word_counts = count_words(s);
    for (word, count) in word_counts.iter() {
        println!("Word '{}' occurs {} times", word, count);
    }
}
