//! A progressbar UI control for displaying task completion status.
//!
//! The ProgressBar control provides a visual indicator of an operation's progress.
//! It can show both determinate progress (with known percentage) and indeterminate activity.

mod initialization_flags;
mod progressbar;
#[cfg(test)]
mod tests;

pub use self::initialization_flags::Flags;
pub use self::progressbar::ProgressBar;
