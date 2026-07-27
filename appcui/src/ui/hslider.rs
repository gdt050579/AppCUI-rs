//! A hyperlink UI control that displays a clickable text and triggers an action when activated.
//!
//! The HyperLink control shows a text (usually associated with an URL) that can be
//! activated with the mouse or with the Enter key. It is underlined when hovered or focused.

mod initialization_flags;
mod hslider;
pub mod events;

pub use self::hslider::HSlider;
pub use self::initialization_flags::Type;
pub use self::initialization_flags::Flags;
