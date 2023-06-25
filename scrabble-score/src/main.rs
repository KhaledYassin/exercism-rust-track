use scrabble_score as sc;

fn main() {
    let c = 'ß'.to_string();

    println!("{}", sc::score(&c))
}
