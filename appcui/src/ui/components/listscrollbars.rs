use super::scrollbars_components::ProcessEventResult;
use super::searchbar::SearchBar;
use super::scrollbars_components::HScrollBar;
use super::scrollbars_components::VScrollBar;
use crate::graphics::*;
use crate::input::*;
use crate::system::Theme;
use crate::ui::ControlBase;

// prefered size of a search bar: 14 characters (1 left, 1 right, 3 for count, 1 space, 8 for text)
// minim size of a search bar: 5 characters (1 left, 1 right, 1 for text)

pub struct ListScrollBars {
    horizontal: HScrollBar,
    vertical: VScrollBar,
    search: SearchBar,
    should_paint: bool,
    has_scrollbars: bool,
    has_searchbar: bool,
}
impl ListScrollBars {
    pub fn new(scrollbars: bool, searchbar: bool) -> Self {
        Self {
            horizontal: HScrollBar::new(scrollbars),
            vertical: VScrollBar::new(scrollbars),
            search: SearchBar::new(searchbar),
            should_paint: false,
            has_scrollbars: scrollbars,
            has_searchbar: searchbar,
        }
    }
    pub fn update(&mut self, horizontal_indexes: u64, vertical_indexes: u64, size: Size) {
        self.horizontal.update(size.width as u64, horizontal_indexes);
        self.vertical.update(size.height as u64, vertical_indexes);
    }
    pub fn paint(&self, surface: &mut Surface, theme: &Theme, control: &ControlBase) {
        if self.has_scrollbars {
            self.horizontal.paint(surface, theme, control);
            self.vertical.paint(surface, theme, control);
        }
        if self.has_searchbar {
            self.search.paint(surface, theme);
        }
    }
    pub fn process_mouse_event(&mut self, event: &MouseEvent) -> bool {
        let mut r = ProcessEventResult::PassToControl;
        if self.has_scrollbars {
            r |= self.horizontal.on_mouse_event(event);
            r |= self.vertical.on_mouse_event(event);
        }
        if self.has_searchbar {
            r |= self.search.on_mouse_event(event);
        }
        self.should_paint = r.should_repaint();
        r.should_update()
    }
    pub fn process_key_pressed(&mut self, key: Key, character: char) -> bool {
        self.should_paint = false;
        if self.has_searchbar {
            self.should_paint = self.search.process_key_pressed(key, character);
            self.should_paint
        } else {
            false
        }
    }
    pub fn resize(&mut self, horizontal_indexes: u64, vertical_indexes: u64, control: &ControlBase) {
        let control_size = control.size();
        let left_margin = control.left_components_margin as i32;
        let top_margin = control.top_components_margin as i32;
        let w = (control_size.width as i32) - (left_margin + 1); // 2 space from right
        let h = (control_size.height as i32) - top_margin; // 1 space from bottom
        let x = left_margin;
        let y = top_margin;
        self.vertical.recompute_position(y, h, control_size);
        if self.has_searchbar && self.has_scrollbars {
            // leave 6 characters for the search bar
            let search_width = self.search.recompute_layout(x, w - 6, control_size);
            self.horizontal.recompute_layout(x + search_width, w - search_width, control_size);
        } else if self.has_scrollbars {
            self.horizontal.recompute_layout(x, w, control_size);
        } else if self.has_searchbar {
            self.search.recompute_layout(x, w, control_size);
        }
        self.update(horizontal_indexes, vertical_indexes, control_size);
    }
    pub fn should_repaint(&self) -> bool {
        self.should_paint
    }
    pub fn set_indexes(&mut self, horizontal: u64, vertical: u64) {
        if self.has_scrollbars {
            self.horizontal.set_index(horizontal);
            self.vertical.set_index(vertical);
        }
    }
    
    #[inline(always)]
    pub fn horizontal_index(&self) -> u64 {
        self.horizontal.index()
    }
    
    #[inline(always)]
    pub fn vertical_index(&self) -> u64 {
        self.vertical.index()
    }
    
    #[inline(always)]
    pub fn search_text(&self) -> &str {
        self.search.text()
    }

    #[inline(always)]
    pub fn set_match_count(&mut self, count: usize) {
        self.search.set_match_count(count);
    }

    #[inline(always)]
    pub fn clear_match_count(&mut self) {
        self.search.clear_match_count();
    }

    #[inline(always)]
    pub fn is_in_edit_mode(&self) -> bool {
        self.search.is_in_edit_mode()
    }

    #[inline(always)]
    pub fn exit_edit_mode(&mut self) {
        self.search.exit_edit_mode()
    }
}
