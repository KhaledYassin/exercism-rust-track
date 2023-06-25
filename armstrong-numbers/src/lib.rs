pub fn is_armstrong_number(number: u32) -> bool {
    number == required_sum(number)
}

fn required_sum(mut number: u32) -> u32 {
    let number_of_digits = number_of_digits(number);
    let mut sum = 0_u32;

    while number > 0 {
        let digit = number % 10;
        number /= 10;
        sum += digit.pow(number_of_digits);
    }

    sum
}

fn number_of_digits(number: u32) -> u32 {
    std::iter::successors(Some(number), |&n| (n >= 10).then(|| n / 10)).count() as u32
}
