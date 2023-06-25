// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

fn word_count_map(words: &[&str]) -> HashMap<String, i32> {
    words.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c.to_string()).or_insert(0) += 1;
        acc
    })
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map = word_count_map(&magazine);
    let note_map = word_count_map(&note);

    let mut word_check = note_map.keys().all(|k| magazine_map.contains_key(k));

    if (word_check) {
        magazine_map.retain(|k, v| note_map.contains_key(k));
        println!(
            "Magazine map: {:?} - Note map: {:?}",
            magazine_map, note_map
        );
        magazine_map
            .iter()
            .for_each(|(k, v)| match note_map.get(k) {
                Some(x) => word_check = x <= v,
                None => word_check = false,
            });
    }

    word_check
}
