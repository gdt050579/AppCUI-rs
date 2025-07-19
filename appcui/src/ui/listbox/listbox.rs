use super::events::EventData;
use super::Flags;
use super::Item;
use crate::ui::components::ListScrollBars;
use listbox::events::ListBoxEventTypes;
use appcui_proc_macro::*;

#[CustomControl(overwrite = OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal = true)]
pub struct ListBox {
    items: Vec<Item>,
    flags: Flags,
    top_view: usize,
    left_view: usize,
    pos: usize,
    max_chars: u32,
    comp: ListScrollBars,
    empty_message: String,
}
impl ListBox {
    /// Creates a new list box with the specified layout and flags
    /// The flags can be a combination of the following values:
    /// - `Flags::ScrollBars` - adds scrollbars to the list box
    /// - `Flags::CheckBoxes` - adds checkboxes to the list box
    /// - `Flags::SearchBar` - adds a search bar to the list box
    /// - `Flags::AutoScroll` - automatically scrolls to the last item when a new item is added
    /// - `Flags::HighlightSelectedItemWhenInactive` - highlights the selected item even when the listbox is not active
    ///
    /// # Example
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let lbox = ListBox::new(Layout::new("d:c"),
    ///                         listbox::Flags::ScrollBars |
    ///                         listbox::Flags::CheckBoxes |
    ///                         listbox::Flags::SearchBar);
    /// let simple_lbox = ListBox::new(Layout::new("d:c"),listbox::Flags::None);
    /// ```
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self::with_capacity(0, layout, flags)
    }

    /// Creates a new list box with the specified layout, flags and capacity
    /// The flags can be a combination of the following values:
    /// - `Flags::ScrollBars` - adds scrollbars to the list box
    /// - `Flags::CheckBoxes` - adds checkboxes to the list box
    /// - `Flags::SearchBar` - adds a search bar to the list box
    /// - `Flags::AutoScroll` - automatically scrolls to the last item when a new item is added
    /// - `Flags::HighlightSelectedItemWhenInactive` - highlights the selected item even when the listbox is not active
    ///
    /// # Example
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// // a listbox with a capacity of 100 items, with scrollbars
    /// let lbox = ListBox::with_capacity(100,
    ///                                   Layout::new("d:c"),
    ///                                   listbox::Flags::ScrollBars);
    /// ```   
    pub fn with_capacity(capacity: usize, layout: Layout, flags: Flags) -> Self {
        let mut status_flags = StatusFlags::Enabled | StatusFlags::Visible | StatusFlags::AcceptInput;
        if flags.contains(Flags::ScrollBars) {
            status_flags |= StatusFlags::IncreaseBottomMarginOnFocus;
            status_flags |= StatusFlags::IncreaseRightMarginOnFocus;
        }
        if flags.contains(Flags::SearchBar) {
            status_flags |= StatusFlags::IncreaseBottomMarginOnFocus;
        }
        Self {
            base: ControlBase::with_status_flags(layout, status_flags),
            items: if capacity == 0 { Vec::new() } else { Vec::with_capacity(capacity) },
            top_view: 0,
            left_view: 0,
            max_chars: 0,
            pos: usize::MAX,
            flags,
            empty_message: String::new(),
            comp: ListScrollBars::new(flags.contains(Flags::ScrollBars), flags.contains(Flags::SearchBar)),
        }
    }

    /// Adds a new item to the list by providing a string value
    /// if AutoScroll flag is set, the list will automatically scroll to the newly added item
    pub fn add(&mut self, value: &str) {
        self.add_item(listbox::Item::new(value, false));
    }

    /// Adds a new item to the list by providing a string value and a checked flag
    /// if AutoScroll flag is set, the list will automatically scroll to the newly added item
    pub fn add_item(&mut self, item: listbox::Item) {
        self.items.push(item);
        if self.items.len() == 1 {
            self.max_chars = self.items[0].count;
            // when first item is added, we should select it
            self.update_position(0usize, false);
            self.top_view = 0; // force the view to start from the first item
        } else {
            self.max_chars = self.max_chars.max(self.items.last().unwrap().count);
            self.update_scrollbars();
        }
        // recompute the scroll bars
        let extra = if self.flags.contains(Flags::CheckBoxes) { 2 } else { 0 };
        self.comp.resize(self.max_chars as u64 + extra, self.items.len() as u64, &self.base, self.size());
        // if auto scroll is enabled, we should scroll to the last item
        if self.flags.contains(Flags::AutoScroll) {
            self.update_position(self.items.len() - 1, false);
        }
    }

