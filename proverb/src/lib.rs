const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];

pub fn build_proverb(list: &[&str]) -> String {
    let length = list.len();
    match length {
        0 => String::new(),
        1 => terminal(list[0]),
        _ => {
            let mut output = (0..length - 1)
                .into_iter()
                .map(|i| line(list[i], list[i + 1]))
                .collect::<Vec<String>>();
            output.push(terminal(list[0]));
            output.join("\n")
        }
    }
}

fn terminal(word: &str) -> String {
    let article = article(word);
    format!("And all for the want of {} {}.", article, word)
}

fn line(word: &str, next_word: &str) -> String {
    let article = article(word);
    format!(
        "For want of {} {} the {} was lost.",
        article, word, next_word
    )
}

fn article(word: &str) -> &str {
    if VOWELS.iter().any(|v| word.starts_with(*v)) {
        "an"
    } else {
        "a"
    }
}
