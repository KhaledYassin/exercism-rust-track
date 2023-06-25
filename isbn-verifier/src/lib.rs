/// Determines whether the supplied string is a valid ISBN number

const RADIX: u32 = 10;
const X: char = 'X';

pub fn is_valid_isbn(isbn: &str) -> bool {
    let digits = isbn
        .chars()
        .filter(|c| c == &X || c.is_numeric())
        .map(|c| {
            if c == X {
                10
            } else {
                c.to_digit(RADIX).unwrap()
            }
        })
        .collect::<Vec<_>>();

    if (digits.len() != 10) || (isbn.contains(X) && digits[9] != 10) {
        return false;
    }

    (1..=10)
        .rev()
        .into_iter()
        .zip(digits)
        .fold(0, |acc, (x, y)| acc + x * y)
        % 11
        == 0
}
