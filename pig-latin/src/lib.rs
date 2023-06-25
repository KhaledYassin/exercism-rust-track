const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn translate(input: &str) -> String {
    let words = input.split_ascii_whitespace();
    words
        .map(|word| translate_to_pig_latin(word))
        .collect::<Vec<_>>()
        .join(" ")
}

fn translate_to_pig_latin(word: &str) -> String {
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();
    let rest_of_word: String = chars.collect();
    let second_char = rest_of_word.chars().next().unwrap();
    if is_vowel(&first_char)
        || ((first_char == 'y' || first_char == 'x') && !is_vowel(&second_char))
    {
        return format!("{}ay", word);
    } else if word.starts_with("qu") {
        return format!("{}{}ay", &word[2..], "qu");
    } else if !is_vowel(&first_char) && rest_of_word.starts_with("qu") {
        return format!(
            "{}{}ay",
            &rest_of_word[2..],
            vec![first_char, 'q', 'u'].iter().collect::<String>()
        );
    } else if !is_vowel(&first_char) && first_char != 'y' {
        let first_vowel = word.chars().find(|c| is_vowel(c) || c == &'y');

        match first_vowel {
            Some(vowel) => {
                if first_char != vowel
                    && rest_of_word.contains(vowel)
                    && !rest_of_word.ends_with(vowel)
                {
                    let position = word.chars().position(|c| c == vowel).unwrap();
                    return format!(
                        "{}{}{}ay",
                        vowel,
                        &rest_of_word[position..],
                        &word[..position]
                    );
                }
            }
            None => return format!("{}{}ay", rest_of_word, first_char),
        }

        let y_position = word.chars().position(|c| c == 'y').unwrap();
        return format!("y{}{}ay", &rest_of_word[y_position..], &word[..y_position]);
    } else {
        return format!("{}{}ay", rest_of_word, first_char);
    }
}

fn is_vowel(c: &char) -> bool {
    VOWELS.contains(c)
}
