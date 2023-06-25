use num_bigint::BigInt;
use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub},
};

#[derive(Clone, Debug, Eq)]
/// Type implementing arbitrary-precision decimal arithmetic
pub struct Decimal {
    int: BigInt,
    scale: usize,
}

impl Decimal {
    pub fn new(int: BigInt, scale: usize) -> Self {
        let mut value = Decimal { int, scale };
        value.strip_trailing_zeros();
        value
    }

    pub fn try_from(input: &str) -> Option<Decimal> {
        let length = input.len();
        let mut s = String::with_capacity(length);
        let mut decimal_point_position = 0;

        for (i, ch) in input.chars().enumerate() {
            match ch {
                '0'..='9' | '-' | '+' => s.push(ch),
                '.' => decimal_point_position = i,
                _ => {}
            }
        }

        Some(Decimal {
            int: s.parse().ok()?,
            scale: if decimal_point_position == 0 {
                0
            } else {
                (length - 1) - decimal_point_position
            },
        })
    }

    fn unify_scale(lhs: &mut Decimal, rhs: &mut Decimal) {
        let scale_difference = lhs.scale.abs_diff(rhs.scale);
        match lhs.scale.cmp(&rhs.scale) {
            Ordering::Less => Decimal::resize_scale(lhs, scale_difference),
            Ordering::Greater => Decimal::resize_scale(rhs, scale_difference),
            Ordering::Equal => {}
        }
    }

    fn resize_scale(decimal: &mut Decimal, scale_diff: usize) {
        decimal.int *= num_traits::pow(BigInt::from(10_usize), scale_diff);
        decimal.scale += scale_diff;
    }

    fn strip_trailing_zeros(&mut self) {
        let trailing_zeros = self
            .int
            .to_string()
            .chars()
            .rev()
            .take(self.scale)
            .take_while(|&c| c == '0')
            .count();
        self.int = &self.int / num_traits::pow(BigInt::from(10_usize), trailing_zeros);
        self.scale -= trailing_zeros;
    }
}

macro_rules! impl_decimal_cmp_operations {
    ($trait:ident, $function_name:ident, $cmp_operation:expr, $type:ty) => {
        impl $trait for Decimal {
            fn $function_name(&self, other: &Self) -> $type {
                if self.scale == other.scale {
                    $cmp_operation(&self.int, &other.int)
                } else {
                    let mut lhs = self.clone();
                    let mut rhs = other.clone();
                    Decimal::unify_scale(&mut lhs, &mut rhs);
                    lhs.$function_name(&rhs)
                }
            }
        }
    };
}

impl_decimal_cmp_operations!(PartialEq, eq, |lhs, rhs| lhs == rhs, bool);
impl_decimal_cmp_operations!(
    Ord,
    cmp,
    |lhs: &BigInt, rhs: &BigInt| lhs.cmp(rhs),
    Ordering
);

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

macro_rules! impl_decimal_math_operations {
    ($trait:ident, $function_name:ident, $math_operation:expr, $scale_operation:expr) => {
        impl $trait for Decimal {
            type Output = Self;

            fn $function_name(mut self, mut other: Self) -> Self {
                Decimal::unify_scale(&mut self, &mut other);
                Decimal::new(
                    $math_operation(self.int, other.int),
                    $scale_operation(self.scale, other.scale),
                )
            }
        }
    };
}

impl_decimal_math_operations!(Add, add, |lhs, rhs| lhs + rhs, |lhs, _| lhs);
impl_decimal_math_operations!(Sub, sub, |lhs, rhs| lhs - rhs, |lhs, _| lhs);
impl_decimal_math_operations!(Mul, mul, |lhs, rhs| lhs * rhs, |lhs, rhs| lhs + rhs);
