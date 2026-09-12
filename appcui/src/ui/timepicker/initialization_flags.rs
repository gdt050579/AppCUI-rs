use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::TimePicker`].
///
/// Combine values with `|`. `Flags::None` shows hours and minutes in 24-hour form
/// (`HH:MM`).
pub enum Flags {
    /// Show a 12-hour clock with an AM/PM indicator, for example `02:35 PM`.
    AMPM = 0x0001,
    /// Include seconds in the time, for example `14:35:56`.
    Seconds = 0x0002,
}
