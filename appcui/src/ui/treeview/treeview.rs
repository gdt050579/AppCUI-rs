use super::{Flags, Item, TreeDataManager};
use components::listitem::render_method::RenderData;
use AppCUIProcMacro::*;

#[derive(Clone, Copy)]
enum CheckMode {
    True,
    False,
    Reverse,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum HoverStatus {
    None,
    OverCheckMark(i32, usize),
    OverFoldButton(i32, usize),
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct TreeView<T>
where
    T: ListItem + 'static,
{
    flags: Flags,
    manager: TreeDataManager<T>,
    filter: Vec<Handle<Item<T>>>,
    header: ColumnsHeader,
    comp: ListScrollBars,
    top_view: usize,
    pos: usize,
    icon_width: u8,
    hover_status: HoverStatus,
    refilter_enabled: bool,
}
impl<T> TreeView<T>
where
    T: ListItem + 'static,
{
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self::with_capacity(16, layout, flags)
    }
    pub fn with_capacity(capacity: usize, layout: Layout, flags: Flags) -> Self {
        let mut status_flags = StatusFlags::Enabled | StatusFlags::Visible | StatusFlags::AcceptInput;
        if flags.contains(Flags::ScrollBars) {
            status_flags |= StatusFlags::IncreaseBottomMarginOnFocus;
            status_flags |= StatusFlags::IncreaseRightMarginOnFocus;
        }
        if flags.contains(Flags::SearchBar) {
            status_flags |= StatusFlags::IncreaseBottomMarginOnFocus;
        }
        if flags.contains(Flags::CheckBoxes | Flags::NoSelection) {
            panic!("Invalid flags combination. `CheckBoxes` and `NoSelection` flags cannot be used together !");
        }

        let mut lv = Self {
            base: ControlBase::with_status_flags(layout, status_flags),
            flags,
            top_view: 0,
            pos: 0,
            manager: TreeDataManager::with_capacity(capacity as u32),
            filter: Vec::with_capacity(capacity),
            header: ColumnsHeader::with_capacity(4),
            comp: ListScrollBars::new(flags.contains(Flags::ScrollBars), flags.contains(Flags::SearchBar)),
            icon_width: if flags.contains(Flags::LargeIcons) {
                3 // includes the extra space
            } else if flags.contains(Flags::SmallIcons) {
                2 // includes the extra space
            } else {
                0 // No extra space
            },
            refilter_enabled: true,
            //start_mouse_select: 0,
            //mouse_check_mode: CheckMode::False,
            hover_status: HoverStatus::None,
            //selected_items_count: 0,
        };
        // add columnes (if described in the type T)
        for i in 0..T::columns_count() {
            lv.header.add(T::column(i));
        }
        lv
    }
    #[inline(always)]
    pub fn add(&mut self, item: T) -> Handle<Item<T>> {
        self.add_item_to_parent(Item::from(item), Handle::None)
    }
    #[inline(always)]
    pub fn add_to_parent(&mut self, item: T, parent: Handle<Item<T>>) -> Handle<Item<T>> {
        self.add_item_to_parent(Item::from(item), parent)
    }
    #[inline(always)]
    pub fn add_item(&mut self, item: Item<T>) -> Handle<Item<T>> {
        self.add_item_to_parent(item, Handle::None)
    }
    #[inline(always)]
    pub fn add_item_to_parent(&mut self, mut item: Item<T>, parent: Handle<Item<T>>) -> Handle<Item<T>> {
        // override selection state if the NoSelection flag is set
        if self.flags.contains(Flags::NoSelection) {
            item.set_checked(false);
        }
        let h = self.manager.add(item, parent);
        // refilter everything
        self.refilter();
        h
    }
    pub fn add_batch<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self),
    {
        let old_refilter = self.refilter_enabled;
        self.refilter_enabled = false;
        f(self);
        // restore original refilter state
        self.refilter_enabled = old_refilter;
        self.refilter();
    }

