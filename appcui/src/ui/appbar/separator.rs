use super::{ItemBase, Side};
use crate::graphics::*;
use crate::system::Theme;

/// A separator in the app bar.
/// 
/// # Examples
/// 
/// ```rust, no_run
/// use appcui::prelude::*;
/// 
/// let separator = appbar::Separator::new(0, appbar::Side::Left);
/// ```
pub struct Separator {
    pub(super) base: ItemBase,
}

impl Separator {
    /// Creates a new separator with the specified order and position.
    /// 
    /// # Parameters
    /// 
    /// * `order` - The order of the separator (a number that determines the order of the separator in the app bar - lower numbers are displayed first from either **left** or **right** depending on the **pos** parameter)
    /// * `pos` - The position of the separator (`Left` or `Right`)
    /// 
    /// # Example
    /// 
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let separator = appbar::Separator::new(0, appbar::Side::Left);
    /// ```
    pub fn new(order: u8, pos: Side) -> Self {
        Self {
            base: ItemBase::new(1, order, pos, false),
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.write_char(self.base.x(), 0, Character::with_attributes('|', theme.menu.text.inactive));
    }
}
