use super::attribute_text::AttributeText;
use super::undo::{LastAction, UndoEntry, UndoOp, MAX_UNDO_DEPTH};
use crate::prelude::*;
use crate::ui::common::{ControlEvent, ControlEventData};
use crate::ui::textfield::selection::Selection;
use crate::ui::textfield::events::{EventData, TextFieldEventsType};
use crate::ui::textfield::{CharClass, Flags};

struct Cursor {
    pos: usize,
    start: usize,
    end: usize,
}

#[inline(always)]
fn default_character(code: char) -> Character {
    Character {
        code,
        foreground: Color::Transparent,
        background: Color::Transparent,
        flags: CharFlags::None,
    }
}

#[inline(always)]
fn is_variation_selector(c: char) -> bool {
    matches!(c, '\u{FE00}'..='\u{FE0F}' | '\u{E0100}'..='\u{E01EF}')
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize+OnFocus, internal=true)]
pub struct RichTextField {
    chars: Vec<Character>,
    text_cache: String,
    parser: Option<fn(&mut AttributeText, &Theme)>,
    cursor: Cursor,
    selection: Selection,
    drag_started: bool,
    flags: Flags,
    undo_stack: Vec<UndoEntry>,
    redo_stack: Vec<UndoEntry>,
    last_action: LastAction,
}

impl RichTextField {
    /// Creates a new `RichTextField` without a parser callback.
    ///
    /// The control starts with `text` content, `layout` position/size and behavior defined by `flags`.
    pub fn new(text: &str, layout: Layout, flags: Flags) -> Self {
        Self::with_parser_inner(text, layout, flags, None)
    }

    /// Creates a new `RichTextField` with a parser callback.
    ///
    /// The parser is called after each text mutation and can change per-character attributes
    /// (and characters) through [`AttributeText`], using the current [`Theme`].
    pub fn with_parser(text: &str, layout: Layout, flags: Flags, parser: fn(&mut AttributeText, &Theme)) -> Self {
        Self::with_parser_inner(text, layout, flags, Some(parser))
    }

