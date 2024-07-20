use std::cmp::Ordering;

use super::{Flags, Group, GroupInformation, Item, ListItem};
use components::{Column, ColumnsHeader, ColumnsHeaderAction, ListScrollBars};
use AppCUIProcMacro::*;

#[derive(Clone, Copy)]
enum CheckMode {
    True,
    False,
    Reverse,
}
#[derive(Clone, Copy)]
enum Filter {
    Item(u32),
    Group(u16),
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct ListView<T>
where
    T: ListItem,
{
    flags: Flags,
    data: Vec<Item<T>>,
    filter: Vec<Filter>,
    groups: Vec<GroupInformation>,
    header: ColumnsHeader,
    comp: ListScrollBars,
    top_view: usize,
    pos: usize,
    icon_width: u8,
    refilter_enabled: bool,
}

impl<T> ListView<T>
where
    T: ListItem,
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

        let mut lv = Self {
            base: ControlBase::with_status_flags(layout, status_flags),
            flags,
            top_view: 0,
            pos: 0,
            data: Vec::with_capacity(capacity),
            groups: Vec::new(),
            filter: Vec::with_capacity(capacity),
            header: ColumnsHeader::with_capacity(4),
            comp: ListScrollBars::new(flags.contains(Flags::ScrollBars), flags.contains(Flags::SearchBar)),
            icon_width: if flags.contains(Flags::LargeIcon) {
                3 // includes the extra space
            } else if flags.contains(Flags::SmallIcon) {
                2 // includes the extra space
            } else {
                0 // No extra space
            },
            refilter_enabled: true,
        };
        // add a default group
        lv.groups.push(GroupInformation::default());
        lv
    }
    pub fn add_group(&mut self, name: &str) -> Group {
        let index = self.groups.len() as u16;
        self.groups.push(GroupInformation::new(name));
        if self.flags.contains(Flags::ShowGroups) {
            // if groups are being shouwn -> we need to refilter intems
            self.refilter();
        }
        Group::new(index)
    }
    pub fn add_column(&mut self, column: Column) {
        self.header.add(column);
    }
    #[inline(always)]
    pub fn add(&mut self, item: T) {
        self.add_item(Item::from(item));
    }
    #[inline(always)]
    pub fn add_item(&mut self, item: Item<T>) {
        let gid = item.group_id() as usize;
        if gid >= self.groups.len() {
            panic!("Invalid group id `{}`. Have you reused a group id from a previous instantiation ?", gid);
        }
        let count = self.groups[gid].items_count();
        self.groups[gid].set_items_count(count+1);
        self.data.push(item);
        // refilter everything
        self.refilter();
    }
    pub fn add_items(&mut self, items: Vec<T>) {
        self.add_multiple_items(items, Group::None, [0 as char, 0 as char]);
    }
    pub fn add_items_to_group(&mut self, items: Vec<T>, group: Group) {
        self.add_multiple_items(items, group, [0 as char, 0 as char]);
    }
    fn add_multiple_items(&mut self, items: Vec<T>, group: Group, icon: [char; 2]) {
        // disable refiltering while adding all elements
        let old_refilter = self.refilter_enabled;
        self.refilter_enabled = false;
        self.data.reserve(items.len());
        self.filter.reserve(items.len());
        for item in items {
            self.add_item(Item::new(item, false, None, 0, icon, group));
        }
        // restore original refilter state
        self.refilter_enabled = old_refilter;
        self.refilter();
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
    pub fn set_frozen_columns(&mut self, count: u16) {
        self.header.set_frozen_columns(count);
        self.update_scrollbars();
    }
    fn compare_items(a: Filter, b: Filter, column_index: u16, data: &Vec<Item<T>>, ascendent: bool) -> Ordering
    where
        T: ListItem,
    {
        match (a, b) {
            (Filter::Item(index_a), Filter::Item(index_b)) => {
                let rezult = data[index_a as usize].group_id().cmp(&data[index_b as usize].group_id());
                if rezult != Ordering::Equal {
                    rezult
                } else {
                    let item_a = data[index_a as usize].value();
                    let item_b = data[index_b as usize].value();
                    let rezult = ListItem::compare(item_a, item_b, column_index);
                    if ascendent {
                        rezult
                    } else {
                        rezult.reverse()
                    }
                }
            }
            (Filter::Group(index_a), Filter::Group(index_b)) => index_a.cmp(&index_b),
            (Filter::Item(index), Filter::Group(group_id)) => match data[index as usize].group_id().cmp(&group_id) {
                Ordering::Equal => Ordering::Greater,
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
            },
            (Filter::Group(group_id), Filter::Item(index)) => match group_id.cmp(&data[index as usize].group_id()) {
                Ordering::Equal => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
            },
        }
    }
    fn sort_elements(&mut self, column_index: u16, ascendent: bool) {
        // sort elements by column index
        let data = &self.data;
        self.filter.sort_by(|a, b| ListView::compare_items(*a, *b, column_index, data, ascendent));
    }
    fn refilter(&mut self) {
        if !self.refilter_enabled {
            return;
        }
        // refilter elements
        self.filter.clear();
        // reserve space for the entire list + groups
        self.filter.reserve(self.data.len() + self.groups.len());
        // if show groups is present --> add all groups first
        if self.flags.contains(Flags::ShowGroups) {
            if self.flags.contains(Flags::DisplayEmptyGroups) {
                for (index, _) in self.groups.iter().enumerate() {
                    self.filter.push(Filter::Group(index as u16));
                }
            } else {
                for index in self.groups.iter().enumerate().filter(|(_, g)| !g.is_empty()).map(|(i, _)| i) {
                    self.filter.push(Filter::Group(index as u16));
                }
            }
        }
        // add items
        for (index, _) in self.data.iter().enumerate() {
            self.filter.push(Filter::Item(index as u32));
        }
        if let Some(column_index) = self.header.sort_column() {
            self.sort_elements(column_index, self.header.should_sort_ascendent());
        } else {
            self.sort_elements(u16::MAX, true);
        }
    }
    fn autoresize_column(&mut self, _column_index: u16) {
        // auto resize column
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        self.header.scroll_to(self.comp.horizontal_index() as u32);
        self.top_view = (self.comp.vertical_index() as usize).min(self.filter.len());
        //self.update_left_position_for_items();
    }
    fn update_scrollbars(&mut self) {
        self.comp.resize(self.header.width() as u64, self.filter.len() as u64, &self.base, 1);
        self.comp.set_indexes(self.header.scroll_pos() as u64, self.top_view as u64);
    }
    fn execute_column_header_action(&mut self, action: ColumnsHeaderAction) -> bool {
        match action {
            ColumnsHeaderAction::Sort((index, ascendent)) => {
                self.sort_elements(index, ascendent);
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
    #[inline(always)]
    fn visible_items(&self) -> usize {
        self.size().height.saturating_sub(1) as usize
    }
    #[inline(always)]
    fn toggle_current_item_selection(&self) -> CheckMode {
        if self.pos < self.filter.len() {
            match self.filter[self.pos] {
                Filter::Item(index) => {
                    if self.data[index as usize].is_checked() {
                        CheckMode::False
                    } else {
                        CheckMode::True
                    }
                }
                Filter::Group(_) => CheckMode::False,
            }
        } else {
            CheckMode::False
        }
    }
    #[inline(always)]
    fn is_entire_list_selected(&self) -> bool {
        for item in &self.filter {
            match item {
                Filter::Item(idx) => {
                    if !self.data[*idx as usize].is_checked() {
                        return false;
                    }
                }
                _ => {}
            }
        }
        return true;
    }
    fn process_key_pressed(&mut self, key: Key) -> bool {
        // process key for items
        match key.value() {
            // movements
            key!("Ctrl+Left") | key!("Ctrl+Right") => {
                self.header.enter_resize_mode();
                true
            }
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

            // Selection
            key!("Space") => {
                if self.flags.contains(Flags::CheckBoxes) {
                    self.check_item(self.pos, CheckMode::Reverse);
                    true
                } else {
                    false
                }
            }
            key!("Insert") | key!("Shift+Down") => {
                self.check_item(self.pos, CheckMode::Reverse);
                self.update_position(self.pos.saturating_add(1), true);
                true
            }
            key!("Shift+Up") => {
                self.check_item(self.pos, CheckMode::Reverse);
                self.update_position(self.pos.saturating_sub(1), true);
                true
            }
            key!("Shift+Home") => {
                let start = self.pos;
                let mode = self.toggle_current_item_selection();
                self.update_position(0, true);
                self.check_items(start, self.pos, mode);
                true
            }
            key!("Shift+End") => {
                let start = self.pos;
                let mode = self.toggle_current_item_selection();
                self.update_position(self.filter.len(), true);
                self.check_items(start, self.pos, mode);
                true
            }
            key!("Shift+PageUp") => {
                let start = self.pos;
                let mode = self.toggle_current_item_selection();
                self.update_position(self.pos.saturating_sub(self.visible_items()), true);
                self.check_items(start, self.pos, mode);
                true
            }
            key!("Shift+PageDown") => {
                let start = self.pos;
                let mode = self.toggle_current_item_selection();
                self.update_position(self.pos.saturating_add(self.visible_items()), true);
                self.check_items(start, self.pos, mode);
                true
            }
            key!("Ctrl+A") => {
                if self.is_entire_list_selected() {
                    self.check_items(0, self.filter.len(), CheckMode::False);
                } else {
                    self.check_items(0, self.filter.len(), CheckMode::True);
                }
                true
            }
            _ => false,
        }
    }
    fn paint_group(&self, gi: &GroupInformation, y: i32, surface: &mut Surface, theme: &Theme, attr: Option<CharAttribute>) {
        let w = self.size().width;
        surface.draw_horizontal_line_with_size(0, y, w, LineType::Single, attr.unwrap_or(theme.lines.focused));
        let x = 2;
        let w = gi.name_chars_count();
        let mut format = TextFormat::single_line(x, y, attr.unwrap_or(theme.text.focused), TextAlignament::Left);
        format.chars_count = Some(gi.name_chars_count());
        format.width = Some(w);
        surface.write_text(gi.name(), &format);
    }
    fn paint_groups(&self, surface: &mut Surface, theme: &Theme) {
        let has_focus = self.base.has_focus();
        let attr = if !self.is_enabled() {
            Some(theme.text.inactive)
        } else if !has_focus {
            Some(theme.text.normal)
        } else {
            None
        };
        let mut y = 1;
        let max_y = self.size().height as i32;
        let mut idx = self.top_view;
        let max_idx = self.filter.len();
        surface.reset_clip();
        surface.reset_origin();
        while (y < max_y) && (idx < max_idx) {
            match self.filter[idx] {
                Filter::Group(group_id) => {
                    self.paint_group(&self.groups[group_id as usize], y, surface, theme, attr);
                    // paint group
                    if (has_focus) && (idx == self.pos) {
                        surface.fill_horizontal_line_with_size(0, y, self.size().width, Character::with_attributes(0, theme.list_current_item.focus));
                    }
                }
                Filter::Item(_) => {
                }
            }
            y += 1;
            idx += 1;
        }
        surface.reset_clip();
        surface.reset_origin();
    }

    fn paint_item(&self, item: &Item<T>, y: i32, surface: &mut Surface, theme: &Theme, attr: Option<CharAttribute>) {
        let width = self.header.width() as i32;
        let frozen_columns = self.header.frozen_columns();
        let columns = self.header.columns();
        if columns.len() == 0 {
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
        let l = c.x + item.x_offset();
        let r = c.x + c.width as i32;
        let mut extra = 0;
        if (r >= 0) && (r >= min_left) && (l < width) && (c.width != 0) {
            surface.set_relative_clip(l.max(min_left), y, r.max(min_left), y);
            surface.set_origin(l, y);
            if self.flags.contains(Flags::CheckBoxes) {
                if item.is_checked() {
                    surface.write_char(
                        l,
                        0,
                        Character::with_attributes(SpecialChar::CheckMark, attr.unwrap_or(theme.symbol.checked)),
                    );
                } else {
                    surface.write_char(l, 0, Character::with_attributes('x', attr.unwrap_or(theme.symbol.unchecked)));
                }
                extra = 2;
            }
            // icon
            match self.icon_width {
                3 => {
                    surface.write_char(
                        l + extra,
                        y,
                        Character::with_attributes(item.icon_first_character(), attr.unwrap_or(theme.text.focused)),
                    );
                    surface.write_char(
                        l + extra + 1,
                        y,
                        Character::with_attributes(item.icon_second_character(), attr.unwrap_or(theme.text.focused)),
                    );
                }
                2 => {
                    surface.write_char(
                        l + extra,
                        y,
                        Character::with_attributes(item.icon_first_character(), attr.unwrap_or(theme.text.focused)),
                    );
                }
                _ => {}
            }
            extra += self.icon_width as i32;
            if extra > 0 {
                surface.set_relative_clip((l + extra).max(min_left), y, r.max(min_left), y);
                surface.set_origin(l + extra, y);
            }
            if let Some(render_method) = ListItem::render_method(item.value(), 0) {
                if !render_method.paint(surface, theme, c.alignment, c.width as u16, attr) {
                    // custom paint required
                    ListItem::paint(item.value(), 0, c.width as u16, surface, theme)
                }
            }
        }
        for (index, c) in columns.iter().enumerate().skip(1) {
            let r = c.x + c.width as i32;
            if (r < 0) || (r < min_left) || (c.x >= width) || (c.width == 0) {
                continue;
            }
            surface.set_relative_clip(c.x.max(min_left), y, r.max(min_left), y);
            surface.set_origin(c.x, y);
            if let Some(render_method) = ListItem::render_method(item.value(), index as u16) {
                if !render_method.paint(surface, theme, c.alignment, c.width as u16, attr) {
                    // custom paint required
                    ListItem::paint(item.value(), index as u32, c.width as u16, surface, theme)
                }
            }
        }
    }
    fn paint_items(&self, surface: &mut Surface, theme: &Theme) -> bool {
        let has_focus = self.base.has_focus();
        let attr = if !self.is_enabled() {
            Some(theme.text.inactive)
        } else if !has_focus {
            Some(theme.text.normal)
        } else {
            None
        };
        let mut found_groups = false;
        let mut y = 1;
        let max_y = self.size().height as i32;
        let mut idx = self.top_view;
        let max_idx = self.filter.len();
        // very simply code
        while (y < max_y) && (idx < max_idx) {
            match self.filter[idx] {
                Filter::Group(_) => {
                    found_groups = true;
                }
                Filter::Item(index) => {
                    let item = &self.data[index as usize];
                    self.paint_item(item, y, surface, theme, attr);
                    if (item.is_checked()) && (has_focus) && (!self.flags.contains(Flags::CheckBoxes)) {
                        surface.reset_clip();
                        surface.reset_origin();
                        surface.fill_horizontal_line_with_size(
                            0,
                            y,
                            self.size().width,
                            Character::with_attributes(0, theme.list_current_item.selected),
                        );
                    }
                    if (has_focus) && (idx == self.pos) {
                        surface.reset_clip();
                        surface.reset_origin();
                        let current_item_attr = match () {
                            _ if self.flags.contains(Flags::CheckBoxes) => theme.list_current_item.focus,
                            _ if item.is_checked() => theme.list_current_item.over_selection,
                            _ => theme.list_current_item.focus,
                        };
                        surface.fill_horizontal_line_with_size(0, y, self.size().width, Character::with_attributes(0, current_item_attr));
                    }
                }
            }
            y += 1;
            idx += 1;
        }
        surface.reset_clip();
        surface.reset_origin();
        found_groups
    }
    fn update_position(&mut self, new_pos: usize, emit_event: bool) {
        let len = self.filter.len();
        if len == 0 {
            return;
        }
        let new_pos = new_pos.min(len - 1);
        let h = (self.size().height.saturating_sub(1)) as usize;
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
            //     data: ControlEventData::ListBox(EventData {
            //         event_type: ListBoxEventTypes::CurrentItemChanged,
            //         index: new_pos,
            //         checked: false, // not relevant for this event
            //     }),
            // });
        }
    }
    fn move_scroll_to(&mut self, new_poz: usize) {
        if new_poz == self.top_view {
            return;
        }
        let max_value = self.filter.len().saturating_sub(self.size().height.saturating_sub(1) as usize);
        self.top_view = new_poz.min(max_value);
        self.update_scrollbars();
    }
    fn check_item(&mut self, pos: usize, mode: CheckMode) {
        if pos >= self.filter.len() {
            return;
        }
        match self.filter[pos] {
            Filter::Item(index) => {
                let item = &mut self.data[index as usize];
                match mode {
                    CheckMode::True => item.set_checked(true),
                    CheckMode::False => item.set_checked(false),
                    CheckMode::Reverse => item.set_checked(!item.is_checked()),
                }
            }
            Filter::Group(_) => todo!(),
        }
    }
    fn check_items(&mut self, start: usize, end: usize, mode: CheckMode) {
        let len = self.filter.len();
        if len == 0 {
            return;
        }
        let p_start = start.min(end).min(len - 1);
        let p_end = end.max(start).min(len - 1);
        for pos in p_start..=p_end {
            self.check_item(pos, mode);
        }
    }
}

impl<T> OnPaint for ListView<T>
where
    T: ListItem,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        // paint columns
        self.header.paint(surface, theme, &self.base);
        // paint items
        let has_groups = self.paint_items(surface, theme);
        // paint separation lines (columns)
        self.header.paint_columns(surface, theme, &self.base);
        // paint groups if visible
        if has_groups {
            self.paint_groups(surface, theme);
        }
        // paint scroll bars and searh bars
        self.comp.paint(surface, theme, &self.base);
    }
}

impl<T> OnKeyPressed for ListView<T>
where
    T: ListItem,
{
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        let action = self.header.process_key_pressed(key);
        if self.execute_column_header_action(action) {
            return EventProcessStatus::Processed;
        }
        if self.comp.process_key_pressed(key, character) {
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

impl<T> OnMouseEvent for ListView<T>
where
    T: ListItem,
{
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.comp.process_mouse_event(event) {
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
        }
        let action = self.header.process_mouse_event(event);
        if self.execute_column_header_action(action) {
            return EventProcessStatus::Processed;
        }

        // process mouse event for items
        if (action.should_repaint()) || (self.comp.should_repaint()) {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}
impl<T> OnResize for ListView<T>
where
    T: ListItem,
{
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.header.resize(new_size);
        self.comp.resize(self.header.width() as u64, self.filter.len() as u64, &self.base, 1);
    }
}
