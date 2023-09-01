use crate::{
    graphics::{Surface, TextAlignament, TextFormat},
    system::{Handle, Theme},
    utils::Caption
};

use super::{AddToToolbar, Gravity, ItemBase, PaintData, SymbolAttrState, ToolBarItem};

pub struct SingleChoice {
    pub(super) base: ItemBase,
    pub(super) handle: Handle<SingleChoice>,
    caption: Caption,
    group_id: u32,
    selected: bool,
}

impl AddToToolbar<SingleChoice> for SingleChoice {
    fn add(self, toolbar: &mut super::toolbar::ToolBar) -> Handle<SingleChoice> {
        toolbar.items.add(ToolBarItem::SingleChoice(self)).cast()
    }
}

impl SingleChoice {
    pub fn new(gravity: Gravity, text: &str, group_id: u32) -> Self {
        let mut obj = SingleChoice {
            base: ItemBase::new(gravity, true, true),
            handle: Handle::None,
            caption: Caption::new("", false),
            group_id,
            selected: false,            
        };
        obj.set_text(text);
        obj
    }
    pub fn set_text(&mut self, text: &str) {
        self.caption.set_text(text, true);
        self.base.set_width(self.caption.get_chars_count() as u16);
        self.base.request_recompute_layout();
    }
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        self.selected
    }
    #[inline(always)]
    pub fn get_group_id(&self)->u32 {
        self.group_id
    }

    pub(crate) fn update_select_status(&mut self, value: bool) {
        self.selected = value;
    }

    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let mut st = SymbolAttrState::new(data);
        if (self.selected) && (data.focused) {
            st = SymbolAttrState::Pressed;
        }
        let mut format = TextFormat::single_line(
            self.base.get_x(),
            self.base.get_y(),
            st.get_button_attr(theme),
            TextAlignament::Left,
        );
        format.width = Some(self.caption.get_chars_count() as u16);
        format.hotkey_pos = self.caption.get_hotkey_pos();
        if self.caption.has_hotkey() {
            format.hotkey_attr = Some(st.get_hotkey_attr(theme));
        }
        surface.write_text(self.caption.get_text(), &format);
    }
}
