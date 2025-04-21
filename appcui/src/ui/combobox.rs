//! A combobox UI control combining a dropdown list with an editable text field.
//!
//! The ComboBox control provides both dropdown selection and text entry in one component.
//! It allows users to either select from predefined options or enter custom values.

mod combobox;
mod initialization_flags;
mod item;
pub mod events;
#[cfg(test)]
mod tests;

pub use self::combobox::ComboBox;
pub use self::initialization_flags::Flags;
pub use self::item::Item;
