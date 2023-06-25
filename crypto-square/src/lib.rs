pub fn encrypt(input: &str) -> String {
    let plain = input
        .chars()
        .filter(|&c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>();

    let len = plain.len();
    let col = ((len as f32).sqrt().ceil() as usize).min(len);
    let row = ((len as f32) / (col as f32)).ceil() as usize;

    let mut result = String::new();
    for c in 0..col {
        for r in 0..row {
            let i = r * col + c;
            if let Some(ch) = plain.chars().nth(i) {
                result.push(ch);
            } else {
                result.push(' ');
            }
        }
        if c != col - 1 {
            result.push(' ');
        }
    }

    result
}
