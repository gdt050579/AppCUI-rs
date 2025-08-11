use crate::prelude::*;
use crate::ui::charpicker::events::EventData;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent+OnExpand, internal=true)]
pub struct CharPicker {
    code: char,
    header_y_ofs: i32,
    expanded_panel_y: i32,
    mouse_on_color_index: i32,
}
impl CharPicker {
    pub fn new(code: char, layout: Layout) -> Self {
        let mut cp = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            header_y_ofs: 0,
            expanded_panel_y: 1,
            mouse_on_color_index: -1,
            code,
        };
        cp.set_size_bounds(7, 1, u16::MAX, 1);
        cp
    }    
}
impl OnPaint for CharPicker {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {

    }
}
impl OnDefaultAction for CharPicker {
    fn on_default_action(&mut self) {
        if self.is_expanded() {
            self.pack();
        } else {
            // self.expand(
            //     Size::new((TRANSPARENT_CHECKBOX_X_LAST_OFFSET as u32) + 2, 7),
            //     Size::new(self.size().width, 7),
            // );
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
        self.mouse_on_color_index = -1;
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
