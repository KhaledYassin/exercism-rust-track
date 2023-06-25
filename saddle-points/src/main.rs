use saddle_points as sd;
fn main() {
    let input = vec![vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]];
    let result = sd::find_saddle_points(&input);

    println!("{:?}", result)
}