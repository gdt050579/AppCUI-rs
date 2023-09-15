use crate::{
    graphics::{Character, SpecialChar, Surface, TextAlignament, TextFormat},
    system::{Handle, Theme},
    utils::Caption,
};

use super::{AddToToolbar, ItemBase, PaintData, SymbolAttrState, ToolBarItem, Group};

pub struct CheckBox {
    pub(super) base: ItemBase,
    pub(super) caption: Caption,
    checked: bool,
}

impl AddToToolbar<CheckBox> for CheckBox {
    fn add(mut self, toolbar: &mut super::toolbar::ToolBar,  group: Group) -> Handle<CheckBox> {
        self.base.update_group(group);
        toolbar.items.add(ToolBarItem::CheckBox(self)).cast()
    }
}

impl CheckBox {
    pub fn new(text: &str, checked: bool) -> Self {
        let mut obj = CheckBox {
            base: ItemBase::new(true),
            caption: Caption::new("", false),
            checked,
        };
        obj.set_text(text);
        obj
    }
    pub fn set_text(&mut self, text: &str) {
        self.caption.set_text(text, true);
        self.base.set_width((self.caption.get_chars_count() + 2) as u16);
        self.base.request_recompute_layout();
    }
    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
        self.base.request_recompute_layout();
    }
    pub(crate) fn reverse_check(&mut self) {
        self.set_checked(!self.checked);
    }
    #[inline(always)]
    pub fn is_checked(&self) -> bool {
        self.checked
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let st = SymbolAttrState::new(data);
        let text_attr = st.get_button_attr(theme);
        let x = self.base.get_left();
        let y = self.base.get_y();
        let mut format = TextFormat::single_line(x + 2, y, text_attr, TextAlignament::Left);
        format.width = Some(self.caption.get_chars_count() as u16);
        format.hotkey_pos = self.caption.get_hotkey_pos();
        if self.caption.has_hotkey() {
            format.hotkey_attr = Some(st.get_hotkey_attr(theme));
        }
        surface.write_string(x, y, "  ", text_attr, false);
        surface.write_text(self.caption.get_text(), &format);
        if self.checked {
            surface.write_char(
                x,
                y,
                Character::with_attributes(SpecialChar::CheckMark, st.get_attr(theme, theme.symbol.checked)),
            );
        }
    }
}
