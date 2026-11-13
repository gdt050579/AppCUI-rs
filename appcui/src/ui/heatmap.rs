//! A heat map control.
//!
//! [`HeatMap`] is a grid of cells. [`Flags`] adds scroll bars, a margin, and dimming of
//! the cells that are not selected. [`TooltipFormat`] chooses the hover tooltip.
//! [`events::GenericHeatMapEvents`] reports selection changes.

mod cell;
pub mod events;
mod heatmap;
mod initialization_flags;
#[cfg(test)]
mod tests;

use self::cell::Cell;
use self::cell::CellAttribute;
use self::cell::CellFill;
use self::cell::PaintParams;
pub use self::heatmap::HeatMap;
pub use self::initialization_flags::Flags;
pub use self::initialization_flags::TooltipFormat;