    fn with_parser_inner(text: &str, layout: Layout, flags: Flags, parser: Option<fn(&mut AttributeText, &Theme)>) -> Self {
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            chars: Vec::new(),
            text_cache: String::new(),
            parser,
            cursor: Cursor { pos: 0, start: 0, end: 0 },
            selection: Selection::NONE,
            drag_started: false,
            flags,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            last_action: LastAction::None,
        };
        obj.set_size_bounds(3, 1, u16::MAX, u16::MAX);
        obj.set_text(text);
        obj
    }

    /// Returns `true` if the field is read-only.
    ///
    /// In read-only mode, navigation and selection still work, but text mutations are ignored.
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        self.flags.contains(Flags::Readonly)
    }

    /// Returns the current text as a borrowed string slice.
    ///
    /// The returned value references an internal cache rebuilt after each mutation.
    #[inline(always)]
    pub fn text(&self) -> &str {
        &self.text_cache
    }

    /// Replaces the entire text content.
    ///
    /// Cursor and selection are reset, variation selectors are ignored, and parser output
    /// is recomputed.
    pub fn set_text(&mut self, text: &str) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.last_action = LastAction::None;
        self.chars.clear();
        self.chars
            .extend(text.chars().filter(|c| !is_variation_selector(*c)).map(default_character));
        self.selection = Selection::NONE;
        self.cursor = Cursor { pos: 0, start: 0, end: 0 };
        self.move_cursor_to(self.chars.len(), false, true);
        self.sync_after_mutation();
    }

    /// Installs or replaces the parser callback used for per-character styling.
    ///
    /// The callback is executed immediately on current content.
    pub fn set_parser(&mut self, parser: fn(&mut AttributeText, &Theme)) {
        self.parser = Some(parser);
        self.sync_after_mutation();
    }

    /// Removes the parser callback and resets all character styling to defaults.
    pub fn reset_parser(&mut self) {
        self.parser = None;
        self.sync_after_mutation();
    }

    fn sync_after_mutation(&mut self) {
        self.text_cache.clear();
        for ch in &self.chars {
            self.text_cache.push(ch.code);
        }
        for ch in &mut self.chars {
            ch.foreground = Color::Transparent;
            ch.background = Color::Transparent;
            ch.flags = CharFlags::None;
        }
        if let Some(parser) = self.parser {
            let theme = RuntimeManager::get().theme();
            let mut view = AttributeText { chars: &mut self.chars };
            parser(&mut view, theme);
        }
    }

    fn update_scroll_view(&mut self, force_end_update: bool) {
        if (self.cursor.pos >= self.cursor.start) && (self.cursor.pos < self.cursor.end) {
            if force_end_update {
                let sz = self.size();
                let visible_glyphs = ((sz.width as usize).saturating_sub(2)) * (sz.height as usize);
                self.cursor.end = (self.cursor.start + visible_glyphs).min(self.chars.len());
            }
            return;
        }
        let sz = self.size();
        let visible_glyphs = (if sz.width > 2 {
            ((sz.width as usize) - 2) * (sz.height as usize)
        } else {
            0
        })
        .max(1); 

        if self.cursor.pos < self.cursor.start {
            self.cursor.start = self.cursor.pos;
            self.cursor.end = (self.cursor.start + visible_glyphs).min(self.chars.len());
        } else {
            self.cursor.start = self.cursor.pos.saturating_sub(visible_glyphs.saturating_sub(1));
            // Keep the glyph at/after the cursor visible when scrolling right.
            self.cursor.end = (self.cursor.pos + 1).min(self.chars.len());
        }
    }

    fn move_cursor_with(&mut self, delta: i32, select: bool) {
        let len = self.chars.len() as i32;
        let new_pos = (self.cursor.pos as i32 + delta).clamp(0, len) as usize;
        self.move_cursor_to(new_pos, select, false);
    }

    fn move_cursor_to(&mut self, new_pos: usize, select: bool, force_end_update: bool) {
        let current_pos = self.cursor.pos;
        self.cursor.pos = new_pos.min(self.chars.len());
        self.update_scroll_view(force_end_update);
        if select {
            self.selection.update(current_pos, self.cursor.pos);
        } else {
            self.selection = Selection::NONE;
        }
    }

    fn move_to_next_word(&mut self, select: bool) {
        if self.cursor.pos >= self.chars.len() {
            return;
        }
        let char_class = CharClass::from(self.chars[self.cursor.pos].code);
        let mut pos = self.cursor.pos;
        let mut new_char_class = char_class;
        while pos < self.chars.len() {
            let c = self.chars[pos].code;
            if CharClass::from(c) != char_class {
                new_char_class = CharClass::from(c);
                break;
            }
            pos += 1;
        }
        if (new_char_class != char_class) && (new_char_class == CharClass::Space) {
            while pos < self.chars.len() {
                let c = self.chars[pos].code;
                if CharClass::from(c) != new_char_class {
                    break;
                }
                pos += 1;
            }
        }
        self.move_cursor_to(pos.min(self.chars.len()), select, false);
    }

    fn move_to_previous_word(&mut self, select: bool) {
        if self.cursor.pos == 0 {
            return;
        }
        let char_class = CharClass::from(self.chars[self.cursor.pos - 1].code);
        let mut pos = self.cursor.pos;
        let mut new_char_class = char_class;
        while pos > 0 {
            let c = self.chars[pos - 1].code;
            if CharClass::from(c) != char_class {
                new_char_class = CharClass::from(c);
                break;
            }
            pos -= 1;
        }
        if (new_char_class != char_class) && (char_class == CharClass::Space) {
            while pos > 0 {
                let c = self.chars[pos - 1].code;
                if CharClass::from(c) != new_char_class {
                    break;
                }
                pos -= 1;
            }
        }
        self.move_cursor_to(pos, select, false);
    }

    #[inline]
    fn char_index_to_byte_offset(&self, char_index: usize) -> usize {
        if char_index == 0 {
            return 0;
        }
        if char_index >= self.chars.len() {
            return self.text_cache.len();
        }
        self.text_cache
            .char_indices()
            .nth(char_index)
            .map_or(self.text_cache.len(), |(byte_offset, _)| byte_offset)
    }

    fn copy_text(&mut self) {
        if self.selection.is_empty() {
            return;
        }
        let start = self.char_index_to_byte_offset(self.selection.start);
        let end = self.char_index_to_byte_offset(self.selection.end);
        RuntimeManager::get().backend_mut().set_clipboard_text(&self.text_cache[start..end]);
    }

    fn paste_text(&mut self) -> bool {
        if self.is_readonly() {
            return false;
        }
        let mut modified = false;
        if !self.selection.is_empty() {
            modified = self.delete_selection();
        }
        if let Some(txt) = RuntimeManager::get().backend().clipboard_text() {
            if !txt.is_empty() {
                let insert_pos = self.cursor.pos;
                let new_chars: Vec<Character> = txt
                    .chars()
                    .filter(|c| !is_variation_selector(*c))
                    .map(default_character)
                    .collect();
                let count = new_chars.len();
                if count > 0 {
                    let entry = UndoEntry {
                        op: UndoOp::Insert {
                            pos: insert_pos,
                            chars: new_chars.clone(),
                        },
                        cursor_before: self.cursor.pos,
                        cursor_after: self.cursor.pos + count,
                        selection_before: self.selection,
                        selection_after: Selection::NONE,
                    };
                    self.push_undo(entry);
                    self.last_action = LastAction::Other;
                    for (i, ch) in new_chars.iter().enumerate() {
                        self.chars.insert(insert_pos + i, *ch);
                    }
                    self.cursor.pos += count;
                    // Insertion can increase content past previous viewport end.
                    // Keep cursor window in sync so paint does not clip newly added tail.
                    self.update_scroll_view(true);
                    modified = true;
                }
            }
        }
        if modified {
            self.sync_after_mutation();
        }
        modified
    }

    fn cut_text(&mut self) -> bool {
        if self.is_readonly() {
            return false;
        }
        if self.selection.is_empty() {
            return false;
        }
        let start = self.char_index_to_byte_offset(self.selection.start);
        let end = self.char_index_to_byte_offset(self.selection.end);
        RuntimeManager::get().backend_mut().set_clipboard_text(&self.text_cache[start..end]);
        self.delete_selection()
    }

    fn convert_selection_or_word(&mut self, callback: fn(&str) -> String) -> bool {
        if self.is_readonly() {
            return false;
        }
        if self.selection.is_empty() {
            self.select_word(self.cursor.pos);
        }
        if self.selection.is_empty() {
            return false;
        }
        let old_chars: Vec<Character> = self.chars[self.selection.start..self.selection.end].to_vec();
        let old_str: String = old_chars.iter().map(|c| c.code).collect();
        let new_str = callback(old_str.as_str());
        let text_changed = new_str != old_str;
        let new_chars: Vec<Character> = new_str
            .chars()
            .filter(|c| !is_variation_selector(*c))
            .map(default_character)
            .collect();
        let entry = UndoEntry {
            op: UndoOp::Replace {
                pos: self.selection.start,
                old_chars,
                new_chars: new_chars.clone(),
            },
            cursor_before: self.cursor.pos,
            cursor_after: self.selection.start + new_chars.len(),
            selection_before: self.selection,
            selection_after: Selection::NONE,
        };
        self.push_undo(entry);
        self.last_action = LastAction::Other;
        let start = self.selection.start;
        self.chars.splice(self.selection.start..self.selection.end, new_chars);
        self.selection = Selection::NONE;
        self.cursor.pos = start;
        let count = new_str.chars().count() as i32;
        self.move_cursor_with(count, true);
        if text_changed {
            self.sync_after_mutation();
        }
        text_changed
    }

    fn select_all(&mut self) {
        self.selection = Selection::NONE;
        self.selection.update(0, self.chars.len());
        self.move_cursor_to(self.chars.len(), true, false);
    }

    fn delete_selection(&mut self) -> bool {
        if self.selection.is_empty() {
            return false;
        }
        let removed: Vec<Character> = self.chars[self.selection.start..self.selection.end].to_vec();
        let entry = UndoEntry {
            op: UndoOp::Delete {
                pos: self.selection.start,
                chars: removed,
            },
            cursor_before: self.cursor.pos,
            cursor_after: self.selection.start,
            selection_before: self.selection,
            selection_after: Selection::NONE,
        };
        self.push_undo(entry);
        self.last_action = LastAction::Other;
        let new_pos = self.selection.start;
        self.chars.drain(self.selection.start..self.selection.end);
        self.selection = Selection::NONE;
        self.move_cursor_to(new_pos, false, true);
        self.sync_after_mutation();
        true
    }

    fn delete_current_character(&mut self) -> bool {
        if self.is_readonly() {
            return false;
        }
        if !self.selection.is_empty() {
            return self.delete_selection();
        }
        if self.cursor.pos < self.chars.len() {
            let removed = self.chars[self.cursor.pos];
            let entry = UndoEntry {
                op: UndoOp::Delete {
                    pos: self.cursor.pos,
                    chars: vec![removed],
                },
                cursor_before: self.cursor.pos,
                cursor_after: self.cursor.pos,
                selection_before: self.selection,
                selection_after: Selection::NONE,
            };
            self.push_undo(entry);
            self.last_action = LastAction::Delete;
            self.chars.remove(self.cursor.pos);
            self.update_scroll_view(true);
            self.sync_after_mutation();
            return true;
        }
        false
    }

    fn delete_previous_character(&mut self) -> bool {
        if self.is_readonly() {
            return false;
        }
        if !self.selection.is_empty() {
            return self.delete_selection();
        }
        if self.cursor.pos > 0 {
            let removed = self.chars[self.cursor.pos - 1];
            let entry = UndoEntry {
                op: UndoOp::Delete {
                    pos: self.cursor.pos - 1,
                    chars: vec![removed],
                },
                cursor_before: self.cursor.pos,
                cursor_after: self.cursor.pos - 1,
                selection_before: self.selection,
                selection_after: Selection::NONE,
            };
            self.push_undo(entry);
            self.last_action = LastAction::Delete;
            self.chars.remove(self.cursor.pos - 1);
            self.move_cursor_to(self.cursor.pos - 1, false, true);
            self.sync_after_mutation();
            return true;
        }
        false
    }

    fn add_char(&mut self, character: char) -> bool {
        if self.is_readonly() {
            return false;
        }
        if is_variation_selector(character) {
            return false;
        }
        if !self.selection.is_empty() {
            self.delete_selection();
        }
        let new_char_class = CharClass::from(character);
        let can_merge = matches!(&self.last_action, LastAction::AddChar(class) if *class == new_char_class);
        if can_merge {
            if let Some(last) = self.undo_stack.last_mut() {
                if let UndoOp::Insert { chars: inserted, .. } = &mut last.op {
                    inserted.push(default_character(character));
                    last.cursor_after = self.cursor.pos + 1;
                    last.selection_after = Selection::NONE;
                }
            }
        } else {
            let entry = UndoEntry {
                op: UndoOp::Insert {
                    pos: self.cursor.pos,
                    chars: vec![default_character(character)],
                },
                cursor_before: self.cursor.pos,
                cursor_after: self.cursor.pos + 1,
                selection_before: self.selection,
                selection_after: Selection::NONE,
            };
            self.push_undo(entry);
        }
        self.last_action = LastAction::AddChar(new_char_class);
        self.chars.insert(self.cursor.pos, default_character(character));
        // Keep viewport end in sync with insertions (same rationale as paste).
        self.move_cursor_to(self.cursor.pos + 1, false, true);
        self.sync_after_mutation();
        true
    }

    fn select_word(&mut self, offset: usize) {
        if offset >= self.chars.len() {
            return;
        }
        if CharClass::from(self.chars[offset].code) != CharClass::Word {
            return;
        }
        let mut start = offset;
        while start > 0 && CharClass::from(self.chars[start - 1].code) == CharClass::Word {
            start -= 1;
        }
        let mut end = offset;
        while end < self.chars.len() && CharClass::from(self.chars[end].code) == CharClass::Word {
            end += 1;
        }
        self.selection = Selection::NONE;
        self.move_cursor_to(start, false, true);
        self.move_cursor_to(end, true, true);
    }

    fn mouse_pos_to_glyph_offset(&self, x: i32, y: i32, within_control: bool) -> Option<usize> {
        let sz = self.size();
        let w = sz.width as i32;
        let h = sz.height as i32;
        if within_control && ((x < 1) || (x >= w - 1) || (y < 0) || (y >= h)) {
            return None;
        }
        let glyphs_count = (x - 1) + y * (w - 2);
        let len = self.chars.len() as i32;
        let idx = (self.cursor.start as i32 + glyphs_count).clamp(0, len) as usize;
        Some(idx)
    }

    fn push_undo(&mut self, entry: UndoEntry) {
        if self.undo_stack.len() >= MAX_UNDO_DEPTH {
            self.undo_stack.remove(0);
        }
        self.undo_stack.push(entry);
        self.redo_stack.clear();
    }

    fn apply_op_forward(chars: &mut Vec<Character>, op: &UndoOp) {
        match op {
            UndoOp::Insert { pos, chars: inserted } => {
                for (i, ch) in inserted.iter().enumerate() {
                    chars.insert(*pos + i, *ch);
                }
            }
            UndoOp::Delete { pos, chars: deleted } => {
                chars.drain(*pos..*pos + deleted.len());
            }
            UndoOp::Replace { pos, old_chars, new_chars } => {
                chars.splice(*pos..*pos + old_chars.len(), new_chars.iter().cloned());
            }
        }
    }

    fn apply_op_inverse(chars: &mut Vec<Character>, op: &UndoOp) {
        match op {
            UndoOp::Insert { pos, chars: inserted } => {
                chars.drain(*pos..*pos + inserted.len());
            }
            UndoOp::Delete { pos, chars: deleted } => {
                for (i, ch) in deleted.iter().enumerate() {
                    chars.insert(*pos + i, *ch);
                }
            }
            UndoOp::Replace { pos, old_chars, new_chars } => {
                chars.splice(*pos..*pos + new_chars.len(), old_chars.iter().cloned());
            }
        }
    }

    fn restore_state(&mut self, cursor_pos: usize, selection: Selection) {
        self.cursor.pos = cursor_pos.min(self.chars.len());
        self.selection = selection;
        self.update_scroll_view(true);
        self.sync_after_mutation();
        self.notify_text_changed();
    }

    /// Reverts the last text mutation, if any, and restores the cursor and selection
    /// to what they were before that mutation.
    pub fn undo(&mut self) {
        if let Some(entry) = self.undo_stack.pop() {
            Self::apply_op_inverse(&mut self.chars, &entry.op);
            let cursor_before = entry.cursor_before;
            let selection_before = entry.selection_before;
            self.redo_stack.push(entry);
            self.last_action = LastAction::None;
            self.restore_state(cursor_before, selection_before);
        }
    }

    /// Re-applies the most recently undone mutation, if any, and restores the cursor and
    /// selection to what they were right after that mutation.
    pub fn redo(&mut self) {
        if let Some(entry) = self.redo_stack.pop() {
            Self::apply_op_forward(&mut self.chars, &entry.op);
            let cursor_after = entry.cursor_after;
            let selection_after = entry.selection_after;
            self.undo_stack.push(entry);
            self.last_action = LastAction::None;
            self.restore_state(cursor_after, selection_after);
        }
    }

    fn notify_text_changed(&mut self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::RichTextField(EventData {
                evtype: TextFieldEventsType::OnTextChanged,
            }),
        });
    }
}

