#[derive(Copy,Clone)]
pub enum ViewMode {
    Details,
    Columns(u8)
}