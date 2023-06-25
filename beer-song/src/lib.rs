trait Verse {
    fn verse(&self) -> String;
}

enum Count {
    Zero,
    One,
    Many,
}
struct Bottles {
    count: Count,
    number: u32,
}

impl Bottles {
    fn new(n: u32) -> Self {
        Bottles {
            count: count_type(n),
            number: n,
        }
    }
}

impl Verse for Bottles {
    fn verse(&self) -> String {
        match self.count {
            Count::Zero => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
            Count::One => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
            Count::Many => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} {} of beer on the wall.\n", self.number, self.number, self.number - 1, if self.number == 2 { "bottle" } else { "bottles" }),
        }
    }
}

fn count_type(n: u32) -> Count {
    match n {
        0 => Count::Zero,
        1 => Count::One,
        _ => Count::Many,
    }
}

pub fn verse(n: u32) -> String {
    let bottle_s = Bottles::new(n);
    bottle_s.verse()
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .map(|n| verse(n))
        .rev()
        .collect::<Vec<String>>()
        .join("\n")
}
