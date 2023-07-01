pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 2_u64.pow(s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    // Sum of geometric series of n steps
    // (1 - 2^64) / (1 - 2) = 18_446_744_073_709_551_615
    // which is the maximum value of u64
    u64::MAX
}
