use crate::prelude::textfield::selection::Selection;
use crate::prelude::*;
use crate::utils::glyphs::GlyphParser;
use std::marker::PhantomData;


//TODO: make separate cfgs for different OS
const PLATFORM_SEPARATOR_CHARACTER: char = '\\';

struct NavigatorDataCacher<T, E, R>
where
    E: crate::utils::NavigatorEntry,
    R: crate::utils::NavigatorRoot,
    T: crate::utils::Navigator<E, R>,
{
    cached_path: String,
    cached_items: Vec<String>,
    suggestions: Vec<String>,
    _phantom_t: std::marker::PhantomData<T>,
    _phantom_r: std::marker::PhantomData<R>,
    _phantom_e: std::marker::PhantomData<E>,
}

impl<T, E, R> NavigatorDataCacher<T, E, R>
where
    E: crate::utils::NavigatorEntry,
    R: crate::utils::NavigatorRoot,
    T: crate::utils::Navigator<E, R>,
{
    fn new() -> Self {
        Self {
            cached_path: String::new(),
            cached_items: vec![],
            suggestions: vec![],
            _phantom_r: PhantomData,
            _phantom_t: PhantomData,
            _phantom_e: PhantomData,
        }
    }
    fn get_suggestions(&self) -> &Vec<String> {
        &self.suggestions
    }
    fn update_suggestions(&mut self, path: &str, navigator: &T) {
        println!("path = {}", path);
        // get folder for the input path
        let folder = Self::get_folder(path);
        println!("folder = {}", folder);
        if folder != self.cached_path {
            // create cache for this folder
            let folder_contents = navigator.entries(folder);
            if !folder_contents.is_empty() {
                self.cached_items.clear();
                self.cached_path = folder.to_string();
                for entry in folder_contents {
                    if entry.is_container() && !entry.name().ends_with(PLATFORM_SEPARATOR_CHARACTER) {
                        let mut container_path = entry.name().to_string();
                        container_path.push(PLATFORM_SEPARATOR_CHARACTER);
                        self.cached_items.push(container_path);
                    }
                    else {
                        self.cached_items.push(entry.name().to_string());
                    }
                }
            }
        }
        self.suggestions = Self::get_matching_paths(path, &self.cached_items);
    }
    fn get_folder(path: &str) -> &str {
        let mut end = path.len();
        while let Some((ch, sz)) = path.previous_glyph(end) {
            end -= sz as usize;
            if ch == PLATFORM_SEPARATOR_CHARACTER || end == 0 {
                break;
            }
        }
        &path[..end]
    }
    fn get_matching_paths(path: &str, items: &[String]) -> Vec<String> {
        items.iter().filter(|s| s.starts_with(path)).cloned().collect()
    }
}
pub(crate) struct NavigatorComponent<T, E, R>
where
    E: crate::utils::NavigatorEntry,
    R: crate::utils::NavigatorRoot,
    T: crate::utils::Navigator<E, R>,
{   
    is_readonly: bool,
    navigator_cacher: NavigatorDataCacher<T, E, R>,

    // input area
    cursor: usize,
    start: usize,
    end: usize,
    width: u32,
    input_path: String,
    backup_path: String,
    char_count: u32,
    selection: Selection,

    // suggestions area    
    header_y_ofs: i32,
    expanded_panel_y: i32,
    selected: u16,

    // unused
    _phantom_t: std::marker::PhantomData<T>,
    _phantom_r: std::marker::PhantomData<R>,
    _phantom_e: std::marker::PhantomData<E>,
}

pub(crate) trait NavigatorComponentControlFunctions<T, E, R>
where
    E: crate::utils::NavigatorEntry,
    R: crate::utils::NavigatorRoot,
    T: crate::utils::Navigator<E, R>,
{
    fn on_expand(&mut self, control: &ControlBase, direction: ExpandedDirection);
    fn on_focus(&mut self, control: &mut ControlBase);
    fn on_key_pressed(&mut self, control: &mut ControlBase, key: Key, character: char, navigator: &T) -> EventProcessStatus;
    fn expand_suggestions_area(&self, control: &mut ControlBase);
}

