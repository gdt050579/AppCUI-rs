use super::Alignament;
use super::Coordonate;
use super::Dimension;
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

    pub fn x<T>(mut self, x: T) -> Self
    where
        Coordonate: From<T>,
    {
        self.x = Some(x.into());
        self
    }

    pub fn y<T>(mut self, y: T) -> Self
    where
        Coordonate: From<T>,
    {
        self.y = Some(y.into());
        self
    }

    pub fn width<T>(mut self, width: T) -> Self
    where
        Dimension: From<T>,
    {
        self.width = Some(width.into());
        self
    }

    pub fn height<T>(mut self, height: T) -> Self
    where
        Dimension: From<T>,
    {
        self.height = Some(height.into());
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

    pub fn left_anchor<T>(mut self, left_anchor: T) -> Self
    where
        Dimension: From<T>,
    {
        self.left_anchor = Some(left_anchor.into());
        self
    }

    pub fn right_anchor<T>(mut self, right_anchor: T) -> Self
    where
        Dimension: From<T>,
    {
        self.right_anchor = Some(right_anchor.into());
        self
    }

    pub fn top_anchor<T>(mut self, top_anchor: T) -> Self
    where
        Dimension: From<T>,
    {
        self.top_anchor = Some(top_anchor.into());
        self
    }

    pub fn bottom_anchor<T>(mut self, bottom_anchor: T) -> Self
    where
        Dimension: From<T>,
    {
        self.bottom_anchor = Some(bottom_anchor.into());
        self
    }

    pub fn build(self) -> Layout<'static> {
        //Layout::new(self)
        todo!()
    }
}
