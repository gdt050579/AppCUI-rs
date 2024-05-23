use std::marker::PhantomData;

use crate::prelude::*;

const MINSPACE_FOR_DRAWING: u32 = 5;
const MIN_WIDTH_VARIANT_NAME: u32 = 6;
const MINSPACE_FOR_DROPBUTTON_DRAWING: u32 = 3;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum ButtonState {
    Hidden,
    Normal,
    Hovered,
    Inactive,
    Pressed,
}

impl ButtonState {
    #[inline(always)]
    fn color(&self, theme: &Theme) -> CharAttribute {
        match self {
            ButtonState::Hidden => theme.menu.text.inactive,
            ButtonState::Normal => theme.menu.text.normal,
            ButtonState::Hovered => theme.menu.text.hovered,
            ButtonState::Inactive => theme.menu.text.inactive,
            ButtonState::Pressed => theme.menu.text.pressed_or_selectd,
        }
    }
    #[inline(always)]
    fn update_state(&mut self, check: bool, expected_value: ButtonState) -> bool {
        if (check) && (*self != expected_value) {
            *self = expected_value;
            return true;
        }
        if (!check) && (*self == expected_value) {
            *self = ButtonState::Normal;
            return true;
        }
        false
    }
    #[inline(always)]
    fn is_accesible(&self) -> bool {
        !matches!(self, ButtonState::Hidden | ButtonState::Inactive)
    }
}

pub(crate) trait ComboBoxComponentDataProvider {
    fn count(&self) -> u32;
    fn name(&self, index: u32) -> Option<&str>;
    fn description(&self, index: u32) -> &str;
}

