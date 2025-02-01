use super::events::EventData;
use super::{Flags, FoldStatus, Item, TreeDataManager};
use components::listitem::render_method::RenderData;
use AppCUIProcMacro::*;

enum UpdateVisibleItemsOperation {
    Refresh,
    Sort,
    Refilter,
    SortAndRefilter,
    SortAndRefilterWithoutPositionUpdate,
}

#[derive(Clone, Copy)]
enum SelectMode {
    True,
    False,
    Reverse,
}

#[derive(Copy, Clone)]
enum FoldMethod {
    Expand,
    Collapse,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum HoverStatus {
    None,
    OverFoldButton(i32, usize),
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct TreeView<T>
where
    T: ListItem + 'static,
{
    flags: Flags,
    manager: TreeDataManager<T>,
    item_list: Vec<Handle<Item<T>>>,
    header: ColumnsHeader,
    comp: ListScrollBars,
    top_view: usize,
    pos: usize,
    icon_width: u8,
    fold_sign_with: u8,
    hover_status: HoverStatus,
    update_item_list_enabled: bool,
    start_mouse_select: usize,
    mouse_check_mode: SelectMode,
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
        let mut lv = Self {
            base: ControlBase::with_status_flags(layout, status_flags),
            flags,
            top_view: 0,
            pos: 0,
            manager: TreeDataManager::with_capacity(capacity as u32),
            item_list: Vec::with_capacity(capacity),
            header: ColumnsHeader::with_capacity(4),
            comp: ListScrollBars::new(flags.contains(Flags::ScrollBars), flags.contains(Flags::SearchBar)),
            icon_width: if flags.contains(Flags::LargeIcons) {
                3 // includes the extra space
            } else if flags.contains(Flags::SmallIcons) {
                2 // includes the extra space
            } else {
                0 // No extra space
            },
            fold_sign_with: 3,
            update_item_list_enabled: true,
            start_mouse_select: 0,
            mouse_check_mode: SelectMode::False,
            hover_status: HoverStatus::None,
        };
        // add columnes (if described in the type T)
        for i in 0..T::columns_count() {
            lv.header.add(T::column(i));
        }
        lv
    }
    #[inline(always)]
    pub fn add(&mut self, item: T) -> Handle<Item<T>> {
        self.add_item_to_parent(Item::non_expandable(item), Handle::None)
    }
    #[inline(always)]
    pub fn add_to_parent(&mut self, item: T, parent: Handle<Item<T>>) -> Handle<Item<T>> {
        self.add_item_to_parent(Item::non_expandable(item), parent)
    }
    #[inline(always)]
    pub fn add_item(&mut self, item: Item<T>) -> Handle<Item<T>> {
        self.add_item_to_parent(item, Handle::None)
    }
    #[inline(always)]
    pub fn add_item_to_parent(&mut self, mut item: Item<T>, parent: Handle<Item<T>>) -> Handle<Item<T>> {
        // override selection state if the NoSelection flag is set
        if self.flags.contains(Flags::NoSelection) {
            item.set_selected(false);
        }
        let h = self.manager.add(item, parent);
        // refilter everything
        self.update_item_list(UpdateVisibleItemsOperation::SortAndRefilter);
        h
    }
    pub fn add_batch<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self),
    {
        let old_state = self.update_item_list_enabled;
        self.update_item_list_enabled = false;
        f(self);
        // restore original refilter state
        self.update_item_list_enabled = old_state;
        self.update_item_list(UpdateVisibleItemsOperation::SortAndRefilter);
    }

    /// Sets the number of frozen columns. Frozen columns are columns that are always visible, even when the list view is scrolled horizontally. The frozen columns are always the first columns in the list view. Using the value 0 will disable frozen columns.
    pub fn set_frozen_columns(&mut self, count: u16) {
        self.header.set_frozen_columns(count);
        self.update_scrollbars();
    }

