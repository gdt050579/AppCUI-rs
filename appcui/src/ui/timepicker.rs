//! A timepicker UI control for selecting and editing time values.
//!
//! The TimePicker control provides an intuitive way to input and modify time values.
//! It supports both 12-hour (AM/PM) and 24-hour formats, with optional seconds display.

mod timepicker;
mod initialization_flags;
pub mod events;
#[cfg(test)]
mod tests;

pub use self::timepicker::TimePicker;
pub use self::initialization_flags::Flags;
pub use self::events::TimePickerEvents;
