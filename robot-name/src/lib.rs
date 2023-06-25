use rand::{thread_rng, Rng};
use std::cell::RefCell;
use std::collections::HashSet;

const ALPHABETS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

thread_local! { static NAMES_USED: RefCell<HashSet<String>> = RefCell::new(HashSet::new()) }

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: generate_unique_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = generate_unique_name()
    }
}

fn generate_unique_name() -> String {
    let mut name = generate_random_name();

    while !is_name_unique(&name) {
        name = generate_random_name();
    }

    name
}

fn generate_random_name() -> String {
    let mut rng = thread_rng();

    let first_letter = ALPHABETS[rng.gen_range(0..26)];
    let second_letter = ALPHABETS[rng.gen_range(0..26)];
    let number = rng.gen_range(0..1000_u32);

    format!("{}{}{:03}", first_letter, second_letter, number)
}

fn is_name_unique(name: &String) -> bool {
    let mut unique = false;
    NAMES_USED.with(|set| unique = set.borrow_mut().insert(name.to_string()));
    unique
}
