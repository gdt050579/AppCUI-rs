use super::events::*;
use super::ControlManager;
use super::Layout;
use super::StatusFlags;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;
use AppCUIProcMacro::AppCUIControl;

#[AppCUIControl(overwrite=OnPaint)]
pub struct CheckBox {
    caption: String,
    checked: bool,
}

impl CheckBox {
    pub(crate) fn new(caption: &str, layout: Layout, checked: bool) -> Self {
        let mut cb = CheckBox {
            base: ControlManager::new(
                layout,
                StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::Focusable,
            ),
            caption: String::from(caption),
            checked
        };
        cb.base.layout.min_width = 5;
        cb.base.layout.min_height = 1;
        cb
    }
}
impl OnPaint for CheckBox {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
/*
    CREATE_CONTROL_CONTEXT(this, Members, );

    const ColorPair colHK = Members->Flags & GATTR_ENABLE ? Members->Cfg->Text.HotKey : Members->Cfg->Text.Inactive;
    ColorPair colTxt;
    if (!this->IsEnabled())
        colTxt = Members->Cfg->Text.Inactive;
    else if (Members->Focused)
        colTxt = Members->Cfg->Text.Focused;
    else if (Members->MouseIsOver)
        colTxt = Members->Cfg->Text.Hovered;
    else
        colTxt = Members->Cfg->Text.Normal;


    renderer.WriteSingleLineText(0, 0, "[ ] ", colTxt);

    WriteTextParams params(WriteTextFlags::OverwriteColors | WriteTextFlags::HighlightHotKey);
    params.HotKeyPosition = Members->HotKeyOffset;
    params.X              = 4;
    params.Y              = 0;
    if (Members->Layout.Height == 1)
    {
        params.Color       = colTxt;
        params.HotKeyColor = colHK;
        params.Flags |= WriteTextFlags::SingleLine;
    }
    else
    {
        params.Color       = colTxt;
        params.HotKeyColor = colHK;
        params.Flags |= WriteTextFlags::MultipleLines | WriteTextFlags::WrapToWidth;
        params.Width = Members->Layout.Width - 4; // without the '[ ] ' characters
    }
    renderer.WriteText(Members->Text, params);

    if (IsChecked())
    {
        const auto col = (Members->Flags & GATTR_ENABLE) ? Members->Cfg->Symbol.Checked : Members->Cfg->Symbol.Inactive;
        renderer.WriteSpecialCharacter(1, 0, SpecialChars::CheckMark, col);
    }
    if (Members->Focused)
        renderer.SetCursor(1, 0);
*/
    }
}

/*
namespace AppCUI::Controls
{
CheckBox::CheckBox(const ConstString& caption, string_view layout, int controlID)
    : Control(new ControlContext(), caption, layout, true)
{
    auto Members              = reinterpret_cast<ControlContext*>(this->Context);
    Members->Layout.MinWidth  = 5;
    Members->Layout.MinHeight = 1;
    Members->Flags            = GATTR_ENABLE | GATTR_VISIBLE | GATTR_TABSTOP;
    this->SetControlID(controlID);
}

void CheckBox::Paint(Graphics::Renderer& renderer)
{

}
void CheckBox::OnHotKey()
{
    SetChecked(!IsChecked());
    CREATE_CONTROL_CONTEXT(this, Members, );
    if (Members->handlers)
    {
        auto ch = this->Handlers();
        if (ch->OnCheck.obj)
        {
            ch->OnCheck.obj->OnCheck(this, IsChecked());
            return;
        }
    }
    RaiseEvent(Event::CheckedStatusChanged);
}
bool CheckBox::OnKeyEvent(Input::Key KeyCode, char16)
{
    if ((KeyCode == Key::Space) || (KeyCode == Key::Enter))
    {
        OnHotKey();
        return true;
    }
    return false;
}
void CheckBox::OnMouseReleased(int x, int y, Input::MouseButton)
{
    if (IsMouseInControl(x, y))
        OnHotKey();
}
bool CheckBox::OnMouseEnter()
{
    CREATE_CONTROL_CONTEXT(this, Members, false);
    if ((int) Members->Text.Len() >= Members->Layout.Width)
        this->ShowToolTip(Members->Text);
    return true;
}
bool CheckBox::OnMouseLeave()
{
    return true;
}
// handlers covariant
Handlers::CheckState* CheckBox::Handlers()
{
    GET_CONTROL_HANDLERS(Handlers::CheckState);
}



*/
