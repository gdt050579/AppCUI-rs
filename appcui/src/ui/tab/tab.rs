use crate::prelude::*;
use crate::ui::tab::{Flags, Type};
use super::events::EventData;

#[CustomControl(overwrite=OnPaint+OnMouseEvent+OnKeyPressed, internal=true)]
pub struct Tab {
    tab_type: Type,
    flags: Flags,
    tab_width: u8,
    pages: Vec<Caption>,
    hovered_page_idx: Option<usize>,
}

impl Tab {
    /// Creates a new Tab control with the specified layout and flags.
    /// The flags can be a combination of the following values:
    /// * `tab::Flags::TabsBar` - if set, the tabs will be displayed in a bar
    /// * `tab::Flags::TransparentBackground` - if set, the background will be transparent
    /// 
    /// The type of the tab is `Type::OnTop` by default, which means that the tabs will be displayed on top of the control.
    /// 
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let mut tab = Tab::new(layout!("x:1,y:1,w:20,h:10"),
    ///                       tab::Flags::TabsBar);
    /// tab.add_tab("Tab &1");
    /// tab.add_tab("Tab &2");
    /// tab.add_tab("Tab &3");
    /// ```
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
    
    /// Creates a new Tab control with the specified layout, flags and type.
    /// The flags can be a combination of the following values:
    /// * `tab::Flags::TabsBar` - if set, the tabs will be displayed in a bar
    /// * `tab::Flags::TransparentBackground` - if set, the background will be transparent
    ///   and the tab_type will be one of the following values:
    /// * `tab::Type::OnTop` - the tabs will be displayed on top of the control
    /// * `tab::Type::OnBottom` - the tabs will be displayed on the bottom of the control
    /// * `tab::Type::OnLeft` - the tabs will be displayed on the left side of the control
    /// * `tab::Type::HiddenTabs` - the tabs will be hidden. You can still change between them , but manually.
    /// 
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let mut tab = Tab::with_type(layout!("x:1,y:1,w:20,h:10"), 
    ///                              tab::Flags::TabsBar, 
    ///                              tab::Type::OnTop);
    /// tab.add_tab("Tab &1");
    /// tab.add_tab("Tab &2");
    /// tab.add_tab("Tab &3");
    /// ```
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
    
    /// Adds a new tab page with the specified caption. The caption can contain a hotkey, which is indicated by an ampersand (&) before the character.
    /// The function returns the index of the newly created tab page.
    pub fn add_tab(&mut self, caption: &str) -> u32 {
        let idx = self.base.children.len() as u32;
        self.base.add_child(super::TabPage::new(idx == 0));
        self.pages.push(Caption::new(caption, ExtractHotKeyMethod::AltPlusKey));
        idx
    }
    
    /// Ads a new control to a tab page that is specified by the index.
    /// If the tab index is out of bounds, the function returns `Handle::None`. Otherwise, it returns a handle to the newly created control.
    /// 
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let mut tab = Tab::new(layout!("x:1,y:1,w:20,h:10"), tab::Flags::TabsBar);
    /// let idx = tab.add_tab("Tab 1");
    /// let handle = tab.add(idx, Button::new("Button 1", layout!("x:1,y:1,w:20,h:1"), button::Type::Flat));
    /// ```
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
    
    /// Returns the current tab index or `None` if there are no tabs added.
    #[inline(always)]
    pub fn current_tab(&self) -> Option<usize> {
        let idx = self.base.focused_child_index.index();
        if idx < self.base.children.len() {
            Some(idx)
        } else {
            None
        }
    }
    
    /// Sets the current tab to the specified index.
    /// The index must be valid and the control must be enabled and visible to receive input.
    pub fn set_current_tab(&mut self, index: usize) {
        self.internal_set_current_tab(index, false);
    }

    pub fn internal_set_current_tab(&mut self, index: usize, emit_event: bool) {
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
            if emit_event {
                self.raise_event(ControlEvent {
                    emitter: self.handle,
                    receiver: self.event_processor,
                    data: ControlEventData::Tab(EventData {
                        new_tab_index: index as u32,
                        old_tab_index: current_index as u32,
                    }),
                });
            }
        }
    }    
    
    /// Returns the width of the tabs.
    #[inline]
    pub fn tab_width(&self) -> u8 {
        self.tab_width
    }
    
    /// Sets the width of the tabs. The width must be between 3 and 32 characters.
    /// If the width is out of bounds, the function does nothing.
    pub fn set_tab_width(&mut self, width: u8) {
        if (3..=32).contains(&width) {
            self.tab_width = width;
            self.update_margins();
            self.request_update();
        }
    }
    
    /// Returns the caption of the tab at the specified index or `None` if the index is out of bounds.
    #[inline]
    pub fn tab_caption(&self, index: usize) -> Option<&str> {
        if index < self.pages.len() {
            Some(self.pages[index].text())
        } else {
            None
        }
    }
    
    /// Sets the caption of the tab at the specified index.
    /// The caption can contain a hotkey, which is indicated by an ampersand (&) before the character.
    /// If the index is out of bounds, the function does nothing.
    pub fn set_tab_caption(&mut self, index: usize, caption: &str) {
        if index < self.pages.len() {
            self.pages[index].set_text(caption, ExtractHotKeyMethod::AltPlusKey);
        }
    }
    fn update_margins(&mut self) {
        match self.tab_type {
            Type::HiddenTabs => self.base.set_margins(0, 0, 0, 0),
            Type::OnTop => self.base.set_margins(0, 1, 0, 0),
            Type::OnBottom => self.base.set_margins(0, 0, 0, 1),
            Type::OnLeft => self.base.set_margins(self.tab_width, 0, 0, 0),
        }
    }
    fn mouse_position_to_index(&self, x: i32, y: i32) -> Option<usize> {
        let count = self.base.children.len();
        if count == 0 {
            return None;
        }
        match self.tab_type {
            Type::HiddenTabs => None,
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
    fn get_tabsbarattr(&self, theme: &Theme) -> CharAttribute {
        match () {
            _ if !self.is_enabled() => theme.tab.text.inactive,
            _ => theme.tab.text.normal,
        }
    }
    #[inline(always)]
    fn get_tabattr(&self, theme: &Theme, idx: usize) -> (CharAttribute, CharAttribute) {
        if !self.is_enabled() {
            (theme.tab.text.inactive, theme.tab.hotkey.inactive)
        } else if idx == self.focused_child_index.index() {
            (theme.tab.text.pressed_or_selectd, theme.tab.hotkey.pressed_or_selectd)
        } else if let Some(hovered_idx) = self.hovered_page_idx {
            if hovered_idx == idx {
                (theme.tab.text.hovered, theme.tab.hotkey.hovered)
            } else {
                (theme.tab.text.normal, theme.tab.hotkey.normal)
            }
        } else {
            (theme.tab.text.normal, theme.tab.hotkey.normal)
        }
    }
    fn paint_horizontal_tab(&self, surface: &mut Surface, theme: &Theme, y: i32) {
        let mut format = TextFormatBuilder::new()
            .position(1, y)
            .wrap_type(WrapType::SingleLineWrap(self.tab_width as u16 - 2))
            .align(TextAlignment::Center)
            .build();

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
            surface.fill_horizontal_line_with_size(0, y, sz.width, Character::with_attributes(' ', self.get_tabsbarattr(theme)));
        }

        let s1 = (self.tab_width as i32) >> 1;
        let s2 = (self.tab_width as i32) - s1;
        for (index, page) in self.pages.iter().enumerate() {
            let (text_attr, hotkey_attr) = self.get_tabattr(theme, index);
            format.set_attribute(text_attr);
            format.set_chars_count(page.chars_count() as u16);
            format.set_hotkey_from_caption(hotkey_attr, page);

            // fill the tab
            surface.fill_horizontal_line_with_size(format.x, y, self.tab_width as u32, Character::with_attributes(' ', text_attr));

            // print the text
            format.x += s1;
            surface.write_text(page.text(), &format);
            format.x += s2 + 1;
        }
    }
    fn paint_leftside_tab(&self, surface: &mut Surface, theme: &Theme) {
        let sz = self.size();
        if !self.flags.contains(Flags::TransparentBackground) {
            let fill_char = Character::with_attributes(' ', self.get_backattr(theme));
            surface.fill_rect(Rect::new(self.tab_width as i32, 0, sz.width as i32, sz.height as i32), fill_char);
        }

        if self.flags.contains(Flags::TabsBar) {
            surface.fill_rect(
                Rect::new(0, 0, (self.tab_width as i32) - 1, sz.height as i32),
                Character::with_attributes(' ', self.get_tabsbarattr(theme)),
            );
        }
        let mut format = TextFormatBuilder::new()
            .position(1, 1)
            .wrap_type(WrapType::SingleLineWrap(self.tab_width as u16 - 2))
            .align(TextAlignment::Left)
            .build();

        for (index, page) in self.pages.iter().enumerate() {
            let (text_attr, hotkey_attr) = self.get_tabattr(theme, index);
            format.set_attribute(text_attr);
            format.set_chars_count(page.chars_count() as u16);
            format.set_hotkey_from_caption(hotkey_attr, page);

            // fill the tab
            surface.fill_horizontal_line_with_size(0, format.y, self.tab_width as u32, Character::with_attributes(' ', text_attr));

            // write the text
            surface.write_text(page.text(), &format);
            // next pos
            format.y += 1;
        }
    }
    fn paint_hidden_tabs(&self, surface: &mut Surface, theme: &Theme) {
        if !self.flags.contains(Flags::TransparentBackground) {
            surface.clear(Character::with_attributes(' ', self.get_backattr(theme)));
        }
    }
}

impl OnPaint for Tab {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        match self.tab_type {
            Type::HiddenTabs => self.paint_hidden_tabs(surface, theme),
            Type::OnTop => self.paint_horizontal_tab(surface, theme, 0),
            Type::OnBottom => self.paint_horizontal_tab(surface, theme, (self.size().height as i32) - 1),
            Type::OnLeft => self.paint_leftside_tab(surface, theme),
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
                        self.internal_set_current_tab(index, true);
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
        match key.value() {
            key!("Ctrl+Tab") => {
                let mut idx = self.base.focused_child_index;
                idx.add(1, self.base.children.len(), Strategy::RotateFromInvalidState);
                self.internal_set_current_tab(idx.index(), true);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Shift+Tab") => {
                let mut idx = self.base.focused_child_index;
                idx.sub(1, self.base.children.len(), Strategy::RotateFromInvalidState);
                self.internal_set_current_tab(idx.index(), true);
                return EventProcessStatus::Processed;
            }
            _ => {}
        }
        if key.modifier.contains(KeyModifier::Alt) {
            // check if a new tab was selected
            for (index, elem) in self.pages.iter().enumerate() {
                if elem.hotkey() == key {
                    self.internal_set_current_tab(index, true);
                    return EventProcessStatus::Processed;
                }
            }
        }
        EventProcessStatus::Ignored
    }
}
