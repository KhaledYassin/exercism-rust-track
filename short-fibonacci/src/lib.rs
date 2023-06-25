/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

pub fn fibonacci_series(n: usize) -> u8 {
    match n {
        0 | 1 => 1,
        _ => fibonacci_series(n - 1) + fibonacci_series(n - 2),
    }
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let size = 5;

    let mut first_five = create_buffer(size);

    first_five.iter_mut().enumerate().for_each(|(i, value)| {
        *value = fibonacci_series(i);
    });

    first_five
}
