use crate::prelude::*;
use crate::ui::tab::Flags;
use super::AccordionPanel;

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
            (theme.tab.list.inactive, theme.tab.listhotkey.inactive)
        } else {
            if idx == self.focused_child_index.index() {
                (theme.tab.list.pressed_or_selectd, theme.tab.listhotkey.pressed_or_selectd)
            } else {
                if let Some(hovered_idx) = self.hovered_page_idx {
                    if hovered_idx == idx {
                        (theme.tab.list.hovered, theme.tab.listhotkey.hovered)
                    } else {
                        (theme.tab.list.normal, theme.tab.listhotkey.normal)
                    }
                } else {
                    (theme.tab.list.normal, theme.tab.listhotkey.normal)
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
impl OnMouseEvent for Accordion {

}
impl OnKeyPressed for Accordion {

}