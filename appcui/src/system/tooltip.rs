pub (crate) struct ToolTip
{
    visible: bool
}
impl ToolTip {
    pub (crate) fn new()->Self {
        ToolTip {
            visible: false
        }
    }
    #[inline(always)]
    pub (crate) fn is_visible(&self) {
        self.visible
    }
    pub (crate) fn show(&mut self, text: &str, object_rect: &Rect, width: u32, height: u32) {
        todo!();
    }
    pub (crate) fn hide(&mut self) {
        self.visible = false;
    }
    pub (crate) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        if !self.visible { return; }
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