pub fn encode(source: &str) -> String {
    let chars = source.chars().collect::<Vec<_>>();

    let mut encoded = String::new();
    let length = chars.len();
    let mut i = 0;

    while i < length {
        let mut count = 0;

        while i < length - 1 && chars[i] == chars[i + 1] {
            count += 1;
            i += 1;
        }

        let extension = if count == 0 {
            format!("{}", chars[i])
        } else {
            format!("{}{}", count + 1, chars[i])
        };

        encoded.push_str(&extension.as_str());

        i += 1;
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut buffer = vec![];

    for c in source.chars() {
        if c.is_numeric() {
            buffer.push(c);
        } else {
            buffer.push(c);

            let number_buffer = buffer.iter().filter(|c| c.is_numeric()).collect::<String>();
            let n = number_buffer.parse::<usize>().unwrap_or(1);
            let extension = c.to_string().repeat(n);

            decoded.push_str(&extension);
            buffer.clear();
        }
    }

    decoded
}
