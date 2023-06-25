use rand::{thread_rng, Rng};

pub fn encode(key: &str, s: &str) -> Option<String> {
    if is_valid_key(key) {
        let output = s
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let shift =
                    key.chars().nth(i).unwrap_or((b'a' + i as u8) as char) as i8 - b'a' as i8;

                shift_char(c, shift)
            })
            .collect();

        Some(output)
    } else {
        None
    }
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if is_valid_key(key) {
        let output = s
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let shift =
                    key.chars().nth(i).unwrap_or((b'a' + i as u8) as char) as i8 - b'a' as i8;

                shift_char(c, 26 - shift)
            })
            .collect();

        Some(output)
    } else {
        None
    }
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = generate_random_characters(100);
    let encoding = encode(&key, s);

    (key, encoding.unwrap())
}

fn is_valid_key(key: &str) -> bool {
    !key.is_empty()
        && key
            .chars()
            .all(|c| c.is_alphabetic() && c.is_ascii_lowercase())
}

fn shift_char(c: char, shift: i8) -> char {
    let base = b'a';
    let offset = (c as u8 - base) as i8;
    let shifted_offset = (offset + shift + 26) % 26;

    (base + shifted_offset as u8) as char
}

fn generate_random_characters(length: usize) -> String {
    let mut rng = thread_rng();
    let chars: String = (0..length)
        .map(|_| rng.gen_range(0..26) + b'a')
        .map(|c| c as char)
        .collect();
    chars
}
