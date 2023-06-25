pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let words = phrase.split_whitespace();

    for word in words {
        get_composite_words(word)
            .into_iter()
            .filter(|word| !word.is_empty())
            .for_each(|word| {
                let characters = word.chars().collect::<Vec<_>>();
                let caps = characters
                    .iter()
                    .filter(|ch| ch.is_uppercase())
                    .collect::<Vec<_>>();

                let caps_length = caps.len();
                let all_caps = characters.len() == caps_length;
                let camel_case = !all_caps && caps_length >= 2;

                match (all_caps, camel_case) {
                    // the camel case is the special case
                    (false, true) => caps.into_iter().for_each(|ch| acronym.push(*ch)),
                    _ => acronym.push(characters[0]),
                }
            });
    }
    acronym.to_uppercase()
}

fn get_composite_words(word: &str) -> Vec<&str> {
    let separators = word
        .chars()
        .filter(|ch| !ch.is_alphabetic() && ch != &'\'')
        .collect::<Vec<_>>();
    word.split(&separators[..]).collect()
}
