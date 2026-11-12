//! A heat map control.
//!
//! [`HeatMap`] is the base control. Drawing and behavior are added on top of this skeleton.

mod heatmap;
mod initialization_flags;
mod cell;
#[cfg(test)]
mod tests;

pub use self::heatmap::HeatMap;
pub use self::initialization_flags::Flags;
use self::cell::Cell;
use self::cell::PaintParams;
use self::cell::CellAttribute;
use self::cell::CellFill;
