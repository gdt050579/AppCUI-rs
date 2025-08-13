use crate::prelude::*;
use super::set::Set;
use crate::ui::charpicker::events::EventData;



#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent+OnExpand, internal=true)]
pub struct CharPicker {
    code: char,
    header_y_ofs: i32,
    expanded_panel_y: i32,
    chars_per_width: i32,
    start_code: u32,
    sets: Vec<Set>,
}
impl CharPicker {
    pub fn new(code: char, layout: Layout) -> Self {
        let mut cp = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            header_y_ofs: 0,
            expanded_panel_y: 1,
            chars_per_width: 1,
            code,
            start_code: 32,
            sets: Vec::new()
        };
        cp.set_size_bounds(11, 1, u16::MAX, 1);
        cp
    }
    pub fn add_set(&mut self, set: Set) {
        self.sets.push(set);
    }
    pub fn clear_sets(&mut self) {
        self.sets.clear();
    }
}
impl OnPaint for CharPicker {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        // first paint the header
        let size = self.size();
        let col_text = match () {
            _ if !self.is_enabled() => theme.button.text.inactive,
            _ if self.has_focus() => theme.button.text.focused,
            _ if self.is_mouse_over() => theme.button.text.hovered,
            _ => theme.button.text.normal,
        };

        let space_char = Character::with_attributes(' ', col_text);
        // normal bar
        surface.fill_horizontal_line_with_size(0, self.header_y_ofs, size.width.saturating_sub(4), space_char);
        surface.write_char(1, self.header_y_ofs, Character::with_attributes(self.code, col_text));
        let mut arr: [u8; 9] = [b'(', b'U', b'+', b'0', b'0', b'0', b'0', b'0', b')'];
        let mut code = self.code as u32;
        let mut pos = 7;
        while (code > 0) && (pos > 2) {
            let r = (code % 16) as u8;
            if r < 10 {
                arr[pos] = 48 + r;
            } else {
                arr[pos] = 55 + r;
            }
            pos = pos - 1;
            code = code >> 4;
        }
        // paint code
        if size.width > 12 {
            if size.width < 17 {
                surface.write_ascii(3, self.header_y_ofs, &arr[3..8], col_text, false);
            } else {
                surface.write_ascii(3, self.header_y_ofs, arr.as_slice(), col_text, false);
            }
        }
        // drop button
        let px = (size.width - 3) as i32;
        surface.fill_horizontal_line_with_size(px, self.header_y_ofs, 3, space_char);
        surface.write_char(px + 1, self.header_y_ofs, Character::with_attributes(SpecialChar::TriangleDown, col_text));
        // assuming the control is expanded
        if self.is_expanded() {
            let size = self.expanded_size();
            let col = theme.menu.text.normal;
            surface.fill_rect(
                Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
                space_char,
            );
            surface.draw_rect(
                Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
                LineType::Single,
                col,
            );
            surface.draw_horizontal_line(1, self.expanded_panel_y + 2, size.width as i32 - 2, LineType::Single, col);
            let mut y = 4;
            let mut x = 0;
            let mut code = self.start_code;
            let end_code = code + 1000;
            loop {
                let ch = if (code == 128) || (code == 0) {
                    ' '
                } else {
                    char::from_u32(code).unwrap_or('?')
                };
                surface.write_char(x * 3 + 1, y, Character::with_attributes(ch, col));
                if code == self.code as u32 {
                    surface.fill_horizontal_line_with_size(x*3, y, 3, Character::with_attributes(0 as char, theme.menu.text.pressed_or_selectd));
                }
                x += 1;
                if x >= self.chars_per_width {
                    x = 0;
                    y += 1;
                    if ((y as u32) + 1) >= size.height {
                        break;
                    }
                }
                code += 1;
                if code >= end_code {
                    break;
                }
            }
        }
    }
}
impl OnDefaultAction for CharPicker {
    fn on_default_action(&mut self) {
        if self.is_expanded() {
            self.pack();
        } else {
            self.expand(Size::new(11, 7), Size::new(self.size().width, 9));
        }
    }
}
impl OnExpand for CharPicker {
    fn on_expand(&mut self, direction: ExpandedDirection) {
        match direction {
            ExpandedDirection::OnTop => {
                self.expanded_panel_y = 0;
                self.header_y_ofs = (self.expanded_size().height as i32) - 1;
            }
            ExpandedDirection::OnBottom => {
                self.expanded_panel_y = 1;
                self.header_y_ofs = 0;
            }
        }
        self.chars_per_width = (self.expanded_size().width / 3) as i32;
    }
    fn on_pack(&mut self) {
        self.expanded_panel_y = 1;
        self.header_y_ofs = 0;
    }
}
impl OnKeyPressed for CharPicker {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        let expanded = self.is_expanded();

        match key.value() {
            key!("Escape") => {
                if expanded {
                    self.pack();
                    return EventProcessStatus::Processed;
                } else {
                    return EventProcessStatus::Ignored;
                }
            }
            key!("Space") | key!("Enter") => {
                self.on_default_action();
                return EventProcessStatus::Processed;
            }
            // key!("Up") => {
            //     self.next_color(expanded, if expanded { -COLOR_MATRIX_WIDTH } else { -1 });
            //     return EventProcessStatus::Processed;
            // }
            // key!("Down") => {
            //     self.next_color(expanded, if expanded { COLOR_MATRIX_WIDTH } else { 1 });
            //     return EventProcessStatus::Processed;
            // }
            // key!("Left") => {
            //     self.next_color(expanded, -1);
            //     return EventProcessStatus::Processed;
            // }
            // key!("Right") => {
            //     self.next_color(expanded, 1);
            //     return EventProcessStatus::Processed;
            // }
            _ => {}
        }
        EventProcessStatus::Ignored
    }
}
impl OnMouseEvent for CharPicker {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            // MouseEvent::Enter => {
            //     if !self.is_expanded() && self.color.name().len() as i32 > ((self.size().width as i32) - 8) {
            //         self.show_tooltip(self.color.name())
            //     }
            //     EventProcessStatus::Processed
            // }

            // MouseEvent::Leave => {
            //     self.hide_tooltip();
            //     EventProcessStatus::Processed
            // }
            // MouseEvent::Over(p) => {
            //     let idx = self.mouse_to_color_index(p.x, p.y);
            //     if idx != self.mouse_on_color_index {
            //         self.mouse_on_color_index = idx;
            //         return EventProcessStatus::Processed;
            //     }
            //     EventProcessStatus::Ignored
            // }
            // MouseEvent::Pressed(data) => {
            //     let idx = self.mouse_to_color_index(data.x, data.y);
            //     if let Some(col) = Color::from_value(idx) {
            //         if col != self.color {
            //             self.color = col;
            //             self.raise_event(ControlEvent {
            //                 emitter: self.handle,
            //                 receiver: self.event_processor,
            //                 data: ControlEventData::CharPicker(EventData { color: col }),
            //             });
            //         }
            //     }
            //     self.on_default_action();
            //     EventProcessStatus::Processed
            // }
            _ => EventProcessStatus::Ignored,
        }
    }
}
