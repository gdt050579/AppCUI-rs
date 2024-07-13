use crate::prelude::*;

pub enum RenderMethod<'a> {
    Text(&'a str),
    Custom,
}
impl RenderMethod<'_> {
    #[inline(always)]
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, alignment: TextAlignament)->bool {
        match self {
            RenderMethod::Text(txt) => {
                //surface.write_text(0, 0, txt, theme.text);
                true
            }
            RenderMethod::Custom => false
        }   
    }
}