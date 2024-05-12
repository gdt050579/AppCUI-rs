#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Scale {
    NoScale = 1,
    Scale50 = 2,
    Scale33 = 3,
    Scale25 = 4,
    Scale20 = 5,
    Scale10 = 10,
    Scale5 = 20,
}