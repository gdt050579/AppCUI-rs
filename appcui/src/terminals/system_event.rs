use crate::input::{Key, KeyModifier, MouseButton, MouseWheelDirection};

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct MouseButtonDownEvent {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) button: MouseButton,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct MouseButtonUpEvent {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) button: MouseButton,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct MouseDoubleClickEvent {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) button: MouseButton,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct MouseMoveEvent {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) button: MouseButton,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct MouseWheelEvent {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) direction: MouseWheelDirection,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct KeyPressedEvent {
    pub(crate) key: Key,
    pub(crate) character: char,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct KeyModifierChangedEvent {
    pub(crate) new_state: KeyModifier,
    pub(crate) old_state: KeyModifier,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct TimerTick {
    low: u32,
    hi: u32,
}
impl From<u64> for TimerTick {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self {
            low: (value & 0xFFFF_FFFF) as u32,
            hi: (value >> 32) as u32,
        }
    }
}
impl TimerTick {
    pub(crate) fn value(&self) -> u64 {
        (self.low as u64) | ((self.hi as u64) << 32)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct TimerTickUpdateEvent {
    pub(crate) id: u8,
    pub(crate) tick: TimerTick,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct TimerStartEvent {
    pub(crate) id: u8,
    pub(crate) tick: TimerTick,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct TimerPausedEvent {
    pub(crate) id: u8,
    pub(crate) tick: TimerTick,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum SystemEvent {
    AppClose,
    KeyPressed(KeyPressedEvent),
    KeyModifierChanged(KeyModifierChangedEvent),
    Resize(super::Size),
    MouseButtonDown(MouseButtonDownEvent),
    MouseButtonUp(MouseButtonUpEvent),
    MouseDoubleClick(MouseDoubleClickEvent),
    MouseMove(MouseMoveEvent),
    MouseWheel(MouseWheelEvent),
    TimerTickUpdate(TimerTickUpdateEvent),
    TimerStart(TimerStartEvent),
    TimerPaused(TimerPausedEvent),
    BackgroundTaskStart(u32),
    BackgroundTaskEnd(u32),
    BackgroundTaskNotify(u32),
    BackgroundTaskQuery(u32),
}

impl SystemEvent {
    #[inline(always)]
    pub(super) fn should_close(&self) -> bool {
        matches!(self, SystemEvent::AppClose)
    }
}
