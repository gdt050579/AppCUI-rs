//! A heat map control.
//!
//! [`HeatMap`] is a grid of cells. [`Flags`] adds scroll bars, a margin, and dimming of
//! the cells that are not selected. [`TooltipFormat`] chooses the hover tooltip.
//! [`CellFill`] chooses the characters a cell is drawn with.
//! [`CellAttribute`] chooses its colors. [`CellBuilder`] builds a cell with a label, fill, and attribute.
//! The `heatmap!` macro creates a heat map from a text description.
//! [`events::GenericHeatMapEvents`] reports selection changes.

mod cell;
pub mod events;
mod heatmap;
mod initialization_flags;
#[cfg(test)]
mod tests;

pub use self::cell::Cell;
pub use self::cell::CellAttribute;
pub use self::cell::CellBuilder;
pub use self::cell::CellFill;
use self::cell::PaintParams;
pub use self::heatmap::HeatMap;
pub use self::initialization_flags::Flags;
pub use self::initialization_flags::TooltipFormat;