    fn refilter(&mut self) {
        if !self.refilter_enabled {
            return;
        }
        // refilter elements
        self.filter.clear();
        // reserve space for the entire list + groups
        self.filter.reserve(self.manager.len());
        // populate filter with items
        self.manager.populate(&mut self.filter);
        //self.manager.
        // let handle = self.manager.first();
        // while !handle.is_none() {
        //     if let Some(item) = self.manager.get(handle) {
        //         // if !self.is_item_filtered_out(item) {
        //         //     self.filter.push(handle);
        //         // }
        //     }
        // }

        // if let Some(column_index) = self.header.sort_column() {
        //     self.sort(column_index, self.header.should_sort_ascendent());
        // } else {
        //     self.sort(u16::MAX, true);
        // }
    }
    fn goto_handle(&mut self, handle: Handle<Item<T>>, emit_event: bool) -> bool {
        if let Some(index) = self.filter.iter().position(|h| *h == handle) {
            self.update_position(index, emit_event);
            true 
        } else {
            false
        }
    }
    fn filter_items(&mut self) {
        if self.manager.len() == 0 {
            return;
        }
        let current_handle = if self.pos < self.filter.len() {
            self.filter[self.pos]
        } else {
            Handle::None
        };
        self.refilter();
        if current_handle.is_none() {
            self.update_position(0, true);
        } else {
            self.goto_handle(current_handle, true);
        }
    }

    pub fn sort(&mut self, column_index: u16, ascendent: bool) {
        self.header.set_sort_column(column_index, ascendent, true);
        if self.filter.is_empty() {
            // no need to sort
            return;
        }
        let current_handle = if self.pos < self.filter.len() {
            self.filter[self.pos]
        } else {
            Handle::None
        };
        // sort elements by column index
        self.manager.sort(column_index, ascendent);
        // repopulate 
        self.refilter();
        // find the new position after sorting
        if !current_handle.is_none() {
            // on the same item --> no need to emit an event
            self.goto_handle(current_handle, false);
        }
    }

    #[inline(always)]
    fn visible_space(&self) -> Size {
        let mut sz = self.size();
        if !self.flags.contains(Flags::HideHeader) {
            sz.height = sz.height.saturating_sub(1);
        }
        sz
    }
    #[inline(always)]
    fn visible_items(&self) -> usize {
        let count = self.size().height as usize;
        if !self.flags.contains(Flags::HideHeader) {
            count.saturating_sub(1)
        } else {
            count
        }
    }
    #[inline(always)]
    fn item_width(&self) -> u32 {
        self.size().width
    }
    #[inline(always)]
    fn paint_icon(&self, x: i32, item: &Item<T>, attr: Option<CharAttribute>, surface: &mut Surface, theme: &Theme) {
        let attr = attr.unwrap_or(theme.list_current_item.icon);
        match self.icon_width {
            3 => {
                surface.write_char(x, 0, Character::with_attributes(item.icon_first_character(), attr));
                surface.write_char(x + 1, 0, Character::with_attributes(item.icon_second_character(), attr));
            }
            2 => {
                surface.write_char(x, 0, Character::with_attributes(item.icon_first_character(), attr));
            }
            _ => {}
        }
    }

