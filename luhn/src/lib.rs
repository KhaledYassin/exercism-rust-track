/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let luhn_text = code.replace(' ', "");

    if luhn_text.len() < 2 || !luhn_text.chars().all(char::is_numeric) {
        return false;
    }

    luhn_text
        .chars()
        .rev()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(|(i, n)| if i % 2 == 1 { double(n) } else { n })
        .sum::<u32>()
        % 10
        == 0
}

fn double(n: u32) -> u32 {
    if n * 2 > 9 {
        n * 2 - 9
    } else {
        n * 2
    }
}
