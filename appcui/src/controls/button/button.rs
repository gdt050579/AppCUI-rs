use super::super::events::*;
use super::super::menu::*;
use super::super::ControlBase;
use super::super::Layout;
use super::super::StatusFlags;
use super::ButtonClickedEvent;
use super::ButtonFlags;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;
use crate::utils::*;
use AppCUIProcMacro::AppCUIControl;

#[AppCUIControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent)]
pub struct Button {
    flags: ButtonFlags,
    caption: Caption,
    pressed: bool,
    handler: Option<Box<dyn Fn(Handle)>>,
}
impl Button {
    /// Creates a new button with the specified caption, layout and flags
    /// # Examples
    /// ```
    /// use appcui::controls::*;
    /// let mut button = Button::new("Click me!", Layout::new("x:1,y:1,w:15"), ButtonFlags::None);
    /// ```
    pub fn new(caption: &str, layout: Layout, flags: ButtonFlags) -> Self {
        let mut but = Button {
            base: ControlBase::new(
                layout,
                StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput,
            ),
            caption: Caption::new(caption, true),
            flags,
            pressed: false,
            handler: None,
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
    pub fn set_handler(&mut self, handler: impl Fn(Handle) + 'static) {
        self.handler = Some(Box::new(handler));
    }
}
impl OnDefaultAction for Button {
    fn on_default_action(&mut self) {
        self.raise_event(Event::ButtonClicked(ButtonClickedEvent {
            handle: self.handle,
        }));
        let my_handler = self.handle;
        if let Some(handler) = &mut self.handler {
            handler(my_handler);
        }
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
        let w = self.get_size().width;
        let mut format =
            TextFormat::single_line((w / 2) as i32, 0, col_text, TextAlignament::Center);
        format.chars_count = Some(self.caption.get_chars_count() as u16);
        if self.caption.has_hotkey() {
            format.hotkey_attr = Some(match () {
                _ if !self.is_enabled() => theme.button.hotkey.inactive,
                _ if self.has_focus() => theme.button.hotkey.focused,
                _ if self.is_mouse_over() => theme.button.hotkey.hovered,
                _ => theme.button.hotkey.normal,
            });
            format.hotkey_pos = self.caption.get_hotkey_pos();
        }
        if self.flags.contains(ButtonFlags::Flat) {
            surface.clear(Character::with_attributes(' ', col_text));
            format.width = Some(w as u16);
            surface.write_text(self.caption.get_text(), &format);
        } else {
            format.width = Some((w - 1) as u16);
            if self.pressed {
                surface.fill_horizontal_line_with_size(
                    1,
                    0,
                    w - 1,
                    Character::with_attributes(' ', col_text),
                );
                format.x += 1;
                surface.write_text(self.caption.get_text(), &format);
            } else {
                surface.fill_horizontal_line_with_size(
                    0,
                    0,
                    w - 1,
                    Character::with_attributes(' ', col_text),
                );
                surface.write_text(self.caption.get_text(), &format);
                surface.fill_horizontal_line_with_size(
                    1,
                    1,
                    w - 1,
                    Character::with_attributes(SpecialChar::BlockUpperHalf, theme.button.shadow),
                );
                surface.write_char(
                    (w as i32) - 1,
                    0,
                    Character::with_attributes(SpecialChar::BlockLowerHalf, theme.button.shadow),
                );
            }
        }
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
            MouseEvent::Pressed(_) => {
                self.pressed = true;
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
