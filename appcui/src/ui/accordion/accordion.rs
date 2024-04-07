use super::AccordionPanel;
use crate::prelude::*;
use crate::ui::tab::Flags;

#[CustomControl(overwrite=OnPaint+OnMouseEvent+OnKeyPressed, internal=true)]
pub struct Accordion {
    flags: Flags,
    pages: Vec<Caption>,
    hovered_page_idx: Option<usize>,
}
impl Accordion {
    #[inline(always)]
    fn get_panelattr(&self, theme: &Theme, idx: usize) -> (CharAttribute, CharAttribute) {
        if !self.is_enabled() {
            (theme.accordion.text.inactive, theme.accordion.hotkey.inactive)
        } else {
            if idx == self.focused_child_index.index() {
                (theme.accordion.text.pressed_or_selectd, theme.accordion.hotkey.pressed_or_selectd)
            } else {
                if let Some(hovered_idx) = self.hovered_page_idx {
                    if hovered_idx == idx {
                        (theme.accordion.text.hovered, theme.accordion.hotkey.hovered)
                    } else {
                        (theme.accordion.text.normal, theme.accordion.hotkey.normal)
                    }
                } else {
                    (theme.accordion.text.normal, theme.accordion.hotkey.normal)
                }
            }
        }
    }
    #[inline(always)]
    fn get_backattr(&self, theme: &Theme) -> CharAttribute {
        match () {
            _ if !self.is_enabled() => theme.tab.text.inactive,
            _ if self.has_focus() => theme.tab.text.pressed_or_selectd,
            _ => theme.tab.text.pressed_or_selectd,
        }
    }
    fn mouse_position_to_index(&self, x: i32, y: i32) -> Option<usize> {
        let sz = self.size();
        if (y < 0) || (x < 0) || (x>=sz.width as i32) {
            return None;
        }
        let count = self.base.children.len();
        let fc = self.base.focused_child_index.index();
        // check top allignament
        if y as usize <= fc {
            return Some(y as usize);
        }
        if fc >= count {
            return None;
        }
        // check bottom allignament
        let bottom_index = (count - fc) as i32;
        let h = sz.height as i32;
        if h < bottom_index {
            return None;
        }
        if y >= (h - bottom_index) && (y < h) {
            Some(fc + 1 + ((h - bottom_index) as usize))
        } else {
            None
        }
    }
}
impl OnPaint for Accordion {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if !self.flags.contains(Flags::TransparentBackground) {
            surface.clear(Character::with_attributes(' ', self.get_backattr(theme)));
        }
        let sz = self.size();
        let mut format = TextFormat {
            x: 1,
            y: 1,
            width: Some(if sz.width > 2 { (sz.width as u16) - 2 } else { 1 }),
            align: TextAlignament::Left,
            text_wrap: TextWrap::None,
            multi_line: false,
            ..Default::default()
        };

        let cidx = self.base.focused_child_index.index();
        let count = self.base.children.len();
        for (index, page) in self.pages.iter().enumerate() {
            let (text_attr, hotkey_attr) = self.get_panelattr(theme, index);
            format.chars_count = Some(page.chars_count() as u16);
            format.hotkey_pos = page.hotkey_pos();
            format.char_attr = text_attr;
            format.hotkey_attr = Some(hotkey_attr);
            // position
            if index <= cidx {
                format.y = index as i32;
            } else {
                format.y = (sz.height as i32) - ((count - index) as i32);
            }

            // fill the tab
            surface.fill_horizontal_line_with_size(0, format.y, sz.width as u32, Character::with_attributes(' ', text_attr));

            // write the text
            surface.write_text(page.text(), &format);
        }
    }
}
impl OnMouseEvent for Accordion {}
impl OnKeyPressed for Accordion {}
