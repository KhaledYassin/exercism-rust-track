use itertools::Itertools;
use std::collections::{BTreeMap, HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let both_sides = input.split("==").map(|side| side.trim()).collect_vec();

    if both_sides.len() != 2 {
        return None;
    }

    let lhs = both_sides[0];
    let rhs = both_sides[1];

    let lhs_words = lhs
        .split("+")
        .into_iter()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();

    let rhs_words = vec![rhs.as_bytes()];

    let lhs_factors = get_factors_by_letters(&lhs_words);
    let rhs_factors = get_factors_by_letters(&rhs_words);

    let mut characters = lhs
        .to_owned()
        .chars()
        .into_iter()
        .chain(rhs.to_owned().chars().into_iter())
        .filter(|char| char.is_alphabetic())
        .map(|char| char as u8)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect_vec();

    characters.sort_unstable();

    let number_of_chars = characters.len();

    let first_positions = get_positions_of_first_chars(lhs, rhs, &characters);

    for permutation in (0_u8..10).into_iter().permutations(number_of_chars) {
        if permutation[first_positions.0] == 0 || permutation[first_positions.1] == 0 {
            continue;
        }

        let character_map = zip_permutation(&characters, &permutation);

        let left_side = translate_phrase(&lhs_factors, &character_map);
        let right_side = translate_phrase(&rhs_factors, &character_map);

        if left_side == right_side {
            return Some(
                character_map
                    .into_iter()
                    .map(|(char, int)| (*char as char, *int))
                    .collect(),
            );
        }
    }

    None
}

fn get_positions_of_first_chars(lhs: &str, rhs: &str, characters: &Vec<u8>) -> (usize, usize) {
    characters
        .iter()
        .position(|c| *c == lhs.chars().next().unwrap() as u8)
        .zip(
            characters
                .iter()
                .position(|c| *c == rhs.chars().next().unwrap() as u8),
        )
        .unwrap()
}

fn zip_permutation<'a>(characters: &'a Vec<u8>, integers: &'a Vec<u8>) -> HashMap<&'a u8, &'a u8> {
    characters.iter().zip(integers).collect()
}

fn translate_phrase<'a>(
    character_factors: &'a BTreeMap<&'a u8, u64>,
    character_map: &'a HashMap<&'a u8, &'a u8>,
) -> u64 {
    let mut result = 0;

    character_factors.iter().for_each(|(char, factor)| {
        let int = **character_map.get(char).unwrap();
        result += factor * int as u64;
    });

    result
}

pub fn get_factors_by_letters<'a>(words: &'a Vec<&[u8]>) -> BTreeMap<&'a u8, u64> {
    let mut factors = BTreeMap::new();

    words.iter().for_each(|word| {
        let word_length = word.len();
        word.iter().enumerate().for_each(|(i, c)| {
            let score = 10_u64.pow((word_length - i - 1) as u32);
            *factors.entry(c).or_default() += score;
        })
    });

    factors
}
