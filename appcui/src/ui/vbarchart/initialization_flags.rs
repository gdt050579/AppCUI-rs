use EnumBitFlags::EnumBitFlags;

use crate::ui::{common::Number, vbarchart::BarSpan};

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::VBarChart`].
///
/// Combine values with `|`. `Flags::None` draws the bars without extra decorations.
pub enum Flags {}

pub enum BarScale<T: Number + 'static> {
    FromZero,
    FromZeroMinRange { min: T, max: T },
    FitData,
    Fixed { min: T, max: T },
}

pub enum XAxisLabelMode<'a> {
    None,
    Index { start: i32 },
    BarLabels,
    Custom(&'a [BarSpan]),
}

#[derive(Copy,Clone)]
pub(super) enum XAxis {
    None,
    Index { start: i32 },
    BarLabels,
    Custom,
}
impl XAxis {
    #[inline(always)]
    pub(crate) fn is_none(&self) -> bool {
        matches!(self, XAxis::None)
    }
    pub(crate) fn height(&self) -> u8 {
        match self {
            XAxis::None => 0,
            _ => 2
        }
    }
}