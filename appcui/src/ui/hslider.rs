//! A horizontal slider UI control that lets the user pick a numeric value from a range.
//!
//! The HSlider control displays a horizontal bar with a movable marker. The value can be
//! changed by dragging the marker with the mouse or by using the arrow keys, and is always
//! kept within the configured minimum and maximum bounds.

mod initialization_flags;
mod hslider;
pub mod events;

pub use self::hslider::HSlider;
pub use self::initialization_flags::Type;
pub use self::initialization_flags::Flags;
