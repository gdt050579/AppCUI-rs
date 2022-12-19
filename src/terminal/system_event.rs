#[derive(Copy,Clone,PartialEq)]
pub enum SystemEvent
{
    None,
    AppClose,
    Key(super::Key)
}