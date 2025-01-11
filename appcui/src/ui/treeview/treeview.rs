use super::{Flags, Item, TreeDataManager};
use components::listitem::render_method::RenderData;
use AppCUIProcMacro::*;

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
impl<T> OnKeyPressed for TreeView<T> where T: ListItem + 'static {}
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
