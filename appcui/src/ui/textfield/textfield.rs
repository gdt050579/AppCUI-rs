use super::Flags;
use crate::prelude::*;
use crate::ui::textfield::events::EventData;
use crate::utils::Glyphs;

struct Cursor {
    pos: usize,
    start: usize,
    end: usize,
}
struct Selection {
    start: usize,
    end: usize,
    origin: usize,
}
impl Selection {
    const NONE: Selection = Selection {
        start: usize::MAX,
        end: usize::MAX,
        origin: usize::MAX,
    };
    #[inline(always)]
    fn has_selection(&self) -> bool {
        self.origin != usize::MAX
    }
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct TextField {
    cursor: Cursor,
    selection: Selection,
    glyphs: Glyphs,
    flags: Flags,
}
impl TextField {
    pub fn new(text: &str, layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            cursor: Cursor { pos: 0, start: 0, end: 0 },
            selection: Selection::NONE,
            glyphs: Glyphs::from(text),
            flags,
        }
    }
}
impl OnPaint for TextField {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let attr = match () {
            _ if !self.is_enabled() => theme.editor.inactive,
            _ if self.has_focus() => theme.editor.focused,
            _ if self.is_mouse_over() => theme.editor.hovered,
            _ => theme.editor.normal,
        };
        surface.clear(Character::with_attributes(' ', attr));
        // paint
        let sz = self.size();
        let w = (sz.width - 1) as i32;
        let mut count = (sz.width - 2) * sz.height;
        let mut pos = self.cursor.start;
        let mut x = 1;
        let mut y = 0;
        let mut ch = Character::with_attributes(' ', attr);
        while let Some((code, glyph_size)) = self.glyphs.character(pos) {
            ch.code = code;
            surface.write_char(x, y, ch);
            x += 1;
            if x >= w {
                x = 0;
                y += 1;
            }
            pos += glyph_size as usize;
            count -= 1;
            if count==0 { break; }
        }
    }
}
impl OnKeyPressed for TextField {}
impl OnMouseEvent for TextField {}
