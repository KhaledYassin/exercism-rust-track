use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    const MINUTES_IN_DAY: i32 = 1440;
    const MINUTES_IN_HOUR: i32 = 60;

    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * Clock::MINUTES_IN_HOUR + minutes;
        let modulo_minutes = total_minutes.rem_euclid(Clock::MINUTES_IN_DAY);
        let modulo_hours = modulo_minutes / Clock::MINUTES_IN_HOUR;
        let residual_minutes = modulo_minutes - modulo_hours * Clock::MINUTES_IN_HOUR;

        Clock {
            hours: modulo_hours,
            minutes: residual_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
