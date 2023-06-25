pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut substrings = Vec::new();

    if len > digits.len() {
        return substrings;
    }

    for i in 0..digits.len() - len + 1 {
        substrings.push(digits[i..i + len].to_string());
    }

    substrings
}