    fn update_item_list(&mut self, op: UpdateVisibleItemsOperation) {
        let current_handle = if self.pos < self.item_list.len() {
            self.item_list[self.pos]
        } else {
            Handle::None
        };
        // sorting
        if matches!(
            op,
            UpdateVisibleItemsOperation::SortAndRefilter
                | UpdateVisibleItemsOperation::Sort
                | UpdateVisibleItemsOperation::SortAndRefilterWithoutPositionUpdate
        ) {
            if let Some(column_index) = self.header.sort_column() {
                self.manager.sort(column_index, self.header.should_sort_ascendent());
            }
        }
        // refilter
        if matches!(
            op,
            UpdateVisibleItemsOperation::SortAndRefilter
                | UpdateVisibleItemsOperation::Refilter
                | UpdateVisibleItemsOperation::SortAndRefilterWithoutPositionUpdate
        ) {
            self.manager.refilter(
                self.comp.search_text(),
                if self.flags.contains(Flags::CustomFilter) {
                    None
                } else {
                    Some(&self.header)
                },
            );
        }
        // clear all elements
        self.item_list.clear();
        // reserve space for the entire list + groups
        self.item_list.reserve(self.manager.len());
        // populate filter with items
        self.manager.populate(&mut self.item_list);
        // restore previous position
        let update_position = !matches!(op, UpdateVisibleItemsOperation::SortAndRefilterWithoutPositionUpdate);
        if (!current_handle.is_none()) && update_position {
            // check to see if the current handle has match
            let matched = if let Some(item) = self.manager.get(current_handle) {
                item.has_matched()
            } else {
                false
            };
            if matched {
                self.goto_handle(current_handle, false);
            } else {
                // the current handle is not matched anymore
                // go to the first visible
                self.goto_next_match(0, true);
            }
        }
    }

