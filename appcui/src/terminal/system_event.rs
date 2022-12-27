#[derive(Copy,Clone,PartialEq,Debug)]
pub enum SystemEvent
{
    None,
    AppClose,
    Key(super::Key),
    Resize(super::Size),
    Mouse(super::MouseEvent)
}