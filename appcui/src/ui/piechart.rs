//! A pie chart UI control that shows a single series of values as parts of a whole.
//!
//! [`PieChart`] stores one slice per value. Each slice has a `u64` value, an optional
//! label, and an optional [`CharAttribute`](crate::graphics::CharAttribute); slices
//! without an attribute get an automatic color. [`Flags`] selects optional behavior.

mod draw_mode;
pub mod events;
mod initialization_flags;
mod labels;
mod piechart;
mod shapes;
mod selection;
mod slice;
#[cfg(test)]
mod tests;

pub use self::draw_mode::{Direction, DrawModeRenderer, DrawModeShape, HollowThickness};
pub use self::initialization_flags::Flags;
pub use self::labels::{LabelContent, LabelLayout};
pub use self::piechart::PieChart;

use self::slice::Slice;
