use lazy_static::lazy_static;
use std::collections::HashMap;

const STDUENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

lazy_static! {
    static ref PLANTS: HashMap<char, &'static str> = {
        let mut map = HashMap::new();
        map.insert('V', "violets");
        map.insert('R', "radishes");
        map.insert('C', "clover");
        map.insert('G', "grass");
        map
    };
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut diagram_rows = diagram.split_terminator('\n');

    let index = STDUENTS
        .iter()
        .position(|&c| c == student)
        .unwrap_or_else(|| panic!("Invalid student name: {}", student));

    let mut plants: Vec<&str> = Vec::new();

    let row1 = diagram_rows
        .next()
        .unwrap_or_else(|| panic!("First diagram row is not present!"))
        .chars()
        .collect::<Vec<char>>();

    let row2 = diagram_rows
        .next()
        .unwrap_or_else(|| panic!("Second diagram row is not present!"))
        .chars()
        .collect::<Vec<char>>();

    let column = index * 2;

    let slice1 = &row1[column..column + 2];
    let slice2 = &row2[column..column + 2];

    for c in slice1.iter().chain(slice2) {
        plants.push(PLANTS.get(c).unwrap());
    }

    plants
}
