use crate::system::Handle;

#[derive(Copy,Clone)]
pub struct WindowDecoratorButtonPressedEvent
{
    pub command_id: u32
}

#[derive(Copy,Clone)]
pub struct WindowDecoratorCheckBoxStateChangedEvent
{
    pub command_id: u32,
    pub checked: bool,
}
#[derive(Copy,Clone)]
pub struct WindowDecoratorSingleChoiceSelectedEvent
{
    pub command_id: u32
}

#[derive(Copy,Clone)]
pub struct WindowCloseEvent
{
    pub handle: Handle
}