pub fn number(user_number: &str) -> Option<String> {
    let numbers: String = user_number
        .chars()
        .filter(|c| c.is_numeric())
        .skip_while(|c| c == &'1')
        .enumerate()
        .filter_map(|(i, c)| match i {
            0 | 3 => {
                if c != '0' && c != '1' {
                    Some(c)
                } else {
                    None
                }
            }
            _ => Some(c),
        })
        .collect();

    if numbers.len() == 10 {
        Some(numbers)
    } else {
        None
    }
}
