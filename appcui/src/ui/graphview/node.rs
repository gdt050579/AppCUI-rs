use super::GraphNode;
use crate::graphics::*;

// inactive - culoara data de mine
// selected - culoarea data de mine
// focus - culoarea data de utilizator (sau daca e None - culoarea sugerata de mine)
// normal - culoarea data de mine
// pentru margine si pentru text

pub struct Node<T: GraphNode> {
    pub(super) obj: T,
    pub(super) rect: Rect,
    pub(super) margin: Option<LineType>,
    pub(super) text_align: TextAlignment,
    pub(super) margin_attr: Option<CharAttribute>,
    pub(super) text_attr: Option<CharAttribute>,
}
impl<T> Node<T>
where
    T: GraphNode,
{
    pub(super) fn new(obj: T) -> Self {
        let mut sz = obj.prefered_size();
        sz.width += 2; // left-right padding
        Self {
            obj,
            rect: Rect::with_point_and_size(Point::ORIGIN, sz),
            margin: None,
            text_align: TextAlignment::Center,
            margin_attr: None,
            text_attr: None,
        }
    }
    #[inline]
    pub(super) fn contains(&self, x: i32, y: i32) -> bool {
        self.rect.contains(Point::new(x, y))
    }
    pub(super) fn paint(&self, surface: &mut Surface, attr: CharAttribute, out: &mut String) {
        let mut cx = self.rect.center_x();
        let cy = if self.margin.is_some() { 1 } else { 0 } + self.rect.top();
        let w = self.rect.width().saturating_sub(2) as u16;
        if (w > 0) && ((w & 1) == 0) {
            cx += 1;
        }
        let format = TextFormatBuilder::new()
            .align(self.text_align)
            .attribute(attr)
            .wrap_type(WrapType::WordWrap(w))
            .position(cx, cy)
            .build();
        let mut sz = self.rect.size();
        if self.margin.is_some() {
            sz = sz.reduce_by(2);
        }
        self.obj.write(out, sz);
        surface.write_text(&out, &format);
    }
}
