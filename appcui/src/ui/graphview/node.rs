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
    pub(super) border: Option<LineType>,
    pub(super) text_align: TextAlignment,
    pub(super) text_attr: Option<CharAttribute>,
    pub(super) edges_in: Vec<u32>,
    pub(super) edges_out: Vec<u32>,
    pub(super) filtered: bool,
}
impl<T> Node<T>
where
    T: GraphNode,
{
    fn resize(&mut self, mut size: Size) {
        let p = self.rect.top_left();
        if self.border.is_some() {
            // 2 characters (left/right for the border)
            // 2 characters (top/bottom for the border)
            size.width += 2;
            size.height += 2;
        } else {
            // one extra space on left and right
            size.width += 2;
        }
        self.rect = Rect::with_point_and_size(p, size);
    }

    #[inline]
    pub(super) fn contains(&self, x: i32, y: i32) -> bool {
        self.rect.contains(Point::new(x, y))
    }
    pub(super) fn paint(&self, surface: &mut Surface, attr: CharAttribute, out: &mut String) {
        surface.fill_rect(self.rect, Character::with_attributes(' ', attr));
        if let Some(line_type) = self.border {
            surface.draw_rect(self.rect, line_type, attr);
            surface.set_relative_clip(self.rect.left() + 1, self.rect.top() + 1, self.rect.right() - 1, self.rect.bottom() - 1);
        } else {
            surface.set_relative_clip(self.rect.left(), self.rect.top(), self.rect.right(), self.rect.bottom());
        }
        let mut cx = self.rect.center_x();
        let cy = if self.border.is_some() { 1 } else { 0 } + self.rect.top();
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
        if self.border.is_some() {
            sz = sz.reduce_by(2);
        }
        if self.obj.write_label(out, sz).is_err() {
            out.clear();
            out.push_str("???");
        }
        surface.write_text(&out, &format);
        surface.reset_clip();
    }
}

pub struct NodeBuilder<T>
where
    T: GraphNode,
{
    node: Node<T>,
    size: Option<Size>,
}
impl<T> NodeBuilder<T>
where
    T: GraphNode,
{
    /// Create a new NodeBuilder
    pub fn new(obj: T) -> Self {
        Self {
            node: Node {
                obj,
                rect: Rect::new(0, 0, 0, 0),
                border: None,
                text_align: TextAlignment::Center,
                text_attr: None,
                edges_in: Vec::new(),
                edges_out: Vec::new(),
                filtered: false,
            },
            size: None,
        }
    }
    /// Set a border for the node
    pub fn border(mut self, line_type: LineType) -> Self {
        self.node.border = Some(line_type);
        self
    }
    /// Set the text attribute for the node label and border
    /// If not set, the default attribute will be used
    /// These attributes will only be used if the graphview control is focused and the node is not filtered
    pub fn text_attribute(mut self, attr: CharAttribute) -> Self {
        self.node.text_attr = Some(attr);
        self
    }

    /// Set the text alignment for the node label (default is center)
    pub fn text_alignment(mut self, align: TextAlignment) -> Self {
        self.node.text_align = align;
        self
    }

    /// Set the size of the node (if not set, the prefered size as it is returned by the GraphNode implementation will be used)
    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    /// Set the position of the node (top-left corner)
    pub fn position(mut self, p: Point) -> Self {
        self.node.rect.set_left(p.x, true);
        self.node.rect.set_top(p.y, true);
        self
    }

    /// Builds the actual node
    #[inline(always)]
    pub fn build(mut self) -> Node<T> {
        if let Some(size) = self.size {
            self.node.resize(size);
        } else {
            let sz = self.node.obj.prefered_size();
            self.node.resize(sz);
        }
        self.node
    }
}
