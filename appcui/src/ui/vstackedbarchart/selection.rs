#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Selection {
    #[default]
    None,
    Stack(u32),
    Cell(u32, u32),
}
