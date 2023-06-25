#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .map(|value| convert_to_bytes(*value))
        .into_iter()
        .flatten()
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut vlq = vec![];
    let mut value = 0_u32;

    for (i, byte) in bytes.iter().enumerate() {
        value <<= 7;
        value |= (byte & 0b01111111) as u32;

        if byte & 0b010000000 == 0 {
            vlq.push(value);
            value = 0;
        } else if i == bytes.len() - 1 {
            return Err(Error::IncompleteNumber);
        }

        if value.leading_zeros() < 7 {
            return Err(Error::Overflow);
        }
    }

    Ok(vlq)
}

fn convert_to_bytes(value: u32) -> Vec<u8> {
    let mut vlq = Vec::new();
    let mut temp = value;

    let mut byte = (temp & 0b01111111) as u8;
    vlq.push(byte);

    temp >>= 7;

    while temp != 0 {
        byte = (temp as u8 & 0b01111111) | 0b010000000;
        vlq.push(byte);
        temp >>= 7;
    }

    vlq.reverse();
    vlq
}
