pub fn answer(command: &str) -> Option<i32> {
    if let Some(_command_expression) = process_string_command(command) {
        return evaluate_expression(_command_expression);
    }

    None
}

pub fn process_string_command(command: &str) -> Option<String> {
    if command.starts_with("What is") {
        let statement = command.replace("What is", "").replace('?', "");

        if let Some(char) = statement.trim().chars().next() {
            // either next character is numeric or a signed number
            if char.is_numeric() || char == '-' {
                let words_to_symbols = statement
                    .replace("plus", "+")
                    .replace("minus", "-")
                    .replace("divided by", "/")
                    .replace("multiplied by", "*")
                    .replace("raised to the", "**")
                    .replace("th", "")
                    .replace("st", "")
                    .replace("nd", "")
                    .replace("rd", "")
                    .replace("power", "");
                return Some(words_to_symbols.trim().to_string());
            }
        }
    }

    None
}

fn evaluate_expression(expression: String) -> Option<i32> {
    let words = expression.split_whitespace().collect::<Vec<_>>();
    let mut result = None;
    let mut current_operator = None;

    for word in words {
        if let Ok(num) = word.parse::<i32>() {
            if let Some(mut value) = result {
                if let Some(operator) = current_operator {
                    match operator {
                        "+" => value += num,
                        "-" => value -= num,
                        "*" => value *= num,
                        "/" => value /= num,
                        "**" => value = (0..num).into_iter().map(|_| value).product(),
                        _ => return None,
                    }
                    result = Some(value);
                    current_operator = None;
                } else {
                    return None;
                }
            } else {
                result = Some(num);
            }
        } else if current_operator.is_none() {
            match word {
                "+" | "-" | "*" | "/" | "**" => current_operator = Some(word),
                _ => return None,
            }
        } else {
            return None;
        }
    }

    if current_operator.is_some() {
        return None;
    }

    result
}