pub(crate) trait NavigatorComponentPaintFunctions {
    fn on_paint(&self, control: &ControlBase, surface: &mut Surface, theme: &Theme);
    fn paint_textbox_in_focus(&self, control: &ControlBase, surface: &mut Surface, attr: CharAttribute, attr_selected: CharAttribute);
    fn paint_textbox_out_of_focus(&self, control: &ControlBase, surface: &mut Surface, attr: CharAttribute);
    fn paint_suggestions_area(&self, control: &ControlBase, surface: &mut Surface, attr: CharAttribute, attr_selected: CharAttribute);
}

impl<T, E, R> NavigatorComponent<T, E, R>
where
    E: crate::utils::NavigatorEntry,
    R: crate::utils::NavigatorRoot,
    T: crate::utils::Navigator<E, R>,
{
    const PADDING_LEFT: u16 = 1;
    const PADDING_RIGHT: u16 = 1;
    const PADDING: u16 = Self::PADDING_LEFT + Self::PADDING_RIGHT;
    const PATH_CHAR_SEPARTOR: SpecialChar = SpecialChar::TriangleRight;
    const PATH_CHAR_DOTS: SpecialChar = SpecialChar::ThreePointsHorizontal;
    const PATH_FINDER_VISIBLE_RESULTS: u16 = 5;
    const PATH_FINDER_RESULTS_Y_OFFSET: u16 = 2;

    pub(crate) fn new(path: &str, readonly: bool) -> Self {
        Self {
            input_path: path.to_string(),
            backup_path: path.to_string(),
            char_count: path.chars().count() as u32,
            cursor: 0,
            start: 0,
            end: 0,
            selection: Selection::NONE,
            is_readonly: readonly,
            width: 0,
            //suggestions: vec![],
            header_y_ofs: 0,
            expanded_panel_y: 1,
            selected: 0,
            navigator_cacher: NavigatorDataCacher::new(),
            _phantom_r: PhantomData,
            _phantom_t: PhantomData,
            _phantom_e: PhantomData,
        }
    }

    #[inline(always)]
    fn set_input_path(&mut self, text: &str, overwrite_prev: bool) {
        if overwrite_prev {
            self.backup_path = self.input_path.clone();
        }
        self.input_path = text.to_string();
        self.update_char_count(self.input_path.chars().count() as i32, true);
    }

    #[inline(always)]
    fn restore_path_from_backup(&mut self) {
        self.input_path = self.backup_path.clone();
        self.update_char_count(self.input_path.chars().count() as i32, true);
    }

    #[inline(always)]
    fn update_char_count(&mut self, update_size: i32, set_absolute: bool) {
        if set_absolute {
            self.char_count = update_size as u32;
            return;
        }
        if self.char_count as i32 + update_size > 0 {
            self.char_count = (self.char_count as i32 + update_size) as u32;
        } else {
            self.char_count = 0;
        }
    }

    fn update_text_area_view(&mut self, force_end_update: bool) {
        if (self.cursor >= self.start) && (self.cursor < self.end) {
            if force_end_update {
                let visible_glyphs = (self.width as usize) - Self::PADDING as usize;
                self.end = self.input_path.next_pos(self.start, visible_glyphs);
            }
            return;
        }
        let visible_glyphs = (if self.width > Self::PADDING as u32 {
            (self.width as usize) - Self::PADDING as usize
        } else {
            0
        })
        .max(1);

        if self.cursor < self.start {
            self.start = self.cursor;
            self.end = self.input_path.next_pos(self.cursor, visible_glyphs);
        } else {
            // scroll to the right
            self.start = self.input_path.previous_pos(self.cursor, visible_glyphs - Self::PADDING_RIGHT as usize);
            // we add ONE to the end pot to satisfy (self.pos < self.end) condition
            self.end = self.cursor;
        }
    }

    fn move_cursor_with(&mut self, no_of_glyphs: i32, select: bool) {
        let new_poz = if no_of_glyphs >= 0 {
            self.input_path.next_pos(self.cursor, no_of_glyphs as usize)
        } else {
            self.input_path.previous_pos(self.cursor, (-no_of_glyphs) as usize)
        };
        self.move_cursor_to(new_poz, select, false);
    }

    fn move_cursor_to(&mut self, new_offset: usize, select: bool, force_end_update: bool) {
        let current_pos = self.cursor;
        self.cursor = new_offset.min(self.input_path.len());
        self.update_text_area_view(force_end_update);
        if select {
            self.selection.update(current_pos, self.cursor);
        } else {
            self.selection = Selection::NONE;
        }
    }

    fn move_cursor_at_end(&mut self) {
        self.move_cursor_to(self.input_path.len(), false, false);
    }

    fn delete_selection(&mut self) {
        if !self.selection.is_empty() {
            let new_pos = self.selection.start;
            self.input_path.replace_range(self.selection.start..self.selection.end, "");
            self.selection = Selection::NONE;
            self.move_cursor_to(new_pos, false, true);
        }
    }

    fn delete_current_character(&mut self) {
        if self.is_readonly {
            return;
        }
        if self.selection.is_empty() {
            let next_pos = self.input_path.next_pos(self.cursor, 1);
            if self.cursor < next_pos {
                self.input_path.replace_range(self.cursor..next_pos, "");
                self.update_text_area_view(true);
            }
        } else {
            self.delete_selection();
        }
    }

    fn delete_previous_character(&mut self) {
        if self.is_readonly {
            return;
        }
        if self.selection.is_empty() {
            let prev_pos = self.input_path.previous_pos(self.cursor, 1);
            if prev_pos < self.cursor {
                let end_pos = self.cursor;
                self.input_path.replace_range(prev_pos..end_pos, "");
                self.move_cursor_to(prev_pos, false, true);
            }
        } else {
            self.delete_selection();
        }
    }

    fn get_out_of_focus_char(&self, ch: char, is_middle: bool) -> char {
        if is_middle {
            return char::from(Self::PATH_CHAR_DOTS);
        }
        match ch {
            PLATFORM_SEPARATOR_CHARACTER => char::from(Self::PATH_CHAR_SEPARTOR),            
            _ => ch,
        }
    }

    fn update_suggestions_selection(&mut self, offset: i32) -> Option<String> {
        let result: i32 = self.selected as i32 + offset;
        let max_visible_selected = self.navigator_cacher.suggestions.len().min(Self::PATH_FINDER_VISIBLE_RESULTS as usize);

        if result <= 0 {
            self.selected = 0;
        } else if result < max_visible_selected as i32 {
            self.selected = result as u16;
        } else {
            self.selected = max_visible_selected as u16;
        }
        if self.selected > 0 {
            return Some(self.navigator_cacher.suggestions[self.selected as usize - 1].clone());
        }
        None
    }
}

