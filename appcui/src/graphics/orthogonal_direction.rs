/// How an orthogonal polyline chooses between horizontal and vertical segments.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum OrthogonalDirection {
    /// Draw the horizontal run first, then the vertical, for example `──┐` / `  │`.
    HorizontalFirst,
    /// Draw the vertical run first, then the horizontal, for example `│` / `└──`.
    VerticalFirst,
    /// Go horizontally until the midpoint, then vertically, then horizontally again.
    HorizontalUntilMiddle,
    /// Go vertically until the midpoint, then horizontally, then vertically again.
    VerticalUntilMiddle,
    /// Pick a routing automatically from the start and end points.
    #[default]
    Auto,
}
