use EnumBitFlags::EnumBitFlags;

use crate::{
    graphics::{CharAttribute, Character, SpecialChar, Surface, TextAlignament, TextFormat},
    system::Theme,
    utils::Caption,
};

use super::SymbolAttrState;

pub(super) struct DecoratorPaintData {
    pub(super) focused: bool,
    pub(super) current: bool,
    pub(super) maximized: bool,
    pub(super) is_current_item_pressed: bool,
    pub(super) sep_attr: CharAttribute,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub(super) enum DecoratorType {
    None,
    HotKeY,
    CloseButton,
    MaximizeRestoreButton,
    WindowResize,
    Tag,
    Button,
    SingleChoice,
    CheckBox,
    Text,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub(super) enum DecoratorLayout {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

#[EnumBitFlags(bits = 8)]
enum StatusFlags {
    Visible = 0x01,
    Hidden = 0x02,
    Checked = 0x04,
    LeftGroupMarker = 0x08,
    RightGroupMarker = 0x10,
}
pub(super) struct Decorator {
    tooltip: String,
    text: Caption,
    decorator_type: DecoratorType,
    status: StatusFlags,
    x: i32,
    y: i32,
    width: u16,
    id: u32,
    layout: DecoratorLayout,
}

impl Decorator {
    pub(super) fn with_type(
        decorator_type: DecoratorType,
        layout: DecoratorLayout,
        width: u16,
        tooltip: &str,
    ) -> Self {
        Self {
            tooltip: String::from(tooltip),
            text: Caption::default(),
            decorator_type,
            status: StatusFlags::None,
            x: 0,
            y: 0,
            width,
            id: 0,
            layout,
        }
    }
    pub(super) fn new(
        decorator_type: DecoratorType,
        layout: DecoratorLayout,
        text: Caption,
    ) -> Self {
        Self {
            tooltip: String::new(),
            text,
            decorator_type,
            status: StatusFlags::None,
            x: 0,
            y: 0,
            width: 0,
            id: 0,
            layout,
        }
    }
    #[inline(always)]
    pub(super) fn is_visible(&self) -> bool {
        self.status.contains(StatusFlags::Visible)
    }
    #[inline(always)]
    pub(super) fn is_hidden(&self) -> bool {
        self.status.contains(StatusFlags::Hidden)
    }
    #[inline(always)]
    pub(super) fn is_checked(&self) -> bool {
        self.status.contains(StatusFlags::Checked)
    }
    #[inline(always)]
    pub(super) fn set_checked(&mut self, value: bool) {
        if value {
            self.status |= StatusFlags::Checked;
        } else {
            self.status.remove(StatusFlags::Checked);
        }
    }
    #[inline(always)]
    pub(super) fn center_x(&self) -> i32 {
        self.x + ((self.width / 2) as i32)
    }
    #[inline(always)]
    pub(super) fn get_y(&self) -> i32 {
        self.y
    }
    #[inline(always)]
    pub(super) fn contains(&self, x: i32, y: i32) -> bool {
        (y == self.y)
            && (x >= self.x)
            && (x < (self.x + (self.width as i32)))
            && ((self.status & (StatusFlags::Visible | StatusFlags::Hidden))
                == StatusFlags::Visible)
    }

    #[inline(always)]
    pub(super) fn is_part_of_group(&self) -> bool {
        match self.decorator_type {
            DecoratorType::Button
            | DecoratorType::SingleChoice
            | DecoratorType::CheckBox
            | DecoratorType::Text => true,
            _ => false,
        }
    }
    #[inline(always)]
    pub(super) fn hide(&mut self) {
        self.status |= StatusFlags::Hidden;
    }
    #[inline(always)]
    pub(super) fn unhide(&mut self) {
        self.status.remove(StatusFlags::Hidden);
    }
    #[inline(always)]
    pub(super) fn set_right_marker(&mut self) {
        self.status |= StatusFlags::RightGroupMarker;
    }
    #[inline(always)]
    pub(super) fn set_left_marker(&mut self) {
        self.status |= StatusFlags::LeftGroupMarker;
    }
    #[inline(always)]
    pub(super) fn set_visible(&mut self) {
        self.status |= StatusFlags::Visible;
    }
    #[inline(always)]
    pub(super) fn clear(&mut self) {
        self.status.remove(
            StatusFlags::Visible | StatusFlags::LeftGroupMarker | StatusFlags::RightGroupMarker,
        );
    }

    #[inline(always)]
    pub(super) fn get_id(&self) -> u32 {
        self.id
    }

    #[inline(always)]
    pub(super) fn get_type(&self) -> DecoratorType {
        self.decorator_type
    }
    #[inline(always)]
    pub(super) fn get_layout(&self) -> DecoratorLayout {
        self.layout
    }

    pub(super) fn set_text(&mut self, text: &str, process_hotkey: bool) {
        self.text.set_text(text, process_hotkey);
    }
    pub(super) fn get_text(&self) -> &str {
        self.text.get_text()
    }
    pub(super) fn set_tooltip(&mut self, tooltip: &str) {
        self.tooltip.clear();
        if tooltip.len()>0 {
            self.tooltip.push_str(tooltip);
        }
    }
    pub(super) fn get_tooltip(&self) -> &str {
        &self.tooltip
    }

    fn paint_hotkey(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        paint_data: &DecoratorPaintData,
    ) -> bool {
        surface.write_char(
            self.x,
            self.y,
            Character::with_attributes('[', paint_data.sep_attr),
        );
        let attr = match paint_data.focused {
            true => theme.text.normal,
            false => theme.text.inactive,
        };
        surface.write_string(self.x + 1, self.y, self.text.get_text(), attr, false);
        surface.write_char(
            self.x + (self.width as i32) - 1,
            self.y,
            Character::with_attributes(']', paint_data.sep_attr),
        );
        return false;
    }
    fn paint_tag(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        paint_data: &DecoratorPaintData,
    ) -> bool {
        surface.write_char(
            self.x,
            self.y,
            Character::with_attributes('[', paint_data.sep_attr),
        );
        let attr = match paint_data.focused {
            true => theme.text.enphasized_2,
            false => theme.text.inactive,
        };
        surface.write_string(self.x + 1, self.y, self.text.get_text(), attr, false);
        surface.write_char(
            self.x + (self.width as i32) - 1,
            self.y,
            Character::with_attributes(']', paint_data.sep_attr),
        );
        return false;
    }
    fn paint_text(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        paint_data: &DecoratorPaintData,
    ) -> bool {
        let attr = match paint_data.focused {
            true => theme.text.normal,
            false => theme.text.inactive,
        };
        surface.write_string(self.x + 1, self.y, self.text.get_text(), attr, false);
        return true;
    }
    fn paint_close_button(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        paint_data: &DecoratorPaintData,
    ) -> bool {
        let st = SymbolAttrState::new(paint_data);
        surface.write_string(
            self.x,
            self.y,
            "[ ]",
            st.get_attr(theme, paint_data.sep_attr),
            false,
        );
        surface.write_char(
            self.x + 1,
            self.y,
            Character::with_attributes('x', st.get_attr(theme, theme.symbol.close)),
        );
        return false;
    }
    fn paint_max_button(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        paint_data: &DecoratorPaintData,
    ) -> bool {
        let st = SymbolAttrState::new(paint_data);
        surface.write_string(
            self.x,
            self.y,
            "[ ]",
            st.get_attr(theme, paint_data.sep_attr),
            false,
        );
        let ch = match paint_data.maximized {
            true => SpecialChar::ArrowUpDown,
            false => SpecialChar::ArrowUp,
        };
        surface.write_char(
            self.x + 1,
            self.y,
            Character::with_attributes(ch, st.get_attr(theme, theme.symbol.maximized)),
        );
        return false;
    }
    fn paint_resize_button(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        paint_data: &DecoratorPaintData,
    ) -> bool {
        if paint_data.focused {
            let st = SymbolAttrState::new(paint_data);
            surface.write_char(
                self.x,
                self.y,
                Character::with_attributes(
                    SpecialChar::BoxBottomRightCornerSingleLine,
                    st.get_attr(theme, theme.symbol.resize),
                ),
            );
        }
        return false;
    }
    fn paint_button(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        paint_data: &DecoratorPaintData,
    ) -> bool {
        let st = SymbolAttrState::new(paint_data);
        let show_checked =
            paint_data.focused && self.is_checked() && (st != SymbolAttrState::Pressed);
        let mut format = TextFormat::single_line(
            self.x,
            self.y,
            if show_checked {
                theme.button.text.pressed_or_selectd
            } else {
                st.get_button_attr(theme)
            },
            TextAlignament::Left,
        );
        format.width = Some(self.text.get_chars_count() as u16);
        format.hotkey_pos = self.text.get_hotkey_pos();
        if self.text.has_hotkey() {
            format.hotkey_attr = Some(if show_checked {
                theme.button.hotkey.pressed_or_selectd
            } else {
                st.get_hotkey_attr(theme)
            });
        }
        surface.write_text(self.text.get_text(), &format);
        return true;
    }
    fn paint_checkbox(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        paint_data: &DecoratorPaintData,
    ) -> bool {
        let st = SymbolAttrState::new(paint_data);
        let text_attr = st.get_button_attr(theme);
        let mut format =
            TextFormat::single_line(self.x + 2, self.y, text_attr, TextAlignament::Left);
        format.width = Some(self.text.get_chars_count() as u16);
        format.hotkey_pos = self.text.get_hotkey_pos();
        if self.text.has_hotkey() {
            format.hotkey_attr = Some(st.get_hotkey_attr(theme));
        }
        surface.fill_horizontal_line(
            self.x,
            self.y,
            self.x + 1,
            Character::with_attributes(' ', text_attr),
        );
        surface.write_text(self.text.get_text(), &format);
        if self.is_checked() {
            surface.write_char(
                self.x,
                self.y,
                Character::with_attributes(
                    SpecialChar::CheckMark,
                    st.get_attr(theme, theme.symbol.checked),
                ),
            );
        }
        return true;
    }
    pub(super) fn paint(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        paint_data: &DecoratorPaintData,
    ) {
        if (self.is_visible() == false) || (self.is_hidden()) {
            return;
        }

        let from_left = match self.layout {
            DecoratorLayout::TopLeft | DecoratorLayout::BottomLeft => true,
            _ => false,
        };
        let draw_separators = match self.decorator_type {
            DecoratorType::None => false,
            DecoratorType::HotKeY => self.paint_hotkey(surface, theme, paint_data),
            DecoratorType::CloseButton => self.paint_close_button(surface, theme, paint_data),
            DecoratorType::MaximizeRestoreButton => {
                self.paint_max_button(surface, theme, paint_data)
            }
            DecoratorType::WindowResize => self.paint_resize_button(surface, theme, paint_data),
            DecoratorType::Tag => self.paint_tag(surface, theme, paint_data),
            DecoratorType::Button => self.paint_button(surface, theme, paint_data),
            DecoratorType::SingleChoice => self.paint_button(surface, theme, paint_data),
            DecoratorType::CheckBox => self.paint_checkbox(surface, theme, paint_data),
            DecoratorType::Text => self.paint_text(surface, theme, paint_data),
        };
        // separators
        if draw_separators {
            if self.status.contains(StatusFlags::LeftGroupMarker) {
                surface.write_char(
                    self.x - 1,
                    self.y,
                    Character::with_attributes('[', paint_data.sep_attr),
                );
            } else if from_left {
                surface.write_char(
                    self.x - 1,
                    self.y,
                    Character::with_attributes('|', paint_data.sep_attr),
                );
            }
            if self.status.contains(StatusFlags::RightGroupMarker) {
                surface.write_char(
                    self.x + (self.width as i32),
                    self.y,
                    Character::with_attributes(']', paint_data.sep_attr),
                );
            } else if !from_left {
                surface.write_char(
                    self.x + (self.width as i32),
                    self.y,
                    Character::with_attributes('|', paint_data.sep_attr),
                );
            }
        }
    }

    pub(super) fn update_position_from_left(
        &mut self,
        x: i32,
        y: i32,
        last: DecoratorType,
    ) -> (i32, bool) {
        let part_of_group = self.is_part_of_group();
        let mut extra_space = 0;
        let mut right_group_marker = false;
        if part_of_group {
            extra_space = 1;
            if self.decorator_type != last {
                right_group_marker = true;
                extra_space += 1;
            }
        } else {
            if last != DecoratorType::None {
                right_group_marker = true;
            }
        }
        self.y = y;
        self.x = x + extra_space;
        let next = self.x + (self.width as i32) + 1;
        if part_of_group && (self.decorator_type != last) {
            self.status |= StatusFlags::LeftGroupMarker;
        }

        (next, right_group_marker)
    }
    pub(super) fn update_position_from_right(
        &mut self,
        x: i32,
        y: i32,
        last: DecoratorType,
    ) -> (i32, bool) {
        let part_of_group = self.is_part_of_group();
        let mut extra_space = 0;
        let mut left_group_marker = false;
        if part_of_group {
            extra_space = 1;
            if self.decorator_type != last {
                left_group_marker = true;
                extra_space += 1;
            }
        } else {
            if last != DecoratorType::None {
                left_group_marker = true;
            }
        }
        self.y = y;
        self.x = (x - self.width as i32) + 1;
        self.x -= extra_space;
        let next = self.x - 1;
        if part_of_group && (self.decorator_type != last) {
            self.status |= StatusFlags::RightGroupMarker;
        }

        (next, left_group_marker)
    }
}
