use crate::graphics::SpecialChar;
use crate::prelude::*;
use crate::ui::markdown_composer::emoji::{EMOJIS, EMOTICONS};
use crate::ui::markdown_composer::events::{EventData, MarkdownComposerEventsType};
use crate::ui::markdown_composer::parser::{Span, SpanType};
use crate::ui::markdown_composer::Flags;
use crate::ui::markdown_composer::parser::Parser;
use crate::ui::markdown_composer::{List, ListFlags};

const WHEEL_ROWS: u32 = 3;
const BULLET: SpecialChar = SpecialChar::CircleFilled;
const QUOTE_BAR: SpecialChar = SpecialChar::BoxVerticalSingleLine;
const POPUP_ROWS: u32 = 4;
const POPUP_MIN_HEIGHT: u32 = 3;
const POPUP_MIN_WIDTH: u32 = 12;
const POPUP_MAX_WIDTH: u32 = 40;
const INLINE_MARKERS: [&str; 3] = ["**", "_", "`"];
const FENCE: &str = "```";

struct Popup {
    list: usize,
    start: u32,
    matches: Vec<u32>,
    index: u32,
    first: u32,
}

impl Popup {
    fn scroll_to_index(&mut self) {
        if self.index < self.first {
            self.first = self.index;
        } else if self.index >= self.first + POPUP_ROWS {
            self.first = self.index - POPUP_ROWS + 1;
        }
    }
}

#[CustomControl(overwrite=OnPaint+OnResize+OnMouseEvent+OnKeyPressed+OnExpand+OnFocus, internal=true)]
/// A control for writing and editing a markdown text, with the formatting applied while typing.
///
/// The markdown markers are hidden and only their effect is visible, unless
/// [`Flags::ShowMarkers`] is used. Suggestion lists can be opened by a trigger character (for
/// example `:` for emoji or `@` for names) and text emoticons can be replaced with emoji.
pub struct MarkdownComposer {
    text: String,
    surface: Surface,
    parser: Parser,
    cursor_offset: u32,
    cursor_x: u32,
    cursor_y: u32,
    anchor: Option<u32>,
    first_row: u32,
    rows: u32,
    lists: Vec<List>,
    popup: Option<Popup>,
    expanded: bool,
    expanded_offset: i32,
    packed_origin: i32,
    packed_visible: i32,
    emoticons: bool,
    read_only: bool,
    preferred_x: Option<u32>,
}

