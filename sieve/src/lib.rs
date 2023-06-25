pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let limit = (upper_bound  + 1) as usize;
    let mut prime = vec![true; limit];
    for i in 2..limit {
        if prime[i] {
            let long = i as u128;
            let range = ((long * long)..limit as u128).step_by(i);
            for j in range {
                prime[j as usize] = false;
            }
        }
    }

    prime
        .iter()
        .enumerate()
        .skip(2)
        .filter(|&(_, p)| *p)
        .map(|(i, _)| i as u64)
        .collect()
}