    /// Clers all items from the list
    #[inline(always)]
    pub fn clear(&mut self) {
        self.items.clear();
        self.top_view = 0;
        self.pos = usize::MAX;
        self.max_chars = 0;
        self.comp.resize(0, 0, &self.base, self.size());
    }

    /// Returns the item from the listbox at the specified index
    #[inline(always)]
    pub fn item(&self, index: usize) -> Option<&Item> {
        self.items.get(index)
    }

    /// Returns the index of the current selected item from the listbox
    #[inline(always)]
    pub fn index(&self) -> usize {
        self.pos
    }

    /// Sets the new selected item from the listbox
    /// if the index is invalid, the function will do nothing
    pub fn set_index(&mut self, index: usize) {
        if index < self.items.len() {
            self.update_position(index, false);
        }
    }

    /// Returns the total number of items fom the listbox
    #[inline(always)]
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Returns the number of checked items from the listbox
    /// This function is only relevant if the listbox was created with the CheckBoxes flag
    /// If the CheckBoxes flag is not set, this function will always return 0
    /// This method will iterate through all items from the listbox, so it might be slow for large lists
    ///
    /// # Example
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let mut lbox = ListBox::new(Layout::new("a:c,w:100%,h:100%"), listbox::Flags::CheckBoxes);
    /// lbox.add_item(listbox::Item::new("Item 1", false));
    /// lbox.add_item(listbox::Item::new("Item 2", true));
    /// lbox.add_item(listbox::Item::new("Item 3", false));
    /// lbox.add_item(listbox::Item::new("Item 4", true));
    /// let count = lbox.count_checked();
    /// // we should have 2 checked items
    /// ```
    pub fn count_checked(&self) -> usize {
        if self.flags.contains(Flags::CheckBoxes) {
            self.items.iter().filter(|i| i.checked).count()
        } else {
            0
        }
    }

    /// Sets the empty message that will be displayed when the listbox is empty
    pub fn set_empty_message(&mut self, message: &str) {
        self.empty_message.clear();
        self.empty_message.push_str(message);
    }

    /// Sorts the items from the listbox (alphabetically)
    /// The current selected item will remain the same
    pub fn sort(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let current_value = self.items[self.pos].value.clone();
        self.items.sort_by(|a, b| a.value.cmp(&b.value));
        self.update_position(self.items.iter().position(|i| i.value == current_value).unwrap_or(0), false);
    }

    /// Sorts the items from the listbox using a custom function to compare two items
    /// The current selected item will remain the same
    pub fn sort_by<F>(&mut self, f: F)
    where
        F: FnMut(&Item, &Item) -> std::cmp::Ordering,
    {
        if self.items.is_empty() {
            return;
        }
        let current_value = self.items[self.pos].value.clone();
        self.items.sort_by(f);
        self.update_position(self.items.iter().position(|i| i.value == current_value).unwrap_or(0), false);
    }

    fn update_scrollbars(&mut self) {
        self.comp.set_indexes(self.left_view as u64, self.top_view as u64);
    }
    fn update_left_position_for_items(&mut self) {
        let len = self.items.len();
        if len == 0 {
            return;
        }
        let last_index = (len - 1).min(self.top_view + self.size().height as usize);
        for i in self.items[self.top_view..=last_index].iter_mut() {
            i.update_left_pos(self.left_view as u32);
        }
    }
    fn update_position(&mut self, new_pos: usize, emit_event: bool) {
        let len = self.items.len();
        if len == 0 {
            return;
        }
        let new_pos = new_pos.min(len - 1);
        let h = self.size().height as usize;

        // check the top view
        if self.top_view + h >= len {
            self.top_view = len.saturating_sub(h);
        }
        if new_pos < self.top_view {
            self.top_view = new_pos;
        } else {
            let diff = new_pos - self.top_view;
            if (diff >= h) && (h > 0) {
                self.top_view = new_pos - h + 1;
            }
        }
        // update scrollbars
        self.update_scrollbars();
        self.update_left_position_for_items();
        let should_emit = (self.pos != new_pos) && emit_event;
        self.pos = new_pos;
        if should_emit {
            self.raise_event(ControlEvent {
                emitter: self.handle,
                receiver: self.event_processor,
                data: ControlEventData::ListBox(EventData {
                    event_type: ListBoxEventTypes::CurrentItemChanged,
                    index: new_pos,
                    checked: false, // not relevant for this event
                }),
            });
        }
    }

