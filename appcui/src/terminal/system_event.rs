use crate::input::{MouseButton, MouseWheelDirection};

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

#[derive(Copy,Clone,PartialEq,Debug)]
pub (crate) enum SystemEvent
{
    None,
    AppClose,
    KeyEvent(super::KeyEvent),
    Resize(super::Size),
    MouseButtonDown(MouseButtonDownEvent),
    MouseButtonUp(MouseButtonUpEvent),
    MouseDoubleClick(MouseDoubleClickEvent),
    MouseMove(MouseMoveEvent),
    MouseWheel(MouseWheelEvent)
}

