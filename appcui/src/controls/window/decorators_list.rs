use super::{DecoratorPaintData, Decorator};
use crate::{utils::VectorIndex, graphics::Surface, system::Theme};

pub(super) struct DecoratorsList {
    items: Vec<Decorator>,
    current: VectorIndex,
    pressed: bool,
}

impl DecoratorsList {
    pub(super) fn new() -> Self {
        Self {
            items: Vec::with_capacity(4),
            current: VectorIndex::invalid(),
            pressed: false,
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, focused: bool, maximized: bool) {
        let mut paint_data = DecoratorPaintData {
            focused,
            current: false,
            maximized,
            is_current_item_pressed: self.pressed,
            sep_attr: if focused { theme.lines.normal } else { theme.lines.inactive },
        };
        let current_bar_index = self.current.index();
        // paint bar items
        for (index, item) in self.items.iter().enumerate() {
            paint_data.current = index == current_bar_index;
            item.paint(surface, theme, &paint_data);
        }
    }
}
