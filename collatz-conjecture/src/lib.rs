pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut number = n;
    let mut steps = 0;

    let result = std::panic::catch_unwind(move || {
        while number != 1 {
            number = if number % 2 == 0 {
                number / 2
            } else {
                3 * number + 1
            };
            steps += 1
        }

        steps
    });

    match result {
        Ok(steps) => Some(steps),
        Err(_) => None,
    }
}
