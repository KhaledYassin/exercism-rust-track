use run_length_encoding as rle;
fn main() {
    let string = "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB";

    let encoded = rle::encode(string);

    println!("{}", encoded);

    let mut buffer = vec![];
    let mut decoded = String::new();

    for c in encoded.chars() {
        if c.is_numeric() {
            buffer.push(c);
        } else {
            buffer.push(c);

            let number_buffer = buffer.iter().filter(|c| c.is_numeric()).collect::<String>();

            let n = if number_buffer.is_empty() {
                1
            } else {
                number_buffer.parse::<usize>().unwrap()
            };

            let extension = c.to_string().repeat(n);

            decoded.push_str(&extension);

            buffer.clear();
        }
    }

    println!("{}", decoded);
}
