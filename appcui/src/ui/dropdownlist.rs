//! A dropdownlist UI control for selecting an item from a collapsible list.
//!
//! The DropDownList control provides a compact way to present multiple options in a single control.
//! It displays the currently selected item and expands to show all options when activated.

mod dropdownlist;
pub mod events;
mod initialization_flags;
#[cfg(test)]
mod tests;

pub use self::dropdownlist::DropDownList;
pub use self::initialization_flags::DropDownListType;
pub use self::initialization_flags::Flags;
