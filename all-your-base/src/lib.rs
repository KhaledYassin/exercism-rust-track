#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    match (from_base, to_base) {
        (_from @ 0..=1, _) => return Err(Error::InvalidInputBase),
        (_, _to @ 0..=1) => return Err(Error::InvalidOutputBase),
        _ => {
            if !number.contains(&from_base) {
                let mut output = vec![];

                let mut quotient = quotient(number, from_base);

                while quotient > 0 {
                    output.push(quotient % to_base);
                    quotient /= to_base
                }

                if output.is_empty() {
                    output.push(0);
                }

                output.reverse();

                Ok(output)
            } else {
                return Err(Error::InvalidDigit(from_base));
            }
        }
    }
}

pub fn quotient(number: &[u32], from_base: u32) -> u32 {
    let length = number.len();
    (0..length).rev().into_iter().fold(0_u32, |mut acc, elem| {
        acc += number[length - elem - 1] * from_base.pow(elem as u32);
        acc
    })
}
