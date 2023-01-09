#[derive(Copy,Clone,PartialEq,Debug)]
pub enum SystemEvent
{
    None,
    AppClose,
    KeyEvent(super::KeyEvent),
    Resize(super::Size),
    MouseEvent(super::MouseEvent)
}