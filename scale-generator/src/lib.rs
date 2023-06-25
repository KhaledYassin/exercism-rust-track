use std::error::Error;

// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.

const CHROMATIC_SCALE_SHARPS: &[&str; 12] = &[
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];
const CHROMATIC_SCALE_FLATS: &[&str; 12] = &[
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

#[derive(Debug)]
pub struct Scale {
    notes: Vec<String>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Box<dyn Error>> {
        let chromatic_scale = match tonic {
            "C" | "a" | "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#"
            | "d#" => CHROMATIC_SCALE_SHARPS,
            "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" | "d" | "g" | "c" | "f" | "bb" | "eb" => {
                CHROMATIC_SCALE_FLATS
            }
            _ => return Err(format!("Invalid tonic: {}", tonic).into()),
        };

        let mut tonic_string = tonic.to_string();

        if let Some(c) = tonic_string.get_mut(0..1) {
            c.make_ascii_uppercase()
        }

        let mut notes = Vec::new();

        let starting_index = chromatic_scale
            .iter()
            .position(|&note| note == tonic_string)
            .unwrap();

        let mut current_index = starting_index;
        notes.push(chromatic_scale[current_index].to_string());

        for interval in intervals.chars() {
            current_index += match interval {
                'M' => 2,
                'm' => 1,
                'A' => 3,
                _ => return Err(format!("Invalid interval pattern: {}", interval).into()),
            };
            notes.push(chromatic_scale[current_index % chromatic_scale.len()].to_string());
        }

        Ok(Scale { notes })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Box<dyn Error>> {
        Scale::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.notes.clone()
    }
}
