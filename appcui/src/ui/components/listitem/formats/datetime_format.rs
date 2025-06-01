use chrono::Duration;

use crate::utils::FormatNumber;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DateTimeFormat {
    Full,
    Normal,
    Short,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TimeFormat {
    Short,
    AMPM,
    Normal,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DateFormat {
    Full,
    YearMonthDay,
    DayMonthYear,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DurationFormat {
    Auto,
    Seconds,
    Details,
}

const SECONDS: FormatNumber = FormatNumber::new(10).suffix(" sec");
impl DurationFormat {
    pub(crate) fn seconds<'a>(value: &Duration, output: &'a mut [u8]) -> Option<&'a str> {
        SECONDS.write_number(value.num_seconds(), output)
    }
}
