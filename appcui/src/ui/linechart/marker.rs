use crate::ui::components::{BarDrawMode, BarLargePointType, BarPointType};

/// The marker drawn at every point of a [`struct@super::LineChart`].
///
/// The marker decides the width of a point: every column of the chart is as wide as the
/// widest marker used by any series, so points of different series stay aligned.
/// [`Small`](Self::Small) with [`BarPointType::Bullet`] is the default.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Marker {
    /// No marker is drawn; only the line is visible. The point is one cell wide.
    None,
    /// A one-cell marker (same as [`BarDrawMode::Point`]).
    Small(BarPointType),
    /// A 3-by-2 marker (same as [`BarDrawMode::LargePoint`]).
    Large(BarLargePointType),
}

impl Marker {
    /// Returns the width of the marker, in cells.
    ///
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// assert_eq!(linechart::Marker::None.width(), 1);
    /// assert_eq!(linechart::Marker::Small(BarPointType::Bullet).width(), 1);
    /// assert_eq!(linechart::Marker::Large(BarLargePointType::Circle).width(), 3);
    /// ```
    #[inline(always)]
    pub fn width(&self) -> u8 {
        match self {
            Marker::None | Marker::Small(_) => 1,
            Marker::Large(_) => 3,
        }
    }

    /// Returns the height of the marker, in rows (2 for [`Marker::Large`], 1 otherwise).
    #[inline(always)]
    pub(crate) fn height(&self) -> u8 {
        match self {
            Marker::None | Marker::Small(_) => 1,
            Marker::Large(_) => 2,
        }
    }

    /// Translates a per-point [`BarDrawMode`] into a marker.
    ///
    /// Only [`BarDrawMode::Point`] and [`BarDrawMode::LargePoint`] have a marker equivalent;
    /// every other draw mode returns `None` and the chart default marker is used instead.
    #[inline(always)]
    pub(crate) fn from_draw_mode(draw_mode: BarDrawMode) -> Option<Marker> {
        match draw_mode {
            BarDrawMode::Point(point_type) => Some(Marker::Small(point_type)),
            BarDrawMode::LargePoint(point_type) => Some(Marker::Large(point_type)),
            _ => None,
        }
    }
}

impl Default for Marker {
    fn default() -> Self {
        Marker::Small(BarPointType::Bullet)
    }
}
