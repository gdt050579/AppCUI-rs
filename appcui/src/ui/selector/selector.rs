use std::marker::PhantomData;

use super::EnumSelector;
use super::Flags;
use crate::prelude::*;

const MINSPACE_FOR_DRAWING: u32 = 5;
const MIN_WIDTH_VARIANT_NAME: u32 = 6;
const MINSPACE_FOR_DROPBUTTON_DRAWING: u32 = 3;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent+OnExpand, internal=true)]
pub struct Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    current_index: u32,
    header_y_ofs: i32,
    flags: Flags,
    _phanton: PhantomData<T>,
}
impl<T> Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    pub fn new(value: Option<T>, layout: Layout, flags: Flags) -> Self {
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            current_index: u32::MAX,
            header_y_ofs: 0,
            flags,
            _phanton: PhantomData,
        };
        if let Some(val) = value {
            let count = T::count();
            for i in 0..count {
                if T::from_index(i) == Some(val) {
                    obj.current_index = i;
                    break;
                }
            }
        } else {
            // value is None
            if !obj.flags.contains(Flags::AllowNoneVariant) {
                panic!(
                    "You can not instantiate a selector with `None` value without setting the flags `AllowNoneVariant`. Have you forgot to do this ?"
                );
            }
        }
        obj.set_size_bounds(7, 1, u16::MAX, 1);
        obj
    }
    #[inline(always)]
    pub fn value(&self) -> T {
        EnumSelector::from_index(self.current_index).unwrap()
    }
    #[inline(always)]
    pub fn try_value(&self) -> Option<T> {
        EnumSelector::from_index(self.current_index)
    }
}
impl<T> OnPaint for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        // first paint the header
        let size = self.size();
        let col_text = match () {
            _ if !self.is_enabled() => theme.button.text.inactive,
            _ if self.has_focus() => theme.button.text.focused,
            _ if self.is_mouse_over() => theme.button.text.hovered,
            _ => theme.button.text.normal,
        };

        let space_char = Character::with_attributes(' ', col_text);
        if size.width > MINSPACE_FOR_DRAWING {
            surface.fill_horizontal_line(0, self.header_y_ofs, (size.width - MINSPACE_FOR_DRAWING) as i32, space_char);
            if size.width > MIN_WIDTH_VARIANT_NAME {
                let mut format = TextFormat::single_line(1, self.header_y_ofs, col_text, TextAlignament::Left);
                format.width = Some((size.width - MIN_WIDTH_VARIANT_NAME) as u16);
                if let Some(value) = T::from_index(self.current_index) {
                    surface.write_text(value.name(), &format);
                } else {
                    surface.write_text("None", &format);
                }
            }
        }
        if size.width >= MINSPACE_FOR_DROPBUTTON_DRAWING {
            let px = (size.width - MINSPACE_FOR_DROPBUTTON_DRAWING) as i32;
            surface.fill_horizontal_line_with_size(px, self.header_y_ofs, 3, space_char);
            surface.write_char(px + 1, self.header_y_ofs, Character::with_attributes(SpecialChar::TriangleDown, col_text));
        }
        // assuming the control is expanded
        // if self.is_expanded() {
        //     let size = self.expanded_size();
        //     let col = theme.menu.text.normal;
        //     let mut space_char = Character::with_attributes(' ', col);
        //     surface.fill_rect(
        //         Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
        //         space_char,
        //     );
        //     surface.draw_rect(
        //         Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
        //         LineType::Single,
        //         col,
        //     );
        //     for y in 0..COLOR_MATRIX_HEIGHT {
        //         for x in 0..COLOR_MATRIX_WIDTH {
        //             space_char.background = Color::from_value(y * COLOR_MATRIX_WIDTH + x).unwrap();
        //             surface.fill_horizontal_line_with_size(
        //                 x * SPACES_PER_COLOR + 1,
        //                 y + 1 + self.expanded_panel_y,
        //                 SPACES_PER_COLOR as u32,
        //                 space_char,
        //             );
        //             if space_char.background == self.color {
        //                 surface.write_char(
        //                     x * SPACES_PER_COLOR + ((SPACES_PER_COLOR + 1) >> 1),
        //                     y + 1 + self.expanded_panel_y,
        //                     Character::new(
        //                         SpecialChar::CheckMark,
        //                         REVERSED_COLORS[(y * COLOR_MATRIX_WIDTH + x) as usize],
        //                         space_char.background,
        //                         CharFlags::None,
        //                     ),
        //                 );
        //             }
        //             if self.mouse_on_color_index == (y * COLOR_MATRIX_WIDTH + x) {
        //                 let x_p = x * SPACES_PER_COLOR + 1;
        //                 let y_p = y + 1 + self.expanded_panel_y;
        //                 let c_attr = CharAttribute::new(
        //                     REVERSED_COLORS[(y * COLOR_MATRIX_WIDTH + x) as usize],
        //                     space_char.background,
        //                     CharFlags::None,
        //                 );
        //                 surface.write_char(x_p, y_p, Character::with_attributes(SpecialChar::TriangleLeft, c_attr));
        //                 surface.write_char(x_p + 2, y_p, Character::with_attributes(SpecialChar::TriangleRight, c_attr));
        //             }
        //         }
        //     }

        //     // transparent part
        //     let attr = match () {
        //         _ if self.color == Color::Transparent => theme.menu.text.focused,
        //         _ if self.mouse_on_color_index == 16 => theme.menu.text.hovered,
        //         _ => theme.menu.text.normal,
        //     };
        //     surface.write_string(TRANSPARENT_CHECKBOX_X_OFFSET, 1 + self.expanded_panel_y, "[ ] Transparent", attr, false);
        //     if self.color == Color::Transparent {
        //         surface.write_char(
        //             TRANSPARENT_CHECKBOX_X_OFFSET + 1,
        //             1 + self.expanded_panel_y,
        //             Character::with_attributes(SpecialChar::CheckMark, theme.menu.symbol.normal),
        //         );
        //         surface.set_cursor(TRANSPARENT_CHECKBOX_X_OFFSET + 1, 1 + self.expanded_panel_y);
        //     }
        // }
    }
}
impl<T> OnExpand for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_expand(&mut self, _direction: ExpandedDirection) {}

    fn on_pack(&mut self) {}
}
impl<T> OnDefaultAction for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_default_action(&mut self) {}
}
impl<T> OnKeyPressed for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl<T> OnMouseEvent for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
