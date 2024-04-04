use crate::prelude::*;
use crate::ui::tab::{Flags, Type};

#[CustomControl(overwrite=OnPaint+OnMouseEvent+OnKeyPressed, internal=true)]
pub struct Tab {
    tab_type: Type,
    flags: Flags,
    tab_width: u8,
    pages: Vec<Caption>,
    hovered_page_idx: Option<usize>,
}

impl Tab {
    pub fn new(layout: Layout, flags: Flags) -> Self {
        let mut t = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            tab_type: Type::OnTop,
            flags,
            tab_width: 12,
            hovered_page_idx: None,
            pages: Vec::with_capacity(4),
        };
        t.update_margins();
        t
    }
    pub fn with_type(layout: Layout, flags: Flags, tab_type: Type) -> Self {
        let mut t = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            tab_type,
            flags,
            tab_width: 12,
            hovered_page_idx: None,
            pages: Vec::with_capacity(4),
        };
        t.update_margins();
        t
    }
    pub fn add_tab(&mut self, caption: &str) -> u32 {
        let idx = self.base.children.len() as u32;
        self.base.add_child(super::TabPage::new(idx == 0));
        self.pages.push(Caption::new(caption, ExtractHotKeyMethod::AltPlusKey));
        idx
    }
    #[inline(always)]
    pub fn add<T>(&mut self, tabindex: u32, control: T) -> Handle<T>
    where
        T: Control + NotWindow + NotDesktop + 'static,
    {
        if (tabindex as usize) < self.base.children.len() {
            let h = self.base.children[tabindex as usize];
            let cm = RuntimeManager::get().get_controls_mut();            
            if let Some(tabpage) = cm.get_mut(h)
            {
                tabpage.get_base_mut().add_child(control)
            } else {
                Handle::None
            }
        } else {
            Handle::None
        }
    }
    pub fn set_current_tabindex(&mut self, index: usize) {
        // Q: what is the tab is disabled ? can it still change a page
        // for the moment we will not allow this behavior
        // meaning that the tab must be able to receive focus (be visibale and enabled) in order to be able to change the page
        if !self.can_receive_input() {
            return;
        }
        if (index < self.base.children.len()) && (index != self.base.focused_child_index.index()) {
            // its a different page (valid)
            let cm = RuntimeManager::get().get_controls_mut();
            for (child_index, handle_child) in self.base.children.iter().enumerate() {
                if let Some(control) = cm.get_mut(*handle_child) {
                    control.get_base_mut().set_visible(index == child_index);
                    if index == child_index {
                        control.get_base_mut().request_focus();
                    }
                }
            }
        }
    }
    fn update_margins(&mut self) {
        match self.tab_type {
            Type::Hidden => self.base.set_margins(0, 0, 0, 0),
            Type::OnTop => self.base.set_margins(0, 1, 0, 0),
            Type::OnBottom => self.base.set_margins(0, 0, 0, 1),
            Type::OnLeft => self.base.set_margins(self.tab_width, 0, 0, 0),
            Type::List => {
                let idx = self.base.focused_child_index.index();
                let cnt = self.base.children.len();
                if idx < cnt {
                    self.base.set_margins(0, 1 + idx as u8, 0, (cnt - (idx + 1)) as u8);
                } else {
                    self.base.set_margins(0, 0, 0, 0);
                }
            }
        }
    }
    fn mouse_position_to_index(&self, x: i32, y: i32) -> Option<usize> {
        let count = self.base.children.len();
        if count == 0 {
            return None;
        }
        match self.tab_type {
            Type::Hidden => None,
            Type::OnTop => {
                if (y != 0) || (x < 1) {
                    return None;
                }
                let idx = (x as usize - 1) / ((self.tab_width as usize) + 1usize);
                if idx >= count {
                    return None;
                }
                Some(idx)
            }
            Type::OnBottom => {
                if (y != self.size().height as i32 - 1) || (x < 1) {
                    return None;
                }
                let idx = (x as usize - 1) / ((self.tab_width as usize) + 1usize);
                if idx >= count {
                    return None;
                }
                Some(idx)
            }
            Type::OnLeft => {
                if (x < 0) || (x > self.tab_width as i32) || (y < 1) {
                    return None;
                }
                let idx = y as usize - 1;
                if idx >= count {
                    return None;
                }
                Some(idx)
            }
            Type::List => {
                if y < 0 {
                    return None;
                }
                let fc = self.base.focused_child_index.index();
                // check top allignament
                if y as usize <= fc {
                    return Some(y as usize);
                }
                if fc >= count {
                    return None;
                }
                // check bottom allignament
                let bottom_index = (count - fc) as i32;
                let h = self.size().height as i32;
                if h < bottom_index {
                    return None;
                }
                if y >= (h - bottom_index) && (y < h) {
                    Some(fc + 1 + ((h - bottom_index) as usize))
                } else {
                    None
                }
            }
        }
    }
    #[inline(always)]
    fn get_backattr(&self, theme: &Theme) -> CharAttribute {
        match () {
            _ if !self.is_enabled() => theme.tab.text.inactive,
            _ if self.has_focus() => theme.tab.text.pressed_or_selectd,
            _ => theme.tab.text.pressed_or_selectd,
        }
    }
    #[inline(always)]
    fn get_barattr(&self, theme: &Theme) -> CharAttribute {
        match () {
            _ if !self.is_enabled() => theme.tab.text.inactive,
            _ if self.has_focus() => theme.tab.text.hovered,
            _ => theme.tab.text.normal,
        }
    }
    #[inline(always)]
    fn get_tabattr(&self, theme: &Theme, idx: usize) -> (CharAttribute, CharAttribute) {
        if !self.is_enabled() {
            (theme.tab.text.inactive, theme.tab.hotkey.inactive)
        } else if self.has_focus() {
            if idx == self.focused_child_index.index() {
                (theme.tab.text.pressed_or_selectd, theme.tab.hotkey.pressed_or_selectd)
            } else {
                if let Some(hovered_idx) = self.hovered_page_idx {
                    if hovered_idx == idx {
                        (theme.tab.text.hovered, theme.tab.hotkey.hovered)
                    } else {
                        (theme.tab.text.normal, theme.tab.hotkey.normal)
                    }
                } else {
                    (theme.tab.text.normal, theme.tab.hotkey.normal)
                }
            }
        } else {
            (theme.tab.text.normal, theme.tab.hotkey.normal)
        }
    }
    fn paint_horizontal_tab(&self, surface: &mut Surface, theme: &Theme, y: i32) {
        let mut format = TextFormat {
            x: 1,
            y,
            width: Some(self.tab_width as u16 - 2),
            align: TextAlignament::Center,
            text_wrap: TextWrap::None,
            multi_line: false,
            ..Default::default()
        };

        let sz = self.size();
        if !self.flags.contains(Flags::TransparentBackground) {
            let fill_char = Character::with_attributes(' ', self.get_backattr(theme));
            if y == 0 {
                surface.fill_rect(Rect::new(0, 1, sz.width as i32, sz.height as i32), fill_char);
            } else {
                surface.fill_rect(Rect::new(0, 0, sz.width as i32, sz.height as i32 - 2), fill_char);
            }
        }

        if self.flags.contains(Flags::TabsBar) {
            surface.fill_horizontal_line_with_size(0, y, sz.width, Character::with_attributes(' ', self.get_barattr(theme)));
        }

        let s1 = (self.tab_width as i32) >> 1;
        let s2 = (self.tab_width as i32) - s1;
        for (index, page) in self.pages.iter().enumerate() {
            let (text_attr, hotkey_attr) = self.get_tabattr(theme, index);
            format.chars_count = Some(page.chars_count() as u16);
            format.hotkey_pos = page.hotkey_pos();
            format.char_attr = text_attr;
            format.hotkey_attr = Some(hotkey_attr);

            // fill the tab
            surface.fill_horizontal_line_with_size(format.x, y, self.tab_width as u32, Character::with_attributes(' ', text_attr));

            // print the text
            format.x += s1;
            surface.write_text(page.text(), &format);
            format.x += s2 + 1;
        }
    }
}