impl OnResize for RichTextField {
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        let visible_chars = if new_size.width > 2 {
            ((new_size.width - 2) as usize) * (new_size.height as usize)
        } else {
            0
        };
        self.cursor.end = (self.cursor.start + visible_chars).min(self.chars.len());
        self.update_scroll_view(false);
    }
}

impl OnPaint for RichTextField {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let attr = match () {
            _ if !self.is_enabled() => theme.editor.inactive,
            _ if self.has_focus() => theme.editor.focused,
            _ if self.is_mouse_over() => theme.editor.hovered,
            _ => theme.editor.normal,
        };
        surface.clear(Character::with_attributes(' ', attr));
        let show_cursor = self.has_focus();
        let use_parser = self.parser.is_some() && show_cursor;
        let sz = self.size();
        let w = (sz.width - 1) as i32;
        let mut x = 1i32;
        let mut y = 0i32;
        let slice_end = self.cursor.end.min(self.chars.len());
        let slice_start = self.cursor.start.min(slice_end);
        for (i, ch) in self.chars[slice_start..slice_end].iter().enumerate() {
            let actual_index = slice_start + i;
            let char_to_paint = if show_cursor && self.selection.contains(actual_index) {
                Character::with_attributes(ch.code, theme.editor.pressed_or_selected)
            } else if use_parser {
                *ch
            } else {
                Character::with_attributes(ch.code, attr)
            };
            surface.write_char(x, y, char_to_paint);
            if show_cursor && actual_index == self.cursor.pos {
                surface.set_cursor(x, y);
            }
            x += 1;
            if x >= w {
                x = 1;
                y += 1;
            }
        }
        // Cursor can be exactly at the right edge of the current viewport (`slice_end`),
        // not only at end-of-text. In that case it must still be visible.
        if show_cursor && self.cursor.pos == slice_end {
            if (y == sz.height as i32) && (x == 1) {
                surface.set_cursor(sz.width as i32 - 1, sz.height as i32 - 1);
            } else {
                surface.set_cursor(x, y);
            }
        }
    }
}