impl MarkdownComposer {
    /// Creates a new, empty MarkdownComposer control with the specified layout and flags.
    /// The flags can be a combination of the following values:
    /// * `Flags::ShowMarkers` - if set, the markdown markers (`**`, `_`, `` ` ``, `-`, `>`) remain visible instead of being hidden
    /// * `Flags::Emoticons` - if set, text emoticons and emoji names written between colons (for example `:B):` or `:smile:`) are replaced with emoji while typing
    /// * `Flags::ReadOnly` - if set, the text can be read, selected, copied and scrolled, but not edited
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// use appcui::ui::markdown_composer::{Flags, MarkdownComposer};
    ///
    /// let mc = MarkdownComposer::new(layout!("x:1,y:1,w:40,h:10"), Flags::None);
    /// ```
    pub fn new(layout: Layout, flags: Flags) -> Self {
        let mut mc = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            text: String::new(),
            parser: Parser::new(),
            surface: Surface::new(1, 1),
            cursor_offset: 0,
            cursor_x: 0,
            cursor_y: 0,
            anchor: None,
            first_row: 0,
            rows: 1,
            lists: Vec::new(),
            popup: None,
            expanded: false,
            expanded_offset: 0,
            packed_origin: 0,
            packed_visible: 0,
            emoticons: flags.contains(Flags::Emoticons),
            read_only: flags.contains(Flags::ReadOnly),
            preferred_x: None,
        };
        mc.parser.set_show_markers(flags.contains(Flags::ShowMarkers));
        mc
    }

    /// Creates a new MarkdownComposer control with the specified text, layout and flags.
    /// Windows line endings (`\r\n`) and lone `\r` characters in the text are converted to `\n`.
    /// The flags can be a combination of the following values:
    /// * `Flags::ShowMarkers` - if set, the markdown markers (`**`, `_`, `` ` ``, `-`, `>`) remain visible instead of being hidden
    /// * `Flags::Emoticons` - if set, text emoticons and emoji names written between colons (for example `:B):` or `:smile:`) are replaced with emoji while typing
    /// * `Flags::ReadOnly` - if set, the text can be read, selected, copied and scrolled, but not edited
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// use appcui::ui::markdown_composer::{Flags, MarkdownComposer};
    ///
    /// let mc = MarkdownComposer::from("**Hello** _world_",
    ///                                 layout!("x:1,y:1,w:40,h:10"),
    ///                                 Flags::Emoticons);
    /// ```
    pub fn from(text: &str, layout: Layout, flags: Flags) -> Self {
        let mut mc = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            text: Self::normalize_newlines(text),
            parser: Parser::new(),
            surface: Surface::new(1, 1),
            cursor_offset: 0,
            cursor_x: 0,
            cursor_y: 0,
            anchor: None,
            first_row: 0,
            rows: 1,
            lists: Vec::new(),
            popup: None,
            expanded: false,
            expanded_offset: 0,
            packed_origin: 0,
            packed_visible: 0,
            emoticons: flags.contains(Flags::Emoticons),
            read_only: flags.contains(Flags::ReadOnly),
            preferred_x: None,
        };
        mc.parser.set_show_markers(flags.contains(Flags::ShowMarkers));
        mc
    }

    /// Returns the text of the MarkdownComposer control, including the markdown markers.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Sets the text of the MarkdownComposer control.
    ///
    /// The cursor is moved to the beginning of the text and the selection is cleared.
    /// Unlike an edit made by the user, this method does not raise the `on_text_changed` event.
    pub fn set_text(&mut self, text: &str) {
        self.popup_close_list();
        self.text = Self::normalize_newlines(text);
        self.cursor_offset = 0;
        self.anchor = None;
        self.first_row = 0;
        self.update_surface();
    }

    /// Returns **true** if the markdown markers are visible, **false** if they are hidden.
    pub fn show_markers(&self) -> bool {
        self.parser.show_markers()
    }

    /// Shows (**true**) or hides (**false**) the markdown markers.
    ///
    /// This is the runtime equivalent of the `Flags::ShowMarkers` flag.
    pub fn set_show_markers(&mut self, show_markers: bool) {
        if self.parser.show_markers() == show_markers {
            return;
        }
        self.parser.set_show_markers(show_markers);
        self.update_surface();
    }

    /// Returns the current flags of the MarkdownComposer control.
    ///
    /// The result reflects the changes made at runtime with [`MarkdownComposer::set_show_markers`],
    /// [`MarkdownComposer::set_emoticons_enabled`] and [`MarkdownComposer::set_read_only`].
    pub fn flags(&self) -> Flags {
        let mut flags = Flags::None;
        if self.parser.show_markers() {
            flags |= Flags::ShowMarkers;
        }
        if self.emoticons {
            flags |= Flags::Emoticons;
        }
        if self.read_only {
            flags |= Flags::ReadOnly;
        }
        flags
    }

    /// Returns **true** if the MarkdownComposer control is read-only, **false** otherwise.
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    /// Makes the MarkdownComposer control read-only (**true**) or editable (**false**).
    ///
    /// This is the runtime equivalent of the `Flags::ReadOnly` flag. Making the control
    /// read-only closes the suggestion list, if one is open.
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
        if read_only {
            self.popup_close_list();
        }
    }

    /// Returns **true** if text emoticons and emoji names written between colons (for example `:B):` or `:smile:`)
    /// are replaced with emoji while typing, **false** otherwise.
    pub fn emoticons_enabled(&self) -> bool {
        self.emoticons
    }

    /// Enables (**true**) or disables (**false**) the replacement of text emoticons with emoji.
    ///
    /// This is the runtime equivalent of the `Flags::Emoticons` flag. Emoticons already in
    /// the text are not changed; only the ones typed afterwards are affected.
    pub fn set_emoticons_enabled(&mut self, enabled: bool) {
        self.emoticons = enabled;
    }

    /// Returns the position of the cursor, as a byte offset in the text returned by
    /// [`MarkdownComposer::text`].
    pub fn cursor_offset(&self) -> u32 {
        self.cursor_offset
    }

    /// Moves the cursor to the specified byte offset and clears the selection.
    ///
    /// An offset past the end of the text moves the cursor to the end. An offset that falls
    /// inside a multi-byte character moves the cursor to the beginning of that character.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// use appcui::ui::markdown_composer::{Flags, MarkdownComposer};
    ///
    /// let mut mc = MarkdownComposer::from("Hello", layout!("x:1,y:1,w:40,h:10"), Flags::None);
    /// let end = mc.text().len() as u32;
    /// mc.set_cursor_offset(end);
    /// ```
    pub fn set_cursor_offset(&mut self, offset: u32) {
        let mut offset = (offset as usize).min(self.text.len());
        while !self.text.is_char_boundary(offset) {
            offset -= 1;
        }
        self.move_to(offset as u32, false);
    }

    /// Returns the selected text, including the markdown markers, or `None` if nothing is selected.
    pub fn selected_text(&self) -> Option<&str> {
        let (start, end) = self.selection()?;
        Some(&self.text[start as usize..end as usize])
    }

    /// Selects the whole text, the same as pressing Ctrl+A.
    pub fn select_all(&mut self) {
        self.anchor = Some(0);
        let offset = self.text.len() as u32;
        self.move_to(offset, true);
    }

    /// Clears the selection, leaving the cursor where it is.
    pub fn clear_selection(&mut self) {
        self.anchor = None;
    }

    fn normalize_newlines(text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\r' {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                out.push('\n');
            } else {
                out.push(ch);
            }
        }

        out
    }

    fn update_surface(&mut self) {
        self.preferred_x = None;
        let width = self.size().width.max(1);
        let height = self.size().height.max(1);

        if self.surface.size() != Size::new(width, height) {
            self.surface = Surface::new(width, height);
        }

        self.parser.parse(&self.text, width);
        self.rows = self.parser.rows(&self.text);

        let (cursor_x, cursor_y) = self.parser.get_position_from_offset(&self.text, self.cursor_offset);
        self.cursor_x = cursor_x;
        self.cursor_y = cursor_y;

        self.clamp_first_row();
        self.ensure_visible();
        self.redraw();
    }

    fn redraw(&mut self) {
        let theme = RuntimeManager::get().theme();
        let background = theme.editor.normal.background;

        self.surface
            .clear(Character::with_attributes(' ', Self::span_attr(SpanType::Normal, theme, background)));

        Self::paint_normal(&self.text, self.parser.spans(), &mut self.surface, theme, background, self.first_row);
    }

    fn notify_text_changed(&mut self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::MarkdownComposer(EventData {
                evtype: MarkdownComposerEventsType::OnTextChanged,
            }),
        });
    }

    fn clamp_first_row(&mut self) {
        let height = self.size().height.max(1);
        self.first_row = self.first_row.min(self.rows.saturating_sub(height));
    }

    fn ensure_visible(&mut self) {
        let height = self.size().height.max(1);

        if self.cursor_y < self.first_row {
            self.first_row = self.cursor_y;
        } else if self.cursor_y >= self.first_row + height {
            self.first_row = self.cursor_y - height + 1;
        }
    }

    fn scroll_to(&mut self, row: u32) {
        let height = self.size().height.max(1);
        let row = row.min(self.rows.saturating_sub(height));

        if row != self.first_row {
            self.first_row = row;
            self.redraw();
        }
    }

    /// Adds a suggestion list that opens when `trigger` is typed at the beginning of a word.
    ///
    /// While the list is open, the text typed after the trigger filters the items. Choosing
    /// an item (Enter, Tab, Right or a mouse click) inserts it in the text. If a list with the
    /// same trigger already exists, it is replaced.
    ///
    /// The `flags` parameter can be:
    /// * `ListFlags::None` - the trigger character is kept (`@Ana`)
    /// * `ListFlags::RemoveTrigger` - the trigger character is removed on insertion (`Ana`)
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// use appcui::ui::markdown_composer::{Flags, ListFlags, MarkdownComposer};
    ///
    /// let mut mc = MarkdownComposer::new(layout!("x:1,y:1,w:40,h:10"), Flags::None);
    /// mc.add_list('@', &["Ana", "Bogdan", "Cristina"], ListFlags::None);
    /// ```
    pub fn add_list(&mut self, trigger: char, items: &[&str], flags: ListFlags) {
        self.popup_close_list();
        let list = List::with_items(trigger, items, flags);
        match self.lists.iter().position(|item| item.trigger() == trigger) {
            Some(index) => self.lists[index] = list,
            None => self.lists.push(list),
        }
    }

    /// Adds a suggestion list in which every item has a separate value.
    ///
    /// The popup shows the value next to each name, and choosing an item inserts its value
    /// instead of its name. Everything else works as in [`MarkdownComposer::add_list`].
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// use appcui::ui::markdown_composer::{Flags, ListFlags, MarkdownComposer};
    ///
    /// let mut mc = MarkdownComposer::new(layout!("x:1,y:1,w:40,h:10"), Flags::None);
    /// mc.add_list_with_values('#', &[("bug", "🐛"), ("todo", "📝")], ListFlags::RemoveTrigger);
    /// ```
    pub fn add_list_with_values(&mut self, trigger: char, items: &[(&str, &str)], flags: ListFlags) {
        self.popup_close_list();
        let list = List::with_values(trigger, items, flags);
        match self.lists.iter().position(|item| item.trigger() == trigger) {
            Some(index) => self.lists[index] = list,
            None => self.lists.push(list),
        }
    }

    /// Adds the predefined emoji list, opened by `trigger`.
    ///
    /// The popup shows each emoji next to its name. Choosing an item replaces the trigger
    /// and the typed name with the emoji (for example `:pizza` becomes 🍕).
    pub fn add_emoji_list(&mut self, trigger: char) {
        self.add_list_with_values(trigger, EMOJIS, ListFlags::RemoveTrigger);
    }

    /// Same as [`MarkdownComposer::add_list`], but takes and returns the control, so that
    /// lists can be added in the same expression that creates it.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// use appcui::ui::markdown_composer::{Flags, ListFlags, MarkdownComposer};
    ///
    /// let mc = MarkdownComposer::new(layout!("x:1,y:1,w:40,h:10"), Flags::None)
    ///     .with_emoji_list(':')
    ///     .with_list('@', &["Ana", "Bogdan"], ListFlags::RemoveTrigger)
    ///     .with_list_values('#', &[("bug", "🐛")], ListFlags::None);
    /// ```
    pub fn with_list(mut self, trigger: char, items: &[&str], flags: ListFlags) -> Self {
        self.add_list(trigger, items, flags);
        self
    }

    /// Same as [`MarkdownComposer::add_list_with_values`], but takes and returns the control,
    /// so that it can be chained after the constructor (see [`MarkdownComposer::with_list`]).
    pub fn with_list_values(mut self, trigger: char, items: &[(&str, &str)], flags: ListFlags) -> Self {
        self.add_list_with_values(trigger, items, flags);
        self
    }

    /// Same as [`MarkdownComposer::add_emoji_list`], but takes and returns the control,
    /// so that it can be chained after the constructor (see [`MarkdownComposer::with_list`]).
    pub fn with_emoji_list(mut self, trigger: char) -> Self {
        self.add_emoji_list(trigger);
        self
    }

    /// Returns the suggestion list opened by `trigger`, or `None` if there is no such list.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// use appcui::ui::markdown_composer::{Flags, ListFlags, MarkdownComposer};
    ///
    /// let mc = MarkdownComposer::new(layout!("x:1,y:1,w:40,h:10"), Flags::None)
    ///     .with_list('@', &["Ana", "Bogdan"], ListFlags::None);
    /// assert_eq!(mc.list('@').unwrap().item(1), Some("Bogdan"));
    /// ```
    pub fn list(&self, trigger: char) -> Option<&List> {
        self.lists.iter().find(|list| list.trigger() == trigger)
    }

    /// Returns the suggestion list opened by `trigger` so that its items and flags can be
    /// changed while the application runs, or `None` if there is no such list.
    ///
    /// The popup is closed before the list is handed over, so that it is never shown with
    /// items that have been removed in the meantime.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// use appcui::ui::markdown_composer::{Flags, ListFlags, MarkdownComposer};
    ///
    /// let mut mc = MarkdownComposer::new(layout!("x:1,y:1,w:40,h:10"), Flags::None);
    /// mc.add_list('@', &["Ana"], ListFlags::None);
    /// if let Some(list) = mc.list_mut('@') {
    ///     list.add("Bogdan");
    ///     list.add_value("me", "@covita");
    /// }
    /// ```
    pub fn list_mut(&mut self, trigger: char) -> Option<&mut List> {
        self.popup_close_list();
        self.lists.iter_mut().find(|list| list.trigger() == trigger)
    }

    /// Removes the suggestion list opened by `trigger`.
    ///
    /// Returns **true** if the list existed and was removed, **false** otherwise.
    pub fn remove_list(&mut self, trigger: char) -> bool {
        self.popup_close_list();
        match self.lists.iter().position(|list| list.trigger() == trigger) {
            Some(index) => {
                self.lists.remove(index);
                true
            }
            None => false,
        }
    }

    fn popup_find_list(&self, trigger: char) -> Option<usize> {
        self.lists.iter().position(|list| list.trigger() == trigger)
    }

    fn popup_is_list_start(&self, offset: u32) -> bool {
        if offset == 0 {
            return true;
        }
        let bytes = self.text.as_bytes();
        let previous = Parser::prev_offset(&self.text, offset) as usize;
        if previous >= bytes.len() {
            return false;
        }
        let len = Parser::get_char_len(bytes[previous]);
        Parser::get_char(bytes, previous, len).is_whitespace()
    }

    fn popup_open_list(&mut self, trigger: char, start: u32) {
        let Some(list) = self.popup_find_list(trigger) else {
            return;
        };

        self.popup = Some(Popup {
            list,
            start,
            matches: Vec::new(),
            index: 0,
            first: 0,
        });
        self.popup_match_items();
    }

    fn popup_close_list(&mut self) {
        self.popup = None;
        self.pack();
    }

    fn popup_match_items(&mut self) {
        let Some((list, start)) = self.popup.as_ref().map(|popup| (popup.list, popup.start)) else {
            return;
        };

        let from = start as usize + self.lists[list].trigger().len_utf8();
        let to = self.cursor_offset as usize;

        if from > to || to > self.text.len() || self.text[from..to].contains('\n') {
            self.popup_close_list();
            return;
        }

        let filter = self.text[from..to].to_lowercase();
        let mut matches = Vec::new();

        for index in 0..self.lists[list].len() {
            let matched = match self.lists[list].item(index) {
                Some(item) => filter.is_empty() || item.to_lowercase().contains(&filter),
                None => false,
            };
            if matched {
                matches.push(index);
            }
        }

        if matches.is_empty() {
            self.popup_close_list();
            return;
        }

        if let Some(popup) = self.popup.as_mut() {
            popup.matches = matches;
            popup.index = 0;
            popup.first = 0;
        }

        self.popup_expand_list();
    }

    fn popup_move_selection(&mut self, delta: i32) {
        let Some(popup) = self.popup.as_mut() else {
            return;
        };

        let count = popup.matches.len() as i32;
        if count == 0 {
            return;
        }

        popup.index = (popup.index as i32).saturating_add(delta).clamp(0, count - 1) as u32;
        popup.scroll_to_index();
    }

    fn popup_insert_item(&mut self) {
        let Some(popup) = self.popup.as_ref() else {
            return;
        };

        let start = popup.start as usize;

        let Some(&item) = popup.matches.get(popup.index as usize) else {
            self.popup_close_list();
            return;
        };
        let list = &self.lists[popup.list];
        let Some(value) = list.value(item) else {
            self.popup_close_list();
            return;
        };

        let mut replacement = String::new();
        if !list.flags().contains(ListFlags::RemoveTrigger) {
            replacement.push(list.trigger());
        }
        replacement.push_str(value);

        self.popup_close_list();

        let end = (self.cursor_offset as usize).min(self.text.len()).max(start);
        self.text.replace_range(start..end, &replacement);
        self.cursor_offset = (start + replacement.len()) as u32;
        self.anchor = None;
        self.update_surface();
        self.notify_text_changed();
    }

    fn popup_get_trigger_row(&self) -> i32 {
        let Some(popup) = self.popup.as_ref() else {
            return 0;
        };
        let (_, trigger_y) = self.parser.get_position_from_offset(&self.text, popup.start);
        trigger_y as i32 - self.first_row as i32
    }

    fn text_width(text: &str) -> u32 {
        let bytes = text.as_bytes();
        let mut width = 0;
        let mut i = 0;

        while i < bytes.len() {
            let len = Parser::get_char_len(bytes[i]);
            width += Parser::get_char_width(Parser::get_char(bytes, i, len)) as u32;
            i += len;
        }

        width
    }

    fn write_text_at(surface: &mut Surface, x: i32, y: i32, limit: i32, text: &str, attr: CharAttribute) -> i32 {
        let bytes = text.as_bytes();
        let mut pos = x;
        let mut i = 0;

        while i < bytes.len() && pos < limit {
            let len = Parser::get_char_len(bytes[i]);
            let ch = Parser::get_char(bytes, i, len);
            surface.write_char(pos, y, Character::with_attributes(ch, attr));
            pos += Parser::get_char_width(ch);
            i += len;
        }

        pos
    }

    fn popup_value_column(list: &List, matches: &[u32]) -> u32 {
        let mut width = 0;
        for &item in matches {
            let (Some(name), Some(value)) = (list.item(item), list.value(item)) else {
                continue;
            };
            if value != name {
                width = width.max(Self::text_width(value));
            }
        }

        if width == 0 {
            0
        } else {
            width + 1
        }
    }

    fn popup_get_list_size(&self) -> Option<(u32, u32)> {
        let popup = self.popup.as_ref()?;
        let list = self.lists.get(popup.list)?;

        let mut name_width = 0;
        for &item in &popup.matches {
            if let Some(name) = list.item(item) {
                name_width = name_width.max(Self::text_width(name));
            }
        }

        let text_width = Self::popup_value_column(list, &popup.matches) + name_width;
        let width = (text_width + 4).clamp(POPUP_MIN_WIDTH, POPUP_MAX_WIDTH);
        let height = (popup.matches.len() as u32).min(POPUP_ROWS) + 2;
        Some((width, height))
    }

    fn popup_fit(row: i32, bottom: i32, height: u32) -> Option<(i32, u32)> {
        let below = row + 1;
        let space_below = if below >= 0 { bottom - below } else { 0 };
        let space_above = row.min(bottom);

        if height as i32 <= space_below {
            return Some((below, height));
        }
        if height as i32 <= space_above {
            return Some((row - height as i32, height));
        }

        if space_below >= space_above && space_below >= POPUP_MIN_HEIGHT as i32 {
            return Some((below, space_below as u32));
        }
        if space_above >= POPUP_MIN_HEIGHT as i32 {
            return Some((row - space_above, space_above as u32));
        }

        let height = height.min(bottom.max(0) as u32);
        if height < POPUP_MIN_HEIGHT {
            return None;
        }
        let y = below.clamp(0, bottom - height as i32);
        Some((y, height))
    }

    fn popup_get_list_rect(&self) -> Option<Rect> {
        let popup = self.popup.as_ref()?;
        let (width, height) = self.popup_get_list_size()?;

        let size = self.size();
        let (trigger_x, _) = self.parser.get_position_from_offset(&self.text, popup.start);
        let x = (trigger_x as i32).clamp(0, (size.width as i32 - width as i32).max(0));
        let row = self.popup_get_trigger_row() + self.expanded_offset;

        let bottom = if self.expanded {
            (self.expanded_size().height as i32).max(self.expanded_offset + self.packed_visible)
        } else {
            self.visible_height()
        };

        let (y, height) = Self::popup_fit(row, bottom, height)?;
        Some(Rect::with_size(x, y, width as u16, height as u16))
    }

    fn visible_height(&self) -> i32 {
        let height = self.size().height as i32;
        if self.expanded {
            return self.packed_visible.clamp(0, height);
        }
        (self.screen_clip.bottom - self.screen_origin.y + 1).clamp(0, height)
    }

    fn popup_fits_inside(&self) -> bool {
        let Some((_, height)) = self.popup_get_list_size() else {
            return true;
        };

        let row = self.popup_get_trigger_row();
        let below = row + 1;

        let fits_below = below >= 0 && below + height as i32 <= self.visible_height();
        let fits_above = row - height as i32 >= 0;

        fits_below || fits_above
    }

    fn popup_expand_list(&mut self) {
        let Some((_, height)) = self.popup_get_list_size() else {
            self.pack();
            return;
        };

        let size = self.size();
        let row = self.popup_get_trigger_row();
        let below = row + 1;

        if self.popup_fits_inside() {
            self.pack();
            return;
        }

        if self.expanded {
            return;
        }

        let visible = self.visible_height();
        let height = height as i32;
        let smallest = POPUP_MIN_HEIGHT as i32;
        let origin = self.screen_origin.y;
        let terminal = RuntimeManager::get().terminal_size().height as i32;
        let space_below = terminal - (2 + origin);
        let space_above = origin - 1;

        let least_below = (below + smallest).max(visible);
        let full_below = (below + height).max(visible);
        let least_above = (smallest - row + 1).max(space_below + 1).max(1);
        let full_above = (height - row + 1).max(least_above);

        let (least, full) = if full_below <= space_below {
            (least_below, full_below)
        } else if full_above <= space_above {
            (least_above, full_above)
        } else if least_below <= space_below {
            (least_below, full_below)
        } else {
            (least_above, full_above)
        };

        let minimum = Size::new(size.width, least as u32);
        let prefered = Size::new(size.width, full as u32);
        self.packed_origin = origin;
        self.packed_visible = visible;
        self.expand(minimum, prefered);
    }

    fn popup_get_first_visible(&self, rows: u32) -> u32 {
        let Some(popup) = self.popup.as_ref() else {
            return 0;
        };
        if rows > 0 && popup.index >= popup.first + rows {
            popup.index - rows + 1
        } else {
            popup.first
        }
    }

    fn popup_contains(&self, x: i32, y: i32) -> bool {
        match self.popup_get_list_rect() {
            Some(frame) => frame.contains(Point::new(x, y)),
            None => false,
        }
    }

    fn popup_get_item_at(&self, x: i32, y: i32) -> Option<u32> {
        let popup = self.popup.as_ref()?;
        let frame = self.popup_get_list_rect()?;

        if !frame.contains(Point::new(x, y)) {
            return None;
        }

        let rows = frame.height() - 2;
        let row = y - frame.top() - 1;
        if row < 0 || row >= rows as i32 {
            return None;
        }

        let index = self.popup_get_first_visible(rows) + row as u32;
        if (index as usize) < popup.matches.len() {
            Some(index)
        } else {
            None
        }
    }

    fn popup_paint_list(&self, surface: &mut Surface, theme: &Theme) {
        let Some(popup) = self.popup.as_ref() else {
            return;
        };
        let Some(frame) = self.popup_get_list_rect() else {
            return;
        };
        let Some(list) = self.lists.get(popup.list) else {
            return;
        };

        let normal = theme.menu.text.normal;
        let selected = theme.menu.text.pressed_or_selected;
        let x = frame.left();
        let y = frame.top();

        surface.fill_rect(frame, Character::with_attributes(' ', normal));
        surface.draw_rect(frame, LineType::Single, normal);

        let inner = frame.width() - 2;
        let rows = frame.height() - 2;

        let first = self.popup_get_first_visible(rows);
        let visible = (popup.matches.len() as u32 - first).min(rows);

        let limit = x + inner as i32;
        let column = Self::popup_value_column(list, &popup.matches) as i32;

        for row in 0..visible {
            let index = first + row;
            let Some(&item) = popup.matches.get(index as usize) else {
                continue;
            };
            let Some(name) = list.item(item) else {
                continue;
            };

            let line = y + 1 + row as i32;
            let attr = if index == popup.index {
                surface.fill_horizontal_line_with_size(x + 1, line, inner, Character::with_attributes(' ', selected));
                selected
            } else {
                normal
            };

            if let Some(value) = list.value(item) {
                if value != name {
                    Self::write_text_at(surface, x + 2, line, limit, value, attr);
                }
            }
            Self::write_text_at(surface, x + 2 + column, line, limit, name, attr);
        }
    }

    fn span_attr(span_type: SpanType, theme: &Theme, background: Color) -> CharAttribute {
        let markdown = &theme.markdown;
        let mut attr = markdown.text;

        if span_type.contains(SpanType::Bold) {
            attr.foreground = markdown.bold.foreground;
            attr.flags |= markdown.bold.flags;
        }
        if span_type.contains(SpanType::Italic) {
            attr.foreground = markdown.italic.foreground;
            attr.flags |= markdown.italic.flags;
        }
        if span_type.contains(SpanType::Bullet) {
            attr.foreground = markdown.unordered_list.foreground;
            attr.flags |= markdown.unordered_list.flags;
        }
        if span_type.contains(SpanType::Quote) {
            attr.foreground = theme.text.inactive.foreground;
            attr.flags |= theme.text.inactive.flags;
        }
        if span_type.contains_one(SpanType::Link | SpanType::Email) {
            attr.foreground = markdown.link.foreground;
            attr.flags |= markdown.link.flags;
        }
        if span_type.contains(SpanType::Code) {
            attr.foreground = markdown.code.foreground;
            attr.background = markdown.code.background;
            attr.flags |= markdown.code.flags;
        } else if span_type.contains(SpanType::CodeBlock) {
            attr.foreground = markdown.code_block.foreground;
            attr.background = markdown.code_block.background;
            attr.flags |= markdown.code_block.flags;
        }

        if attr.background == Color::Transparent {
            attr.background = background;
        }
        attr
    }

    fn code_block_attr(theme: &Theme, background: Color) -> CharAttribute {
        Self::span_attr(SpanType::CodeBlock, theme, background)
    }

    fn quote_bar_attr(theme: &Theme, background: Color) -> CharAttribute {
        Self::span_attr(SpanType::Quote, theme, background)
    }

    fn span_width(bytes: &[u8], span: &Span) -> u32 {
        let mut width = 0;
        let mut visible = 0;
        let mut i = span.start as usize;

        while i < span.end as usize {
            let len = Parser::get_char_len(bytes[i]);
            let ch = Parser::get_char(bytes, i, len);

            if ch != '\n' {
                width += Parser::get_char_width(ch) as u32;
                if !ch.is_whitespace() {
                    visible = width;
                }
            }

            i += len;
        }

        visible
    }

    fn fence_offsets(bytes: &[u8]) -> Vec<usize> {
        let mut offsets = Vec::new();
        let mut index = 0;
        while index < bytes.len() {
            if (index == 0 || bytes[index - 1] == b'\n') && bytes[index..].starts_with(FENCE.as_bytes()) {
                offsets.push(index);
            }
            index += 1;
        }
        offsets
    }

    fn fence_group(fences: &[usize], offset: usize) -> usize {
        fences.partition_point(|fence| *fence < offset)
    }

    fn paint_code_blocks(bytes: &[u8], spans: &[Span], surface: &mut Surface, theme: &Theme, background: Color, first_row: u32) {
        let height = surface.size().height as i32;
        let attr = Self::code_block_attr(theme, background);
        let fences = Self::fence_offsets(bytes);

        let mut index = 0;
        while index < spans.len() {
            if !spans[index].span_type.contains(SpanType::CodeBlock) {
                index += 1;
                continue;
            }

            let start = index;
            let group = Self::fence_group(&fences, spans[index].start as usize);
            while index < spans.len()
                && spans[index].span_type.contains(SpanType::CodeBlock)
                && Self::fence_group(&fences, spans[index].start as usize) == group
            {
                index += 1;
            }

            let mut top = u32::MAX;
            let mut bottom = 0;
            let mut left = u32::MAX;
            let mut right = 0;

            for span in &spans[start..index] {
                top = top.min(span.y_pos);
                bottom = bottom.max(span.y_pos);
                left = left.min(span.x_pos);
                right = right.max(span.x_pos + Self::span_width(bytes, span));
            }

            if top == 0 || left == 0 || right <= left {
                continue;
            }

            let busy = spans.iter().any(|span| span.y_pos == top - 1 || span.y_pos == bottom + 1);
            if busy {
                continue;
            }

            let outer_top = top as i32 - first_row as i32 - 1;
            let outer_bottom = bottom as i32 - first_row as i32 + 1;
            if outer_bottom < 0 || outer_top >= height {
                continue;
            }

            surface.draw_rect(Rect::new(left as i32 - 1, outer_top, right as i32, outer_bottom), LineType::Single, attr);
        }
    }

    fn paint_normal(text: &str, spans: &[Span], surface: &mut Surface, theme: &Theme, background: Color, first_row: u32) {
        let bytes = text.as_bytes();
        let size = surface.size();
        let height = size.height as i32;

        for span in spans {
            if !span.span_type.contains(SpanType::Quote) {
                continue;
            }

            let y = span.y_pos as i32 - first_row as i32;
            if y < 0 || y >= height {
                continue;
            }

            surface.write_char(0, y, Character::with_attributes(QUOTE_BAR, Self::quote_bar_attr(theme, background)));
        }

        for span in spans {
            if span.span_type.contains(SpanType::QuoteMark) {
                continue;
            }

            let y = span.y_pos as i32 - first_row as i32;
            if y < 0 || y >= height {
                continue;
            }

            let attr = Self::span_attr(span.span_type, theme, background);
            let bullet = span.span_type.contains(SpanType::Bullet);

            let mut x = span.x_pos as i32;
            let mut i = span.start as usize;

            while i < span.end as usize {
                let len = Parser::get_char_len(bytes[i]);
                let source = Parser::get_char(bytes, i, len);
                let ch = if bullet { BULLET.into() } else { source };

                surface.write_char(x, y, Character::with_attributes(ch, attr));

                x += Parser::get_char_width(source);
                i += len;
            }
        }

        Self::paint_code_blocks(bytes, spans, surface, theme, background, first_row);
    }

    fn paint_selection(&self, surface: &mut Surface, theme: &Theme) {
        let Some((start, end)) = self.selection() else {
            return;
        };

        let attr = theme.editor.pressed_or_selected;
        let bytes = self.text.as_bytes();
        let height = surface.size().height as i32;

        for span in self.parser.spans() {
            if span.end <= start || span.start >= end {
                continue;
            }

            let y = span.y_pos as i32 - self.first_row as i32 + self.expanded_offset;
            if y < 0 || y >= height {
                continue;
            }

            let bullet = span.span_type.contains(SpanType::Bullet);
            let quote_mark = span.span_type.contains(SpanType::QuoteMark);
            let mut x = span.x_pos as i32;
            let mut i = span.start as usize;

            while i < span.end as usize {
                let len = Parser::get_char_len(bytes[i]);
                let source = Parser::get_char(bytes, i, len);

                if i >= start as usize && i < end as usize {
                    let ch = if bullet {
                        BULLET.into()
                    } else if quote_mark {
                        QUOTE_BAR.into()
                    } else {
                        source
                    };
                    surface.write_char(x, y, Character::with_attributes(ch, attr));
                }

                x += Parser::get_char_width(source);
                i += len;
            }
        }
    }

    fn selection(&self) -> Option<(u32, u32)> {
        let limit = self.text.len() as u32;
        let anchor = self.anchor?.min(limit);
        let cursor_offset = self.cursor_offset.min(limit);

        if anchor == cursor_offset {
            return None;
        }
        Some((anchor.min(cursor_offset), anchor.max(cursor_offset)))
    }

    fn remove_selection(&mut self) -> bool {
        let selection = self.selection();
        self.anchor = None;
        let Some((start, end)) = selection else {
            return false;
        };

        if self.parser.show_markers() {
            self.text.replace_range(start as usize..end as usize, "");
            self.cursor_offset = start;
            return true;
        }

        let (left, right, replacement, cursor) = self.selection_removal(start as usize, end as usize);
        self.text.replace_range(left..right, &replacement);
        self.cursor_offset = cursor as u32;
        true
    }

    fn is_hidden(&self, offset: usize) -> bool {
        !self
            .parser
            .spans()
            .iter()
            .any(|span| span.start as usize <= offset && offset < span.end as usize)
    }

    fn hidden_marker_before(&self, offset: usize) -> Option<&'static str> {
        INLINE_MARKERS.into_iter().find(|marker| {
            offset >= marker.len()
                && self.text[..offset].ends_with(marker)
                && (offset - marker.len()..offset).all(|position| self.is_hidden(position))
        })
    }

    fn hidden_marker_after(&self, offset: usize) -> Option<&'static str> {
        INLINE_MARKERS
            .into_iter()
            .find(|marker| self.text[offset..].starts_with(marker) && (offset..offset + marker.len()).all(|position| self.is_hidden(position)))
    }

    fn selection_removal(&self, start: usize, end: usize) -> (usize, usize, String, usize) {
        let mut left = start;
        let mut left_markers = Vec::new();
        while let Some(marker) = self.hidden_marker_before(left) {
            left -= marker.len();
            left_markers.push(marker.to_string());
        }
        left_markers.reverse();

        let mut middle = Vec::new();
        let bytes = self.text.as_bytes();
        let mut index = start;
        while index < end {
            if !self.is_hidden(index) || bytes[index] == b'\n' {
                index = Parser::next_offset(&self.text, index as u32) as usize;
                continue;
            }
            let line_start = index == 0 || bytes[index - 1] == b'\n';
            let ticks = bytes[index..end].iter().take_while(|byte| **byte == b'`').count();
            if line_start && ticks >= 3 {
                let mut fence_end = index + ticks;
                if fence_end < end && bytes[fence_end] == b'\n' {
                    fence_end += 1;
                }
                middle.push((self.text[index..fence_end].to_string(), FENCE.to_string()));
                index = fence_end;
            } else if let Some(marker) = self.hidden_marker_after(index) {
                middle.push((marker.to_string(), marker.to_string()));
                index += marker.len();
            } else {
                let next = Parser::next_offset(&self.text, index as u32) as usize;
                middle.push((self.text[index..next].to_string(), String::new()));
                index = next;
            }
        }

        let mut right = end;
        let mut right_markers = Vec::new();
        while let Some(marker) = self.hidden_marker_after(right) {
            right += marker.len();
            right_markers.push(marker.to_string());
        }

        let mut kept: Vec<(String, String, bool)> = Vec::new();
        let tokens = left_markers
            .into_iter()
            .map(|marker| (marker.clone(), marker, true))
            .chain(middle.into_iter().map(|(token, key)| (token, key, false)))
            .chain(right_markers.into_iter().map(|marker| (marker.clone(), marker, false)));
        for (token, key, from_left) in tokens {
            match kept.last() {
                Some((_, previous, _)) if !key.is_empty() && *previous == key => {
                    kept.pop();
                }
                _ => kept.push((token, key, from_left)),
            }
        }

        let mut replacement = String::new();
        let mut cursor = left;
        for (token, _, from_left) in kept {
            let line_start = replacement.ends_with('\n') || (replacement.is_empty() && (left == 0 || bytes[left - 1] == b'\n'));
            if token.starts_with(FENCE) && !line_start {
                replacement.push('\n');
            }
            replacement.push_str(&token);
            if from_left {
                cursor = left + replacement.len();
            }
        }
        (left, right, replacement, cursor)
    }

    fn word_at(&self, offset: u32) -> (u32, u32) {
        let bytes = self.text.as_bytes();
        let offset = (offset as usize).min(bytes.len());

        let mut start = offset;
        while start > 0 && !bytes[start - 1].is_ascii_whitespace() {
            start = Parser::prev_offset(&self.text, start as u32) as usize;
        }

        let mut end = offset;
        while end < bytes.len() && !bytes[end].is_ascii_whitespace() {
            end = Parser::next_offset(&self.text, end as u32) as usize;
        }

        (start as u32, end as u32)
    }

    fn next_word(&self, offset: u32) -> u32 {
        let bytes = self.text.as_bytes();
        let len = bytes.len();
        let mut i = (offset as usize).min(len);

        if i >= len {
            return len as u32;
        }

        if bytes[i].is_ascii_whitespace() {
            while i < len && bytes[i].is_ascii_whitespace() {
                i = Parser::next_offset(&self.text, i as u32) as usize;
            }
            return i as u32;
        }

        while i < len && !bytes[i].is_ascii_whitespace() {
            i = Parser::next_offset(&self.text, i as u32) as usize;
        }
        while i < len && bytes[i].is_ascii_whitespace() && bytes[i] != b'\n' {
            i = Parser::next_offset(&self.text, i as u32) as usize;
        }

        i as u32
    }

    fn prev_word(&self, offset: u32) -> u32 {
        let bytes = self.text.as_bytes();
        let mut i = (offset as usize).min(bytes.len());

        while i > 0 {
            let previous = Parser::prev_offset(&self.text, i as u32) as usize;
            if !bytes[previous].is_ascii_whitespace() {
                break;
            }
            i = previous;
        }
        while i > 0 {
            let previous = Parser::prev_offset(&self.text, i as u32) as usize;
            if bytes[previous].is_ascii_whitespace() {
                break;
            }
            i = previous;
        }

        i as u32
    }

    fn emoticon(code: &str) -> Option<&'static str> {
        let find = |key: &str| EMOTICONS.iter().find(|(name, _)| *name == key).map(|(_, value)| *value);
        let named = |key: &str| EMOJIS.iter().find(|(name, _)| name.eq_ignore_ascii_case(key)).map(|(_, value)| *value);
        find(code).or_else(|| code.strip_prefix(':').and_then(find)).or_else(|| named(code))
    }

    fn is_inside_code(&self, offset: u32) -> bool {
        let before = &self.text[..offset as usize];
        if before.matches("```").count() % 2 == 1 {
            return true;
        }
        let line = &before[before.rfind('\n').map_or(0, |index| index + 1)..];
        let ticks = line.bytes().filter(|byte| *byte == b'`').count() - 3 * line.matches("```").count();
        ticks % 2 == 1
    }

    fn replace_emoticon(&mut self) -> bool {
        let end = self.cursor_offset as usize;
        let bytes = self.text.as_bytes();
        if end < 3 || bytes[end - 1] != b':' {
            return false;
        }

        let codes = EMOTICONS.iter().chain(EMOJIS.iter());
        let longest = codes.map(|(code, _)| code.len()).max().unwrap_or(0) + 1;
        for len in (1..=longest).rev() {
            if len + 2 > end {
                continue;
            }
            let start = end - len - 2;
            if self.text.as_bytes()[start] != b':' {
                continue;
            }
            let Some(value) = Self::emoticon(&self.text[start + 1..end - 1]) else {
                continue;
            };
            if !self.popup_is_list_start(start as u32) || self.is_inside_code(start as u32) {
                continue;
            }

            self.popup_close_list();
            self.text.replace_range(start..end, value);
            self.cursor_offset = (start + value.len()) as u32;
            self.anchor = None;
            self.update_surface();
            self.notify_text_changed();
            return true;
        }
        false
    }

    fn insert(&mut self, character: char) {
        self.remove_selection();
        self.text.insert(self.cursor_offset as usize, character);
        self.cursor_offset += character.len_utf8() as u32;
        self.update_surface();
        self.notify_text_changed();
        self.popup_match_items();
    }

    fn insert_text(&mut self, added: &str) {
        self.popup_close_list();
        self.remove_selection();
        self.text.insert_str(self.cursor_offset as usize, added);
        self.cursor_offset += added.len() as u32;
        self.update_surface();
        self.notify_text_changed();
    }

    fn delete_previous(&mut self) {
        if self.remove_selection() {
            self.update_surface();
            self.notify_text_changed();
            return;
        }
        let previous = Parser::prev_offset(&self.text, self.cursor_offset);
        if previous == self.cursor_offset {
            return;
        }
        self.text.replace_range(previous as usize..self.cursor_offset as usize, "");
        self.cursor_offset = previous;
        self.update_surface();
        self.notify_text_changed();
        self.popup_match_items();
    }

    fn delete_current(&mut self) {
        if self.remove_selection() {
            self.update_surface();
            self.notify_text_changed();
            return;
        }
        let next = Parser::next_offset(&self.text, self.cursor_offset);
        if next == self.cursor_offset {
            return;
        }
        self.text.replace_range(self.cursor_offset as usize..next as usize, "");
        self.update_surface();
        self.notify_text_changed();
    }

    fn delete_previous_word(&mut self) {
        if self.remove_selection() {
            self.update_surface();
            self.notify_text_changed();
            return;
        }
        let start = self.prev_word(self.cursor_offset);
        if start == self.cursor_offset {
            return;
        }
        self.text.replace_range(start as usize..self.cursor_offset as usize, "");
        self.cursor_offset = start;
        self.update_surface();
        self.notify_text_changed();
        self.popup_match_items();
    }

    fn delete_next_word(&mut self) {
        if self.remove_selection() {
            self.update_surface();
            self.notify_text_changed();
            return;
        }
        let end = self.next_word(self.cursor_offset);
        if end == self.cursor_offset {
            return;
        }
        self.text.replace_range(self.cursor_offset as usize..end as usize, "");
        self.update_surface();
        self.notify_text_changed();
    }

    fn copy_selection(&self) -> bool {
        let Some((start, end)) = self.selection() else {
            return false;
        };
        Clipboard::set_text(&self.selection_markdown(start, end));
        true
    }

    fn inline_markers(span_type: SpanType) -> Vec<&'static str> {
        if span_type.contains(SpanType::Code) {
            return vec!["`"];
        }
        let mut markers = Vec::new();
        if span_type.contains(SpanType::Bold) {
            markers.push("**");
        }
        if span_type.contains(SpanType::Italic) {
            markers.push("_");
        }
        markers
    }

    fn switch_markers(out: &mut String, open: &mut Vec<&'static str>, wanted: &[&'static str]) {
        let keep = open.iter().zip(wanted).take_while(|(current, next)| current == next).count();
        while open.len() > keep {
            if let Some(marker) = open.pop() {
                out.push_str(marker);
            }
        }
        for marker in &wanted[keep..] {
            out.push_str(marker);
            open.push(marker);
        }
    }

    fn copy_gap(gap: &str, out: &mut String, open: &mut Vec<&'static str>) -> bool {
        if gap.contains('\n') {
            Self::switch_markers(out, open, &[]);
        }
        let mut fence = false;
        for part in gap.split_inclusive('\n') {
            let body = part.trim_end_matches('\n');
            if body.len() >= 3 && body.bytes().all(|byte| byte == b'`') {
                out.push_str(part);
                fence = true;
            } else if part.ends_with('\n') {
                out.push('\n');
            }
        }
        fence
    }

    fn selection_markdown(&self, start: u32, end: u32) -> String {
        if self.parser.show_markers() {
            return self.text[start as usize..end as usize].to_string();
        }

        let mut out = String::new();
        let mut open = Vec::new();
        let mut position = start;
        let mut fence_seen = false;
        let mut in_block = false;

        for span in self.parser.spans() {
            let from = span.start.max(start);
            let to = span.end.min(end);
            if from >= to {
                continue;
            }
            if from > position {
                fence_seen |= Self::copy_gap(&self.text[position as usize..from as usize], &mut out, &mut open);
            }
            let block = span.span_type.contains(SpanType::CodeBlock);
            if block && !in_block && !fence_seen {
                out.push_str("```\n");
            }
            in_block = block;
            fence_seen = false;
            Self::switch_markers(&mut out, &mut open, &Self::inline_markers(span.span_type));
            out.push_str(&self.text[from as usize..to as usize]);
            position = to;
        }

        let closing_fence = end > position && Self::copy_gap(&self.text[position as usize..end as usize], &mut out, &mut open);
        Self::switch_markers(&mut out, &mut open, &[]);
        if in_block && !closing_fence {
            out.push_str("\n```");
        }
        out
    }

    fn cut_selection(&mut self) {
        if self.copy_selection() && self.remove_selection() {
            self.update_surface();
            self.notify_text_changed();
        }
    }

    fn paste(&mut self) {
        let Some(pasted) = Clipboard::text() else {
            return;
        };
        if pasted.is_empty() {
            return;
        }
        if pasted.contains('\r') {
            let normalized = Self::normalize_newlines(&pasted);
            self.insert_text(&normalized);
        } else {
            self.insert_text(&pasted);
        }
    }

    fn move_vertical(&mut self, row: u32, select: bool) {
        let column = self.preferred_x.unwrap_or(self.cursor_x);
        let offset = self.parser.get_offset_from_position(&self.text, column, row);
        self.move_to(offset, select);
        self.preferred_x = Some(column);
    }

    fn move_to(&mut self, offset: u32, select: bool) {
        self.preferred_x = None;
        self.popup_close_list();
        if select {
            if self.anchor.is_none() {
                self.anchor = Some(self.cursor_offset);
            }
        } else {
            self.anchor = None;
        }

        if offset == self.cursor_offset {
            return;
        }
        self.cursor_offset = offset;
        let (cursor_x, cursor_y) = self.parser.get_position_from_offset(&self.text, self.cursor_offset);
        self.cursor_x = cursor_x;
        self.cursor_y = cursor_y;

        let first_row = self.first_row;
        self.ensure_visible();
        if self.first_row != first_row {
            self.redraw();
        }
    }

    fn offset_at_screen(&self, x: i32, y: i32) -> u32 {
        let height = self.size().height.max(1) as i32;
        let y = (y - self.expanded_offset).clamp(0, height - 1) as u32 + self.first_row;
        self.parser.get_offset_from_position(&self.text, x.max(0) as u32, y)
    }

    fn scroll_towards(&mut self, y: i32) {
        let height = self.size().height.max(1) as i32;
        let y = y - self.expanded_offset;

        if y < 0 {
            let row = self.first_row.saturating_sub((-y) as u32);
            self.scroll_to(row);
        } else if y >= height {
            let row = self.first_row + (y - height + 1) as u32;
            self.scroll_to(row);
        }
    }
}

