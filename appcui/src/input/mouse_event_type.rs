#[derive(Copy,Clone,Debug,PartialEq)]
#[repr(u8)]
pub enum MouseEventType {
    ButtonDown,
    ButtonUp,
    DoubleClick,
    Move,
    Wheel
}