use super::super::SystemEvent;
use crate::graphics::*;
use crate::input::KeyModifier;
use crate::terminals::utils::win32;
use crate::terminals::utils::win32::constants::*;
use crate::terminals::utils::win32::structs::*;
use crate::terminals::SystemEventReader;
use std::sync::Arc;
use std::sync::Mutex;

pub(crate) struct Input {
    console: win32::Console,
    shift_state: KeyModifier,
    last_mouse_pos: Point,
    shared_visible_region: Arc<Mutex<SMALL_RECT>>,
}

impl Input {
    pub(super) fn new(console: win32::Console, shared_visible_region: Arc<Mutex<SMALL_RECT>>) -> Self {
        Self {
            console,
            shift_state: KeyModifier::None,
            last_mouse_pos: Point::new(i32::MAX, i32::MAX),
            shared_visible_region,
        }
    }
}

impl SystemEventReader for Input {
    fn read(&mut self) -> Option<SystemEvent> {
        let mut ir = INPUT_RECORD {
            event_type: 0,
            event: WindowsTerminalEvent { extra: 0 },
        };
        let mut nr_read = 0u32;

        unsafe {
            if (win32::api::ReadConsoleInputW(self.console.stdin(), &mut ir, 1, &mut nr_read) == FALSE) || (nr_read != 1) {
                return None;
            }
            //println!("Event: {}",ir.event_type);
        }

        // Key processings
        if ir.event_type == KEY_EVENT {
            return unsafe { ir.event.key_event.to_system_event(&mut self.shift_state) };
        }

        // mouse processing
        if ir.event_type == MOUSE_EVENT {
            return unsafe {
                ir.event.mouse_event.to_system_event(
                    Point::new(self.console.visible_region().left as i32, self.console.visible_region().top as i32),
                    &mut self.last_mouse_pos,
                )
            };
        }

        // resize
        if ir.event_type == WINDOW_BUFFER_SIZE_EVENT {
            if let Ok(info) = self.console.query_screen_buffer_info() {
                let w = (info.window.right as u32) + 1 - (info.window.left as u32);
                let h = (info.window.bottom as u32) + 1 - (info.window.top as u32);
                self.console.set_visible_region(info.window);
                if let Ok(mut shared_data) = self.shared_visible_region.lock() {
                    *shared_data = info.window;
                }
                return Some(SystemEvent::Resize(Size::new(w, h)));
            }
        }

        None
    }
}