    fn paint_item(&self, item: &Item<T>, y: i32, surface: &mut Surface, theme: &Theme, attr: Option<CharAttribute>) {
        let width = self.header.width() as i32;
        let frozen_columns = self.header.frozen_columns();
        let columns = self.header.columns();
        if columns.is_empty() {
            return;
        }
        let min_left = if frozen_columns == 0 {
            columns[0].x
        } else {
            let c = &columns[frozen_columns as usize - 1];
            c.x + c.width as i32 + 1
        };
        // first column
        let c = &columns[0];
        let d = if item.depth == 0 { 0 } else { (item.depth as i32) * 6 - 2 };
        let l = c.x + d;
        let r = c.x + c.width as i32;
        let mut extra = 0;
        let mut rd = RenderData {
            theme,
            alignment: TextAlignament::Left,
            width: 0,
            attr: None,
        };
        if (r >= 0) && (l < width) && (c.width != 0)
        /*&& (r >= min_left)*/
        {
            if d > 0 {
                surface.set_relative_clip(c.x - 2, y, r.max(min_left), y);
                surface.set_origin(c.x - 2, y);
                for i in 1..item.depth {
                    surface.write_char(
                        (i as i32) * 6,
                        0,
                        Character::with_attributes(SpecialChar::BoxVerticalSingleLine, attr.unwrap_or(theme.text.normal)),
                    );
                }
            }
            if frozen_columns == 0 {
                surface.set_relative_clip(l.max(min_left), y, r.max(min_left), y);
                surface.set_origin(l, y);
            } else {
                surface.set_relative_clip(l, y, r, y);
                surface.set_origin(l, y);
            }
            if d > 0 {
                surface.write_string(extra, 0, "├─", attr.unwrap_or(theme.text.normal), false);
                extra += 2;
            }
            surface.write_string(extra, 0, "[ ]", attr.unwrap_or(theme.text.normal), false);
            extra += 4;
            if self.flags.contains(Flags::CheckBoxes) {
                if item.is_checked() {
                    surface.write_char(
                        extra,
                        0,
                        Character::with_attributes(SpecialChar::CheckMark, attr.unwrap_or(theme.symbol.checked)),
                    );
                } else {
                    surface.write_char(0, 0, Character::with_attributes('x', attr.unwrap_or(theme.symbol.unchecked)));
                }
                extra = 2;
            }
            // icon
            if self.icon_width > 0 {
                self.paint_icon(extra, item, attr, surface, theme);
                extra += self.icon_width as i32;
            }
            if extra > 0 {
                if frozen_columns == 0 {
                    surface.set_relative_clip((l + extra).max(min_left), y, r.max(min_left), y);
                    surface.set_origin(l + extra, y);
                } else {
                    surface.set_relative_clip(l + extra, y, r, y);
                    surface.set_origin(l + extra, y);
                }
            }
            if let Some(render_method) = ListItem::render_method(item.value(), 0) {
                rd.width = c.width as u16;
                rd.alignment = c.alignment;
                rd.attr = if attr.is_none() { item.render_attr() } else { attr };
                if !render_method.paint(surface, &rd) {
                    // custom paint required
                    ListItem::paint(item.value(), 0, c.width.saturating_sub(extra as u8) as u16, surface, theme, rd.attr)
                }
            }
        }
        rd.attr = if attr.is_none() { item.render_attr() } else { attr };
        for (index, c) in columns.iter().enumerate().skip(1) {
            let r = c.x + c.width as i32;
            if (r < 0) /*|| (r < min_left)*/ || (c.x >= width) || (c.width == 0) {
                continue;
            }
            if index < frozen_columns as usize {
                surface.set_relative_clip(c.x, y, r, y);
            } else {
                surface.set_relative_clip(c.x.max(min_left), y, r.max(min_left), y);
            }
            surface.set_origin(c.x, y);
            if let Some(render_method) = ListItem::render_method(item.value(), index as u16) {
                rd.width = c.width as u16;
                rd.alignment = c.alignment;

                if !render_method.paint(surface, &rd) {
                    // custom paint required
                    ListItem::paint(item.value(), index as u32, c.width as u16, surface, theme, rd.attr)
                }
            }
        }
    }

