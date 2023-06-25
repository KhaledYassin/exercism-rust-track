use itertools::Itertools;

const M: i32 = 25;
const ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut result = String::new();
    for chunk in &plain
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
        .chunks(5)
    {
        for c in chunk {
            if c.is_alphabetic() {
                let x = M - roman_index(c) as i32;
                result.push(ASCII_LOWER[x as usize])
            } else {
                result.push(c);
            }
        }
        result.push(' ')
    }

    result.trim().to_owned()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut result = String::new();
    for c in cipher
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
    {
        if c.is_alphabetic() {
            let y = M - roman_index(c) as i32;
            result.push(ASCII_LOWER[y as usize])
        } else {
            result.push(c);
        }
    }

    result
}

fn roman_index(c: char) -> usize {
    c as usize - 'a' as usize
}
