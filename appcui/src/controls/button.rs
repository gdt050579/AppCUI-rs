use super::events::*;
use super::ControlBase;
use super::Layout;
use super::StatusFlags;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;
use crate::utils::*;
use AppCUIProcMacro::AppCUIControl;
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum ButtonFlags {
    Flat = 0x01,
}

#[AppCUIControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent)]
pub struct Button {
    flags: ButtonFlags,
    caption: Caption,
    pressed: bool,
}
impl Button {
    pub fn new(caption: &str, layout: Layout, flags: ButtonFlags) -> Self {
        let mut but = Button {
            base: ControlBase::new(
                layout,
                StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput,
            ),
            caption: Caption::new(caption, true),
            flags,
            pressed: false
        };

        if flags.contains(ButtonFlags::Flat) {
            but.set_size_bounds(3, 1, u16::MAX, 1);
        } else {
            but.set_size_bounds(4, 2, u16::MAX, 2);
        }
        let hotkey = but.caption.get_hotkey();
        but.set_hotkey(hotkey);
        but
    }
}
impl OnDefaultAction for Button {
    fn on_default_action(&mut self) {
        self.raise_event(Event::ButtonClicked);
    }
}
impl OnKeyPressed for Button {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        if (key.modifier == KeyModifier::None)
            && ((key.code == KeyCode::Space) || (key.code == KeyCode::Enter))
        {
            self.on_default_action();
            return EventProcessStatus::Processed;
        }
        return EventProcessStatus::Ignored;
    }
}

impl OnPaint for Button {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let col_text = match () {
            _ if !self.is_enabled() => theme.button.text.inactive,
            _ if self.has_focus() => theme.button.text.focused,
            _ if self.is_mouse_over() => theme.button.text.hovered,
            _ => theme.button.text.normal,
        };

        let col_hot_key = match () {
            _ if !self.is_enabled() => theme.button.hotkey.inactive,
            _ if self.has_focus() => theme.button.hotkey.focused,
            _ if self.is_mouse_over() => theme.button.hotkey.hovered,
            _ => theme.button.hotkey.normal,
        };

        if self.flags.contains(ButtonFlags::Flat) {
            surface.clear(Character::with_attributes(' ', col_text));
        } else {

        }
/*
    WriteTextParams params(
          WriteTextFlags::SingleLine | WriteTextFlags::OverwriteColors | WriteTextFlags::HighlightHotKey |
          WriteTextFlags::ClipToWidth | WriteTextFlags::FitTextToWidth);

    const auto btnState   = Members->GetControlState(ControlStateFlags::All);
    params.Color          = Members->Cfg->Button.Text.GetColor(btnState);
    params.HotKeyColor    = Members->Cfg->Button.HotKey.GetColor(btnState);
    bool pressed          = IsChecked();
    params.Y              = 0;
    params.HotKeyPosition = Members->HotKeyOffset;
    params.Align          = TextAlignament::Center;

    if (Members->Flags && ButtonFlags::Flat)
    {
        params.X     = 0;
        params.Width = Members->Layout.Width;
        renderer.FillHorizontalLine(0, 0, Members->Layout.Width, ' ', params.Color);
        renderer.WriteText(Members->Text, params);
    }
    else
    {
        params.Width = Members->Layout.Width - 1;
        if (pressed)
        {
            renderer.FillHorizontalLine(1, 0, Members->Layout.Width, ' ', params.Color);
            params.X = 1;
            renderer.WriteText(Members->Text, params);
        }
        else
        {
            renderer.FillHorizontalLine(0, 0, Members->Layout.Width - 2, ' ', params.Color);
            params.X = 0;
            renderer.WriteText(Members->Text, params);

            renderer.FillHorizontalLineWithSpecialChar(
                  1, 1, Members->Layout.Width, SpecialChars::BlockUpperHalf, Members->Cfg->Button.ShadowColor);
            renderer.WriteSpecialCharacter(
                  Members->Layout.Width - 1, 0, SpecialChars::BlockLowerHalf, Members->Cfg->Button.ShadowColor);
        }
    }


*/
    }
}
impl OnMouseEvent for Button {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => {
                if self.caption.get_chars_count() > (self.get_size().width - 2) as usize {
                    self.show_tooltip(self.caption.get_text());
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Leave => EventProcessStatus::Processed,
            MouseEvent::Released(data) => {
                self.pressed = false;
                if self.is_coord_in_control(data.x, data.y) {
                    self.on_default_action();
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Drag(data) => {
                if self.pressed && (!self.is_coord_in_control(data.x, data.y)) {
                    self.pressed = false;
                    return EventProcessStatus::Processed;
                }
                EventProcessStatus::Ignored
            }
            MouseEvent::Pressed(_) => { self.pressed = true; EventProcessStatus::Processed }
            _ => EventProcessStatus::Ignored,
        }
    }
}
