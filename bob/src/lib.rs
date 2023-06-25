pub fn reply(message: &str) -> &str {
    
    let trimmed_message = message.trim();

    let is_empty = trimmed_message.is_empty();

    if is_empty {
        return "Fine. Be that way!";
    }

    let is_question = trimmed_message.ends_with("?");

    let is_yelling = is_yelling(trimmed_message);

    match (is_yelling, is_question) {
        (false, true) => "Sure.",
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, false) => "Whatever.",
    }
}

fn is_yelling(message: &str) -> bool {
    message.to_uppercase() == message && message.chars().any(|c| c.is_uppercase())
}
