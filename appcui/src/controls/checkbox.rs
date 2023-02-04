use super::events::*;
use super::ControlManager;
use super::Layout;
use super::StatusFlags;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;
use crate::utils::*;
use AppCUIProcMacro::AppCUIControl;

#[AppCUIControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent)]
pub struct CheckBox {
    caption: Caption,
    checked: bool,
}

impl CheckBox {
    pub(crate) fn new(caption: &str, layout: Layout, checked: bool) -> Self {
        let mut cb = CheckBox {
            base: ControlManager::new(
                layout,
                StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput,
            ),
            caption: Caption::new(caption,true),
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
        let w = self.get_width();
        if w > 4 {
            let mut format =
                TextFormat::new(4, 0, col_text, TextAlignament::Left, self.get_height() > 1);
            if format.multi_line {
                format.text_wrap = TextWrap::Word;
                format.width = Some(w-4);
            }
            /*
                params.HotKeyPosition = Members->HotKeyOffset;
                params.HotKeyColor    = colHK;
            */
            surface.write_text(&self.caption.get_text(), &format);
        }
        if self.checked {
            let col = if self.is_enabled() {
                theme.symbol.checked
            } else {
                theme.symbol.inactive
            };
            surface.set(
                1,
                0,
                Character::with_attributes(SpecialChar::CheckMark, col),
            );
        }
        if self.has_focus() {
            surface.set_cursor(1, 0);
        }
    }
}
impl OnDefaultAction for CheckBox
{
    fn on_default_action(&mut self) {
        self.checked = !self.checked;
        // RaiseEvent(Event::CheckedStatusChanged); ???
    }
}
impl OnKeyPressed for CheckBox {
    fn on_key_pressed(&mut self, key: Key, _character: char) 
    {
        if (key.modifier == KeyModifier::None) && ((key.code == KeyCode::Space) || (key.code == KeyCode::Enter)) {
            self.on_default_action();
        }
    }
}
impl OnMouseEvent for CheckBox {
    fn on_mouse_event(&mut self, event: &MouseEvent) {
        match event {
            MouseEvent::Enter => todo!(),
            MouseEvent::Leave => todo!(),
            MouseEvent::Over => todo!(),
            MouseEvent::Pressed => todo!(),
            MouseEvent::Released => todo!(),
            MouseEvent::DoubleClick => todo!(),
            MouseEvent::Drag => todo!(),
            MouseEvent::Wheel => todo!(),
        }
    }
}
/*
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
