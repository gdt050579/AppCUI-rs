use crate::prelude::*;

pub enum RenderMethod<'a> {
    Text(&'a str),
    Custom,
}
impl RenderMethod<'_> {
    #[inline(always)]
    fn paint_text(txt: &str, surface: &mut Surface, theme: &Theme, alignment: TextAlignament, width: u16, attr: Option<CharAttribute>) {
        let format = TextFormat {
            x: match alignment {
                TextAlignament::Left => 0,
                TextAlignament::Center => (width as i32) / 2,
                TextAlignament::Right => (width as i32) - 1,
            },
            y: 0,
            width: Some(width),
            char_attr: attr.unwrap_or(theme.text.focused),
            hotkey_attr: None,
            hotkey_pos: None,
            chars_count: None,
            align: alignment,
            text_wrap: TextWrap::None,
            multi_line: false,
        };
        surface.write_text(txt, &format);
    }
    #[inline(always)]
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, alignment: TextAlignament, width: u16, attr: Option<CharAttribute>) -> bool {
        match self {
            RenderMethod::Text(txt) => {
                RenderMethod::paint_text(txt, surface, theme, alignment, width, attr);
                true
            }
            RenderMethod::Custom => false,
        }
    }
}
