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
    /// Multi-selection membership when multi-select mode is enabled on the owning `GraphView`.
    pub(super) selected: bool,
}
impl<T> Node<T>
where
    T: GraphNode,
{
    /// Returns a reference to the user data (`T`) stored in this node.
    pub fn value(&self) -> &T {
        &self.obj
    }

    /// Whether this node is included in the multi-selection set (only meaningful when multi-select UI is enabled).
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// Label size derived from the current bounds (inverse of [`Node::resize`](Self::resize) padding rules).
    ///
    /// `multiselect_ui` must match how this node’s `rect` was last laid out (same flag passed to [`Node::resize`](Self::resize)).
    pub(super) fn label_content_size(&self, multiselect_ui: bool) -> Size {
        let gutter = if multiselect_ui { 2u32 } else { 0u32 };
        let mut w = self.rect.width();
        let mut h = self.rect.height();
        if self.border.is_some() {
            w = w.saturating_sub(2);
            h = h.saturating_sub(2);
            // Bordered + multi-select: first inner row is reserved for ☑/☐ (see `resize` / `paint`).
            if multiselect_ui {
                h = h.saturating_sub(1);
            }
        } else {
            w = w.saturating_sub(2);
        }
        w = w.saturating_sub(gutter);
        Size::new(w, h)
    }

    pub(super) fn resize(&mut self, mut size: Size, multiselect_ui: bool) {
        let p = self.rect.top_left();
        let gutter = if multiselect_ui { 2u32 } else { 0u32 };
        if self.border.is_some() {
            // First inner row for checkbox when multi-select is on (glyph on first row of frame).
            if multiselect_ui {
                size.height += 1;
            }
            // 2 characters (left/right for the border)
            // 2 characters (top/bottom for the border)
            size.width += 2;
            size.height += 2;
        } else {
            // one extra space on left and right
            size.width += 2;
        }
        size.width += gutter;
        self.rect = Rect::with_point_and_size(p, size);
    }

    #[inline]
    pub(super) fn contains(&self, x: i32, y: i32) -> bool {
        self.rect.contains(Point::new(x, y))
    }
    pub(super) fn paint(&self, surface: &mut Surface, attr: CharAttribute, out: &mut String, multiselect_ui: bool) {
        surface.fill_rect(self.rect, Character::with_attributes(' ', attr));
        if !multiselect_ui {
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
            surface.write_text(out, &format);
            surface.reset_clip();
            return;
        }

        const G: i32 = 2;
        let mark = if self.selected { '☑' } else { '☐' };
        if let Some(line_type) = self.border {
            surface.draw_rect(self.rect, line_type, attr);
            let il = self.rect.left() + 1;
            let it = self.rect.top() + 1;
            let ir = self.rect.right() - 1;
            let ib = self.rect.bottom() - 1;
            // Top-left of inner frame, first row (spec: bordered — glyph on first row inside border).
            surface.write_char(il, it, Character::with_attributes(mark, attr));
            let clip_left = il + G;
            let text_top = it + 1;
            surface.set_relative_clip(clip_left, text_top, ir, ib);
            let inner_w = (ir - clip_left + 1) as u32;
            let mut cx = clip_left + (inner_w as i32 / 2);
            if (inner_w > 0) && ((inner_w & 1) == 0) {
                cx += 1;
            }
            let cy = text_top;
            let w = inner_w as u16;
            let format = TextFormatBuilder::new()
                .align(self.text_align)
                .attribute(attr)
                .wrap_type(WrapType::WordWrap(w))
                .position(cx, cy)
                .build();
            let mut sz = self.rect.size().reduce_by(2);
            sz.width = sz.width.saturating_sub(2);
            sz.height = sz.height.saturating_sub(1);
            if self.obj.write_label(out, sz).is_err() {
                out.clear();
                out.push_str("???");
            }
            surface.write_text(out, &format);
        } else {
            surface.write_char(
                self.rect.left() + 1,
                self.rect.top(),
                Character::with_attributes(mark, attr),
            );
            let clip_left = self.rect.left() + 1 + G;
            surface.set_relative_clip(clip_left, self.rect.top(), self.rect.right(), self.rect.bottom());
            let inner_w = self.rect.width().saturating_sub(2 + 2);
            let mut cx = clip_left + (inner_w as i32 / 2);
            if (inner_w > 0) && ((inner_w & 1) == 0) {
                cx += 1;
            }
            let cy = self.rect.top();
            let w = inner_w as u16;
            let format = TextFormatBuilder::new()
                .align(self.text_align)
                .attribute(attr)
                .wrap_type(WrapType::WordWrap(w))
                .position(cx, cy)
                .build();
            let mut sz = self.rect.size();
            sz.width = sz.width.saturating_sub(2 + 2);
            if self.obj.write_label(out, sz).is_err() {
                out.clear();
                out.push_str("???");
            }
            surface.write_text(out, &format);
        }
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
                selected: false,
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

    /// Builds the actual node. If the size was not set, the prefered size as it is returned by the GraphNode implementation will be used
    #[inline(always)]
    pub fn build(mut self) -> Node<T> {
        if let Some(size) = self.size {
            self.node.resize(size, false);
        } else {
            let sz = self.node.obj.prefered_size();
            self.node.resize(sz, false);
        }
        self.node
    }
}

pub struct EditableNode<'a, T: GraphNode + 'a> {
    node: &'a mut Node<T>,
    changed: &'a mut bool,
}
impl<'a, T> EditableNode<'a, T>
where
    T: GraphNode + 'a,
{
    pub(super) fn new(node: &'a mut Node<T>, changed: &'a mut bool) -> Self {
        Self { node, changed }
    }
    /// Returns an immutable reference to the node's user data.
    #[inline(always)]
    pub fn value(&self) -> &T {
        &self.node.obj
    }
    /// Returns a mutable reference to the node's user data.
    #[inline(always)]
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.node.obj
    }
    /// Replaces the node's user data and marks the graph as changed.
    #[inline(always)]
    pub fn set_value(&mut self, value: T) {
        self.node.obj = value;
        *self.changed = true;
    }
    /// Returns the node's bounding rectangle in graph coordinates.
    #[inline(always)]
    pub fn bounds(&self) -> Rect {
        self.node.rect
    }
    /// Sets the node's bounding rectangle. No-op if unchanged; otherwise marks the graph as changed.
    #[inline(always)]
    pub fn set_bounds(&mut self, r: Rect) {
        if self.node.rect != r {
            self.node.rect = r;
            *self.changed = true;
        }
    }
    /// Returns the top-left corner of the node's bounds.
    #[inline(always)]
    pub fn position(&self) -> Point {
        self.node.rect.top_left()
    }
    /// Moves the node so its top-left is `p`, preserving size.
    #[inline(always)]
    pub fn set_position(&mut self, p: Point) {
        self.set_bounds(Rect::with_point_and_size(p, self.node.rect.size()));
    }
    /// Returns the width and height of the node's bounds.
    #[inline(always)]
    pub fn size(&self) -> Size {
        self.node.rect.size()
    }
    /// Resizes the node, keeping its current top-left corner.
    #[inline(always)]
    pub fn set_size(&mut self, size: Size) {
        self.set_bounds(Rect::with_point_and_size(self.node.rect.top_left(), size));
    }
    /// Returns how the label text is aligned inside the node.
    #[inline(always)]
    pub fn text_alignment(&self) -> TextAlignment {
        self.node.text_align
    }
    /// Sets label text alignment. Marks the graph as changed if the value differs.
    #[inline(always)]
    pub fn set_text_alignment(&mut self, align: TextAlignment) {
        if self.node.text_align != align {
            self.node.text_align = align;
            *self.changed = true;
        }
    }
    /// Returns the optional custom text (and border) attribute when the control is focused.
    #[inline(always)]
    pub fn text_attribute(&self) -> Option<CharAttribute> {
        self.node.text_attr
    }
    /// Sets a custom text attribute. Marks the graph as changed if the value differs.
    #[inline(always)]
    pub fn set_text_attribute(&mut self, attr: CharAttribute) {
        if self.node.text_attr != Some(attr) {
            self.node.text_attr = Some(attr);
            *self.changed = true;
        }
    }
    /// Clears a previously set text attribute so theme defaults apply.
    #[inline(always)]
    pub fn clear_text_attribute(&mut self) {
        if self.node.text_attr.is_some() {
            self.node.text_attr = None;
            *self.changed = true;
        }
    }
    /// Returns the border line style, if any.
    #[inline(always)]
    pub fn border(&self) -> Option<LineType> {
        self.node.border
    }
    /// Sets the border style. Ensures minimum height when enabling a border. Marks the graph as changed if needed.
    #[inline(always)]
    pub fn set_border(&mut self, border: LineType) {
        if self.node.border != Some(border) {
            self.node.border = Some(border);
            if self.node.rect.height() < 3 {
                self.node.rect.set_bottom(self.node.rect.top() + 2, false);
            }            
            *self.changed = true;
        }
    }
    /// Removes the node's border. Marks the graph as changed if a border was present.
    #[inline(always)]
    pub fn clear_border(&mut self) {
        if self.node.border.is_some() {
            self.node.border = None;
            *self.changed = true;
        }
    }

    /// Sets the node's selected state. Marks the graph as changed if the value differs.
    #[inline(always)]
    pub fn set_selected(&mut self, selected: bool) {
        if self.node.selected != selected {
            self.node.selected = selected;
            *self.changed = true;
        }
    }
    /// Returns whether the node is selected.
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        self.node.selected
    }
}
