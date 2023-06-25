const DELTAS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    let length = size as isize;

    let mut matrix = vec![vec![0; size]; size];

    let mut d = 0;
    let mut delta = DELTAS[d % 4];

    let mut i = 0_isize;
    let mut j = 0_isize;

    for n in 1..=(size * size) as u32 {
        matrix[i as usize][j as usize] = n;

        let x = i + delta.0;
        let y = j + delta.1;

        if x < length && y >= 0 && x >= 0 && y < length && matrix[x as usize][y as usize] == 0 {
            i = x;
            j = y;
        } else {
            d += 1;
            delta = DELTAS[d % 4];
            i += delta.0;
            j += delta.1;
        }
    }

    matrix
}
