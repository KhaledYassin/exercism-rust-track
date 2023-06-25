pub fn count(lines: &[&str]) -> u32 {
    // Find the positions of all '+' characters, which denote corners of rectangles
    let corners: Vec<(usize, usize)> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(move |(x, c)| if is_corner(*c) { Some((x, y)) } else { None })
        })
        .collect();

    corners
        .iter()
        .enumerate()
        .flat_map(|(i, &(x1, y1))| {
            corners
                .iter()
                .skip(i + 1)
                .map(move |&(x2, y2)| (x1, y2, x2, y1))
        })
        .filter(|&(x1, y1, x2, y2)| find_rectangle(lines, x1, y1, x2, y2))
        .count() as u32
}

pub fn count_without_collecting(lines: &[&str]) -> u32 {
    // Find the positions of all '+' characters, which denote corners of rectangles
    let corners = lines.iter().enumerate().flat_map(|(y, line)| {
        line.as_bytes()
            .iter()
            .enumerate()
            .filter_map(move |(x, c)| if is_corner(*c) { Some((x, y)) } else { None })
    });

    let mut count = 0;

    for (x1, y1) in corners {
        for (y2, line) in lines.iter().enumerate().skip(y1 + 1) {
            count += line
                .as_bytes()
                .iter()
                .enumerate()
                .filter(|(x2, h)| is_corner(**h) && find_rectangle(lines, *x2, y2, x1, y1))
                .count();
        }
    }

    count as u32
}

fn find_rectangle(lines: &[&str], x2: usize, y2: usize, x1: usize, y1: usize) -> bool {
    // Check if the pair of corners form a rectangle by looking for '-', '|' and '+' characters along the sides

    let first_line_bytes = lines[y1].as_bytes();
    let second_line_bytes = lines[y2].as_bytes();

    // Check that all vertical sides are '|' or '+'.
    x2 > x1 && y2 > y1 && (y1 + 1..y2).all(|y| {
                is_correct_vertical_line(lines[y].as_bytes(), x1, x2)
            })

            // Check that all horizontal sides are '-' or '+'.
            && is_correct_horizontal_line(first_line_bytes, x1, x2)
            && is_correct_horizontal_line(second_line_bytes, x1, x2)

            // Check that each end of the rectangle is a corner.
            && is_corner(first_line_bytes[x1])
            && is_corner(first_line_bytes[x2])
            && is_corner(second_line_bytes[x1])
            && is_corner(second_line_bytes[x2])
}

fn is_corner(byte: u8) -> bool {
    byte == b'+'
}

fn is_correct_vertical_line(line: &[u8], x1: usize, x2: usize) -> bool {
    let first_edge = line[x1];
    let second_edge = line[x2];
    is_vertical_side(first_edge) && is_vertical_side(second_edge)
}

fn is_vertical_side(byte: u8) -> bool {
    byte == b'|' || byte == b'+'
}

fn is_correct_horizontal_line(line: &[u8], x1: usize, x2: usize) -> bool {
    line[x1..x2].iter().all(|c| is_horizontal_side(*c))
}

fn is_horizontal_side(byte: u8) -> bool {
    byte == b'-' || byte == b'+'
}
