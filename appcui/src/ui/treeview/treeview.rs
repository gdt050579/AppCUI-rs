use super::{Flags, Item, TreeDataManager};
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
}
impl<T> TreeView<T>
where
    T: ListItem + 'static,
{
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
