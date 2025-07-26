use crate::prelude::*;
use crate::ui::accordion::events::EventData;
use crate::ui::accordion::Flags;

#[CustomControl(overwrite=OnPaint+OnMouseEvent+OnKeyPressed+OnResize, internal=true)]
pub struct Accordion {
    flags: Flags,
    panels: Vec<Caption>,
    hovered_page_idx: Option<usize>,
}
impl Accordion {
    /// Creates a new Accordion control with the specified `layout` and `flags`.
    /// The flags parameter is a bitmask tthat contains the following flags:
    /// - `TransparentBackground`: If set, the background of the accordion will be transparent.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use appcui::prelude::*;
    /// let mut ac = Accordion::new(layout!("x:1,y:1,w:15,h:10"), accordion::Flags::None);
    /// ac.add_panel("Panel 1");
    /// ac.add_panel("Panel 2");
    /// ac.add_panel("Panel 3");
    /// ```
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            hovered_page_idx: None,
            panels: Vec::with_capacity(4),
        }
    }
    fn update_margins_for(&mut self, index: usize) {
        let count = self.children.len();
        let h = self.size().height as usize;
        if index < count {
            let bottom_elements = count - (index + 1);
            if h > bottom_elements {
                self.set_margins(0, (index + 1) as u8, 0, bottom_elements as u8);
                self.request_update();
                return;
            }
        }
        // we can not paint the object so we will set up an invalid marging
        self.set_margins(0, 0, 0, h as u8); // invalid margins
        self.request_update();
    }
    fn update_margins(&mut self) {
        self.update_margins_for(self.focused_child_index.index());
    }
    #[inline(always)]
    fn panelattr(&self, theme: &Theme, idx: usize) -> (CharAttribute, CharAttribute) {
        if !self.is_enabled() {
            (theme.accordion.text.inactive, theme.accordion.hotkey.inactive)
        } else if idx == self.focused_child_index.index() {
            (theme.accordion.text.pressed_or_selectd, theme.accordion.hotkey.pressed_or_selectd)
        } else if let Some(hovered_idx) = self.hovered_page_idx {
            if hovered_idx == idx {
                (theme.accordion.text.hovered, theme.accordion.hotkey.hovered)
            } else {
                (theme.accordion.text.normal, theme.accordion.hotkey.normal)
            }
        } else {
            (theme.accordion.text.normal, theme.accordion.hotkey.normal)
        }
    }
    #[inline(always)]
    fn backattr(&self, theme: &Theme) -> CharAttribute {
        match () {
            _ if !self.is_enabled() => theme.accordion.text.inactive,
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
        if fc >= count {
            return None;
        }
        // check top allignament
        if y as usize <= fc {
            return Some(y as usize);
        }
        // check bottom allignament
        let bottom_index = (count - fc) as i32;
        let h = sz.height as i32;
        if h < bottom_index {
            return None;
        }
        if y >= (h - bottom_index) && (y < h) {
            Some(fc + ((y - h + bottom_index) as usize))
        } else {
            None
        }
    }

    /// Adds a new panel to the accordion with the given `caption` parameter.
    /// The `caption` parameter is the text that will be displayed on the panel.
    /// This method returns the index of the newly created panel.
    pub fn add_panel(&mut self, caption: &str) -> u32 {
        let idx = self.base.children.len() as u32;
        self.base.add_child(super::AccordionPanel::new(idx == 0));
        self.panels.push(Caption::new(caption, ExtractHotKeyMethod::AltPlusKey));
        if idx == 0 {
            self.update_margins_for(0);
        }
        idx
    }

    /// Adds a new control to the accordion panel designated by the `tabindex` parameter.
    /// The `tabindex` parameter must be a valid index of the accordion panels.
    /// This method returns a handle to the control that was added.
    #[inline(always)]
    pub fn add<T>(&mut self, tabindex: u32, control: T) -> Handle<T>
    where
        T: Control + NotWindow + NotDesktop + 'static,
    {
        if (tabindex as usize) < self.base.children.len() {
            let h = self.base.children[tabindex as usize];
            let cm = RuntimeManager::get().get_controls_mut();
            if let Some(tabpage) = cm.get_mut(h) {
                tabpage.base_mut().add_child(control)
            } else {
                Handle::None
            }
        } else {
            Handle::None
        }
    }
    /// Returns the current panel index or None if the accordion is empty.
    /// The index is the index of the panel that is currently selected.
    #[inline(always)]
    pub fn current_panel(&self) -> Option<usize> {
        let idx = self.base.focused_child_index.index();
        if idx < self.base.children.len() {
            Some(idx)
        } else {
            None
        }
    }
    /// Sets the current panel to the one with the given `index` parameter.
    /// The `index` parameter must be a valid index of the accordion panels.
    pub fn set_current_panel__(&mut self, index: usize) {
        self.internal_set_current_panel(index, false);
    }

    pub fn internal_set_current_panel(&mut self, index: usize, emit_event: bool) {
        // Q: what is the tab is disabled ? can it still change a page
        // for the moment we will not allow this behavior
        // meaning that the tab must be able to receive focus (be visibale and enabled) in order to be able to change the page
        if !self.can_receive_input() {
            return;
        }
        let mut idx = None;
        let current_index = self.base.focused_child_index.index();
        if (index < self.base.children.len()) && (index != self.base.focused_child_index.index()) {
            // its a different page (valid)
            let cm = RuntimeManager::get().get_controls_mut();
            for (child_index, handle_child) in self.base.children.iter().enumerate() {
                if let Some(control) = cm.get_mut(*handle_child) {
                    control.base_mut().set_visible(index == child_index);
                    if index == child_index {
                        control.base_mut().request_focus();
                        idx = Some(index);
                    }
                }
            }
        }
        if let Some(index) = idx {
            self.update_margins_for(index);
            if emit_event {
                self.raise_event(ControlEvent {
                    emitter: self.handle,
                    receiver: self.event_processor,
                    data: ControlEventData::Accordion(EventData {
                        new_panel_index: index as u32,
                        old_panel_index: current_index as u32,
                    }),
                });
            }
        }
    }

    /// Returns the caption o a giverm panel from the accordion or None if the `index` parameter is invalid
    #[inline]
    pub fn panel_caption(&self, index: usize) -> Option<&str> {
        if index < self.panels.len() {
            Some(self.panels[index].text())
        } else {
            None
        }
    }
    /// Sets the panel caption for a given panel from the accordion that is refered by its index.
    pub fn set_panel_caption(&mut self, index: usize, caption: &str) {
        if index < self.panels.len() {
            self.panels[index].set_text(caption, ExtractHotKeyMethod::AltPlusKey);
        }
    }
}
impl OnPaint for Accordion {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if !self.flags.contains(Flags::TransparentBackground) {
            surface.clear(Character::with_attributes(' ', self.backattr(theme)));
        }
        let sz = self.size();
        // let mut format = TextFormat {
        //     x: 1,
        //     y: 1,
        //     width: Some(if sz.width > 2 { (sz.width as u16) - 2 } else { 1 }),
        //     align: TextAlignment::Left,
        //     text_wrap: TextWrap::Character,
        //     multi_line: false,
        //     ..Default::default()
        // };
        let mut format = TextFormatBuilder::new()
            .position(1, 1)
            .wrap_type(WrapType::SingleLineWrap(if sz.width > 2 { (sz.width as u16) - 2 } else { 1 }))
            .align(TextAlignment::Left)
            .build();

