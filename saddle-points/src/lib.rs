pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = vec![];
    let length = input.len();

    for (i, row) in input.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            if row.iter().all(|x| x <= column) && (0..length).all(|y| input[y][j] >= *column) {
                saddle_points.push((i, j))
            }
        }
    }

    saddle_points
}
