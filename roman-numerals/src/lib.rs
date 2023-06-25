use std::fmt::{Display, Formatter, Result};

const ROMAN_MAP: [(u32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];
pub struct Roman {
    num: u32,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut num = self.num;

        let mut roman = String::new();

        for &(value, symbol) in ROMAN_MAP.iter() {
            while num >= value {
                roman.push_str(symbol);
                num -= value;
            }
        }

        write!(f, "{}", roman)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman { num }
    }
}
