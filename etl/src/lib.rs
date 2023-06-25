use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut transform = BTreeMap::new();
    h.iter().for_each(|(score, chars)| {
        transform.extend(
            chars
                .iter()
                .map(|c| (c.to_ascii_lowercase(), *score))
                .into_iter(),
        );
    });
    transform
}
