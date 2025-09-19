use super::{ItemBase, Side};
use crate::graphics::*;
use crate::system::Theme;

pub struct Label {
    text: String,
    desc: String,
    pub(super) base: ItemBase,
}

impl Label {
    const MAX_CAPTION_SIZE: usize = 128; // must be smaller than 255 (u8::MAX)
    pub fn new(caption: &str, order: u8, pos: Side) -> Self {
        let width = caption.chars().count();
        let s = if width > Label::MAX_CAPTION_SIZE {
            caption.chars().take(Label::MAX_CAPTION_SIZE).collect()
        } else {
            caption.to_string()
        };
        Self {
            text: s,
            desc: String::new(),
            base: ItemBase::new(width.min(Label::MAX_CAPTION_SIZE) as u8, order, pos, false),
        }
    }
    #[inline(always)]
    pub fn caption(&self) -> &str {
        self.text.as_str()
    }
    #[inline(always)]
    pub fn set_caption(&mut self, text: &str) {
        self.text.clear();
        self.text.push_str(text);
        let mut width = 0;
        let mut tpos = None;
        for (pos, _) in self.text.char_indices() {
            if pos >= Label::MAX_CAPTION_SIZE {
                tpos = Some(pos);
                break;
            }
            width += 1;
        }
        if let Some(tp) = tpos {
            self.text.truncate(tp);
        }
        //let orig_width = self.base.width();
        self.base.set_width(width as u8);
        self.base.refresh();
    }
    #[inline(always)]
    pub fn tooltip(&self) -> &str {
        &self.desc
    }
    #[inline(always)]
    pub fn set_tooltip(&mut self, text: &str) {
        if self.desc != text {
            self.desc.clear();
            self.desc.push_str(text);
            self.base.refresh();
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.write_string(self.base.x(), 0, &self.text, theme.menu.text.normal, false);
    }
}
