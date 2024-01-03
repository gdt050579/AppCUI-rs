#[repr(u8)]
#[derive(Eq,PartialEq, Copy, Clone)]
pub enum ScrollBarType {
    None,
    Inside,
    External
}