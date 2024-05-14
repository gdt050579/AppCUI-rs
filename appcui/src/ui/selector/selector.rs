use std::marker::PhantomData;

use super::EnumSelector;
use super::Flags;
use crate::prelude::*;

const MINSPACE_FOR_DRAWING: u32 = 5;
const MIN_WIDTH_VARIANT_NAME: u32 = 6;
const MINSPACE_FOR_DROPBUTTON_DRAWING: u32 = 3;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent+OnExpand, internal=true)]
pub struct Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    current_index: u32,
    header_y_ofs: i32,
    expanded_panel_y: i32,
    flags: Flags,
    _phanton: PhantomData<T>,
}
impl<T> Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    pub fn new(value: Option<T>, layout: Layout, flags: Flags) -> Self {
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            current_index: u32::MAX,
            header_y_ofs: 0,
            expanded_panel_y: 1,
            flags,
            _phanton: PhantomData,
        };
        if let Some(val) = value {
            let count = T::count();
            for i in 0..count {
                if T::from_index(i) == Some(val) {
                    obj.current_index = i;
                    break;
                }
            }
        } else {
            // value is None
            if !obj.flags.contains(Flags::AllowNoneVariant) {
                panic!(
                    "You can not instantiate a selector with `None` value without setting the flags `AllowNoneVariant`. Have you forgot to do this ?"
                );
            }
        }
        obj.set_size_bounds(7, 1, u16::MAX, 1);
        obj
    }
    #[inline(always)]
    pub fn value(&self) -> T {
        EnumSelector::from_index(self.current_index).unwrap()
    }
    #[inline(always)]
    pub fn try_value(&self) -> Option<T> {
        EnumSelector::from_index(self.current_index)
    }
}
impl<T> OnPaint for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
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
        if size.width > MINSPACE_FOR_DRAWING {
            surface.fill_horizontal_line(0, self.header_y_ofs, (size.width - MINSPACE_FOR_DRAWING) as i32, space_char);
            if size.width > MIN_WIDTH_VARIANT_NAME {
                let mut format = TextFormat::single_line(1, self.header_y_ofs, col_text, TextAlignament::Left);
                format.width = Some((size.width - MIN_WIDTH_VARIANT_NAME) as u16);
                if let Some(value) = T::from_index(self.current_index) {
                    surface.write_text(value.name(), &format);
                } else {
                    surface.write_text("None", &format);
                }
            }
        }
        if size.width >= MINSPACE_FOR_DROPBUTTON_DRAWING {
            let px = (size.width - MINSPACE_FOR_DROPBUTTON_DRAWING) as i32;
            surface.fill_horizontal_line_with_size(px, self.header_y_ofs, 3, space_char);
            surface.write_char(px + 1, self.header_y_ofs, Character::with_attributes(SpecialChar::TriangleDown, col_text));
        }
        // assuming the control is expanded
        if self.is_expanded() {
            let size = self.expanded_size();
            let col = theme.menu.text.normal;
            let mut space_char = Character::with_attributes(' ', col);
            surface.fill_rect(
                Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
                space_char,
            );
            surface.draw_rect(
                Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
                LineType::Single,
                col,
            );
        }
    }
}
impl<T> OnExpand for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
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
        //self.mouse_on_color_index = -1;
    }
    fn on_pack(&mut self) {
        self.expanded_panel_y = 1;
        self.header_y_ofs = 0;
    }
}
impl<T> OnDefaultAction for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_default_action(&mut self) {
        if self.is_expanded() {
            self.pack();
        } else {
            let w = self.size().width;
            let h = if self.flags.contains(Flags::AllowNoneVariant) {
                T::count() + 3
            } else {
                T::count() + 2
            };
            self.expand(Size::new(w, h.min(4)), Size::new(w, h));
        }
    }
}
impl<T> OnKeyPressed for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
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
impl<T> OnMouseEvent for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => {
                if !self.is_expanded() {
                    if let Some(value) = T::from_index(self.current_index) {
                        let desc = EnumSelector::description(&value);
                        if !desc.is_empty() {
                            self.show_tooltip(desc);
                        }
                    }
                }
                EventProcessStatus::Processed
            }

            MouseEvent::Leave => {
                self.hide_tooltip();
                EventProcessStatus::Processed
            }
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
            //                 data: ControlEventData::ColorPicker(EventData { color: col }),
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
