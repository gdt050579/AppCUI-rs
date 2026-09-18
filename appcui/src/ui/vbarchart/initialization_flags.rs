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
pub(super) enum XAxisLabelFormat {
    None,
    Index { start: i32 },
    BarLabels,
    Custom,
}
impl XAxisLabelFormat {
    #[inline(always)]
    pub(crate) fn is_none(&self) -> bool {
        matches!(self, XAxisLabelFormat::None)
    }
    pub(crate) fn height(&self) -> u8 {
        match self {
            XAxisLabelFormat::None => 0,
            _ => 2
        }
    }
}