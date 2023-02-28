use std::default;

use crate::graphics::{Point, Rect, SpecialChar, Surface, TextFormat};

use super::Theme;

pub(crate) struct ToolTip {
    visible: bool,
    arrow_pos: Point,
    arrow_char: SpecialChar,
    format: TextFormat,
    canvas: Surface,
}
impl ToolTip {
    pub(crate) fn new() -> Self {
        ToolTip {
            visible: false,
            arrow_pos: Point::default(),
            arrow_char: SpecialChar::ArrowDown,
            format: TextFormat::default(),
            canvas: Surface::new(16,16),
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
            let best_x = x;
            x = x.min((screen_width as i32) - (best_width as i32)).max(0);
            self.arrow_pos = Point::new(((best_width / 2) as i32) + (best_x - x), nr_lines as i32);
            self.arrow_char = SpecialChar::ArrowDown;
            self.format.multi_line = nr_lines > 1;
            self.format.width = Some((best_width - 2) as u16);
            self.format.x = x + 1;
            self.format.y = object_rect.get_top() - ((nr_lines + 1) as i32);
            self.format.chars_count = Some(chars_count as u16);
            self.canvas.resize(best_width, nr_lines);
            self.visible = true;
            return true;
        }
        /*
        if (objRect.GetTop() >= (nrLines + 1))
        {
            [DONE]    const int cx = objRect.GetCenterX();
            [DONE]    int x        = cx - bestWidth / 2;
            [DONE]    auto bestX   = x;
            [DONE]    x            = std::min<>(x, screenWidth - bestWidth);
            [DONE]    x            = std::max<>(x, 0);

            ScreenClip.Set(x, objRect.GetTop() - (nrLines + 1), bestWidth, nrLines + 1);
            TextRect.Create(0, 0, bestWidth, nrLines, Alignament::TopLeft);
            [DONE]    Arrow.Set(bestWidth / 2 + (bestX - x), nrLines);
            [DONE]    TxParams.X     = 1;
            [DONE]    TxParams.Y     = 0;
            TxParams.Color = Cfg->ToolTip.Text;
            [DONE]    TxParams.Width = bestWidth - 2;
            [DONE]    ArrowChar      = SpecialChars::ArrowDown;

            [DONE]    Visible = true;
            [DONE]    return true;
        }
        */
        // no solution --> ToolTip will not be shown
        return false;
    }
    pub(crate) fn hide(&mut self) {
        self.visible = false;
    }
    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        if !self.visible {
            return;
        }
        todo!();
    }
}

/*
    class ToolTipController
    {
        Graphics::CharacterBuffer Text;
        Application::Config* Cfg;
        Graphics::Rect TextRect;
        Graphics::Point Arrow;
        Graphics::SpecialChars ArrowChar;
        Graphics::WriteTextParams TxParams;

      public:
        Graphics::Clip ScreenClip;

        bool Visible;

      public:
        ToolTipController();
        bool Show(const ConstString& text, Graphics::Rect& objRect, int screenWidth, int screenHeight);
        void Hide();
        void Paint(Graphics::Renderer& renderer);
    };


namespace AppCUI::Internal
{
using namespace Graphics;


bool ToolTipController::Show(const ConstString& text, Graphics::Rect& objRect, int screenWidth, int screenHeight)
{
    Visible = false;
    // update Cfg
    if (!this->Cfg)
        this->Cfg = Application::GetAppConfig();
    CHECK(Text.Set(text), false, "Fail to copy text");
    // compute best size
    auto p        = Text.GetBuffer();
    auto e        = p + Text.Len();
    int nrLines   = 0;
    int maxWidth  = screenWidth / 2;
    int w         = 0;
    int bestWidth = 0;
    while (p < e)
    {
        if (p->Code == NEW_LINE_CODE)
        {
            bestWidth = std::max<>(bestWidth, w);
            p++;
            w = 0;
            if (p < e)
                nrLines++;
            continue;
        }
        p++;
        w++;
        if (w >= maxWidth)
        {
            bestWidth = maxWidth;
            w         = 0;
            if (p < e)
                nrLines++;
        }
    }
    if (w > 0)
    {
        bestWidth = std::max<>(bestWidth, w);
        nrLines++;
    }
    // max number of lines must not be bigger than 25% of the height
    nrLines   = std::min<>(nrLines, screenHeight / 4);
    nrLines   = std::max<>(nrLines, 1);   // minimum one line  (sanity check)
    bestWidth = std::max<>(bestWidth, 5); // minimum 5 chars width (sanity check)
    bestWidth += 2;                       // one character padding (left & right)

    // set TextParams
    if (nrLines == 1)
        TxParams.Flags = WriteTextFlags::OverwriteColors | WriteTextFlags::SingleLine | WriteTextFlags::ClipToWidth;
    else
        TxParams.Flags = WriteTextFlags::OverwriteColors | WriteTextFlags::MultipleLines | WriteTextFlags::WrapToWidth;

    // find best position  (prefer on-top)
    if (objRect.GetTop() >= (nrLines + 1))
    {
        const int cx = objRect.GetCenterX();
        int x        = cx - bestWidth / 2;
        auto bestX   = x;
        x            = std::min<>(x, screenWidth - bestWidth);
        x            = std::max<>(x, 0);
        ScreenClip.Set(x, objRect.GetTop() - (nrLines + 1), bestWidth, nrLines + 1);
        TextRect.Create(0, 0, bestWidth, nrLines, Alignament::TopLeft);
        Arrow.Set(bestWidth / 2 + (bestX - x), nrLines);
        TxParams.X     = 1;
        TxParams.Y     = 0;
        TxParams.Color = Cfg->ToolTip.Text;
        TxParams.Width = bestWidth - 2;
        ArrowChar      = SpecialChars::ArrowDown;

        Visible = true;
        return true;
    }
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

void ToolTipController::Paint(Graphics::Renderer& renderer)
{
    if (!Visible)
        return;

    renderer.FillRect(
          TextRect.GetLeft(), TextRect.GetTop(), TextRect.GetRight(), TextRect.GetBottom(), ' ', Cfg->ToolTip.Text);
    renderer.WriteSpecialCharacter(Arrow.X, Arrow.Y, ArrowChar, Cfg->ToolTip.Arrow);
    renderer.SetClipMargins(
          TextRect.GetLeft(),
          TextRect.GetTop(),
          ScreenClip.ClipRect.Width - (TextRect.GetRight() + 1),
          ScreenClip.ClipRect.Height - (TextRect.GetBottom() + 1));
    renderer.WriteText(this->Text, TxParams);
}
} // namespace AppCUI::Internal


*/
