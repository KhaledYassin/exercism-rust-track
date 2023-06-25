use itertools::Itertools;

const M: i32 = 26;
const ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if are_coprimes(a, M) {
        let mut result = String::new();
        for chunk in &plaintext
            .chars()
            .filter(char::is_ascii_alphanumeric)
            .map(|c| c.to_ascii_lowercase())
            .chunks(5)
        {
            for c in chunk {
                if c.is_alphabetic() {
                    let x = roman_index(c) as i32;
                    let e = (a * x + b) % M;
                    result.push(ASCII_LOWER[e as usize])
                } else {
                    result.push(c);
                }
            }

            result.push(' ')
        }

        Ok(result.trim().to_owned())
    } else {
        Err(AffineCipherError::NotCoprime(a))
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    match modular_inverse(a, M) {
        Some(a_inv) => {
            let mut result = String::new();
            for c in ciphertext
                .chars()
                .filter(char::is_ascii_alphanumeric)
                .map(|c| c.to_ascii_lowercase())
            {
                if c.is_alphabetic() {
                    let y = roman_index(c) as i32;
                    let e = ((a_inv * (y - b)) % M + M) % M;
                    result.push(ASCII_LOWER[e as usize])
                } else {
                    result.push(c);
                }
            }

            Ok(result)
        }
        None => Err(AffineCipherError::NotCoprime(a)),
    }
}

fn are_coprimes(a: i32, m: i32) -> bool {
    gcd(a, m) == 1
}

fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn roman_index(c: char) -> usize {
    c as usize - 'a' as usize
}

fn modular_inverse(a: i32, m: i32) -> Option<i32> {
    let (mut old_r, mut r) = (a, m);
    let (mut old_s, mut s) = (1, 0);

    while r != 0 {
        let quotient = old_r / r;

        let tmp_r = r;
        r = old_r - quotient * r;
        old_r = tmp_r;

        let tmp_s = s;
        s = old_s - quotient * s;
        old_s = tmp_s;
    }

    if old_r != 1 {
        return None; // a has no inverse modulo m
    }

    Some(old_s)
}
