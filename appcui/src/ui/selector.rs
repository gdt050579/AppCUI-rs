//! A selector UI control for choosing items from a categorized selection list.
//!
//! The Selector control provides a specialized interface for selecting items organized in categories.
//! It displays a scrollable list with grouping capabilities and supports keyboard navigation.

pub mod events;
mod initialization_flags;
mod selector;
#[cfg(test)]
mod tests;

pub use self::initialization_flags::EnumSelector;
pub use self::initialization_flags::Flags;
pub use self::selector::Selector;
