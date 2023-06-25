pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - b'A') as usize;
    let mut output = vec![];
    for i in 0..=n {
        let spaces = " ".repeat(n - i);
        if i == 0 {
            output.push(format!("{}A{}", spaces, spaces));
        } else {
            let ch = (b'A' + i as u8) as char;
            let inner_spaces = " ".repeat(i * 2 - 1);
            output.push(format!("{}{}{}{}{}", spaces, ch, inner_spaces, ch, spaces));
        }
    }
    for i in (0..n).rev() {
        let spaces = " ".repeat(n - i);
        if i == 0 {
            output.push(format!("{}A{}", spaces, spaces));
        } else {
            let ch = (b'A' + i as u8) as char;
            let inner_spaces = " ".repeat(i * 2 - 1);
            output.push(format!("{}{}{}{}{}", spaces, ch, inner_spaces, ch, spaces));
        }
    }

    output
}
