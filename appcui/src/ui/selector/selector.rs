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
    start_index: u32,
    current_index: u32,
    mouse_index: u32,
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
            start_index: 0,
            current_index: 0,
            header_y_ofs: 0,
            expanded_panel_y: 1,
            mouse_index: u32::MAX,
            flags,
            _phanton: PhantomData,
        };
        if let Some(val) = value {
            let count = T::COUNT;
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
            obj.current_index = T::COUNT;
        }
        if T::COUNT == 0 {
            panic!("You should have at least one variant in the enum associated with the seclector control !");
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

    fn visible_items(&self) -> u32 {
        let height = self.expanded_size().height;
        return if height > 3 { height - 3 } else { 1 };
    }
    fn update_current_index(&mut self, pos: u32) {
        let expanded_size = self.expanded_size();
        // there should be atleast one item visible
        let visible_items = if expanded_size.height > 3 { expanded_size.height - 3 } else { 1 };
        let count = T::COUNT;
        let last_item_index = if self.flags.contains(Flags::AllowNoneVariant) {
            count
        } else {
            count - 1
        };
        let last_current_index = self.current_index;
        self.current_index = pos.min(last_item_index);
        if self.start_index >= self.current_index {
            self.start_index = self.current_index;
        } else if self.start_index + visible_items <= self.current_index {
            self.start_index = self.current_index + 1 - visible_items;
        }
        if self.start_index + visible_items > (last_item_index + 1) {
            self.start_index = last_item_index + 1 - visible_items;
        }
        if last_current_index != self.current_index {
            // emit event
        }
    }
    fn mouse_pos_to_index(&self, x: i32, y: i32) -> u32 {
        if !self.is_expanded() {
            return u32::MAX;
        }
        let size = self.expanded_size();
        let visible_items = (if size.height > 3 { size.height - 3 } else { 1 }) as i32;
        if (x > 0) && (x < size.width as i32) && (y > self.expanded_panel_y) && (y <= self.expanded_panel_y + visible_items) {
            return self.start_index + (y - (self.expanded_panel_y + 1)) as u32;
        }
        return u32::MAX;
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
            let visible_items = if size.height > 3 { size.height - 3 } else { 1 };
            let col = theme.menu.text.normal;
            surface.fill_rect(
                Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
                Character::with_attributes(' ', col),
            );
            surface.draw_rect(
                Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
                LineType::Single,
                col,
            );
            let mut format = TextFormat::single_line(2, self.expanded_panel_y + 1, col_text, TextAlignament::Left);
            format.width = Some((size.width - 4) as u16);

            for i in self.start_index..self.start_index + visible_items {
                if let Some(value) = T::from_index(i) {
                    format.char_attr = theme.menu.text.normal;
                    surface.write_text(value.name(), &format);
                } else {
                    format.char_attr = theme.menu.text.inactive;
                    surface.write_text("None", &format);
                }
                if i == self.current_index {
                    surface.fill_horizontal_line(
                        1,
                        format.y,
                        (size.width - 2) as i32,
                        Character::with_attributes(0, theme.menu.text.hovered),
                    );
                } else if i == self.mouse_index {
                    surface.fill_horizontal_line(
                        1,
                        format.y,
                        (size.width - 2) as i32,
                        Character::with_attributes(0, theme.menu.hotkey.normal),
                    );
                }
                format.y += 1;
            }
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
        self.update_current_index(self.current_index);
        self.mouse_index = u32::MAX;
    }
    fn on_pack(&mut self) {
        self.expanded_panel_y = 1;
        self.header_y_ofs = 0;
        self.mouse_index = u32::MAX;
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
                T::COUNT + 4
            } else {
                T::COUNT + 3
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
            key!("Up") => {
                if self.current_index > 0 {
                    self.update_current_index(self.current_index - 1);
                };
                return EventProcessStatus::Processed;
            }
            key!("Down") => {
                self.update_current_index(self.current_index + 1);
                return EventProcessStatus::Processed;
            }
            key!("Home") => {
                self.update_current_index(0);
                return EventProcessStatus::Processed;
            }
            key!("End") => {
                self.update_current_index(u32::MAX);
                return EventProcessStatus::Processed;
            }
            key!("PageUp") => {
                let page_count = self.visible_items();
                if self.current_index > page_count {
                    self.update_current_index(self.current_index - page_count);
                } else {
                    self.update_current_index(0);
                }
                return EventProcessStatus::Processed;
            }
            key!("PageDown") => {
                self.update_current_index(self.current_index + self.visible_items());
                return EventProcessStatus::Processed;
            }
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
                self.mouse_index = u32::MAX;
                EventProcessStatus::Processed
            }
            MouseEvent::Over(p) => {
                let idx = self.mouse_pos_to_index(p.x, p.y);
                if idx != self.mouse_index {
                    self.mouse_index = idx;
                    return EventProcessStatus::Processed;
                }
                EventProcessStatus::Ignored
            }
            MouseEvent::Pressed(data) => {
                let idx = self.mouse_pos_to_index(data.x, data.y);
                if idx != u32::MAX {
                    self.update_current_index(idx);
                }
                self.on_default_action();
                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(direction) => {
                match direction {
                    MouseWheelDirection::Up => {
                        if self.current_index > 0 {
                            self.update_current_index(self.current_index - 1);
                        }
                    }
                    MouseWheelDirection::Down => self.update_current_index(self.current_index + 1),
                    _ => {}
                }
                EventProcessStatus::Processed
            }

            _ => EventProcessStatus::Ignored,
        }
    }
}
