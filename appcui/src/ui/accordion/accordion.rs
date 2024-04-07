use crate::prelude::*;
use crate::ui::tab::Flags;

#[CustomControl(overwrite=OnPaint+OnMouseEvent+OnKeyPressed, internal=true)]
pub struct Accordion {
    flags: Flags,
    pages: Vec<Caption>,
    hovered_page_idx: Option<usize>,
}
impl OnPaint for Accordion {

}
impl OnMouseEvent for Accordion {

}
impl OnKeyPressed for Accordion {
    
}