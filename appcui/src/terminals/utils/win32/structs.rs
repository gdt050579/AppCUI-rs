use crate::{input::{Key, KeyCode, KeyModifier}, terminals::{system_event::KeyModifierChangedEvent, KeyPressedEvent, SystemEvent}};
use super::constants::*;

#[allow(clippy::upper_case_acronyms)]
pub(crate) type HANDLE = usize;
#[allow(clippy::upper_case_acronyms)]
pub(crate) type BOOL = u32;

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct SIZE {
    pub width: u16,
    pub height: u16,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct COORD {
    pub x: i16,
    pub y: i16,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct SMALL_RECT {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}
#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct CONSOLE_SCREEN_BUFFER_INFO {
    pub size: COORD,
    pub cursor_pos: COORD,
    pub attributes: u16,
    pub window: SMALL_RECT,
    pub max_size: COORD,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct CONSOLE_CURSOR_INFO {
    pub size: u32,
    pub visible: BOOL,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct CHAR_INFO {
    pub code: u16,
    pub attr: u16,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct KEY_EVENT_RECORD {
    pub key_down: BOOL,
    pub repeat_count: u16,
    pub virtual_key_code: u16,
    pub virtual_scan_code: u16,
    pub unicode_char: u16,
    pub control_key_state: u32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct MOUSE_EVENT_RECORD {
    pub mouse_position: COORD,
    pub button_state: u32,
    pub control_key_state: u32,
    pub event_flags: u32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub(crate) union WindowsTerminalEvent {
    pub key_event: KEY_EVENT_RECORD,
    pub mouse_event: MOUSE_EVENT_RECORD,
    pub window_buffer_size_event: SIZE,
    pub extra: u32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub(crate) struct INPUT_RECORD {
    pub event_type: u16,
    pub event: WindowsTerminalEvent,
}

impl KEY_EVENT_RECORD {
    pub(crate) fn to_system_event(&self, current_modifier: &mut KeyModifier) -> Option<SystemEvent> {
        let mut key_code = KeyCode::None;
        let mut key_modifier = KeyModifier::None;
        let mut character = '\0';
        if (self.unicode_char >= 32) && (self.key_down == TRUE) {
            let res = char::from_u32(self.unicode_char as u32);
            if res.is_some() {
                character = res.unwrap();
            }
        }
        if self.virtual_key_code < 256 {
            key_code = TRANSLATION_MATRIX[self.virtual_key_code as usize];
        }

        if (self.control_key_state & (LEFT_ALT_PRESSED | RIGHT_ALT_PRESSED)) != 0 {
            key_modifier |= KeyModifier::Alt;
        }
        if (self.control_key_state & (LEFT_CTRL_PRESSED | RIGHT_CTRL_PRESSED)) != 0 {
            key_modifier |= KeyModifier::Ctrl;
        }
        if (self.control_key_state & SHIFT_PRESSED) != 0 {
            key_modifier |= KeyModifier::Shift;
        }

        // if ALT or CTRL are pressed, clear the ascii code
        if key_modifier.contains_one(KeyModifier::Alt | KeyModifier::Ctrl) {
            character = '\0';
        }
        if (key_code != KeyCode::None) || (character != '\0') {
            if self.key_down == FALSE {
                // key is up (no need to send)
                return None;
            }
        } else {
            // check for change in modifier
            if *current_modifier == key_modifier {
                // nothing changed --> return
                return None;
            }
            let old_state = *current_modifier;
            *current_modifier= key_modifier;
            return Some(SystemEvent::KeyModifierChanged(KeyModifierChangedEvent {
                new_state: key_modifier,
                old_state,
            }));
        }
        return Some(SystemEvent::KeyPressed(KeyPressedEvent {
            key: Key::new(key_code, key_modifier),
            character,
        }));
    }
}