impl OnPaint for MarkdownComposer {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let text_top = self.screen_origin.y + self.expanded_offset;
        let text_bottom = text_top + self.packed_visible - 1;

        if self.expanded {
            surface.set_base_clip(self.screen_clip.left, text_top, self.screen_clip.right, text_bottom);
            surface.reset_clip();
        }

        surface.draw_surface(0, self.expanded_offset, &self.surface);
        self.paint_selection(surface, theme);

        if self.expanded {
            surface.set_base_clip(
                self.screen_clip.left,
                self.screen_clip.top.min(text_top),
                self.screen_clip.right,
                self.screen_clip.bottom.max(text_bottom),
            );
            surface.reset_clip();
        }

        self.popup_paint_list(surface, theme);

        if self.has_focus() {
            surface.set_cursor(self.cursor_x as i32, self.cursor_y as i32 - self.first_row as i32 + self.expanded_offset);
        } else {
            surface.hide_cursor();
        }

        if self.expanded {
            surface.set_base_clip(
                self.screen_clip.left,
                self.screen_clip.top,
                self.screen_clip.right,
                self.screen_clip.bottom,
            );
            surface.reset_clip();
        }
    }
}

impl OnResize for MarkdownComposer {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        self.popup_close_list();
        self.update_surface();
    }
}

