/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let distance = s1
        .as_bytes()
        .iter()
        .zip(s2.as_bytes().iter())
        .into_iter()
        .filter(|(c1, c2)| c1 != c2)
        .count();

    Some(distance)
}
