#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub enum Dock {
    Left = 0,
    Right = 1,
    Top = 2,
    Bottom = 3,
    Fill = 4,
}
