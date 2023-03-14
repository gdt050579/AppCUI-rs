#[derive(Copy, Clone)]
pub struct MenuItemHandle {
    index: u32,
}
impl MenuItemHandle {
    fn new(index: u32) -> Self {
        Self { index }
    }
}
