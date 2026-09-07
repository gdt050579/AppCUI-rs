#[derive(Copy,Clone, Eq, PartialEq, Debug)]
/// How items are arranged in a [`super::ListView`].
///
/// `Details` is a classic multi-column table. `Columns(n)` lays items out in `n`
/// side-by-side columns.
pub enum ViewMode {
    Details,
    Columns(u8)
}