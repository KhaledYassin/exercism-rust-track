const FACTORS: &[u32] = &[3, 5, 7];

pub fn raindrops(n: u32) -> String {
    let mut sound = String::new();

    FACTORS
        .iter()
        .filter(|f| n % *f == 0)
        .for_each(|f| sound.push_str(drop(*f)));

    if sound.is_empty() {
        sound.push_str(&n.to_string());
    }

    sound
}

fn drop(factor: u32) -> &'static str {
    match factor {
        3 => "Pling",
        5 => "Plang",
        7 => "Plong",
        _ => "",
    }
}
