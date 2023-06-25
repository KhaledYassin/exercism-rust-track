use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    let mut max_product = 0;

    if string_digits.len() >= span {
        if string_digits.is_empty() || span == 0 {
            return Ok(1);
        }

        for window in string_digits.chars().collect_vec().windows(span) {
            let mut product = 1;
            for c in window {
                match c.to_digit(10) {
                    Some(value) => product *= value as u64,
                    None => return Err(Error::InvalidDigit(*c)),
                }
            }

            if product > max_product {
                max_product = product;
            }
        }

        Ok(max_product)
    } else {
        Err(Error::SpanTooLong)
    }
}
