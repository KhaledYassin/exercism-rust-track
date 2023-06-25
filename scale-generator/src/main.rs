fn main() {
    let note = "bb";

    let mut note_copied = note.to_string();

    if let Some(c) = note_copied.get_mut(0..1) {
        c.make_ascii_uppercase()
    }

    println!("{}", note_copied)
}
