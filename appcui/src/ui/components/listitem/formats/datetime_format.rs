use chrono::Duration;

use crate::utils::FormatNumber;

#[derive(Copy, Clone, Eq, PartialEq)]
/// How a date-and-time list-item value is rendered.
///
/// `Full`, `Normal`, and `Short` select progressively more compact layouts.
pub enum DateTimeFormat {
    /// Long form, for example `Monday, 7 September 2026 09:15:00`.
    Full,
    /// Medium form, for example `2026-09-07 09:15:00`.
    Normal,
    /// Compact form, for example `09/07/26 09:15`.
    Short,
}

#[derive(Copy, Clone, Eq, PartialEq)]
/// How a time-of-day list-item value is rendered.
///
/// Choose a compact clock, 12-hour AM/PM, or a standard 24-hour time.
pub enum TimeFormat {
    /// Hours and minutes only, for example `09:15`.
    Short,
    /// 12-hour clock with AM/PM, for example `9:15 AM`.
    AMPM,
    /// Hours, minutes, and seconds, for example `09:15:00`.
    Normal,
}

#[derive(Copy, Clone, Eq, PartialEq)]
/// How a calendar-date list-item value is rendered.
///
/// `Full` is the long form; the other variants fix year-month-day or day-month-year
/// order.
pub enum DateFormat {
    /// Long weekday form, for example `Monday, 7 September 2026`.
    Full,
    /// ISO-style `YYYY-MM-DD`, for example `2026-09-07`.
    YearMonthDay,
    /// Day-first `DD-MM-YYYY`, for example `07-09-2026`.
    DayMonthYear
}

#[derive(Copy, Clone, Eq, PartialEq)]
/// How a duration list-item value is rendered.
///
/// `Auto` picks a readable unit, `Seconds` always uses seconds, and `Details`
/// expands hours, minutes, and seconds.
pub enum DurationFormat {
    /// Pick a readable unit automatically, for example `5 min`.
    Auto,
    /// Always seconds, for example `300 sec`.
    Seconds,
    /// Hours, minutes, and seconds, for example `1h 05m 00s`.
    Details
}

const SECONDS: FormatNumber = FormatNumber::new(10).suffix(" sec");
impl DurationFormat {
    pub (crate) fn seconds<'a>(value: &Duration, output: &'a mut [u8]) -> Option<&'a str> {
        SECONDS.write_number(value.num_seconds(), output)
    }
}