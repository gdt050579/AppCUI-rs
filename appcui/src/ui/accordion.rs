//! An accordion UI control that allows multiple panels to be displayed with expandable/collapsible headers.
//!
//! The Accordion control provides a space-efficient way to display multiple sections of content
//! in a single container. Only one panel is visible at a time, with headers for all panels always visible.
mod accordion;
mod accordion_panel;
mod initialization_flags;
#[cfg(test)]
mod tests;

pub use self::accordion::Accordion;
use self::accordion_panel::AccordionPanel;
pub use self::initialization_flags::Flags;
