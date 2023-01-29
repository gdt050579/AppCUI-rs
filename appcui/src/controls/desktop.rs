use super::events::*;
use super::ControlManager;
use super::Layout;
use super::StatusFlags;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;
use AppCUIProcMacro::AppCUIControl;

#[AppCUIControl(overwrite=OnPaint)]
pub struct Desktop {}

impl Desktop {
    pub(crate) fn new() -> Self {
        Desktop {
            base: ControlManager::new(
                Layout::new("x:0,y:0,w:100%,h:100%"),
                StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput,
            ),
        }
    }
}
impl OnPaint for Desktop {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.clear(theme.desktop.character);
        surface.write_string(
            0,
            0,
            "Desktop",
            CharAttribute::with_color(Color::White, Color::Red),
            false,
        );
    }
}