        let cidx = self.base.focused_child_index.index();
        let count = self.base.children.len();
        for (index, page) in self.panels.iter().enumerate() {
            let (text_attr, hotkey_attr) = self.panelattr(theme, index);
            format.set_chars_count(page.chars_count() as u16);
            format.set_attribute(text_attr);
            format.set_hotkey_from_caption(hotkey_attr, page);

            // position
            if index <= cidx {
                format.y = index as i32;
            } else {
                format.y = (sz.height as i32) - ((count - index) as i32);
            }

            // fill the tab
            surface.fill_horizontal_line_with_size(0, format.y, sz.width, Character::with_attributes(' ', text_attr));

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
                        self.internal_set_current_panel(index, true);
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
        match key.value() {
            key!("Ctrl+Tab") => {
                let mut idx = self.base.focused_child_index;
                idx.add(1, self.base.children.len(), Strategy::RotateFromInvalidState);
                self.internal_set_current_panel(idx.index(), true);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Shift+Tab") => {
                let mut idx = self.base.focused_child_index;
                idx.sub(1, self.base.children.len(), Strategy::RotateFromInvalidState);
                self.internal_set_current_panel(idx.index(), true);
                return EventProcessStatus::Processed;
            }
            _ => {}
        }
        if key.modifier.contains(KeyModifier::Alt) {
            // check if a new tab was selected
            for (index, elem) in self.panels.iter().enumerate() {
                if elem.hotkey() == key {
                    self.internal_set_current_panel(index, true);
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
