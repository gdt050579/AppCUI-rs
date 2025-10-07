use super::ResizeMoveStatus;
use crate::graphics::*;
use crate::system::*;

#[derive(Default)]
pub(super) struct Border {
    size: Size,
}
impl Border {
    pub(super) fn new() -> Self {
        Self { size: Size::new(0, 0) }
    }
    pub(super) fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, status: ResizeMoveStatus, has_focus: bool) {
        let (color_border, line_type) = if has_focus {
            if status == ResizeMoveStatus::None {
                (theme.border.focused, LineType::Double)
            } else {
                (theme.border.pressed_or_selectd, LineType::Single)
            }
        } else {
            (theme.border.normal, LineType::Single)
        };
        surface.draw_rect(
            Rect::with_size(0, 0, self.size.width as u16, self.size.height as u16),
            line_type,
            color_border,
        );
    }
}
