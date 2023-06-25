use std::collections::HashSet;

pub fn find(n: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();

    for m in 1..n {
        let denominator = 2 * (n - m);
        let numerator = 2 * m * m + n * n - 2 * n * m;

        if numerator % denominator == 0 {
            let c = numerator / denominator;
            if m + c < n {
                let b = n - m - c;
                if b > m {
                    triplets.insert([m, b, c]);
                }
            }
        }
    }

    triplets
}
