use std::cmp::max;
use std::cmp::min;

const MINE: u8 = 42;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len() as i32;
    let mut result = Vec::<String>::new();

    for i in 0..rows {
        let row = minefield[i as usize].as_bytes();
        let columns = row.len() as i32;
        let mut line = String::new();
        for j in 0..columns {
            let mut input = '*';
            if row[j as usize] != MINE {
                let mut adjacent_bombs: u8 = 0;

                for x in max(i - 1, 0)..=min(i + 1, rows - 1) {
                    let square = minefield[x as usize].as_bytes();
                    for y in max(j - 1, 0)..=min(j + 1, columns - 1) {
                        if square[y as usize] == MINE {
                            adjacent_bombs += 1;
                        }
                    }
                }

                input = if adjacent_bombs > 0 {
                    char::from_digit(adjacent_bombs.into(), 10).unwrap()
                } else {
                    ' '
                };
            }
            line.push(input);
        }
        result.push(line);
    }

    result
}
