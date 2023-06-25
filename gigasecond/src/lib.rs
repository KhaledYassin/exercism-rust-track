use time::PrimitiveDateTime as DateTime;
use core::time::Duration;

// Returns a DateTime one billion seconds after start.
const GIGA_SECONDS: Duration = Duration::from_secs(1000000000);

pub fn after(start: DateTime) -> DateTime {
    start + GIGA_SECONDS
}