    fn goto_handle(&mut self, handle: Handle<Item<T>>, emit_event: bool) -> bool {
        if let Some(index) = self.item_list.iter().position(|h| *h == handle) {
            self.update_position(index, emit_event);
            true
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn item(&self, item_handle: Handle<Item<T>>) -> Option<&Item<T>> {
        self.manager.get(item_handle)
    }

    #[inline(always)]
    pub fn item_mut(&mut self, item_handle: Handle<Item<T>>) -> Option<&mut Item<T>> {
        self.manager.get_mut(item_handle)
    }

    #[inline(always)]
    pub fn current_item_handle(&self) -> Option<Handle<Item<T>>> {
        if self.pos < self.item_list.len() {
            Some(self.item_list[self.pos])
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn current_item(&self) -> Option<&Item<T>> {
        if self.pos < self.item_list.len() {
            self.manager.get(self.item_list[self.pos])
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn current_item_mut(&mut self) -> Option<&mut Item<T>> {
        if self.pos < self.item_list.len() {
            let h = self.item_list[self.pos];
            self.manager.get_mut(h)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn root_items(&self) -> &[Handle<Item<T>>] {
        self.manager.roots()
    }
    #[inline(always)]
    pub fn root_item(&self, index: usize) -> Option<&Item<T>> {
        let len = self.manager.roots().len();
        if index < len {
            self.manager.get(self.manager.roots()[index])
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn root_item_mut(&mut self, index: usize) -> Option<&mut Item<T>> {
        let len = self.manager.roots().len();
        if index < len {
            self.manager.get_mut(self.manager.roots()[index])
        } else {
            None
        }
    }

    pub fn delete_item(&mut self, item_handle: Handle<Item<T>>) {
        let pos = self.pos;
        self.manager.delete(item_handle);
        self.update_item_list(UpdateVisibleItemsOperation::SortAndRefilterWithoutPositionUpdate);
        let len = self.item_list.len();
        self.pos = len;
        if pos < len {
            self.update_position(pos, true);
        } else {
            self.update_position(len.saturating_sub(1), true);
        }
    }

    pub fn clear(&mut self) {
        self.manager.clear();
        self.pos = 0;
        self.update_item_list(UpdateVisibleItemsOperation::Refresh);
        self.update_scrollbars();
    }

    pub fn delete_item_children(&mut self, item_handle: Handle<Item<T>>) {
        self.manager.delete_children(item_handle);
        self.update_item_list(UpdateVisibleItemsOperation::SortAndRefilter);
    }

    fn inner_fold_item(&mut self, item_handle: Handle<Item<T>>, method: FoldMethod, emit_event: bool, recursive: bool) -> bool {
        let changed = if recursive {
            match method {
                FoldMethod::Expand => self.manager.set_fold_status(item_handle, FoldStatus::Expanded),
                FoldMethod::Collapse => self.manager.set_fold_status(item_handle, FoldStatus::Collapsed),
            }
        } else {
            match method {
                FoldMethod::Expand => self.manager.get_mut(item_handle).map(|f| f.expand_fold()).unwrap_or(false),
                FoldMethod::Collapse => self.manager.get_mut(item_handle).map(|f| f.collapse_fold()).unwrap_or(false),
            }
        };

        if changed {
            self.update_item_list(UpdateVisibleItemsOperation::Refresh);
            if emit_event {
                self.emit_expand_collapse_action_event(self.pos, matches!(method, FoldMethod::Expand), recursive);
            }
            true
        } else {
            false
        }
    }
    pub fn collapse_item(&mut self, item_handle: Handle<Item<T>>, recursive: bool) {
        self.inner_fold_item(item_handle, FoldMethod::Collapse, false, recursive);
    }
    pub fn expand_item(&mut self, item_handle: Handle<Item<T>>, recursive: bool) {
        self.inner_fold_item(item_handle, FoldMethod::Expand, false, recursive);
    }
    pub fn collapse_all(&mut self) {
        let current_handle = if self.pos < self.item_list.len() {
            self.item_list[self.pos]
        } else {
            Handle::None
        };
        self.manager.collapse_all();
        self.update_item_list(UpdateVisibleItemsOperation::Refresh);
        if !self.goto_handle(current_handle, false) {
            self.update_position(0, false);
        }
        self.update_scrollbars();
    }

    pub fn expand_all(&mut self) {
        let current_handle = if self.pos < self.item_list.len() {
            self.item_list[self.pos]
        } else {
            Handle::None
        };
        self.manager.expand_all();
        self.update_item_list(UpdateVisibleItemsOperation::Refresh);
        if !self.goto_handle(current_handle, false) {
            self.update_position(0, false);
        }
        self.update_scrollbars();
    }

    /// Returns the number of items in the tree view
    pub fn items_count(&self) -> usize {
        self.manager.count()
    }
    /// Returns the number of selected items
    pub fn selected_items_count(&self) -> usize {
        self.manager.selected_count()
    }

    /// Change the selection state of the item at the specified index
    pub fn select_item(&mut self, item_handle: Handle<Item<T>>, selected: bool) {
        if let Some(item) = self.manager.get_mut(item_handle) {
            let current_value = item.is_selected();
            item.set_selected(selected);
            if current_value != selected {
                self.manager.update_selected_count(selected);
            }
        }
    }

    fn goto_next_match(&mut self, start: usize, emit_event: bool) {
        let len = self.item_list.len();
        if len == 0 {
            return;
        }
        for index in 0..len {
            let new_pos = (index + start) % len;
            let handle = self.item_list[new_pos];
            if let Some(item) = self.manager.get(handle) {
                if item.has_matched() {
                    self.update_position(new_pos, emit_event);
                    return;
                }
            }
        }
    }

    pub fn sort(&mut self, column_index: u16, ascendent: bool) {
        self.header.set_sort_column(column_index, ascendent, true);
        self.update_item_list(UpdateVisibleItemsOperation::Sort);
    }

    /// Clears the content of the search bar
    pub fn clear_search(&mut self) {
        self.comp.clear_search();
        self.update_item_list(UpdateVisibleItemsOperation::Refresh);
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
        let d = item.x_offset(self.fold_sign_with, self.icon_width, false);
        let l = c.x + d;
        let r = c.x + c.width as i32;
        let mut extra = 0;
        let mut rd = RenderData {
            theme,
            alignment: TextAlignament::Left,
            width: 0,
            attr: if item.is_visible_because_of_children() {
                Some(theme.text.inactive)
            } else {
                None
            },
        };
        let attr = if item.is_visible_because_of_children() {
            Some(theme.text.inactive)
        } else {
            attr
        };
        if (r >= 0) && (l < width) && (c.width != 0)
        /*&& (r >= min_left)*/
        {
            if d > 0 {
                surface.set_relative_clip(c.x - 2, y, r.max(min_left), y);
                surface.set_origin(c.x - 2, y);
                let line_mask = item.line_mask;
                let space = (self.icon_width + 6) as i32;
                for i in 1..item.depth {
                    if line_mask & (1 << (i - 1)) != 0 {
                        surface.write_char(
                            (i as i32) * space,
                            0,
                            Character::with_attributes(SpecialChar::BoxVerticalSingleLine, attr.unwrap_or(theme.text.inactive)),
                        );
                    }
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
                if item.depth < 32 {
                    let mask = 1 << (item.depth - 1);
                    if item.line_mask & mask != 0 {
                        surface.write_string(extra, 0, "├─", attr.unwrap_or(theme.text.inactive), false);
                    } else {
                        surface.write_string(extra, 0, "└─", attr.unwrap_or(theme.text.inactive), false);
                    }
                }
                extra += 2;
            }
            let (s, fold_attr) = match item.fold_status {
                FoldStatus::Collapsed => ("[+]", theme.text.normal),
                FoldStatus::Expanded => ("[-]", theme.text.normal),
                FoldStatus::NonExpandable => ("──▷", theme.text.inactive),
            };
            surface.write_string(extra, 0, s, attr.unwrap_or(fold_attr), false);
            extra += (self.fold_sign_with as i32) + 1;
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
        let item_size = self.item_width();
        let max_y = self.size().height as i32;
        let mut idx = self.top_view;
        let max_idx = self.item_list.len();
        let visible_items = self.visible_items();
        let mut item_count = 0;
        let (hover_checkmark_x, hover_pos) = match self.hover_status {
            HoverStatus::OverFoldButton(x, pos) => (x, pos),
            _ => (0, usize::MAX),
        };
        // very simply code
        while (item_count < visible_items) && (idx < max_idx) && (y < max_y) {
            if let Some(item) = self.manager.get(self.item_list[idx]) {
                self.paint_item(item, y, surface, theme, attr);
                if (item.is_selected()) && (has_focus) {
                    surface.reset_clip();
                    surface.reset_origin();
                    //let x = item.x_offset(self.fold_sign_with, self.icon_width, true) + self.header.columns()[0].x;
                    surface.fill_horizontal_line_with_size(0, y, item_size, Character::with_attributes(0, theme.list_current_item.selected));
                }
                if is_enabled {
                    if idx == self.pos {
                        surface.reset_clip();
                        surface.reset_origin();
                        if has_focus {
                            let current_item_attr = match () {
                                _ if item.is_selected() => theme.list_current_item.over_selection,
                                _ => theme.list_current_item.focus,
                            };
                            surface.fill_horizontal_line_with_size(0, y, item_size, Character::with_attributes(0, current_item_attr));
                        }
                    }
                    if idx == hover_pos {
                        surface.reset_clip();
                        surface.reset_origin();
                        surface.fill_horizontal_line_with_size(
                            hover_checkmark_x,
                            y,
                            self.fold_sign_with as u32,
                            Character::with_attributes(0, theme.button.text.hovered),
                        );
                    }
                }
            }
            y += 1;
            idx += 1;
            item_count += 1;
        }
        surface.reset_clip();
        surface.reset_origin();
    }
    fn autoresize_column(&mut self, column_index: u16) {
        let mut new_width = 0u32;
        let mut found = false;
        for handle in self.item_list.iter() {
            if let Some(item) = self.manager.get(*handle) {
                if let Some(rm) = item.value().render_method(column_index) {
                    let x_offset =
                        item.x_offset(self.fold_sign_with, self.icon_width, true) + 1 + self.icon_width as i32 + self.fold_sign_with as i32;
                    if column_index == 0 {
                        new_width = new_width.max(listview::RenderMethod::min_width(&rm) + x_offset as u32);
                    } else {
                        new_width = new_width.max(listview::RenderMethod::min_width(&rm));
                    }
                    found = true;
                }
            }
        }
        if found {
            self.header.set_column_width(column_index, new_width.min(u8::MAX as u32) as u8);
        }
    }
    fn update_scrollbars(&mut self) {
        self.comp
            .resize(self.header.width() as u64, self.item_list.len() as u64, &self.base, self.visible_space());
        self.comp.set_indexes(self.header.scroll_pos() as u64, self.top_view as u64);
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        self.header.scroll_to(self.comp.horizontal_index() as u32);
        self.top_view = (self.comp.vertical_index() as usize).min(self.item_list.len());
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
                self.hover_status = HoverStatus::None;
                self.update_scrollbars();
                true
            }
            ColumnsHeaderAction::Processed => true,
            ColumnsHeaderAction::None => false,
            ColumnsHeaderAction::Repaint => false,
        }
    }
    fn update_position(&mut self, new_pos: usize, emit_event: bool) {
        let len = self.item_list.len();
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
        if (should_emit) && (self.pos < len) {
            self.raise_event(ControlEvent {
                emitter: self.handle,
                receiver: self.event_processor,
                data: ControlEventData::TreeView(EventData {
                    event_type: treeview::events::TreeViewEventTypes::CurrentItemChanged(self.item_list[self.pos].cast()),
                    type_id: std::any::TypeId::of::<T>(),
                }),
            });
        }
    }
    fn move_scroll_to(&mut self, new_poz: usize) {
        if new_poz == self.top_view {
            return;
        }
        let visible_items = self.visible_items();
        let max_value = self.item_list.len().saturating_sub(visible_items);
        self.top_view = new_poz.min(max_value);
        self.hover_status = HoverStatus::None;
        self.update_scrollbars();
    }
    fn emit_selection_update_event(&self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::TreeView(EventData {
                event_type: treeview::events::TreeViewEventTypes::SelectionChanged,
                type_id: std::any::TypeId::of::<T>(),
            }),
        });
    }
    fn emit_item_action_event(&self, index: usize) {
        if index < self.item_list.len() {
            self.raise_event(ControlEvent {
                emitter: self.handle,
                receiver: self.event_processor,
                data: ControlEventData::TreeView(EventData {
                    event_type: treeview::events::TreeViewEventTypes::ItemAction(self.item_list[index].cast()),
                    type_id: std::any::TypeId::of::<T>(),
                }),
            });
        }
    }
    fn emit_expand_collapse_action_event(&self, index: usize, is_expanded: bool, recursive: bool) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::TreeView(EventData {
                event_type: if is_expanded {
                    treeview::events::TreeViewEventTypes::ItemExpanded(self.item_list[index].cast(), recursive)
                } else {
                    treeview::events::TreeViewEventTypes::ItemCollapsed(self.item_list[index].cast(), recursive)
                },
                type_id: std::any::TypeId::of::<T>(),
            }),
        });
    }

    #[inline(always)]
    fn toggle_current_item_selection(&self) -> SelectMode {
        if self.pos < self.item_list.len() {
            if let Some(item) = self.manager.get(self.item_list[self.pos]) {
                if item.is_selected() {
                    SelectMode::False
                } else {
                    SelectMode::True
                }
            } else {
                SelectMode::False
            }
        } else {
            SelectMode::False
        }
    }
    fn select_until_position(&mut self, new_pos: usize) {
        let initial = self.pos;
        let mode = self.toggle_current_item_selection();
        self.update_position(new_pos, true);
        let start = initial.min(self.pos);
        let end = initial.max(self.pos);
        let mut changed = false;
        for i in start..=end {
            changed |= self.select_item_at_pos(i, mode, true);
        }
        if changed {
            self.emit_selection_update_event();
        }
    }
    fn select_item_at_pos(&mut self, pos: usize, mode: SelectMode, emit_event: bool) -> bool {
        if self.flags.contains(Flags::NoSelection) {
            return false;
        }
        if pos >= self.item_list.len() {
            return false;
        }
        let h = self.item_list[pos];
        if let Some(item) = self.manager.get_mut(h) {
            let current_select_status = item.is_selected();
            match mode {
                SelectMode::True => item.set_selected(true),
                SelectMode::False => item.set_selected(false),
                SelectMode::Reverse => item.set_selected(!current_select_status),
            }
            if item.is_selected() != current_select_status {
                self.manager.update_selected_count(!current_select_status);
                if emit_event {
                    self.emit_selection_update_event();
                }
                return true;
            }
        }
        false
    }
    fn select_items(&mut self, start: usize, end: usize, mode: SelectMode, emit_event: bool) {
        if self.flags.contains(Flags::NoSelection) {
            return;
        }
        let len = self.item_list.len();
        if len == 0 {
            return;
        }
        let p_start = start.min(end).min(len - 1);
        let p_end = end.max(start).min(len - 1);
        let mut selection_has_changed = false;
        for pos in p_start..=p_end {
            selection_has_changed |= self.select_item_at_pos(pos, mode, false);
        }
        if (emit_event) && (selection_has_changed) {
            self.emit_selection_update_event();
        }
    }


    fn reverse_fold(&mut self, recursive: bool) -> bool {
        if self.pos < self.item_list.len() {
            let h = self.item_list[self.pos];
            let fold_status = self.manager.get(h).map(|f| f.fold_status).unwrap_or(FoldStatus::NonExpandable);
            match fold_status {
                FoldStatus::Collapsed => self.inner_fold_item(h, FoldMethod::Expand, true, recursive),
                FoldStatus::Expanded => self.inner_fold_item(h, FoldMethod::Collapse, true, recursive),
                FoldStatus::NonExpandable => false,
            }
        } else {
            false
        }
    }
    fn mouse_pos_to_index(&self, x: i32, y: i32) -> Option<usize> {
        let sz = self.size();
        let start_y = if self.flags.contains(Flags::HideHeader) { 0 } else { 1 };
        if (y >= start_y) && (x >= 0) && (x < sz.width as i32) && (y < sz.height as i32) {
            let new_pos = self.top_view + (y - start_y) as usize;
            if new_pos < self.item_list.len() {
                Some(new_pos)
            } else {
                None
            }
        } else {
            None
        }
    }
    fn hover_status_for_mouse_pos(&self, pos: usize, x: i32) -> HoverStatus {
        if (pos >= self.item_list.len()) || (self.header.columns().is_empty()) {
            return HoverStatus::None;
        }
        let left_pos = self.header.columns()[0].x;
        if let Some(item) = self.manager.get(self.item_list[pos]) {
            let p_x = left_pos + item.x_offset(self.fold_sign_with, self.icon_width, true);
            if (item.fold_status != FoldStatus::NonExpandable) && (x >= p_x) && (x < p_x + self.fold_sign_with as i32) {
                return HoverStatus::OverFoldButton(p_x, pos);
            }
        }
        HoverStatus::None
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
                self.update_position(self.item_list.len(), true);
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
            key!("Insert") | key!("Shift+Down") => {
                self.select_item_at_pos(self.pos, SelectMode::Reverse, true);
                self.update_position(self.pos.saturating_add(1), true);
                true
            }
            key!("Shift+Up") => {
                self.select_item_at_pos(self.pos, SelectMode::Reverse, true);
                self.update_position(self.pos.saturating_sub(1), true);
                true
            }
            key!("Shift+Home") => {
                self.select_until_position(0);
                true
            }
            key!("Shift+End") => {
                self.select_until_position(self.item_list.len());
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

            // Action & folding
            key!("Space") => {
                self.reverse_fold(false);
                true
            }
            key!("Ctrl+Space") => {
                self.reverse_fold(true);
                true
            }
            key!("Enter") => {
                if self.comp.is_in_edit_mode() {
                    // will be process separately
                    return false;
                }
                self.emit_item_action_event(self.pos);
                true
            }
            _ => false,
        }
    }
    fn process_mouse_event(&mut self, event: &MouseEvent) -> bool {
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                if self.hover_status != HoverStatus::None {
                    self.hover_status = HoverStatus::None;
                    true
                } else {
                    false
                }
            }
            MouseEvent::Over(point) => {
                let new_hover_status = if let Some(pos) = self.mouse_pos_to_index(point.x, point.y) {
                    self.hover_status_for_mouse_pos(pos, point.x)
                } else {
                    HoverStatus::None
                };
                if new_hover_status != self.hover_status {
                    self.hover_status = new_hover_status;
                    true
                } else {
                    false
                }
            }
            MouseEvent::Pressed(ev) => {
                if let Some(pos) = self.mouse_pos_to_index(ev.x, ev.y) {
                    if pos != self.pos {
                        self.update_position(pos, true);
                    }
                    if let HoverStatus::OverFoldButton(_, _) = self.hover_status_for_mouse_pos(self.pos, ev.x) {
                        self.reverse_fold(ev.modifier.contains(KeyModifier::Ctrl));
                    }
                    self.start_mouse_select = self.pos;
                    self.mouse_check_mode = self.toggle_current_item_selection();
                } else {
                    self.start_mouse_select = usize::MAX;
                }
                true
            }
            MouseEvent::Released(_) => true,
            MouseEvent::DoubleClick(ev) => {
                if let Some(pos) = self.mouse_pos_to_index(ev.x, ev.y) {
                    if pos != self.pos {
                        self.update_position(pos, true);
                    }
                    self.emit_item_action_event(self.pos);
                }
                true
            }
            MouseEvent::Drag(ev) => {
                if self.start_mouse_select != usize::MAX {
                    if let Some(pos) = self.mouse_pos_to_index(ev.x, ev.y) {
                        if pos != self.pos {
                            self.update_position(pos, true);                    
                            self.select_items(self.start_mouse_select, pos, self.mouse_check_mode, true);
                        }
                    }
                }
                true
            }
            MouseEvent::Wheel(dir) => {
                match dir {
                    MouseWheelDirection::Up => self.move_scroll_to(self.top_view.saturating_sub(1)),
                    MouseWheelDirection::Down => self.move_scroll_to(self.top_view.saturating_add(1)),
                    MouseWheelDirection::Left => {
                        OnKeyPressed::on_key_pressed(self, Key::new(KeyCode::Left, KeyModifier::None), 0 as char);
                    }
                    MouseWheelDirection::Right => {
                        OnKeyPressed::on_key_pressed(self, Key::new(KeyCode::Right, KeyModifier::None), 0 as char);
                    }
                    _ => {}
                }
                true
            }
        }
    }
}
impl<T> OnPaint for TreeView<T>
where
    T: ListItem + 'static,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        // paint columns
        if !self.flags.contains(Flags::HideHeader) {
            self.header.paint(surface, theme, &self.base);
        }
        // paint items
        self.paint_items(surface, theme);
        // paint separation lines (columns) - show wheather or not HideHeader is set or not
        self.header.paint_columns(surface, theme, &self.base);
        // paint scroll bars and searh bars
        self.comp.paint(surface, theme, &self.base);
    }
}
impl<T> OnKeyPressed for TreeView<T>
where
    T: ListItem + 'static,
{
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        let action = self.header.process_key_pressed(key);
        if self.execute_column_header_action(action) {
            return EventProcessStatus::Processed;
        }
        if self.comp.process_key_pressed(key, character) {
            self.update_item_list(UpdateVisibleItemsOperation::Refilter);
            return EventProcessStatus::Processed;
        }
        if self.comp.is_in_edit_mode() && key.code == KeyCode::Enter {
            self.goto_next_match(self.pos + 1, true);
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
impl<T> OnMouseEvent for TreeView<T>
where
    T: ListItem + 'static,
{
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.comp.process_mouse_event(event) {
            self.update_scroll_pos_from_scrollbars();
            self.hover_status = HoverStatus::None;
            return EventProcessStatus::Processed;
        }
        let action = if !self.flags.contains(Flags::HideHeader) {
            let action = self.header.process_mouse_event(event);
            if self.execute_column_header_action(action) {
                return EventProcessStatus::Processed;
            }
            action
        } else {
            ColumnsHeaderAction::None
        };
        // process mouse event for items
        if self.process_mouse_event(event) {
            return EventProcessStatus::Processed;
        }
        if (action.should_repaint()) || (self.comp.should_repaint()) {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}
impl<T> OnResize for TreeView<T>
where
    T: ListItem + 'static,
{
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.header.resize(new_size);
        self.comp
            .resize(self.header.width() as u64, self.item_list.len() as u64, &self.base, self.visible_space());
    }
}
