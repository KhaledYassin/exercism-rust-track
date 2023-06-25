use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &'a str) -> Option<&'a str> {
        self.map.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &'a str) -> Option<Vec<&'a str>> {
        let mut proteins = vec![];

        for codon_result in rna
            .as_bytes()
            .chunks(3)
            .map(std::str::from_utf8)
            .into_iter()
        {
            match codon_result {
                Ok(codon) => {
                    let optional_protein = self.map.get(codon);
                    match optional_protein {
                        Some(protein) => {
                            if protein == &"stop codon" {
                                break;
                            }
                            proteins.push(*protein)
                        }
                        None => return None,
                    }
                }
                Err(_) => return None,
            }
        }

        Some(proteins)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        map: pairs.into_iter().collect(),
    }
}