    fn mouse_to_pos(&self, x: i32, y: i32) -> Option<usize> {
        let size = self.size();
        if x < 0 || y < 0 || x >= size.width as i32 || y >= size.height as i32 {
            return None;
        }
        let idx = self.top_view + y as usize;
        if idx < self.items.len() {
            return Some(idx);
        }
        None
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        self.top_view = (self.comp.vertical_index() as usize).min(self.items.len().saturating_sub(1));
        self.left_view = (self.comp.horizontal_index() as usize).min(self.max_chars as usize);
        self.update_left_position_for_items();
    }
    fn move_scroll_to(&mut self, new_poz: usize) {
        if new_poz == self.top_view {
            return;
        }
        let max_value = self.items.len().saturating_sub(self.size().height as usize);
        self.top_view = new_poz.min(max_value);
        self.update_scrollbars();
    }
    fn find_first_item(&mut self, pos: usize) {
        let mut i = if pos >= self.items.len() { 0 } else { pos };
        let mut count = self.items.len();
        while count > 0 {
            if self.items[i].filtered {
                self.update_position(i, true);
                return;
            }
            i = (i + 1) % self.items.len();
            count -= 1;
        }
    }
    fn search(&mut self) {
        let text_to_search = self.comp.search_text();
        if text_to_search.is_empty() {
            for item in self.items.iter_mut() {
                item.filtered = true;
            }
            self.comp.clear_match_count();
        } else {
            let mut count = 0usize;
            for item in self.items.iter_mut() {
                item.filtered = item.visible_text().index_ignoring_case(text_to_search).is_some();
                if item.filtered {
                    count += 1;
                }
            }
            self.comp.set_match_count(count);
            if count > 0 {
                self.find_first_item(self.pos);
            }
        }
    }
    fn send_checked_event(&mut self, index: usize, checked: bool) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::ListBox(EventData {
                event_type: ListBoxEventTypes::ItemChecked,
                index,
                checked,
            }),
        });
    }
}
impl OnPaint for ListBox {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let has_focus = self.has_focus();
        if has_focus && (self.flags.contains_one(Flags::ScrollBars | Flags::SearchBar)) {
            self.comp.paint(surface, theme, self);
            if self.flags.contains(Flags::ScrollBars) {
                surface.reduce_clip_by(0, 0, 1, 1);
            } else {
                surface.reduce_clip_by(0, 0, 0, 1);
            }
        }
        let attr = match () {
            _ if !self.is_active() => theme.text.inactive,
            _ if has_focus => theme.text.focused,
            _ => theme.text.normal,
        };

        let mut y = 0;
        let mut idx = self.top_view;
        let count = self.items.len();
        let h = self.size().height as i32;
        let w = self.size().width as i32;

        // empty message
        if (count == 0) && (!self.empty_message.is_empty()) {
            let empty_attr = match () {
                _ if !self.is_active() => theme.text.inactive,
                _ if has_focus => theme.text.highlighted,
                _ => theme.text.inactive,
            };
            let format = TextFormatBuilder::new()
                .position(w / 2, h / 2)
                .attribute(empty_attr)
                .align(TextAlignament::Center)
                .wrap_type(WrapType::WordWrap(w as u16))
                .build();
            surface.write_text(&self.empty_message, &format);
            return;
        }

        if self.flags.contains(Flags::CheckBoxes) {
            let ch_checked = Character::with_attributes(
                SpecialChar::CheckMark,
                match () {
                    _ if !self.is_active() => theme.text.inactive,
                    _ if has_focus => theme.symbol.checked,
                    _ => theme.text.normal,
                },
            );
            let ch_unchecked = Character::with_attributes(
                'x',
                match () {
                    _ if !self.is_active() => theme.text.inactive,
                    _ if has_focus => theme.symbol.unchecked,
                    _ => theme.text.normal,
                },
            );
            while (y < h) && (idx < count) {
                let item = &self.items[idx];
                if item.checked {
                    surface.write_char(0, y, ch_checked);
                } else {
                    surface.write_char(0, y, ch_unchecked);
                }
                surface.write_string(2, y, item.visible_text(), if item.filtered { attr } else { theme.text.inactive }, false);
                if idx == self.pos {
                    if has_focus {
                        surface.fill_horizontal_line(0, y, w - 1, Character::with_attributes(0, theme.list_current_item.focus));
                    } else if (self.flags.contains(Flags::HighlightSelectedItemWhenInactive)) && (self.is_enabled()) {
                        surface.fill_horizontal_line(0, y, w - 1, Character::with_attributes(0, theme.text.highlighted));
                    }
                }
                y += 1;
                idx += 1;
            }
        } else {
            while (y < h) && (idx < count) {
                surface.write_string(
                    0,
                    y,
                    self.items[idx].visible_text(),
                    if self.items[idx].filtered { attr } else { theme.text.inactive },
                    false,
                );
                if idx == self.pos {
                    if has_focus {
                        surface.fill_horizontal_line(0, y, w - 1, Character::with_attributes(0, theme.list_current_item.focus));
                    } else if (self.flags.contains(Flags::HighlightSelectedItemWhenInactive)) && (self.is_enabled()) {
                        surface.fill_horizontal_line(0, y, w - 1, Character::with_attributes(0, theme.text.highlighted));
                    }
                }

                y += 1;
                idx += 1;
            }
        }
    }
}

