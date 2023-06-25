#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    dna: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    rna: String,
}

const NUCLEOTIDE_PAIRS: [(char, char); 4] = [('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')];

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let dna_strands = NUCLEOTIDE_PAIRS.map(|(dna, _)| dna);

        for (i, c) in dna.chars().enumerate() {
            if !dna_strands.contains(&c) {
                return Err(i);
            }
        }

        Ok(Dna {
            dna: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            rna: self
                .dna
                .chars()
                .map(|c| match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => c,
                })
                .collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let rna_strands = NUCLEOTIDE_PAIRS.map(|(_, rna)| rna);

        for (i, c) in rna.chars().enumerate() {
            if !rna_strands.contains(&c) {
                return Err(i);
            }
        }

        Ok(Rna {
            rna: rna.to_string(),
        })
    }
}
