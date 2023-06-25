pub fn nth(n: u32) -> u32 {
    let x = (n.max(10) + 1) as f64;
    let limit: usize = (x * ((x * x.ln()).ln())).floor() as usize;
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
        .map(|(i, _)| i)
        .nth(n as usize)
        .unwrap_or(1) as u32
}
