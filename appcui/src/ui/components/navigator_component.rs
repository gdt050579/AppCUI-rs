use common::{ControlEvent, ControlEventData};
use pathfinder::events::EventData;
use std::path::PathBuf;

use crate::prelude::textfield::selection::Selection;
use crate::prelude::*;
use crate::utils::glyphs::GlyphParser;
use std::marker::PhantomData;

//TODO: make separate cfgs for different OS
#[cfg(target_os = "windows")]
const PLATFORM_SEPARATOR_CHARACTER: char = '\\';
#[cfg(target_family = "unix")]
const PLATFORM_SEPARATOR_CHARACTER: char = '/';

struct NavigatorDataCacher<T, E, R>
where
    E: crate::utils::NavigatorEntry,
    R: crate::utils::NavigatorRoot,
    T: crate::utils::Navigator<E, R, PathBuf>,
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
    T: crate::utils::Navigator<E, R, PathBuf>,
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
        let folder = Self::get_folder(path);
        if folder != self.cached_path {
            // create cache for this folder
            let folder_contents = navigator.entries(&PathBuf::from(folder.to_string()));
            if !folder_contents.is_empty() {
                self.cached_items.clear();
                self.cached_path = folder.to_string();
                for entry in folder_contents {
                    let cached_item = navigator
                        .join(&PathBuf::from(folder.to_string()), &entry)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    self.cached_items.push(cached_item);
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
    T: crate::utils::Navigator<E, R, PathBuf>,
{
    is_readonly: bool,
    is_case_sensitive: bool,
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
    out_of_focus_surface: Surface,

    // suggestions area
    header_y_ofs: i32,
    expanded_panel_y: i32,
    selected_suggestion_pos: u16,
    start_suggestions_pos: u16,
    expanded_above: bool,

    // unused
    _phantom_t: std::marker::PhantomData<T>,
    _phantom_r: std::marker::PhantomData<R>,
    _phantom_e: std::marker::PhantomData<E>,
}

pub(crate) trait NavigatorComponentControlFunctions<T, E, R>
where
    E: crate::utils::NavigatorEntry,
    R: crate::utils::NavigatorRoot,
    T: crate::utils::Navigator<E, R, PathBuf>,
{
    fn on_resize(&mut self, control: &ControlBase, old_size: Size, new_size: Size);
    fn on_expand(&mut self, control: &ControlBase, direction: ExpandedDirection);
    fn on_focus(&mut self, control: &mut ControlBase);
    fn on_lose_focus(&mut self, control: &mut ControlBase);
    fn on_key_pressed(&mut self, control: &mut ControlBase, key: Key, character: char, navigator: &T) -> EventProcessStatus;
    fn on_paint(&self, control: &ControlBase, surface: &mut Surface, theme: &Theme);
}

impl<T, E, R> NavigatorComponent<T, E, R>
where
    E: crate::utils::NavigatorEntry,
    R: crate::utils::NavigatorRoot,
    T: crate::utils::Navigator<E, R, PathBuf>,
{
    const PADDING_LEFT: u16 = 1;
    const PADDING_RIGHT: u16 = 1;
    const PADDING: u16 = Self::PADDING_LEFT + Self::PADDING_RIGHT;
    const PATH_TRIANGLE_SEPARTOR: SpecialChar = SpecialChar::TriangleRight;
    const PATH_CHAR_DOTS: SpecialChar = SpecialChar::ThreePointsHorizontal;
    const PATH_FINDER_VISIBLE_RESULTS: u16 = 5;
    const PATH_FINDER_RESULTS_Y_OFFSET: u16 = 2;

    pub(crate) fn new(path: &str, readonly: bool, case_sensitive: bool) -> Self {
        Self {
            input_path: path.to_string(),
            backup_path: path.to_string(),
            char_count: path.chars().count() as u32,
            cursor: 0,
            start: 0,
            end: 0,
            selection: Selection::NONE,
            out_of_focus_surface: Surface::new(1, 1),
            is_readonly: readonly,
            is_case_sensitive: case_sensitive,
            width: 0,
            header_y_ofs: 0,
            expanded_panel_y: 1,
            selected_suggestion_pos: 0,
            start_suggestions_pos: 1,
            expanded_above: false,
            navigator_cacher: NavigatorDataCacher::new(),
            _phantom_r: PhantomData,
            _phantom_t: PhantomData,
            _phantom_e: PhantomData,
        }
    }

    #[inline(always)]
    pub(crate) fn path(&self) -> &str {
        &self.input_path
    }

    #[inline(always)]
    pub(crate) fn set_input_path(&mut self, text: &str, overwrite_prev: bool, control: &ControlBase) {
        if overwrite_prev {
            self.backup_path.clear();
            self.backup_path.push_str(self.input_path.as_str());
        };
        self.input_path.clear();
        self.input_path.push_str(text);
        self.update_char_count(self.input_path.chars().count() as i32, true);
        if !control.has_focus() {
            self.update_out_of_focus_surface(control.theme());
        }
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

    fn get_path_items(text: &str) -> Vec<String> {
        text.trim_start_matches(PLATFORM_SEPARATOR_CHARACTER)
            .trim_end_matches(PLATFORM_SEPARATOR_CHARACTER)
            .split(PLATFORM_SEPARATOR_CHARACTER)
            .map(String::from)
            .collect()
    }

    fn update_out_of_focus_surface(&mut self, theme: &Theme) {
        self.out_of_focus_surface.clear(Character::with_attributes(' ', theme.editor.normal));

        let (string_fits, processed_path) = self.text_fits_textbox(&self.input_path);
        if string_fits {
            self.update_fitting_text(theme, &processed_path);
        } else {
            self.update_trimmed_text(theme);
        }
    }

    fn paint_textbox_in_focus(&self, surface: &mut Surface, attr: CharAttribute, attr_selected: CharAttribute) {
        let w = (self.width - 1) as i32;
        let mut count = self.width - 2;
        let mut pos = self.start;
        let mut x = 1;
        let mut y = self.header_y_ofs;
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

        if pos == self.cursor {
            if (y == 1) && (x == 1) {
                surface.set_cursor(self.width as i32 - 1, 0);
            } else {
                surface.set_cursor(x, y);
            }
        }
    }

    fn text_fits_textbox(&self, text: &str) -> (bool, String) {
        let s = text
            .trim_start_matches(PLATFORM_SEPARATOR_CHARACTER)
            .trim_end_matches(PLATFORM_SEPARATOR_CHARACTER)
            .to_string()
            .replace(PLATFORM_SEPARATOR_CHARACTER, &format!(" {} ", char::from(Self::PATH_TRIANGLE_SEPARTOR)));

        (s.chars().count() < self.width as usize, s)
    }

    fn update_fitting_text(&mut self, theme: &Theme, text: &str) {
        self.update_text_at(theme, text, 1);
    }

    fn update_trimmed_text(&mut self, theme: &Theme) {
        let items = Self::get_path_items(&self.input_path);
        if items.is_empty() {
            return;
        }

        let separator_size = 3;
        let mut left_text = String::new();
        let mut right_text = String::new();
        let fitting_chars_no = self.width as usize - Self::PADDING as usize;
        let mut str_start = 0;
        let mut str_end = items.len() - 1;
        let mut no_printed_chars = 0;

        while str_start < str_end {
            let left_item = &items[str_start];
            let left_char_count = left_item.chars().count();
            if no_printed_chars + left_char_count + separator_size < fitting_chars_no {
                left_text.push_str(left_item);
                left_text.push(' ');
                left_text.push(char::from(Self::PATH_TRIANGLE_SEPARTOR));
                left_text.push(' ');
                no_printed_chars = no_printed_chars + left_char_count + separator_size
            } else {
                break;
            }

            let right_item = &items[str_end];
            let right_char_count = right_item.chars().count();
            if no_printed_chars + right_char_count + separator_size < fitting_chars_no {
                right_text.insert_str(0, right_item);
                right_text.insert(0, ' ');
                right_text.insert(0, char::from(Self::PATH_TRIANGLE_SEPARTOR));
                right_text.insert(0, ' ');
                no_printed_chars = no_printed_chars + right_char_count + separator_size
            } else {
                break;
            }

            str_start += 1;
            str_end -= 1;
        }

        left_text.push(char::from(Self::PATH_CHAR_DOTS));
        left_text.push_str(&right_text);
        self.update_text_at(theme, &left_text, 1);
    }

    fn paint_textbox_out_of_focus(&self, surface: &mut Surface, _theme: &Theme) {
        surface.draw_surface(0, 0, &self.out_of_focus_surface);
    }

    fn update_text_at(&mut self, theme: &Theme, text: &str, pos: usize) {
        let mut x = pos as i32;
        for ch in text.chars() {
            let attr = if ch == char::from(Self::PATH_TRIANGLE_SEPARTOR) {
                theme.editor.hovered
            } else {
                theme.editor.normal
            };
            self.out_of_focus_surface.write_char(x, 0, Character::with_attributes(ch, attr));
            x += 1;
        }
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

    fn pack_suggestions_area(&mut self, control: &mut ControlBase) {
        if control.is_expanded() {
            control.pack();
            self.header_y_ofs = 0;
        }
    }

    fn update_suggestions_selection(&mut self, offset: i32) -> Option<String> {
        let offset = match self.expanded_above {
            true => 0 - offset,
            _ => offset
        };

        let suggestions = self.navigator_cacher.get_suggestions();
        let new_pos: i32 = self.selected_suggestion_pos as i32 + offset;
        let end_visible_pos = (self.start_suggestions_pos + Self::PATH_FINDER_VISIBLE_RESULTS - 1).min(suggestions.len() as u16);

        (self.selected_suggestion_pos, self.start_suggestions_pos) = match () {
            _ if suggestions.is_empty() => (0, 1),
            _ if new_pos <= 0 => (0, 1),
            _ if new_pos < self.start_suggestions_pos as i32 => (new_pos as u16, (self.start_suggestions_pos as i32 + offset) as u16),
            _ if new_pos <= end_visible_pos as i32 => (new_pos as u16, self.start_suggestions_pos),
            _ if new_pos <= suggestions.len() as i32 => (new_pos as u16, (self.start_suggestions_pos as i32 + offset) as u16),
            _ => (suggestions.len() as u16, self.start_suggestions_pos),
        };
        if self.selected_suggestion_pos > 0 {
            return Some(suggestions[self.selected_suggestion_pos as usize - 1].clone());
        }
        None
    }

    fn paint_suggestions_area_bottom(&self, control: &ControlBase, surface: &mut Surface, attr: CharAttribute, attr_selected: CharAttribute) {
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

        if self.selected_suggestion_pos > 0 {
            surface.fill_rect(
                Rect::with_size(
                    Self::PADDING_LEFT as i32,
                    self.expanded_panel_y + self.selected_suggestion_pos as i32 - self.start_suggestions_pos as i32 + 1,
                    size.width as u16 - Self::PADDING,
                    1,
                ),
                Character::with_attributes(' ', attr_selected),
            );
        }

        let mut y = self.expanded_panel_y + 1;
        let suggestions = self.navigator_cacher.get_suggestions();
        let start_index: usize = self.start_suggestions_pos as usize - 1;
        let end_index: usize = (start_index + Self::PATH_FINDER_VISIBLE_RESULTS as usize).min(suggestions.len());
        for path_entry in &suggestions[start_index..end_index] {
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

            let style = if self.selected_suggestion_pos as i32 - self.start_suggestions_pos as i32 == y - 2 {
                attr_selected
            } else {
                attr
            };
            surface.write_string(Self::PADDING_LEFT as i32, y, &path[..offset], style, false);
            y += 1;
        }
    }

    fn paint_suggestions_area_top(&self, control: &ControlBase, surface: &mut Surface, attr: CharAttribute, attr_selected: CharAttribute) {
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

        if self.selected_suggestion_pos > 0 {
            surface.fill_rect(
                Rect::with_size(
                    Self::PADDING_LEFT as i32,
                    self.header_y_ofs - 2 - self.selected_suggestion_pos as i32 + self.start_suggestions_pos as i32,
                    size.width as u16 - Self::PADDING,
                    1,
                ),
                Character::with_attributes(' ', attr_selected),
            );
        }

        let mut y = self.header_y_ofs - 2;
        let suggestions = self.navigator_cacher.get_suggestions();
        let start_index: usize = self.start_suggestions_pos as usize - 1;
        let end_index: usize = (start_index + Self::PATH_FINDER_VISIBLE_RESULTS as usize).min(suggestions.len());
        for path_entry in &suggestions[start_index..end_index] {
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

            let style = if self.selected_suggestion_pos as i32 - self.start_suggestions_pos as i32 == self.header_y_ofs - y - 2 {
                attr_selected
            } else {
                attr
            };
            surface.write_string(Self::PADDING_LEFT as i32, y, &path[..offset], style, false);
            y -= 1;
        }
    }

    fn paint_suggestions_area(&self, control: &ControlBase, surface: &mut Surface, attr: CharAttribute, attr_selected: CharAttribute) {
        match self.expanded_above {
            true => self.paint_suggestions_area_top(control, surface, attr, attr_selected),
            _ => self.paint_suggestions_area_bottom(control, surface, attr, attr_selected),
        }
    }
}

impl<T, E, R> NavigatorComponentControlFunctions<T, E, R> for NavigatorComponent<T, E, R>
where
    E: crate::utils::NavigatorEntry,
    R: crate::utils::NavigatorRoot,
    T: crate::utils::Navigator<E, R, PathBuf>,
{
    fn on_resize(&mut self, control: &ControlBase, _old_size: Size, new_size: Size) {
        self.width = new_size.width;
        self.out_of_focus_surface.resize(new_size);
        self.update_out_of_focus_surface(control.theme());
        self.move_cursor_at_end();
    }

    fn on_expand(&mut self, control: &ControlBase, direction: ExpandedDirection) {
        match direction {
            ExpandedDirection::OnTop => {
                self.expanded_panel_y = 0;
                self.header_y_ofs = (control.expanded_size().height as i32) - 1;
                self.expanded_above = true;
            }
            ExpandedDirection::OnBottom => {
                self.expanded_panel_y = 1;
                self.header_y_ofs = 0;
                self.expanded_above = false;
            }
        }
    }

    fn on_focus(&mut self, control: &mut ControlBase) {
        self.width = control.size().width;
        self.move_cursor_at_end();
        self.navigator_cacher.suggestions.clear();
        control.pack();
    }

    fn on_lose_focus(&mut self, control: &mut ControlBase) {
        self.update_out_of_focus_surface(control.theme());
    }

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
            self.paint_textbox_in_focus(surface, attr, theme.editor.pressed_or_selectd);
            if control.is_expanded() {
                self.paint_suggestions_area(control, surface, attr, theme.editor.pressed_or_selectd);
            }
        } else {
            self.paint_textbox_out_of_focus(surface, theme);
        }
    }

    fn on_key_pressed(&mut self, control: &mut ControlBase, key: Key, character: char, navigator: &T) -> EventProcessStatus {
        match key.value() {
            key!("Backspace") => {
                self.delete_previous_character();
                self.update_char_count(-1, false);

                self.selected_suggestion_pos = 0;
                self.start_suggestions_pos = 1;
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
                self.selected_suggestion_pos = 0;
                self.start_suggestions_pos = 1;
                self.pack_suggestions_area(control);
                control.raise_event(ControlEvent {
                    emitter: control.handle,
                    receiver: control.event_processor,
                    data: ControlEventData::PathFinder(EventData {}),
                });
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
                    self.set_input_path(&selected_path, false, control);
                } else {
                    self.restore_path_from_backup();
                }
                self.move_cursor_to(self.input_path.len(), false, false);
                return EventProcessStatus::Processed;
            }
            key!("Down") => {
                if let Some(selected_path) = self.update_suggestions_selection(1) {
                    self.set_input_path(&selected_path, false, control);
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

                    self.selected_suggestion_pos = 0;
                    self.start_suggestions_pos = 1;
                    self.navigator_cacher.update_suggestions(&self.input_path, navigator);
                    self.expand_suggestions_area(control);
                    return EventProcessStatus::Processed;
                }
            }
        }
        EventProcessStatus::Ignored
    }
}
