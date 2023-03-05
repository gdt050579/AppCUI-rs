use crate::{
    controls::events::EventProcessStatus,
    graphics::{CharAttribute, CharFlags, Character, Surface},
    input::{Key, KeyCode, KeyModifier, MouseEvent},
    terminal::{MouseButtonDownEvent, MouseButtonUpEvent, MouseMoveEvent},
};

use super::Theme;

const MAX_KEYS: usize = 64; // no bigger than 255
const MAX_SHIFT_STATES: usize = 8;
const INVALID_INDEX: u32 = 0xFFFFFFFF;

#[derive(Default)]
struct Item {
    text: String,
    key: &'static str,
    left: i32,
    right: i32,
    command: u32,
    version: u32,
    size: u16,
}
pub struct CommandBar {
    width: u32,
    y: i32,
    version: u32,
    modifier: KeyModifier,
    items: Vec<Item>,
    indexes: [Vec<u32>; MAX_SHIFT_STATES],
    has_shifts: [bool; MAX_SHIFT_STATES],
    hovered_index: u32,
    pressed_index: u32,
}

impl CommandBar {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        let mut obj = Self {
            width,
            y: (height as i32) - 1,
            version: 1,
            items: Vec::with_capacity(MAX_KEYS * MAX_SHIFT_STATES),
            indexes: Default::default(),
            has_shifts: [false; MAX_SHIFT_STATES],
            modifier: KeyModifier::None,
            hovered_index: INVALID_INDEX,
            pressed_index: INVALID_INDEX,
        };
        for vec in &mut obj.indexes {
            vec.reserve(MAX_KEYS);
        }
        for _ in 0..(MAX_KEYS * MAX_SHIFT_STATES) {
            obj.items.push(Item {
                text: String::new(),
                key: "",
                left: -1,
                right: -1,
                command: 0,
                version: 0,
                size: 0,
            });
        }
        obj
    }
    pub(crate) fn set_desktop_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.y = (height as i32) - 1;
        self.update_positions();
    }

    pub(crate) fn set_key_modifier(&mut self, modifier: KeyModifier) {
        if modifier != self.modifier {
            self.modifier = modifier;
            self.hovered_index = INVALID_INDEX;
            self.pressed_index = INVALID_INDEX;
        }
    }

    pub(crate) fn clear(&mut self) {
        self.version += 1;
        for has_shift in &mut self.has_shifts {
            *has_shift = false;
        }
        for vec in &mut self.indexes {
            vec.clear();
        }
        self.hovered_index = INVALID_INDEX;
        self.pressed_index = INVALID_INDEX;
    }

    pub fn set(&mut self, key: Key, text: &str, command: u32) -> bool {
        if key.code == KeyCode::None {
            return false;
        }
        let key_index = (key.code as u8) as usize;
        if key_index >= MAX_KEYS {
            return false;
        }
        let shift_state = key.modifier.get_value() as usize;
        if shift_state >= MAX_SHIFT_STATES {
            return false;
        }
        let item = &mut self.items[shift_state * MAX_KEYS + key_index];

        item.text.clear();
        item.text.push_str(text);
        item.text.push(' '); // one extra space
        item.command = command;
        item.left = -1;
        item.right = -1;
        item.key = key.code.get_name_padded();
        item.version = self.version;
        item.size = (item.key.len() + item.text.chars().count()) as u16;

        self.has_shifts[shift_state] = true;

        true
    }

    pub(crate) fn get_command(&self, key: Key) -> Option<u32> {
        if key.code == KeyCode::None {
            return None;
        }
        let key_index = (key.code as u8) as usize;
        if key_index >= MAX_KEYS {
            return None;
        }
        let shift_state = key.modifier.get_value() as usize;
        if shift_state >= MAX_SHIFT_STATES {
            return None;
        }
        let item = &self.items[shift_state * MAX_KEYS + key_index];
        if item.version != self.version {
            return None;
        }
        return Some(item.command);
    }
    pub(crate) fn update_positions(&mut self) {
        // recompute all positions regardless of the shift state
        for shift_state in 0..MAX_SHIFT_STATES {
            let vidx = &mut self.indexes[shift_state];
            vidx.clear();
            if self.has_shifts[shift_state] == false {
                continue;
            }
            let start_index = MAX_KEYS * shift_state;
            let end_index = start_index + MAX_KEYS;
            let mut x = if shift_state == 0 {
                0
            } else {
                KeyModifier::get_name_from_index(shift_state).len() as i32
            };
            for idx in start_index..end_index {
                let item = &mut self.items[idx];
                if item.version != self.version {
                    continue;
                }
                vidx.push(idx as u32);
                item.left = x;
                item.right = x + item.size as i32;
                x = item.right + 1;
                if x > (self.width as i32) {
                    break;
                }
            }
        }
        self.hovered_index = INVALID_INDEX;
        self.pressed_index = INVALID_INDEX;
    }

    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.fill_horizontal_line(
            0,
            self.y,
            self.width as i32,
            Character::with_attributes(' ', theme.menu.text.normal),
        );
        let modifier_name = self.modifier.get_name();
        if modifier_name.len() > 0 {
            surface.write_string(0, self.y, modifier_name, theme.menu.text.inactive, false);
        }
        let shift_idx = self.modifier.get_value() as usize;
        if (shift_idx >= MAX_SHIFT_STATES) || (self.has_shifts[shift_idx] == false) {
            return;
        }
        for idx in &self.indexes[shift_idx] {
            let item = &self.items[(*idx) as usize];

            // write the key
            let col_key = match () {
                _ if (*idx) == self.pressed_index => theme.menu.shortcut.pressed_or_selectd,
                _ if (*idx) == self.hovered_index => theme.menu.shortcut.hovered,
                _ => theme.menu.shortcut.normal,
            };
            surface.write_string(item.left, self.y, item.key, col_key, false);

            // write the text
            let col_text = match () {
                _ if (*idx) == self.pressed_index => theme.menu.text.pressed_or_selectd,
                _ if (*idx) == self.hovered_index => theme.menu.text.hovered,
                _ => theme.menu.text.normal,
            };
            surface.write_string(
                item.left + (item.key.len() as i32),
                self.y,
                &item.text,
                col_text,
                false,
            );
        }
    }

    fn mouse_poseition_to_index(&self, x: i32, y: i32) -> Option<u32> {
        if y != self.y {
            return None;
        }
        let shift_idx = self.modifier.get_value() as usize;
        if (shift_idx >= MAX_SHIFT_STATES) || (self.has_shifts[shift_idx] == false) {
            return None;
        }
        for idx in &self.indexes[shift_idx] {
            let item = &self.items[(*idx) as usize];
            if (x >= item.left) && (x < item.right) {
                return Some(*idx);
            }
        }
        None
    }

    pub(crate) fn on_mouse_move(&mut self, event: &MouseMoveEvent) -> bool {
        if event.y != self.y {
            self.hovered_index = INVALID_INDEX;
            self.pressed_index = INVALID_INDEX;
            return false;
        }
        // check if the current hovered index is not the actual index for current mouse pos
        if (self.hovered_index != INVALID_INDEX) && ((self.hovered_index as usize) < self.items.len()) {
            let item = &self.items[self.hovered_index as usize];
            if (event.x>=item.left) && (event.x<item.right) {
                return true;
            }
        }
        // else check the new index (if any)
        if let Some(idx) = self.mouse_poseition_to_index(event.x, event.y) {
            self.hovered_index = idx;
            return true;
        }
        self.hovered_index = INVALID_INDEX;
        return false;
    }

    pub(crate) fn on_mouse_down(&mut self, event: &MouseButtonDownEvent) -> bool {
        if self.hovered_index != INVALID_INDEX {
            self.pressed_index = self.hovered_index;
            return true;
        }
        return false;
    }
    pub(crate) fn on_mouse_up(&mut self, event: &MouseButtonUpEvent) -> Option<u32> {
        let idx = self.pressed_index as u32;
        self.hovered_index = INVALID_INDEX;
        self.pressed_index = INVALID_INDEX;

        if (idx != INVALID_INDEX) && ((idx as usize) < self.items.len()) {
            return Some(self.items[idx as usize].command);
        }
        return None;
    }
}


