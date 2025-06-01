//! Input handling module.
//!
//! This module contains the definitions for the input handling systems such as keyboard and mouse.

mod key;
mod key_code;
mod key_modifier;
mod mouse_button;
mod mouse_event;
mod mouse_wheel_direction;

#[cfg(test)]
mod tests;

pub use self::key::Key;
pub use self::key_code::KeyCode;
pub use self::key_modifier::KeyModifier;
pub use self::mouse_button::MouseButton;
pub use self::mouse_event::MouseEvent;
pub use self::mouse_event::MouseEventData;
pub use self::mouse_wheel_direction::MouseWheelDirection;
