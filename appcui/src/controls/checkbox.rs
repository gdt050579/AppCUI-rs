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
                StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput,
            ),
            caption: String::from(caption),
            checked,
        };
        cb.set_size_bounds(5, 1, u16::MAX, u16::MAX);
        cb
    }
}
impl OnPaint for CheckBox {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let col_text = match () {
            _ if !self.is_enabled() => theme.text.inactive,
            _ if self.has_focus() => theme.text.focused,
            _ if self.is_mouse_over() => theme.text.hovered,
            _ => theme.text.normal,
        };

        let col_hot_key = if self.is_enabled() {
            theme.text.hot_key
        } else {
            theme.text.inactive
        };

        surface.write_string(0, 0, "[ ] ", col_text, false);
        /*
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
        */
        if self.checked {
            let col = if self.is_enabled() { theme.symbol.checked } else {theme.symbol.inactive };
            surface.set(1,0,Character::with_attributes(SpecialChar::CheckMark,col));
        }
        if self.has_focus() {
            surface.set_cursor(1, 0);
        }
    }
}

/*
namespace AppCUI::Controls
{

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



*/
