pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut value = n;
    let mut divisor = 2;

    while value > 1 {
        if value % divisor == 0 {
            value /= divisor;
            factors.push(divisor);
        } else {
            divisor += 1;
        }
    }

    factors
}