    fn paint_items(&self, surface: &mut Surface, theme: &Theme) {
        let has_focus = self.base.has_focus();
        let is_enabled = self.is_enabled();
        let attr = if !is_enabled {
            Some(theme.text.inactive)
        } else if !has_focus {
            Some(theme.text.normal)
        } else {
            None
        };
        let start_y_poz = if !self.flags.contains(Flags::HideHeader) { 1 } else { 0 };
        let mut y = start_y_poz;
        let mut x = 0;
        let item_size = self.item_width();
        let max_y = self.size().height as i32;
        let mut idx = self.top_view;
        let max_idx = self.filter.len();
        let visible_items = self.visible_items();
        let mut item_count = 0;
        let (hover_checkmark_x, hover_pos) = match self.hover_status {
            HoverStatus::OverCheckMark(x, pos) => (x, pos),
            _ => (0, usize::MAX),
        };
        // very simply code
        while (item_count < visible_items) && (idx < max_idx) {
            if let Some(item) = self.manager.get(self.filter[idx]) {
                self.paint_item(item, y, surface, theme, attr);
                if (item.is_checked()) && (has_focus) && (!self.flags.contains(Flags::CheckBoxes)) {
                    surface.reset_clip();
                    surface.reset_origin();
                    surface.fill_horizontal_line_with_size(x, y, item_size, Character::with_attributes(0, theme.list_current_item.selected));
                }
                if is_enabled {
                    if idx == self.pos {
                        surface.reset_clip();
                        surface.reset_origin();
                        if has_focus {
                            let current_item_attr = match () {
                                _ if self.flags.contains(Flags::CheckBoxes) => theme.list_current_item.focus,
                                _ if item.is_checked() => theme.list_current_item.over_selection,
                                _ => theme.list_current_item.focus,
                            };
                            surface.fill_horizontal_line_with_size(x, y, item_size, Character::with_attributes(0, current_item_attr));
                        }
                    }
                    if idx == hover_pos {
                        surface.reset_clip();
                        surface.reset_origin();
                        surface.write_char(hover_checkmark_x, y, Character::with_attributes(0, theme.button.text.hovered));
                    }
                }
            }
            y += 1;
            idx += 1;
            item_count += 1;
            if y >= max_y {
                y = start_y_poz;
                x += item_size as i32 + 1;
            }
        }
        surface.reset_clip();
        surface.reset_origin();
    }
    fn autoresize_column(&mut self, column_index: u16) {
        let mut new_width = 0u32;
        let mut found = false;
        for handle in self.filter.iter() {
            if let Some(item) = self.manager.get(*handle) {
                if let Some(rm) = item.value().render_method(column_index) {
                    new_width = new_width.max(listview::RenderMethod::min_width(&rm));
                    found = true;
                }
            }
        }
        if found {
            if column_index == 0 {
                if self.flags.contains(Flags::CheckBoxes) {
                    new_width += 2
                };
                new_width += self.icon_width as u32;
            }
            self.header.set_column_width(column_index, new_width.min(u8::MAX as u32) as u8);
        }
    }
    fn update_scrollbars(&mut self) {
        self.comp.resize(self.header.width() as u64, self.filter.len() as u64, &self.base, self.visible_space());
        self.comp.set_indexes(self.header.scroll_pos() as u64, self.top_view as u64);
    }
    fn execute_column_header_action(&mut self, action: ColumnsHeaderAction) -> bool {
        match action {
            ColumnsHeaderAction::Sort((index, ascendent)) => {
                self.sort(index, ascendent);
                self.update_scrollbars();
                true
            }
            ColumnsHeaderAction::AutoResize(index) => {
                self.autoresize_column(index);
                self.update_scrollbars();
                true
            }
            ColumnsHeaderAction::ResizeColumn => {
                self.update_scrollbars();
                true
            }
            ColumnsHeaderAction::UpdateScroll => {
                self.update_scrollbars();
                true
            }
            ColumnsHeaderAction::Processed => true,
            ColumnsHeaderAction::None => false,
            ColumnsHeaderAction::Repaint => false,
        }
    }   
    fn update_position(&mut self, new_pos: usize, emit_event: bool) {
        let len = self.filter.len();
        if len == 0 {
            return;
        }
        let new_pos = new_pos.min(len - 1);
        let h = self.visible_items();
        if h == 0 {
            return;
        }

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
        let should_emit = (self.pos != new_pos) && emit_event;
        self.pos = new_pos;
        if should_emit {
            // self.raise_event(ControlEvent {
            //     emitter: self.handle,
            //     receiver: self.event_processor,
            //     data: ControlEventData::ListView(EventData {
            //         event_type: listview::events::ListViewEventTypes::CurrentItemChanged,
            //         type_id: std::any::TypeId::of::<T>(),
            //     }),
            // });
        }
    }    
    fn move_scroll_to(&mut self, new_poz: usize) {
        if new_poz == self.top_view {
            return;
        }
        let visible_items = self.visible_items();
        let max_value = self.filter.len().saturating_sub(visible_items);
        self.top_view = new_poz.min(max_value);
        self.update_scrollbars();
    }   
    fn emit_selection_update_event(&self) {
        // self.raise_event(ControlEvent {
        //     emitter: self.handle,
        //     receiver: self.event_processor,
        //     data: ControlEventData::ListView(EventData {
        //         event_type: listview::events::ListViewEventTypes::SelectionChanged,
        //         type_id: std::any::TypeId::of::<T>(),
        //     }),
        // });
    }
    fn emit_item_action_event(&self, index: usize) {
        // if index < self.manager.len() {
        //     self.raise_event(ControlEvent {
        //         emitter: self.handle,
        //         receiver: self.event_processor,
        //         data: ControlEventData::ListView(EventData {
        //             event_type: listview::events::ListViewEventTypes::ItemAction(index),
        //             type_id: std::any::TypeId::of::<T>(),
        //         }),
        //     });
        // }
    }
    #[inline(always)]
    fn toggle_current_item_selection(&self) -> CheckMode {
        if self.pos < self.filter.len() {
            if let Some(item) = self.manager.get(self.filter[self.pos]) {
                if item.is_checked() {
                    CheckMode::False
                } else {
                    CheckMode::True
                }
            } else {
                CheckMode::False
            }
        } else {
            CheckMode::False
        }
    }
    fn select_until_position(&mut self, new_pos: usize) {
        let start = self.pos;
        let mode = self.toggle_current_item_selection();
        self.update_position(new_pos, true);
        self.check_items(start, self.pos, mode, true);
    }
    fn check_item(&mut self, pos: usize, mode: CheckMode, update_group_check_count: bool, emit_event: bool) -> bool {
        if self.flags.contains(Flags::NoSelection) {
            return false;
        }
        if pos >= self.filter.len() {
            return false;
        }
        let mut selection_has_changed = false;
        // match self.filter[pos] {
        //     Element::Item(index) => {
        //         let item = &mut self.data[index as usize];
        //         let status = item.is_checked();
        //         match mode {
        //             CheckMode::True => item.set_checked(true),
        //             CheckMode::False => item.set_checked(false),
        //             CheckMode::Reverse => item.set_checked(!status),
        //         }
        //         selection_has_changed = item.is_checked() != status;
        //         if selection_has_changed {
        //             if item.is_checked() {
        //                 self.selected_items_count += 1;
        //             } else {
        //                 self.selected_items_count -= 1;
        //             }
        //         }
        //         if update_group_check_count {
        //             self.update_check_count_for_groups();
        //         }
        //     }
        //     Element::Group(gid) => {
        //         let group = &mut self.groups[gid as usize];
        //         let checked = group.items_checked_count();
        //         let count = group.items_count();
        //         let new_status = checked < count;
        //         if group.is_collapsed() {
        //             // iterate through all items that and check if they are filtered or not and check them
        //             let len = self.data.len();
        //             for idx in 0..len {
        //                 let item = &self.data[idx];
        //                 if item.group_id() != gid {
        //                     continue;
        //                 }
        //                 if self.is_item_filtered_out(item) {
        //                     continue;
        //                 }
        //                 selection_has_changed |= self.select_item_and_update_count(idx, new_status);
        //             }
        //         } else {
        //             let len = self.filter.len();
        //             for idx in pos + 1..len {
        //                 match self.filter[idx] {
        //                     Element::Item(index) => {
        //                         let item = &self.data[index as usize];
        //                         if item.group_id() != gid {
        //                             break;
        //                         }
        //                         selection_has_changed |= self.select_item_and_update_count(index as usize, new_status);
        //                     }
        //                     _ => {
        //                         break;
        //                     }
        //                 }
        //             }
        //         }
        //         let group = &mut self.groups[gid as usize];
        //         group.set_items_checked_count(if checked < count { count } else { 0 });
        //     }
        // }
        if (emit_event) && (selection_has_changed) {
            self.emit_selection_update_event();
        }
        selection_has_changed
    }
    fn check_items(&mut self, start: usize, end: usize, mode: CheckMode, emit_event: bool) {
        if self.flags.contains(Flags::NoSelection) {
            return;
        }
        let len = self.filter.len();
        if len == 0 {
            return;
        }
        let p_start = start.min(end).min(len - 1);
        let p_end = end.max(start).min(len - 1);
        let mut selection_has_changed = false;
        for pos in p_start..=p_end {
            selection_has_changed |= self.check_item(pos, mode, false, false);
        }
        if (emit_event) && (selection_has_changed) {
            self.emit_selection_update_event();
        }
    }
    #[inline(always)]
    fn is_entire_list_selected(&self) -> bool {
        for handle in &self.filter {
            if let Some(item) = self.manager.get(*handle) {
                if !item.is_checked() {
                    return false;
                }
            }
        }
        true
    }      
    fn process_key_pressed(&mut self, key: Key) -> bool {
        // process key for items
        match key.value() {
            // movements
            key!("Up") => {
                self.update_position(self.pos.saturating_sub(1), true);
                true
            }
            key!("Down") => {
                self.update_position(self.pos.saturating_add(1), true);
                true
            }
            key!("Ctrl+Alt+Up") => {
                self.move_scroll_to(self.top_view.saturating_sub(1));
                true
            }
            key!("Ctrl+Alt+Down") => {
                self.move_scroll_to(self.top_view.saturating_add(1));
                true
            }
            key!("Home") => {
                self.update_position(0, true);
                true
            }
            key!("End") => {
                self.update_position(self.filter.len(), true);
                true
            }
            key!("PageUp") => {
                self.update_position(self.pos.saturating_sub(self.visible_items()), true);
                true
            }
            key!("PageDown") => {
                self.update_position(self.pos.saturating_add(self.visible_items()), true);
                true
            }
            key!("Left") => {
                self.update_position(self.pos.saturating_sub(self.size().height as usize), true);
                true
            }
            key!("Right") => {
                self.update_position(self.pos.saturating_add(self.size().height as usize), true);
                true
            }

            // Selection
            key!("Space") => {
                if self.flags.contains(Flags::CheckBoxes) {
                    self.check_item(self.pos, CheckMode::Reverse, true, true);
                    true
                } else {
                    false
                }
            }
            key!("Insert") | key!("Shift+Down") => {
                self.check_item(self.pos, CheckMode::Reverse, true, true);
                self.update_position(self.pos.saturating_add(1), true);
                true
            }
            key!("Shift+Up") => {
                self.check_item(self.pos, CheckMode::Reverse, true, true);
                self.update_position(self.pos.saturating_sub(1), true);
                true
            }
            key!("Shift+Home") => {
                self.select_until_position(0);
                true
            }
            key!("Shift+End") => {
                self.select_until_position(self.filter.len());
                true
            }
            key!("Shift+PageUp") => {
                self.select_until_position(self.pos.saturating_sub(self.visible_items()));
                true
            }
            key!("Shift+PageDown") => {
                self.select_until_position(self.pos.saturating_add(self.visible_items()));
                true
            }
            key!("Shift+Left") => {
                self.select_until_position(self.pos.saturating_sub(self.size().height as usize));
                true
            }
            key!("Shift+Right") => {
                self.select_until_position(self.pos.saturating_add(self.size().height as usize));
                true
            }

            key!("Ctrl+A") => {
                if self.is_entire_list_selected() {
                    self.check_items(0, self.filter.len(), CheckMode::False, true);
                } else {
                    self.check_items(0, self.filter.len(), CheckMode::True, true);
                }
                true
            }

            // Action
            key!("Enter") => {
                // match self.filter.get(self.pos) {
                //     Some(Element::Item(index)) => self.emit_item_action_event(*index as usize),
                //     Some(Element::Group(gid)) => self.toggle_group_collapse_status(*gid, true),
                //     _ => {}
                // }
                true
            }
            _ => false,
        }
    } 
}
impl<T> OnPaint for TreeView<T>
where
    T: ListItem + 'static,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        // paint columns
        self.header.paint(surface, theme, &self.base);
        // paint items
        self.paint_items(surface, theme);
        // paint separation lines (columns)
        self.header.paint_columns(surface, theme, &self.base);
        // paint scroll bars and searh bars
        self.comp.paint(surface, theme, &self.base);
    }
}
impl<T> OnKeyPressed for TreeView<T> where T: ListItem + 'static {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        let action = self.header.process_key_pressed(key);
        if self.execute_column_header_action(action) {
            return EventProcessStatus::Processed;
        }
        if self.comp.process_key_pressed(key, character) {
            self.filter_items();
            return EventProcessStatus::Processed;
        }
        if self.process_key_pressed(key) {
            self.comp.exit_edit_mode();
            return EventProcessStatus::Processed;
        }
        if (action.should_repaint()) || (self.comp.should_repaint()) {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}
impl<T> OnMouseEvent for TreeView<T> where T: ListItem + 'static {}
impl<T> OnResize for TreeView<T>
where
    T: ListItem + 'static,
{
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.header.resize(new_size);
        self.comp
            .resize(self.header.width() as u64, self.filter.len() as u64, &self.base, self.visible_space());
    }
}