impl OnMouseEvent for MarkdownComposer {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Pressed(data) => {
                if self.popup_contains(data.x, data.y) {
                    if let Some(index) = self.popup_get_item_at(data.x, data.y) {
                        if let Some(popup) = self.popup.as_mut() {
                            popup.index = index;
                        }
                        self.popup_insert_item();
                    }
                    return EventProcessStatus::Processed;
                }
                let offset = self.offset_at_screen(data.x, data.y);
                self.move_to(offset, false);
                self.anchor = Some(offset);
                EventProcessStatus::Processed
            }
            MouseEvent::DoubleClick(data) => {
                if self.popup_contains(data.x, data.y) {
                    return EventProcessStatus::Processed;
                }
                let offset = self.offset_at_screen(data.x, data.y);
                let (start, end) = self.word_at(offset);
                self.anchor = Some(start);
                self.move_to(end, true);
                EventProcessStatus::Processed
            }
            MouseEvent::Drag(data) => {
                if self.popup_contains(data.x, data.y) {
                    return EventProcessStatus::Processed;
                }
                self.scroll_towards(data.y);
                let offset = self.offset_at_screen(data.x, data.y);
                self.move_to(offset, true);
                EventProcessStatus::Processed
            }
            MouseEvent::Over(point) => {
                let Some(index) = self.popup_get_item_at(point.x, point.y) else {
                    return EventProcessStatus::Ignored;
                };
                let Some(popup) = self.popup.as_mut() else {
                    return EventProcessStatus::Ignored;
                };
                if popup.index == index {
                    return EventProcessStatus::Ignored;
                }
                popup.index = index;
                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(direction) => {
                if self.popup.is_some() {
                    match direction {
                        MouseWheelDirection::Up => self.popup_move_selection(-1),
                        MouseWheelDirection::Down => self.popup_move_selection(1),
                        _ => return EventProcessStatus::Ignored,
                    }
                    return EventProcessStatus::Processed;
                }
                match direction {
                    MouseWheelDirection::Up => {
                        let row = self.first_row.saturating_sub(WHEEL_ROWS);
                        self.scroll_to(row);
                    }
                    MouseWheelDirection::Down => {
                        let row = self.first_row + WHEEL_ROWS;
                        self.scroll_to(row);
                    }
                    _ => return EventProcessStatus::Ignored,
                }
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnKeyPressed for MarkdownComposer {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        let select = key.modifier.contains(KeyModifier::Shift);
        let height = self.size().height.max(1);

        if self.popup.is_some() {
            match key.value() {
                key!("Escape") => {
                    self.popup_close_list();
                    return EventProcessStatus::Processed;
                }
                key!("Up") => {
                    self.popup_move_selection(-1);
                    return EventProcessStatus::Processed;
                }
                key!("Down") => {
                    self.popup_move_selection(1);
                    return EventProcessStatus::Processed;
                }
                key!("PageUp") => {
                    self.popup_move_selection(-(POPUP_ROWS as i32));
                    return EventProcessStatus::Processed;
                }
                key!("PageDown") => {
                    self.popup_move_selection(POPUP_ROWS as i32);
                    return EventProcessStatus::Processed;
                }
                key!("Enter") | key!("Tab") => {
                    self.popup_insert_item();
                    return EventProcessStatus::Processed;
                }
                _ => {}
            }
        }

        match key.value() {
            key!("Left") | key!("Shift+Left") => {
                let offset = match self.selection() {
                    Some((start, _)) if !select => start,
                    _ => self.parser.prev_visible_offset(&self.text, self.cursor_offset),
                };
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("Right") | key!("Shift+Right") => {
                let offset = match self.selection() {
                    Some((_, end)) if !select => end,
                    _ => self.parser.next_visible_offset(&self.text, self.cursor_offset),
                };
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("Up") | key!("Shift+Up") => {
                if self.cursor_y > 0 {
                    self.move_vertical(self.cursor_y - 1, select);
                }
                return EventProcessStatus::Processed;
            }
            key!("Down") | key!("Shift+Down") => {
                self.move_vertical(self.cursor_y + 1, select);
                return EventProcessStatus::Processed;
            }
            key!("PageUp") | key!("Shift+PageUp") => {
                self.move_vertical(self.cursor_y.saturating_sub(height), select);
                return EventProcessStatus::Processed;
            }
            key!("PageDown") | key!("Shift+PageDown") => {
                self.move_vertical((self.cursor_y + height).min(self.rows.saturating_sub(1)), select);
                return EventProcessStatus::Processed;
            }
            key!("Home") | key!("Shift+Home") => {
                let offset = self.parser.get_offset_from_position(&self.text, 0, self.cursor_y);
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("End") | key!("Shift+End") => {
                let offset = self.parser.get_offset_from_position(&self.text, u32::MAX, self.cursor_y);
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Left") | key!("Ctrl+Shift+Left") => {
                let offset = self.prev_word(self.cursor_offset);
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Right") | key!("Ctrl+Shift+Right") => {
                let offset = self.next_word(self.cursor_offset);
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Home") | key!("Ctrl+Shift+Home") => {
                self.move_to(0, select);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+End") | key!("Ctrl+Shift+End") => {
                let offset = self.text.len() as u32;
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Up") => {
                let row = self.first_row.saturating_sub(1);
                self.scroll_to(row);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Down") => {
                let row = self.first_row + 1;
                self.scroll_to(row);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+PageUp") => {
                let row = self.first_row.saturating_sub(height);
                self.scroll_to(row);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+PageDown") => {
                let row = self.first_row + height;
                self.scroll_to(row);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+A") => {
                self.select_all();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+C") | key!("Ctrl+Insert") => {
                self.copy_selection();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+X") | key!("Shift+Delete") => {
                if !self.read_only {
                    self.cut_selection();
                    return EventProcessStatus::Processed;
                }
            }
            key!("Ctrl+V") | key!("Shift+Insert") => {
                if !self.read_only {
                    self.paste();
                    return EventProcessStatus::Processed;
                }
            }
            key!("Backspace") => {
                if !self.read_only {
                    self.delete_previous();
                    return EventProcessStatus::Processed;
                }
            }
            key!("Delete") => {
                if !self.read_only {
                    self.delete_current();
                    return EventProcessStatus::Processed;
                }
            }
            key!("Ctrl+Backspace") => {
                if !self.read_only {
                    self.delete_previous_word();
                    return EventProcessStatus::Processed;
                }
            }
            key!("Ctrl+Delete") => {
                if !self.read_only {
                    self.delete_next_word();
                    return EventProcessStatus::Processed;
                }
            }
            key!("Enter") => {
                if !self.read_only {
                    self.insert('\n');
                    return EventProcessStatus::Processed;
                }
            }
            key!("Ctrl+Enter") => {
                self.popup_close_list();
                self.raise_event(ControlEvent {
                    emitter: self.handle,
                    receiver: self.event_processor,
                    data: ControlEventData::MarkdownComposer(EventData {
                        evtype: MarkdownComposerEventsType::OnValidate,
                    }),
                });
                return EventProcessStatus::Processed;
            }
            _ => {}
        }

        if (character as u32) > 0 && !self.read_only {
            let trigger = self.popup.is_none() && self.popup_find_list(character).is_some();
            self.insert(character);

            if trigger {
                let start = self.cursor_offset - character.len_utf8() as u32;
                if self.popup_is_list_start(start) {
                    self.popup_open_list(character, start);
                }
            }
            if self.emoticons && character == ':' {
                self.replace_emoticon();
            }
            return EventProcessStatus::Processed;
        }

        EventProcessStatus::Ignored
    }
}

impl OnExpand for MarkdownComposer {
    fn on_expand(&mut self, _direction: ExpandedDirection) {
        self.expanded = true;
        self.expanded_offset = (self.packed_origin - self.screen_origin.y).max(0);
    }

    fn on_pack(&mut self) {
        self.expanded = false;
        self.expanded_offset = 0;

        if self.popup.is_some() && !self.popup_fits_inside() {
            self.popup_close_list();
        }
    }
}

impl OnFocus for MarkdownComposer {
    fn on_lose_focus(&mut self) {
        self.popup_close_list();
    }
}