impl OnPaint for Tab {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        match self.tab_type {
            Type::Hidden => todo!(),
            Type::OnTop => self.paint_horizontal_tab(surface, theme, 0),
            Type::OnBottom => todo!(),
            Type::OnLeft => todo!(),
            Type::List => todo!(),
        }
    }
}
impl OnMouseEvent for Tab {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => EventProcessStatus::Ignored,
            MouseEvent::Leave => {
                if self.hovered_page_idx.is_some() {
                    self.hovered_page_idx = None;
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            MouseEvent::Over(ev) => {
                let idx = self.mouse_position_to_index(ev.x, ev.y);
                if idx != self.hovered_page_idx {
                    self.hovered_page_idx = idx;
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            MouseEvent::Pressed(ev) => {
                let idx = self.mouse_position_to_index(ev.x, ev.y);
                if let Some(index) = idx {
                    if index != self.base.focused_child_index.index() {
                        self.set_current_tabindex(index);
                        EventProcessStatus::Processed
                    } else {
                        EventProcessStatus::Ignored
                    }
                } else {
                    EventProcessStatus::Ignored
                }
            }
            MouseEvent::Released(_) => EventProcessStatus::Ignored,
            MouseEvent::DoubleClick(_) => EventProcessStatus::Ignored,
            MouseEvent::Drag(_) => EventProcessStatus::Ignored,
            MouseEvent::Wheel(_) => EventProcessStatus::Ignored,
        }
    }
}
impl OnKeyPressed for Tab {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.get_compact_code() {
            key!("Ctrl+Tab") => {
                let mut idx = self.base.focused_child_index;
                idx.add(1, self.base.children.len(), Strategy::RotateFromInvalidState);
                self.set_current_tabindex(idx.index());
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Shift+Tab") => {
                let mut idx = self.base.focused_child_index;
                idx.sub(1, self.base.children.len(), Strategy::RotateFromInvalidState);
                self.set_current_tabindex(idx.index());
                return EventProcessStatus::Processed;
            }
            _ => {}
        }
        if key.modifier.contains(KeyModifier::Alt) {
            // check if a new tab was selected
            for (index, elem) in self.pages.iter().enumerate() {
                if elem.hotkey() == key {
                    self.set_current_tabindex(index);
                    return EventProcessStatus::Processed;
                }
            }
        }
        EventProcessStatus::Ignored
    }
}
