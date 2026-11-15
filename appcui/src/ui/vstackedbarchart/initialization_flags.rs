use EnumBitFlags::EnumBitFlags;

use crate::ui::components::BarSpan;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 1,
    DimBarsOnSelection = 2,
    ShowZeroLineOnYAxis = 4,    
}

pub enum XAxisLabelMode<'a> {
    None,
    Index(i32),
    StackLabels,
    Custom(&'a [BarSpan]),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(super) enum XAxisLabelFormat {
    None,
    Index(i32),
    StackLabels,
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
            _ => 2,
        }
    }
}
