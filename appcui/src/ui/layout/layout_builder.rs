use super::Coordonate;
use super::Dimension;
use super::Alignament;
use super::Layout;


pub struct LayoutBuilder {
    pub(super) x: Option<Coordonate>,
    pub(super) y: Option<Coordonate>,
    pub(super) width: Option<Dimension>,
    pub(super) height: Option<Dimension>,
    pub(super) alignament: Option<Alignament>,
    pub(super) dock: Option<Alignament>,
    pub(super) left_anchor: Option<Dimension>,
    pub(super) right_anchor: Option<Dimension>,
    pub(super) top_anchor: Option<Dimension>,
    pub(super) bottom_anchor: Option<Dimension>,
}

impl LayoutBuilder {
    pub fn new() -> Self {
        Self {
            x: None,
            y: None,
            width: None,
            height: None,
            alignament: None,
            dock: None,
            left_anchor: None,
            right_anchor: None,
            top_anchor: None,
            bottom_anchor: None,
        }
    }

    pub fn x(mut self, x: Coordonate) -> Self {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: Coordonate) -> Self {
        self.y = Some(y);
        self
    }

    pub fn width(mut self, width: Dimension) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: Dimension) -> Self {
        self.height = Some(height);
        self
    }

    pub fn alignament(mut self, alignament: Alignament) -> Self {
        self.alignament = Some(alignament);
        self
    }

    pub fn dock(mut self, dock: Alignament) -> Self {
        self.dock = Some(dock);
        self
    }

    pub fn left_anchor(mut self, left_anchor: Dimension) -> Self {
        self.left_anchor = Some(left_anchor);
        self
    }

    pub fn right_anchor(mut self, right_anchor: Dimension) -> Self {
        self.right_anchor = Some(right_anchor);
        self
    }

    pub fn top_anchor(mut self, top_anchor: Dimension) -> Self {
        self.top_anchor = Some(top_anchor);
        self
    }

    pub fn bottom_anchor(mut self, bottom_anchor: Dimension) -> Self {
        self.bottom_anchor = Some(bottom_anchor);
        self
    }

    pub fn build(self) -> Layout<'static> {
        //Layout::new(self)
        todo!()
    }
}