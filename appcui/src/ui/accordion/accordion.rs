use crate::prelude::*;
use crate::ui::tab::Flags;

#[CustomControl(overwrite=OnPaint+OnMouseEvent+OnKeyPressed+OnResize, internal=true)]
pub struct Accordion {
    flags: Flags,
    panels: Vec<Caption>,
    hovered_page_idx: Option<usize>,
}
impl Accordion {
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            hovered_page_idx: None,
            panels: Vec::with_capacity(4),
        }
    }
    fn update_margins(&mut self) {
        let ci = self.focused_child_index.index();
        let count = self.children.len();
        let h = self.size().height as usize;
        if ci < count {
            let bottom_elements = count - (ci + 1);
            if h > bottom_elements {
                self.set_margins(0, ci as u8, 0, bottom_elements as u8);
                self.request_update();
                return;
            }
        }
        // we can not paint the object so we will set up an invalid marging
        self.set_margins(0, 0, 0, h as u8); // invalid margins
        self.request_update();
    }
    #[inline(always)]
    fn get_panelattr(&self, theme: &Theme, idx: usize) -> (CharAttribute, CharAttribute) {
        if !self.is_enabled() {
            (theme.accordion.text.inactive, theme.accordion.hotkey.inactive)
        } else {
            if idx == self.focused_child_index.index() {
                (theme.accordion.text.pressed_or_selectd, theme.accordion.hotkey.pressed_or_selectd)
            } else {
                if let Some(hovered_idx) = self.hovered_page_idx {
                    if hovered_idx == idx {
                        (theme.accordion.text.hovered, theme.accordion.hotkey.hovered)
                    } else {
                        (theme.accordion.text.normal, theme.accordion.hotkey.normal)
                    }
                } else {
                    (theme.accordion.text.normal, theme.accordion.hotkey.normal)
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
    fn mouse_position_to_index(&self, x: i32, y: i32) -> Option<usize> {
        let sz = self.size();
        if (y < 0) || (x < 0) || (x >= sz.width as i32) {
            return None;
        }
        let count = self.base.children.len();
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
        let h = sz.height as i32;
        if h < bottom_index {
            return None;
        }
        if y >= (h - bottom_index) && (y < h) {
            Some(fc + 1 + ((h - bottom_index) as usize))
        } else {
            None
        }
    }
    pub fn add_panel(&mut self, caption: &str) -> u32 {
        let idx = self.base.children.len() as u32;
        self.base.add_child(super::AccordionPanel::new(idx == 0));
        self.panels.push(Caption::new(caption, ExtractHotKeyMethod::AltPlusKey));
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
            if let Some(tabpage) = cm.get_mut(h) {
                tabpage.get_base_mut().add_child(control)
            } else {
                Handle::None
            }
        } else {
            Handle::None
        }
    }
    #[inline(always)]
    pub fn current_panel(&self) -> Option<usize> {
        let idx = self.base.focused_child_index.index();
        if idx < self.base.children.len() {
            Some(idx)
        } else {
            None
        }
    }
    pub fn set_current_panel(&mut self, index: usize) {
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

    #[inline]
    pub fn panel_caption(&self, index: usize) -> Option<&str> {
        if index < self.panels.len() {
            Some(self.panels[index].text())
        } else {
            None
        }
    }
    pub fn set_panel_caption(&mut self, index: usize, caption: &str) {
        if index < self.panels.len() {
            self.panels[index].set_text(caption, ExtractHotKeyMethod::AltPlusKey);
        }
    }
}
impl OnPaint for Accordion {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if !self.flags.contains(Flags::TransparentBackground) {
            surface.clear(Character::with_attributes(' ', self.get_backattr(theme)));
        }
        let sz = self.size();
        let mut format = TextFormat {
            x: 1,
            y: 1,
            width: Some(if sz.width > 2 { (sz.width as u16) - 2 } else { 1 }),
            align: TextAlignament::Left,
            text_wrap: TextWrap::None,
            multi_line: false,
            ..Default::default()
        };

        let cidx = self.base.focused_child_index.index();
        let count = self.base.children.len();
        for (index, page) in self.panels.iter().enumerate() {
            let (text_attr, hotkey_attr) = self.get_panelattr(theme, index);
            format.chars_count = Some(page.chars_count() as u16);
            format.hotkey_pos = page.hotkey_pos();
            format.char_attr = text_attr;
            format.hotkey_attr = Some(hotkey_attr);
            // position
            if index <= cidx {
                format.y = index as i32;
            } else {
                format.y = (sz.height as i32) - ((count - index) as i32);
            }

            // fill the tab
            surface.fill_horizontal_line_with_size(0, format.y, sz.width as u32, Character::with_attributes(' ', text_attr));

            // write the text
            surface.write_text(page.text(), &format);
        }
    }
}
impl OnMouseEvent for Accordion {
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
                        self.set_current_panel(index);
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
impl OnKeyPressed for Accordion {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.get_compact_code() {
            key!("Ctrl+Tab") => {
                let mut idx = self.base.focused_child_index;
                idx.add(1, self.base.children.len(), Strategy::RotateFromInvalidState);
                self.set_current_panel(idx.index());
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Shift+Tab") => {
                let mut idx = self.base.focused_child_index;
                idx.sub(1, self.base.children.len(), Strategy::RotateFromInvalidState);
                self.set_current_panel(idx.index());
                return EventProcessStatus::Processed;
            }
            _ => {}
        }
        if key.modifier.contains(KeyModifier::Alt) {
            // check if a new tab was selected
            for (index, elem) in self.panels.iter().enumerate() {
                if elem.hotkey() == key {
                    self.set_current_panel(index);
                    return EventProcessStatus::Processed;
                }
            }
        }
        EventProcessStatus::Ignored
    }
}
impl OnResize for Accordion {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        self.update_margins();
    }
}