#[derive(Copy,Clone, Eq, PartialEq, Debug)]
pub enum ViewMode {
    Details,
    Columns(u8)
}