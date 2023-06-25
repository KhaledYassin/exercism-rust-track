pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|c| shift_char(c, key)).collect()
}

fn shift_char(c: char, shift: i8) -> char {
    if !c.is_ascii_alphabetic() {
        return c;
    }

    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
    let offset = (c as u8 - base) as i8;
    let shifted_offset = (offset + shift + 26) % 26;
    (base + shifted_offset as u8) as char
}
