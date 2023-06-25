const ZERO: &str = " _ \n| |\n|_|\n   ";
const ONE: &str = "   \n  |\n  |\n   ";
const TWO: &str = " _ \n _|\n|_ \n   ";
const THREE: &str = " _ \n _|\n _|\n   ";
const FOUR: &str = "   \n|_|\n  |\n   ";
const FIVE: &str = " _ \n|_ \n _|\n   ";
const SIX: &str = " _ \n|_ \n|_|\n   ";
const SEVEN: &str = " _ \n  |\n  |\n   ";
const EIGHT: &str = " _ \n|_|\n|_|\n   ";
const NINE: &str = " _ \n|_|\n _|\n   ";

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let splits = input.split_terminator("\n").collect::<Vec<_>>();

    let rows = splits.len();
    let columns = splits.iter().find(|str| str.len() != 3);

    if rows != 4 {
        return Err(Error::InvalidRowCount(rows));
    } else if columns.is_some() {
        return Err(Error::InvalidColumnCount(columns.unwrap().len()));
    }

    let number = match input {
        ZERO => "0",
        ONE => "1",
        TWO => "2",
        THREE => "3",
        FOUR => "4",
        FIVE => "5",
        SIX => "6",
        SEVEN => "7",
        EIGHT => "8",
        NINE => "9",
        _ => "?",
    };

    Ok(number.to_string())
}
