pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let luhn_text = self.0.replace(' ', "");

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
}

fn double(n: u32) -> u32 {
    if n * 2 > 9 {
        n * 2 - 9
    } else {
        n * 2
    }
}

impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Luhn(input.to_string())
    }
}
