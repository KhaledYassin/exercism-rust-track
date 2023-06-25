use std::collections::HashMap;

const NUCLEOTIDES: &[char; 4] = &['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)?
        .remove(&nucleotide)
        .ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = NUCLEOTIDES.iter().map(|c| (*c, 0_usize)).collect();

    for c in dna.chars() {
        counts.get_mut(&c).map(|count| *count += 1).ok_or(c)?
    }

    Ok(counts)
}
