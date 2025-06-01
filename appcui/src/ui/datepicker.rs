//! A datepicker UI control for selecting dates from a calendar interface.
//!
//! The DatePicker control provides an intuitive way to input and select dates.
//! It displays a calendar view that allows navigation between months and years.

mod datepicker;
pub mod events;
mod initialization_flags;
#[cfg(test)]
mod tests;

// pub use self::initialization_flags::Flags;
pub use self::datepicker::DatePicker;
