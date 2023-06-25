fn main() {
    let number = 1234567890;

    println!("{:?}", speak(number))
}

fn speak(number: u64) -> String {
    match number {
        n @ 0..=12 => english_numbers(n),
        _ => {
            let chunks = break_into_chunks(number);
            let length = chunks.len();

            let mut output = vec![];

            for (i, chunk) in chunks.iter().enumerate() {
                let folded = chunk.iter().fold(0, |acc, elem| acc * 10 + elem);
                let magnitude = magnitude((length - i) * 3);

                output.push(english_numbers(folded));
                output.push(magnitude);
            }

            output.join(" ").trim().to_string()
        }
    }
}

fn english_numbers(number: u64) -> String {
    let magnitude = order_of_magnitude(number);
    match number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        n @ 12.. => return teens(n, magnitude as u32),
    }
    .to_string()
}

fn teens(number: u64, magnitude: u32) -> String {
    let teen = "teen".to_string();
    match number {
        0..=12 => english_numbers(number),
        13 => irregulars(3) + &teen,
        14 => english_numbers(4) + &teen,
        15 => irregulars(5) + &teen,
        16 => english_numbers(6) + &teen,
        17 => english_numbers(7) + &teen,
        18 => irregulars(8) + &teen,
        19 => english_numbers(9) + &teen,
        _n @ 19.. => tens(number, magnitude),
    }
}

fn tens(number: u64, magnitude: u32) -> String {
    let ty = "ty".to_string();
    match number {
        0..=19 => english_numbers(number),
        20 => irregulars(2) + &ty,
        30 => irregulars(3) + &ty,
        40 => irregulars(4) + &ty,
        50 => irregulars(5) + &ty,
        80 => irregulars(8) + &ty,
        n @ 100.. => {
            let factor = 10_u64.pow(magnitude);
            let remainder = number % factor;
            let r = english_numbers(remainder);
            format!("{} {} {}", english_numbers(n / factor), "hundred", r)
        }
        _ => {
            let remainder = number % 10;
            if remainder == 0 {
                return english_numbers(number / 10) + &ty;
            }
            let r = english_numbers(remainder);
            format!(
                "{}{}",
                english_numbers(number - remainder),
                if remainder > 0 {
                    "-".to_owned() + &r
                } else {
                    "".to_owned()
                }
            )
        }
    }
}

fn magnitude(number_of_digits: usize) -> String {
    match number_of_digits {
        _n @ 4..=6 => "thousand",
        _n @ 7..=9 => "million",
        _n @ 10..=12 => "billion",
        _n @ 13..=15 => "trillion",
        _n @ 16..=18 => "quadrillion",
        _n @ 19..=21 => "quintillion",
        _ => "",
    }
    .to_string()
}

fn irregulars(number: u64) -> String {
    match number {
        2 => "twen",
        3 => "thir",
        4 => "for",
        5 => "fif",
        8 => "eigh",
        _ => "",
    }
    .to_string()
}

fn order_of_magnitude(number: u64) -> u64 {
    (number as f64).log10().floor() as u64
}

fn break_into_chunks(number: u64) -> Vec<Vec<u64>> {
    number
        .to_string()
        .chars()
        .rev()
        .collect::<Vec<_>>()
        .chunks_mut(3)
        .rev()
        .map(|c| {
            c.reverse();
            c.iter()
                .map(|x| x.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
