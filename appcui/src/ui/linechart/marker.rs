use crate::ui::components::{BarDrawMode, BarFramedPointType, BarPointType};

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
    /// A 5-by-3 framed marker (same as [`BarDrawMode::FramedPoint`]).
    ///
    /// Its middle row is on the value, and lines are attached to the middle of its left and
    /// right sides (box-drawing frames get a junction where a box-drawing line touches them).
    Framed(BarFramedPointType),
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
    /// assert_eq!(linechart::Marker::Framed(BarFramedPointType::RoundSquare).width(), 5);
    /// ```
    #[inline(always)]
    pub fn width(&self) -> u8 {
        match self {
            Marker::None | Marker::Small(_) => 1,
            Marker::Framed(_) => 5,
        }
    }

    /// Returns the height of the marker, in rows (3 for [`Marker::Framed`], 1 otherwise).
    #[inline(always)]
    pub(crate) fn height(&self) -> u8 {
        match self {
            Marker::None | Marker::Small(_) => 1,
            Marker::Framed(_) => 3,
        }
    }

    /// Translates a per-point [`BarDrawMode`] into a marker.
    ///
    /// Only [`BarDrawMode::Point`] and [`BarDrawMode::FramedPoint`] have a marker equivalent;
    /// every other draw mode (including [`BarDrawMode::LargePoint`], whose even height has no
    /// middle row for the value) returns `None` and the chart default marker is used instead.
    #[inline(always)]
    pub(crate) fn from_draw_mode(draw_mode: BarDrawMode) -> Option<Marker> {
        match draw_mode {
            BarDrawMode::Point(point_type) => Some(Marker::Small(point_type)),
            BarDrawMode::FramedPoint(point_type) => Some(Marker::Framed(point_type)),
            _ => None,
        }
    }
}

/// The vertical border character and the junctions used when a line is attached to the
/// left / right side of a framed marker: `(border, junctions for thin, thick, double lines)`,
/// each junction pair being `(left side, right side)`. `None` for frames without box-drawing sides.
pub(crate) fn frame_junctions(frame: BarFramedPointType) -> Option<[(char, char); 3]> {
    match frame {
        BarFramedPointType::RoundSquare
        | BarFramedPointType::Square
        | BarFramedPointType::RoundSquareBullet
        | BarFramedPointType::SquareBullet => Some([('┤', '├'), ('┥', '┝'), ('╡', '╞')]),
        BarFramedPointType::ThickSquare | BarFramedPointType::ThickSquareBullet => Some([('┨', '┠'), ('┫', '┣'), ('┨', '┠')]),
        BarFramedPointType::DoubleLineSquare | BarFramedPointType::DoubleLineSquareBullet => {
            Some([('╢', '╟'), ('╢', '╟'), ('╣', '╠')])
        }
        _ => None,
    }
}

/// Weight of the arm that `ch` has toward the left (`toward_left = true`) or the right:
/// 0 = thin, 1 = thick, 2 = double, `None` = no such arm.
pub(crate) fn horizontal_arm(ch: char, toward_left: bool) -> Option<usize> {
    let (thin, thick, double): (&str, &str, &str) = if toward_left {
        ("─┐┘┤┬┴┼╮╯-", "━┓┛┫┳┻╋", "═╗╝╣╦╩╬")
    } else {
        ("─┌└├┬┴┼╭╰-", "━┏┗┣┳┻╋", "═╔╚╠╦╩╬")
    };
    if thin.contains(ch) {
        Some(0)
    } else if thick.contains(ch) {
        Some(1)
    } else if double.contains(ch) {
        Some(2)
    } else {
        None
    }
}

impl Default for Marker {
    fn default() -> Self {
        Marker::Small(BarPointType::Bullet)
    }
}
