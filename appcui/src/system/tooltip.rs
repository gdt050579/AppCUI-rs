use crate::graphics::{Character, Point, Rect, SpecialChar, Surface, TextFormat, TextWrap};

use super::Theme;

pub(crate) struct ToolTip {
    visible: bool,
    text_pos: Point,
    arrow_pos: Point,
    arrow_char: SpecialChar,
    format: TextFormat,
    canvas: Surface,
}
impl ToolTip {
    pub(crate) fn new() -> Self {
        ToolTip {
            visible: false,
            text_pos: Point::default(),
            arrow_pos: Point::default(),
            arrow_char: SpecialChar::ArrowDown,
            format: TextFormat::default(),
            canvas: Surface::new(16, 16),
        }
    }
    #[inline(always)]
    pub(crate) fn is_visible(&self) -> bool {
        self.visible
    }
    pub(crate) fn show(
        &mut self,
        text: &str,
        object_rect: &Rect,
        screen_width: u32,
        screen_height: u32,
        theme: &Theme,
    ) -> bool {
        self.visible = false;

        let mut nr_lines = 0u32;
        let max_width = screen_width / 2;
        let mut w = 0u32;
        let mut best_width = 0u32;
        let mut chars_count = 0usize;
        for c in text.chars() {
            chars_count += 1;
            if (c == '\n') || (c == '\r') {
                best_width = best_width.max(w);
                w = 0;
                nr_lines += 1;
                continue;
            }
            w += 1;
            if w > max_width {
                best_width = max_width;
                w = 0;
                nr_lines += 1;
            }
        }
        if w > 0 {
            best_width = best_width.max(w);
            nr_lines += 1;
        }
        nr_lines = nr_lines.min(screen_height / 4).max(1);
        best_width = best_width.max(5) + 2;

        // find best position  (prefer on-top)
        if object_rect.get_top() >= ((nr_lines + 1) as i32) {
            let cx = object_rect.get_x_center();
            let mut x = cx - ((best_width / 2) as i32);
            let top = object_rect.get_top();
            let best_x = x;
            x = x.min((screen_width as i32) - (best_width as i32)).max(0);
            self.arrow_pos = Point::new(((best_width / 2) as i32) + (best_x - x), top-1);
            self.arrow_char = SpecialChar::ArrowDown;
            self.text_pos = Point::new(x, top - ((nr_lines + 1) as i32));
            self.format.multi_line = nr_lines > 1;
            self.format.width = Some((best_width - 2) as u16);
            self.format.x = 1;
            self.format.y = 0;
            self.format.chars_count = Some(chars_count as u16);
            self.format.char_attr = theme.tooltip.text;
            self.format.text_wrap = TextWrap::Word;
            self.canvas.resize(best_width, nr_lines);
            self.canvas.clear(Character::with_attributes(' ', theme.tooltip.text));
            self.canvas.write_text(text, &self.format);
            self.visible = true;
            return true;
        }
        return false;
    }
    pub(crate) fn hide(&mut self) {
        self.visible = false;
    }
    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        if !self.visible {
            return;
        }
        surface.draw_surface(self.text_pos.x, self.text_pos.y, &self.canvas);
        surface.set(
            self.arrow_pos.x,
            self.arrow_pos.y,
            Character::with_attributes(self.arrow_char, theme.tooltip.arrow),
        );
    }
}

/*

bool ToolTipController::Show(const ConstString& text, Graphics::Rect& objRect, int screenWidth, int screenHeight)
{

    // check bottom position
    if (objRect.GetBottom() + (nrLines + 1) <= screenHeight)
    {
        const int cx = objRect.GetCenterX();
        int x        = cx - bestWidth / 2;
        auto bestX   = x;
        x            = std::min<>(x, screenWidth - bestWidth);
        x            = std::max<>(x, 0);
        ScreenClip.Set(x, objRect.GetBottom() + 1, bestWidth, nrLines + 1);
        TextRect.Create(0, 1, bestWidth, nrLines, Alignament::TopLeft);
        Arrow.Set(bestWidth / 2 + (bestX - x), 0);
        TxParams.X     = 1;
        TxParams.Y     = 1;
        TxParams.Color = Cfg->ToolTip.Text;
        TxParams.Width = bestWidth - 2;
        ArrowChar      = SpecialChars::ArrowUp;

        Visible = true;
        return true;
    }
    // no solution --> ToolTip will not be shown
    return false;
}
} // namespace AppCUI::Internal


*/
