#[repr(u8)]
#[derive(Copy,Clone,PartialEq)]
pub enum LineType {
    Single ,
    Double,
    SingleThick,
    Border,
    Ascii,
    AsciiRound,
    SingleRound,
}