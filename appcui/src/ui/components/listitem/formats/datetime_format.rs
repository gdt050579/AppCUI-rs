use chrono::Duration;

use crate::utils::FormatNumber;

#[derive(Copy, Clone, Eq, PartialEq)]
/// How a date-and-time list-item value is rendered.
///
/// `Full`, `Normal`, and `Short` select progressively more compact layouts.
pub enum DateTimeFormat {
    Full,
    Normal,
    Short,
}

#[derive(Copy, Clone, Eq, PartialEq)]
/// How a time-of-day list-item value is rendered.
///
/// Choose a compact clock, 12-hour AM/PM, or a standard 24-hour time.
pub enum TimeFormat {
    Short,
    AMPM,
    Normal,
}

#[derive(Copy, Clone, Eq, PartialEq)]
/// How a calendar-date list-item value is rendered.
///
/// `Full` is the long form; the other variants fix year-month-day or day-month-year
/// order.
pub enum DateFormat {
    Full,
    YearMonthDay,
    DayMonthYear
}

#[derive(Copy, Clone, Eq, PartialEq)]
/// How a duration list-item value is rendered.
///
/// `Auto` picks a readable unit, `Seconds` always uses seconds, and `Details`
/// expands hours, minutes, and seconds.
pub enum DurationFormat {
    Auto,
    Seconds,
    Details
}

const SECONDS: FormatNumber = FormatNumber::new(10).suffix(" sec");
impl DurationFormat {
    pub (crate) fn seconds<'a>(value: &Duration, output: &'a mut [u8]) -> Option<&'a str> {
        SECONDS.write_number(value.num_seconds(), output)
    }
}