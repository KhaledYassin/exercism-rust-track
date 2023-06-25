use std::cmp::Ord;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let factors_sum = sum_of_factors(num);

    match factors_sum.cmp(&num) {
        std::cmp::Ordering::Less => Some(Classification::Deficient),
        std::cmp::Ordering::Equal => Some(Classification::Perfect),
        std::cmp::Ordering::Greater => Some(Classification::Abundant),
    }
}

fn sum_of_factors(num: u64) -> u64 {
    (1..num).filter(|factor| num % factor == 0).sum()
}
