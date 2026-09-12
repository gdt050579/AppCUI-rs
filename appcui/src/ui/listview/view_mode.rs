#[derive(Copy,Clone, Eq, PartialEq, Debug)]
/// How items are arranged in a [`super::ListView`].
///
/// `Details` is a classic multi-column table. `Columns(n)` lays items out in `n`
/// side-by-side columns.
pub enum ViewMode {
    /// Multi-column table with a header row, one item per line.
    Details,
    /// Lay items out in the given number of side-by-side columns.
    Columns(u8)
}