impl OnKeyPressed for RichTextField {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        match key.value() {
            key!("Left") | key!("Shift+Left") => {
                self.last_action = LastAction::None;
                self.move_cursor_with(-1, key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Right") | key!("Shift+Right") => {
                self.last_action = LastAction::None;
                self.move_cursor_with(1, key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Up") | key!("Shift+Up") => {
                self.last_action = LastAction::None;
                self.move_cursor_with(-((self.size().width as i32) - 2), key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Down") | key!("Shift+Down") => {
                self.last_action = LastAction::None;
                self.move_cursor_with((self.size().width as i32) - 2, key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Home") | key!("Shift+Home") => {
                self.last_action = LastAction::None;
                self.move_cursor_to(0, key.modifier.contains(KeyModifier::Shift), false);
                return EventProcessStatus::Processed;
            }
            key!("End") | key!("Shift+End") => {
                self.last_action = LastAction::None;
                self.move_cursor_to(self.chars.len(), key.modifier.contains(KeyModifier::Shift), false);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Left") | key!("Ctrl+Shift+Left") => {
                self.last_action = LastAction::None;
                self.move_to_previous_word(key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Right") | key!("Ctrl+Shift+Right") => {
                self.last_action = LastAction::None;
                self.move_to_next_word(key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+C") | key!("Ctrl+Insert") => {
                self.copy_text();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+X") | key!("Shift+Del") => {
                if self.cut_text() {
                    self.notify_text_changed();
                }
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+V") | key!("Shift+Insert") => {
                if self.paste_text() {
                    self.notify_text_changed();
                }
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Shift+U") => {
                if self.convert_selection_or_word(|s| s.to_uppercase()) {
                    self.notify_text_changed();
                }
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+U") => {
                if self.convert_selection_or_word(|s| s.to_lowercase()) {
                    self.notify_text_changed();
                }
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+A") => {
                self.last_action = LastAction::None;
                self.select_all();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Z") => {
                self.undo();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Y") | key!("Ctrl+Shift+Z") => {
                self.redo();
                return EventProcessStatus::Processed;
            }
            key!("Delete") => {
                if self.delete_current_character() {
                    self.notify_text_changed();
                }
                return EventProcessStatus::Processed;
            }
            key!("Back") => {
                if self.delete_previous_character() {
                    self.notify_text_changed();
                }
                return EventProcessStatus::Processed;
            }
            key!("Enter") => {
                if self.flags.contains(Flags::ProcessEnter) {
                    self.raise_event(ControlEvent {
                        emitter: self.handle,
                        receiver: self.event_processor,
                        data: ControlEventData::RichTextField(EventData {
                            evtype: TextFieldEventsType::OnValidate,
                        }),
                    });
                    return EventProcessStatus::Processed;
                }
            }
            _ => {}
        }
        if (character as u32) > 0 {
            if self.add_char(character) {
                self.notify_text_changed();
            }
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}

impl OnFocus for RichTextField {
    fn on_focus(&mut self) {
        if !self.flags.contains(Flags::DisableAutoSelectOnFocus) {
            self.select_all();
        }
    }
}

impl OnMouseEvent for RichTextField {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                self.drag_started = false;
                EventProcessStatus::Processed
            }
            MouseEvent::Over(_) => EventProcessStatus::Ignored,
            MouseEvent::Pressed(data) => {
                if let Some(new_pos) = self.mouse_pos_to_glyph_offset(data.x, data.y, true) {
                    self.last_action = LastAction::None;
                    self.move_cursor_to(new_pos, false, false);
                    self.drag_started = true;
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Released(_) => {
                self.drag_started = false;
                EventProcessStatus::Processed
            }
            MouseEvent::DoubleClick(data) => {
                if let Some(ofs) = self.mouse_pos_to_glyph_offset(data.x, data.y, true) {
                    self.select_word(ofs);
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Drag(data) => {
                if self.drag_started {
                    if let Some(new_pos) = self.mouse_pos_to_glyph_offset(data.x, data.y, false) {
                        self.last_action = LastAction::None;
                        self.move_cursor_to(new_pos, true, true);
                    }
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(_) => EventProcessStatus::Ignored,
        }
    }
}
