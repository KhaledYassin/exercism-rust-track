use nth_prime::nth;

fn main() {
    let n = 10_000;
    let result = nth(n);
    println!("{}", result);
}