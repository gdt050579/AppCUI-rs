use super::ResizeMoveStatus;
use super::Type;
use crate::graphics::*;
use crate::prelude::*;

#[derive(Default)]
pub(super) struct Border {
    size: Size,
    wtype: Type,
}
impl Border {
    pub(super) fn new(window_type: Type) -> Self {
        Self {
            size: Size::new(0, 0),
            wtype: window_type,
        }
    }
    pub(super) fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    #[inline(always)]
    fn paint_classical(&self, surface: &mut Surface, theme: &Theme, status: ResizeMoveStatus, has_focus: bool) {
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
    #[inline(always)]
    fn paint_round(&self, surface: &mut Surface, theme: &Theme, status: ResizeMoveStatus, has_focus: bool) {
        let color_border = if has_focus {
            if status == ResizeMoveStatus::None {
                theme.border.focused
            } else {
                theme.border.pressed_or_selectd
            }
        } else {
            theme.border.normal
        };
        surface.draw_rect(
            Rect::with_size(0, 0, self.size.width as u16, self.size.height as u16),
            LineType::SingleRound,
            color_border,
        );
    }
    #[inline(always)]
    fn paint_panel(&self, surface: &mut Surface, theme: &Theme, status: ResizeMoveStatus, has_focus: bool) {
        surface.fill_horizontal_line(0, 0, self.size.width as i32, char!("' ',black,white"));
    }    
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, status: ResizeMoveStatus, has_focus: bool) {
        match self.wtype {
            Type::Normal => self.paint_classical(surface, theme, status, has_focus),
            Type::Round => self.paint_round(surface, theme, status, has_focus),
            Type::Panel => self.paint_panel(surface, theme, status, has_focus),
        }
    }
}
