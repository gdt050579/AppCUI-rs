#[repr(u8)]
#[derive(Eq,PartialEq)]
pub enum ScrollBarType {
    None,
    Inside,
    External
}