impl<T, E, R> NavigatorComponentControlFunctions<T, E, R> for NavigatorComponent<T, E, R>
where
    E: crate::utils::NavigatorEntry,
    R: crate::utils::NavigatorRoot,
    T: crate::utils::Navigator<E, R>,
{
    fn on_expand(&mut self, control: &ControlBase, direction: ExpandedDirection) {
        match direction {
            ExpandedDirection::OnTop => {
                self.expanded_panel_y = -1;
                self.header_y_ofs = (control.expanded_size().height as i32) - 1;
            }
            ExpandedDirection::OnBottom => {
                self.expanded_panel_y = 1;
                self.header_y_ofs = 0;
            }
        }
    }

    fn on_focus(&mut self, control: &mut ControlBase) {
        self.width = control.size().width;
        self.move_cursor_at_end();
        self.navigator_cacher.suggestions.clear();
        control.pack();
    }

    fn on_key_pressed(&mut self, control: &mut ControlBase, key: Key, character: char, navigator: &T) -> EventProcessStatus {
        match key.value() {
            key!("Backspace") => {
                self.delete_previous_character();
                self.update_char_count(-1, false);
                self.navigator_cacher.update_suggestions(&self.input_path, navigator);
                self.expand_suggestions_area(control);
                return EventProcessStatus::Processed;
            }
            key!("Delete") => {
                self.delete_current_character();
                return EventProcessStatus::Processed;
            }
            key!("Left") | key!("Shift+Left") => {
                self.move_cursor_with(-1, key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Right") | key!("Shift+Right") => {
                self.move_cursor_with(1, key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Home") | key!("Shift+Home") => {
                self.move_cursor_to(0, key.modifier.contains(KeyModifier::Shift), false);
                return EventProcessStatus::Processed;
            }
            key!("End") | key!("Shift+End") => {
                self.move_cursor_to(self.input_path.len(), key.modifier.contains(KeyModifier::Shift), false);
                return EventProcessStatus::Processed;
            }
            key!("Enter") => {
                return EventProcessStatus::Processed;
            }
            key!("Esc") => {
                if control.is_expanded() {
                    control.pack();
                    return EventProcessStatus::Processed;
                }
            }
            key!("Up") => {
                if let Some(selected_path) = self.update_suggestions_selection(-1) {
                    self.set_input_path(&selected_path, false);
                } else {
                    self.restore_path_from_backup();
                }
                self.move_cursor_to(self.input_path.len(), false, false);
                return EventProcessStatus::Processed;
            }
            key!("Down") => {
                if let Some(selected_path) = self.update_suggestions_selection(1) {
                    self.set_input_path(&selected_path, false);
                } else {
                    self.restore_path_from_backup();
                }
                self.move_cursor_to(self.input_path.len(), false, false);
                return EventProcessStatus::Processed;
            }
            _ => {
                if character > 0 as char {                    
                    self.input_path.push(character);
                    self.update_char_count(1, false);
                    self.move_cursor_with(1, key.modifier.contains(KeyModifier::Shift));

                    self.selected = 0;
                    self.navigator_cacher.update_suggestions(&self.input_path, navigator);
                    self.expand_suggestions_area(control);
                    return EventProcessStatus::Processed;
                }
            }
        }
        EventProcessStatus::Ignored
    }

    fn expand_suggestions_area(&self, control: &mut ControlBase) {
        if !control.is_expanded() {
            control.expand(
                Size::new(
                    control.size().width,
                    (Self::PATH_FINDER_VISIBLE_RESULTS + Self::PATH_FINDER_RESULTS_Y_OFFSET + 1) as u32,
                ),
                Size::new(
                    control.size().width,
                    (Self::PATH_FINDER_VISIBLE_RESULTS + Self::PATH_FINDER_RESULTS_Y_OFFSET + 1) as u32,
                ),
            );
        }
    }
}

impl<T, E, R> NavigatorComponentPaintFunctions for NavigatorComponent<T, E, R>
where
    E: crate::utils::NavigatorEntry,
    R: crate::utils::NavigatorRoot,
    T: crate::utils::Navigator<E, R>,
{
    fn on_paint(&self, control: &ControlBase, surface: &mut Surface, theme: &Theme) {
        let attr = match () {
            _ if !control.is_enabled() => theme.editor.inactive,
            _ if control.has_focus() => theme.editor.focused,
            _ if control.is_mouse_over() => theme.editor.hovered,
            _ => theme.editor.normal,
        };
        surface.clear(Character::with_attributes(' ', attr));
        // paint
        if control.has_focus() {
            self.paint_textbox_in_focus(control, surface, attr, theme.editor.pressed_or_selectd);
            if control.is_expanded() {
                self.paint_suggestions_area(control, surface, attr, theme.editor.pressed_or_selectd);
            }
        } else {
            self.paint_textbox_out_of_focus(control, surface, attr);
        }
    }

    fn paint_textbox_in_focus(&self, control: &ControlBase, surface: &mut Surface, attr: CharAttribute, attr_selected: CharAttribute) {
        let sz = control.size();
        let w = (sz.width - 1) as i32;
        let mut count = (sz.width - 2) * sz.height;
        let mut pos = self.start;
        let mut x = 1;
        let mut y = 0;
        let mut ch = Character::with_attributes(' ', attr);
        let mut ch_selected = Character::with_attributes(' ', attr_selected);
        while let Some((code, glyph_size)) = self.input_path.glyph(pos) {
            if self.selection.contains(pos) {
                ch_selected.code = code;
                surface.write_char(x, y, ch_selected);
            } else {
                ch.code = code;
                surface.write_char(x, y, ch);
            }
            if pos == self.cursor {
                surface.set_cursor(x, y);
            }
            x += 1;
            if x >= w {
                x = 1;
                y += 1;
            }
            pos += glyph_size as usize;
            count -= 1;
            if count == 0 {
                break;
            }
        }
        // if it is the last char
        if pos == self.cursor {
            // if the cursor is located on the fist line outside the view --> put it on the last char but on previous line
            if (y == sz.height as i32) && (x == 1) {
                surface.set_cursor(sz.width as i32 - 1, sz.height as i32 - 1);
            } else {
                surface.set_cursor(x, y);
            }
        }
    }

    fn paint_textbox_out_of_focus(&self, control: &ControlBase, surface: &mut Surface, attr: CharAttribute) {
        // out of focus, draw trimmed path
        let string_fits = self.char_count < control.size().width;
        let mut start = 0;
        let mut end = self.input_path.len();
        if end == 0 {
            return;
        }

        while self.input_path.glyph(end).is_none() {
            end -= 1;
        }

        let mut left = Self::PADDING_LEFT;
        let mut right = (control.size().width - Self::PADDING as u32).min(self.char_count) as u16;

        while left <= right {
            let ch_left = self.input_path.glyph(start).unwrap();
            let ch_right = self.input_path.glyph(end).unwrap();

            let mut ch = Character::with_attributes(' ', attr);
            ch.code = self.get_out_of_focus_char(ch_left.0, false);
            surface.write_char(left as i32, 0, ch);
            ch.code = self.get_out_of_focus_char(ch_right.0, (right - left < 2) && !string_fits);
            surface.write_char(right as i32, 0, ch);

            left += 1;
            right -= 1;
            start = self.input_path.next_pos(start, 1);
            end = self.input_path.previous_pos(end, 1);
        }
    }

    fn paint_suggestions_area(&self, control: &ControlBase, surface: &mut Surface, attr: CharAttribute, attr_selected: CharAttribute) {
        let size = control.expanded_size();
        surface.fill_rect(
            Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
            Character::with_attributes(' ', attr),
        );
        surface.draw_rect(
            Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
            LineType::Single,
            attr,
        );

        if self.selected > 0 {
            surface.fill_rect(
                Rect::with_size(
                    Self::PADDING_LEFT as i32,
                    self.expanded_panel_y + self.selected as i32,
                    size.width as u16 - Self::PADDING,
                    1,
                ),
                Character::with_attributes(' ', attr_selected),
            );
        }

        let mut y = self.expanded_panel_y + 1;
        for path_entry in self.navigator_cacher.get_suggestions() {
            let path = path_entry;
            if y - 1 > Self::PATH_FINDER_VISIBLE_RESULTS as i32 {
                break;
            }
            // paint fitting part of the current path
            let mut offset = 0;
            let mut count_chars = 0;
            while count_chars < self.width - Self::PADDING as u32 {
                let old_offset = offset;
                offset = path.next_pos(offset, 1);
                if offset == old_offset {
                    //end of string
                    break;
                }
                count_chars += 1;
            }

            let style = if self.selected as i32 == y - 1 { attr_selected } else { attr };
            surface.write_string(Self::PADDING_LEFT as i32, y, &path[..offset], style, false);
            y += 1;
        }
    }
}
