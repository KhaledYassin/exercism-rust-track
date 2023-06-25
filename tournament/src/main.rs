use tournament;
fn main() {
    let input = "Allegoric Alaskans;Blithering Badgers;win";

    println!("{}", tournament::tally(input))
}