impl OnKeyPressed for ListBox {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        if self.comp.process_key_pressed(key, character) {
            self.search();
            return EventProcessStatus::Processed;
        }
        match key.value() {
            key!("Up") => {
                self.update_position(self.pos.saturating_sub(1), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Down") => {
                self.update_position(self.pos.saturating_add(1), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Left") => {
                self.left_view = self.left_view.saturating_sub(1);
                self.update_left_position_for_items();
                self.update_scrollbars();
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Alt+Left") => {
                self.left_view = 0;
                self.update_left_position_for_items();
                self.update_scrollbars();
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Right") => {
                let d = if self.flags.contains(Flags::CheckBoxes) { 2 } else { 0 };
                let w = self.size().width.saturating_sub(d);
                self.left_view = (self.left_view + 1).min(self.max_chars.saturating_sub(w) as usize);
                self.update_left_position_for_items();
                self.update_scrollbars();
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Alt+Right") => {
                let d = if self.flags.contains(Flags::CheckBoxes) { 2 } else { 0 };
                let w = self.size().width.saturating_sub(d);
                self.left_view = self.max_chars.saturating_sub(w) as usize;
                self.update_left_position_for_items();
                self.update_scrollbars();
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Alt+Up") => {
                self.move_scroll_to(self.top_view.saturating_sub(1));
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Alt+Down") => {
                self.move_scroll_to(self.top_view.saturating_add(1));
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Home") => {
                self.update_position(0, true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("End") => {
                self.update_position(self.items.len(), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("PageUp") => {
                self.update_position(self.pos.saturating_sub(self.size().height as usize), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("PageDown") => {
                self.update_position(self.pos.saturating_add(self.size().height as usize), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Space") => {
                if self.flags.contains(Flags::CheckBoxes) {
                    if let Some(item) = self.items.get_mut(self.pos) {
                        item.checked = !item.checked;
                        let value = item.checked;
                        self.send_checked_event(self.pos, value);
                    }
                    return EventProcessStatus::Processed;
                }
            }
            key!("Enter") => {
                if self.comp.is_in_edit_mode() {
                    self.find_first_item(self.pos + 1);
                    return EventProcessStatus::Processed;
                } else if self.flags.contains(Flags::CheckBoxes) {
                    if let Some(item) = self.items.get_mut(self.pos) {
                        item.checked = !item.checked;
                        let value = item.checked;
                        self.send_checked_event(self.pos, value);
                    }
                    return EventProcessStatus::Processed;
                }
            }

            _ => {}
        }
        if self.comp.should_repaint() {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}
impl OnMouseEvent for ListBox {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.comp.process_mouse_event(event) {
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
        }
        let response = match event {
            MouseEvent::Enter | MouseEvent::Leave => EventProcessStatus::Ignored,
            MouseEvent::Over(_) => EventProcessStatus::Ignored,
            MouseEvent::Pressed(d) | MouseEvent::DoubleClick(d) => {
                if let Some(pos) = self.mouse_to_pos(d.x, d.y) {
                    self.update_position(pos, true);
                }
                if (d.x == 0) && (self.flags.contains(Flags::CheckBoxes)) {
                    if let Some(item) = self.items.get_mut(self.pos) {
                        item.checked = !item.checked;
                        let value = item.checked;
                        self.send_checked_event(self.pos, value);
                    }
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Released(_) => EventProcessStatus::Ignored,
            MouseEvent::Drag(_) => EventProcessStatus::Ignored,
            MouseEvent::Wheel(evn) => {
                match evn {
                    MouseWheelDirection::Up => self.move_scroll_to(self.top_view.saturating_sub(1)),
                    MouseWheelDirection::Down => self.move_scroll_to(self.top_view.saturating_add(1)),
                    _ => {}
                }
                EventProcessStatus::Processed
            }
        };
        // if one of the components require a repaint, than we should repaint even if the canvas required us to ignore the event
        if self.comp.should_repaint() {
            EventProcessStatus::Processed
        } else {
            response
        }
    }
}
impl OnResize for ListBox {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        let extra = if self.flags.contains(Flags::CheckBoxes) { 2 } else { 0 };
        self.comp.resize(self.max_chars as u64 + extra, self.items.len() as u64, &self.base, self.size());
        self.update_position(self.pos, false);
    }
}