pub(crate) struct ComboBoxComponent<DataProvider>
where
    DataProvider: ComboBoxComponentDataProvider,
{
    pub(crate) current_index: u32,
    start_index: u32,
    mouse_index: u32,
    header_y_ofs: i32,
    expanded_panel_y: i32,
    allow_none_value: bool,
    expanded_size: Size,
    count: u32,
    button_up: ButtonState,
    button_down: ButtonState,
    _phantom: PhantomData<DataProvider>,
    none_repr: String,
}
impl<T> ComboBoxComponent<T>
where
    T: ComboBoxComponentDataProvider,
{
    pub(crate) fn new(allow_none_value: bool, count: u32) -> Self {
        Self {
            start_index: 0,
            current_index: 0,
            header_y_ofs: 0,
            expanded_panel_y: 1,
            mouse_index: u32::MAX,
            allow_none_value,
            expanded_size: Size::default(),
            count,
            button_up: ButtonState::Hidden,
            button_down: ButtonState::Hidden,
            none_repr: String::new(),
            _phantom: PhantomData,
        }
    }
    fn visible_items(&self) -> u32 {
        let height = self.expanded_size.height;
        if height > 3 {
            height - 3
        } else {
            1
        }
    }
    pub(crate) fn update_current_index(&mut self, pos: u32) {
        let expanded_size = self.expanded_size;
        // there should be atleast one item visible
        let visible_items = if expanded_size.height > 3 { expanded_size.height - 3 } else { 1 };
        let count = self.count;
        if count > 0 {
            let last_item_index = if self.allow_none_value { count } else { count - 1 };
            self.current_index = pos.min(last_item_index);
            if self.start_index >= self.current_index {
                self.start_index = self.current_index;
            } else if self.start_index + visible_items <= self.current_index {
                self.start_index = self.current_index + 1 - visible_items;
            }
            if self.start_index + visible_items > (last_item_index + 1) {
                self.start_index = last_item_index + 1 - visible_items;
            }
        }
        self.update_button_states();
    }
    fn update_button_states(&mut self) {
        if self.button_up == ButtonState::Hidden {
            return;
        }
        self.button_up = if self.current_index == 0 {
            ButtonState::Inactive
        } else {
            ButtonState::Normal
        };
        if self.count > 0 {
            let last = if self.allow_none_value { self.count } else { self.count - 1 };
            self.button_down = if self.current_index == last {
                ButtonState::Inactive
            } else {
                ButtonState::Normal
            };
        } else {
            self.button_down = ButtonState::Inactive;
        }
    }
    fn move_scrollview_up(&mut self) {
        let cpoz = self.start_index;
        if self.start_index > 0 {
            self.start_index -= 1;
            self.update_current_index(self.current_index);
        }
        if (cpoz == self.start_index) && (self.current_index > 0) {
            // if the view has not changed -> move the selection
            self.update_current_index(self.current_index - 1);
            // try one more time
            if self.start_index > 0 {
                self.start_index -= 1;
                self.update_current_index(self.current_index);
            }
        }
    }
    fn move_scrollview_down(&mut self) {
        let cpoz = self.start_index;
        self.start_index += 1;
        self.update_current_index(self.current_index);
        if cpoz == self.start_index {
            // if the view has not changed -> move the selection
            self.update_current_index(self.current_index + 1);
            // try one more time
            self.start_index += 1;
            self.update_current_index(self.current_index);
        }
    }
    fn find_first_with_letter(&self, data: &T, to_find: char) -> u32 {
        if self.count == 0 {
            return u32::MAX;
        }
        for i in 1..=self.count {
            let poz = (self.current_index + i) % self.count;
            if let Some(name) = data.name(poz) {
                if name
                    .chars()
                    .next()
                    .map(|c| c.to_ascii_lowercase() == to_find.to_ascii_lowercase())
                    .unwrap_or(false)
                {
                    return poz;
                }
            }
        }
        u32::MAX
    }
    fn mouse_pos_to_index(&self, x: i32, y: i32) -> u32 {
        if self.expanded_size.height == 0 {
            return u32::MAX;
        }
        let size = self.expanded_size;
        let visible_items = (if size.height > 3 { size.height - 3 } else { 1 }) as i32;
        if (x > 0) && (x < size.width as i32) && (y > self.expanded_panel_y) && (y <= self.expanded_panel_y + visible_items) {
            return self.start_index + (y - (self.expanded_panel_y + 1)) as u32;
        }
        u32::MAX
    }
    fn is_on_button_up(&self, x: i32, y: i32) -> bool {
        if self.button_up == ButtonState::Hidden {
            return false;
        }
        let x_start_menu = (self.expanded_size.width / 2) as i32 - 1;
        (y == self.expanded_panel_y) && (x >= x_start_menu) && (x < x_start_menu + 3)
    }
    fn is_on_buttom_down(&self, x: i32, y: i32) -> bool {
        if self.button_down == ButtonState::Hidden {
            return false;
        }
        let x_start_menu = (self.expanded_size.width / 2) as i32 - 1;
        (y == self.expanded_panel_y + self.expanded_size.height as i32 - 2) && (x >= x_start_menu) && (x < x_start_menu + 3)
    }
    fn paint_buttons(&self, surface: &mut Surface, theme: &Theme) {
        let x = (self.expanded_size.width / 2) as i32 - 1;
        let attr = self.button_up.color(theme);
        surface.fill_horizontal_line_with_size(x, self.expanded_panel_y, 3, Character::with_attributes(' ', attr));
        surface.write_char(x + 1, self.expanded_panel_y, Character::with_attributes(SpecialChar::TriangleUp, attr));
        let y = self.expanded_panel_y + self.expanded_size.height as i32 - 2;
        let attr = self.button_down.color(theme);
        surface.fill_horizontal_line_with_size(x, y, 3, Character::with_attributes(' ', attr));
        surface.write_char(x + 1, y, Character::with_attributes(SpecialChar::TriangleDown, attr));
    }
    pub(crate) fn on_paint(&self, control: &ControlBase, data: &T, surface: &mut Surface, theme: &Theme) {
        // first paint the header
        let size = control.size();
        let col_text = match () {
            _ if !control.is_enabled() => theme.button.text.inactive,
            _ if control.has_focus() => theme.button.text.focused,
            _ if control.is_mouse_over() => theme.button.text.hovered,
            _ => theme.button.text.normal,
        };

        let space_char = Character::with_attributes(' ', col_text);
        if size.width > MINSPACE_FOR_DRAWING {
            surface.fill_horizontal_line(0, self.header_y_ofs, (size.width - MINSPACE_FOR_DRAWING) as i32, space_char);
            if size.width > MIN_WIDTH_VARIANT_NAME {
                let mut format = TextFormat::single_line(1, self.header_y_ofs, col_text, TextAlignament::Left);
                format.width = Some((size.width - MIN_WIDTH_VARIANT_NAME) as u16);
                if let Some(value) = data.name(self.current_index) {
                    surface.write_text(value, &format);
                } else if !self.none_repr.is_empty() {
                    surface.write_text(&self.none_repr, &format);
                }
            }
        }
        if size.width >= MINSPACE_FOR_DROPBUTTON_DRAWING {
            let px = (size.width - MINSPACE_FOR_DROPBUTTON_DRAWING) as i32;
            surface.fill_horizontal_line_with_size(px, self.header_y_ofs, 3, space_char);
            surface.write_char(px + 1, self.header_y_ofs, Character::with_attributes(SpecialChar::TriangleDown, col_text));
        }
        // assuming the control is expanded
        if control.is_expanded() {
            let size = self.expanded_size;
            let col = theme.menu.text.normal;
            surface.fill_rect(
                Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
                Character::with_attributes(' ', col),
            );
            surface.draw_rect(
                Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
                LineType::Single,
                col,
            );

            if self.button_up != ButtonState::Hidden {
                self.paint_buttons(surface, theme);
            }

            if self.count > 0 {
                let visible_items = if size.height > 3 { size.height - 3 } else { 1 };
                let mut format = TextFormat::single_line(2, self.expanded_panel_y + 1, col_text, TextAlignament::Left);
                format.width = Some((size.width - 4) as u16);

                for i in self.start_index..self.start_index + visible_items {
                    if let Some(value) = data.name(i) {
                        format.char_attr = theme.menu.text.normal;
                        surface.write_text(value, &format);
                    } else if !self.none_repr.is_empty() {
                        format.char_attr = theme.menu.text.inactive;
                        surface.write_text(&self.none_repr, &format);
                    }
                    if i == self.current_index {
                        surface.fill_horizontal_line(
                            1,
                            format.y,
                            (size.width - 2) as i32,
                            Character::with_attributes(0, theme.menu.text.pressed_or_selectd),
                        );
                    } else if i == self.mouse_index {
                        surface.fill_horizontal_line(
                            1,
                            format.y,
                            (size.width - 2) as i32,
                            Character::with_attributes(0, theme.menu.text.hovered),
                        );
                    }
                    format.y += 1;
                }
            }
        }
    }
    pub(crate) fn on_default_action(&mut self, control: &mut ControlBase) {
        if control.is_expanded() {
            control.pack();
        } else {
            let w = control.size().width;
            let h = if self.allow_none_value { self.count + 4 } else { self.count + 3 };
            control.expand(Size::new(w, h.min(4)), Size::new(w, h));
        }
    }
    pub(crate) fn on_key_pressed(&mut self, control: &mut ControlBase, data: &T, key: Key, character: char) -> EventProcessStatus {
        let expanded = control.is_expanded();

        match key.value() {
            key!("Escape") => {
                if expanded {
                    control.pack();
                    return EventProcessStatus::Processed;
                } else {
                    return EventProcessStatus::Ignored;
                }
            }
            key!("Space") | key!("Enter") => {
                self.on_default_action(control);
                return EventProcessStatus::Processed;
            }
            key!("Up") => {
                if self.current_index > 0 {
                    self.update_current_index(self.current_index - 1);
                };
                return EventProcessStatus::Processed;
            }
            key!("Down") => {
                self.update_current_index(self.current_index + 1);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Up") => {
                if expanded {
                    // only if expanded
                    self.move_scrollview_up();
                    return EventProcessStatus::Processed;
                } else {
                    return EventProcessStatus::Ignored;
                }
            }
            key!("Ctrl+Down") => {
                if expanded {
                    // only if expanded
                    self.move_scrollview_down();
                    return EventProcessStatus::Processed;
                } else {
                    return EventProcessStatus::Ignored;
                }
            }
            key!("Home") => {
                self.update_current_index(0);
                return EventProcessStatus::Processed;
            }
            key!("End") => {
                self.update_current_index(u32::MAX);
                return EventProcessStatus::Processed;
            }
            key!("PageUp") => {
                let page_count = self.visible_items();
                if self.current_index > page_count {
                    self.update_current_index(self.current_index - page_count);
                } else {
                    self.update_current_index(0);
                }
                return EventProcessStatus::Processed;
            }
            key!("PageDown") => {
                self.update_current_index(self.current_index + self.visible_items());
                return EventProcessStatus::Processed;
            }
            _ => {}
        }
        if !key.modifier.contains_one(KeyModifier::Alt | KeyModifier::Ctrl) {
            match character {
                'A'..='Z' | 'a'..='z' | '0'..='9' => {
                    let poz = self.find_first_with_letter(data, character);
                    if poz != u32::MAX {
                        self.update_current_index(poz);
                    }
                    return EventProcessStatus::Processed;
                }
                _ => {}
            }
        }
        EventProcessStatus::Ignored
    }
    pub(crate) fn on_mouse_event(&mut self, control: &mut ControlBase, data: &T, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => {
                if !control.is_expanded() {
                    let desc = data.description(self.current_index);
                    if !desc.is_empty() {
                        control.show_tooltip(desc);
                    }
                }
                EventProcessStatus::Processed
            }

            MouseEvent::Leave => {
                control.hide_tooltip();
                self.mouse_index = u32::MAX;
                EventProcessStatus::Processed
            }
            MouseEvent::Over(p) => {
                let idx = self.mouse_pos_to_index(p.x, p.y);
                let is_on_button_up = self.is_on_button_up(p.x, p.y);
                let is_on_button_down = self.is_on_buttom_down(p.x, p.y);
                let mut result = EventProcessStatus::Ignored;
                if self.button_up.is_accesible() && self.button_up.update_state(is_on_button_up, ButtonState::Hovered) {
                    result = EventProcessStatus::Processed;
                }
                if self.button_down.is_accesible() && self.button_down.update_state(is_on_button_down, ButtonState::Hovered) {
                    result = EventProcessStatus::Processed;
                }
                if idx != self.mouse_index {
                    self.mouse_index = idx;
                    result = EventProcessStatus::Processed;
                }
                result
            }
            MouseEvent::DoubleClick(data) | MouseEvent::Pressed(data) => {
                let is_on_button_up = self.is_on_button_up(data.x, data.y);
                let is_on_button_down = self.is_on_buttom_down(data.x, data.y);
                if is_on_button_up {
                    if self.button_up.is_accesible() && self.current_index > 0 {
                        self.update_current_index(self.current_index - 1);
                    };
                    if self.button_up.is_accesible() {
                        self.button_up = ButtonState::Pressed;
                    }
                    return EventProcessStatus::Processed;
                }
                if is_on_button_down {
                    if self.button_down.is_accesible() {
                        self.update_current_index(self.current_index + 1);
                    }
                    if self.button_down.is_accesible() {
                        self.button_down = ButtonState::Pressed;
                    }
                    return EventProcessStatus::Processed;
                }
                let idx = self.mouse_pos_to_index(data.x, data.y);
                if idx != u32::MAX {
                    self.update_current_index(idx);
                }
                self.on_default_action(control);
                EventProcessStatus::Processed
            }
            MouseEvent::Released(data) => {
                let is_on_button_up = self.is_on_button_up(data.x, data.y);
                let is_on_button_down = self.is_on_buttom_down(data.x, data.y);
                let mut result = EventProcessStatus::Ignored;
                if self.button_up.is_accesible() && self.button_up.update_state(is_on_button_up, ButtonState::Hovered) {
                    result = EventProcessStatus::Processed;
                }
                if self.button_down.is_accesible() && self.button_down.update_state(is_on_button_down, ButtonState::Hovered) {
                    result = EventProcessStatus::Processed;
                }
                result
            }
            MouseEvent::Wheel(direction) => {
                match direction {
                    MouseWheelDirection::Up => {
                        if self.current_index > 0 {
                            self.update_current_index(self.current_index - 1);
                        }
                    }
                    MouseWheelDirection::Down => self.update_current_index(self.current_index + 1),
                    _ => {}
                }
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
    pub(crate) fn on_expand(&mut self, control: &mut ControlBase, direction: ExpandedDirection) {
        match direction {
            ExpandedDirection::OnTop => {
                self.expanded_panel_y = 0;
                self.header_y_ofs = (control.expanded_size().height as i32) - 1;
            }
            ExpandedDirection::OnBottom => {
                self.expanded_panel_y = 1;
                self.header_y_ofs = 0;
            }
        }
        self.expanded_size = control.expanded_size();
        self.update_current_index(self.current_index);
        self.mouse_index = u32::MAX;
        let items_to_show = if self.allow_none_value { self.count + 1 } else { self.count };
        if self.expanded_size.height == items_to_show + 3 {
            self.button_down = ButtonState::Hidden;
            self.button_up = ButtonState::Hidden;
        } else {
            self.button_down = ButtonState::Normal;
            self.button_up = ButtonState::Normal;
            self.update_button_states();
        }
    }
    pub(crate) fn on_pack(&mut self) {
        self.expanded_panel_y = 1;
        self.header_y_ofs = 0;
        self.mouse_index = u32::MAX;
        self.expanded_size = Size::default();
        self.button_down = ButtonState::Hidden;
        self.button_up = ButtonState::Hidden;
    }
    pub(crate) fn set_none_string(&mut self, value: &str) {
        self.none_repr.clear();
        self.none_repr.push_str(value);
    }
    pub(crate) fn update_count(&mut self, new_count: u32) {
        todo!("check logic between the new count and the old one");
        // for example, if new_count is 0, then reset the value to 0
        // same if the new_count is smaller than current index or start view
    }
}
