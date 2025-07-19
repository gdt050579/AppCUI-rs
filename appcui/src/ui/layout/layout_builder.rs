use crate::ui::Dock;

use super::Alignament;
use super::Coordonate16;
use super::Dimension16;
use super::Layout;
use super::LayoutParameters;

pub struct LayoutBuilder {
    pub(super) params: LayoutParameters,
}

impl LayoutBuilder {
    pub fn new() -> Self {
        Self {
            params: LayoutParameters::default(),
        }
    }

    pub fn x<T>(mut self, x: T) -> Self
    where
        Coordonate16: From<T>,
    {
        self.params.x = Some(x.into());
        self
    }

    pub fn y<T>(mut self, y: T) -> Self
    where
        Coordonate16: From<T>,
    {
        self.params.y = Some(y.into());
        self
    }

    pub fn width<T>(mut self, width: T) -> Self
    where
        Dimension16: From<T>,
    {
        self.params.width = Some(width.into());
        self
    }

    pub fn height<T>(mut self, height: T) -> Self
    where
        Dimension16: From<T>,
    {
        self.params.height = Some(height.into());
        self
    }

    pub fn alignament(mut self, alignament: Alignament) -> Self {
        self.params.align = Some(alignament);
        self
    }

    pub fn dock(mut self, dock: Dock) -> Self {
        self.params.dock = Some(dock);
        self
    }

    pub fn left_anchor<T>(mut self, left_anchor: T) -> Self
    where
        Coordonate16: From<T>,
    {
        self.params.a_left = Some(left_anchor.into());
        self
    }

    pub fn right_anchor<T>(mut self, right_anchor: T) -> Self
    where
        Coordonate16: From<T>,
    {
        self.params.a_right = Some(right_anchor.into());
        self
    }

    pub fn top_anchor<T>(mut self, top_anchor: T) -> Self
    where
        Coordonate16: From<T>,
    {
        self.params.a_top = Some(top_anchor.into());
        self
    }

    pub fn bottom_anchor<T>(mut self, bottom_anchor: T) -> Self
    where
        Coordonate16: From<T>,
    {
        self.params.a_bottom = Some(bottom_anchor.into());
        self
    }

    pub fn build(self) -> Layout<'static> {
        Layout::with_layout_params(self.params)
    }
}
