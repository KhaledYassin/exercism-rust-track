// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
const SECONDS_IN_EARTH_YEAR: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration {
    value: f64,
}

macro_rules! impl_planet {
    ($name:ident, $factor_of_earth_years:expr) => {
        pub struct $name;
        impl Planet for $name { 
            fn years_during(d: &Duration) -> f64 {
                d.value / $factor_of_earth_years
            }
        }
    }
}

impl_planet!(Mercury, 0.2408467);
impl_planet!(Venus, 0.61519726);
impl_planet!(Earth, 1.0);
impl_planet!(Mars, 1.8808158);
impl_planet!(Jupiter, 11.862615);
impl_planet!(Saturn, 29.447498);
impl_planet!(Uranus, 84.016846);
impl_planet!(Neptune, 164.79132);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            value: s as f64 / SECONDS_IN_EARTH_YEAR,
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.value
    }
}

