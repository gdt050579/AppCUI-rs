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
    DayMonthYear
}