use crate::input::{MouseButton, MouseWheelDirection, Key, KeyModifier};

#[derive(Copy,Clone,PartialEq,Debug)]
pub (crate) struct MouseButtonDownEvent
{
    pub (crate) x: i32,
    pub (crate) y: i32,
    pub (crate) button: MouseButton
}

#[derive(Copy,Clone,PartialEq,Debug)]
pub (crate) struct MouseButtonUpEvent
{
    pub (crate) x: i32,
    pub (crate) y: i32,
    pub (crate) button: MouseButton
}

#[derive(Copy,Clone,PartialEq,Debug)]
pub (crate) struct MouseDoubleClickEvent
{
    pub (crate) x: i32,
    pub (crate) y: i32,
    pub (crate) button: MouseButton
}

#[derive(Copy,Clone,PartialEq,Debug)]
pub (crate) struct MouseMoveEvent
{
    pub (crate) x: i32,
    pub (crate) y: i32,
    pub (crate) button: MouseButton
}

#[derive(Copy,Clone,PartialEq,Debug)]
pub (crate) struct MouseWheelEvent
{
    pub (crate) x: i32,
    pub (crate) y: i32,
    pub (crate) direction: MouseWheelDirection
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub (crate) struct KeyPressedEvent {
    pub (crate) key: Key,
    pub (crate) character: char,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub (crate) struct KeyModifierChangedEvent {
    pub (crate) new_state: KeyModifier,
    pub (crate) old_state: KeyModifier,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub (crate) struct TimerTickUpdateEvent {
    id: u8, 
    tick: u64,   
}

#[derive(Copy,Clone,PartialEq,Debug)]
pub (crate) enum SystemEvent
{
    None,
    AppClose,
    KeyPressed(KeyPressedEvent),
    KeyModifierChanged(KeyModifierChangedEvent),
    Resize(super::Size),
    MouseButtonDown(MouseButtonDownEvent),
    MouseButtonUp(MouseButtonUpEvent),
    MouseDoubleClick(MouseDoubleClickEvent),
    MouseMove(MouseMoveEvent),
    MouseWheel(MouseWheelEvent),
    //TimerTickUpdate(TimerTickUpdateEvent),
}

