/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if is_palindrome(value) {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {

    let mut minimum = u64::MAX;
    let mut maximum = 0;

    for i in min..=max {
        for j in min..=max {
            let product = i * j;

            if is_palindrome(product) {
                maximum = if product > maximum { product } else { maximum };
                minimum = if product < minimum { product } else { minimum };
            }
        }
    }

    if minimum == u64::MAX || maximum == 0 {
        return None
    }

    Some((Palindrome::new(minimum)?, Palindrome::new(maximum)?))
}

fn is_palindrome(number: u64) -> bool {
    if number < 10 {
        true
    } else {
        let mut mutable_value = number;
        let mut reverse = 0;

        while mutable_value > 0 {
            let dig = mutable_value % 10;
            reverse = reverse * 10 + dig;
            mutable_value /= 10;
        }

        number == reverse
    }
}
