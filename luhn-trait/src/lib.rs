pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
// impl<'a> Luhn for &'a str {
//     fn valid_luhn(&self) -> bool {
//         unimplemented!("Determine if '{self}' is a valid credit card number.");
//     }
// }

impl<T> Luhn for T
where
    T: ToString,
{
    fn valid_luhn(&self) -> bool {
        let luhn_text = self.to_string().replace(' ', "");

        if luhn_text.len() < 2 || !luhn_text.chars().all(char::is_numeric) {
            return false;
        }

        luhn_text
            .chars()
            .rev()
            .filter_map(|c| c.to_digit(10))
            .enumerate()
            .map(|(i, n)| match i % 2 {
                0 => n,
                _ if n == 9 => n,
                _ => (n * 2) % 9,
            })
            .sum::<u32>()
            % 10
            == 0
    }
}
