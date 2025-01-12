use super::events::EventData;
use super::Buttons;
use super::Flags;
use crate::ui::common::Number;
use crate::prelude::*;
use std::fmt::Write;
use std::str::FromStr;
use crate::ui::common::number::Format;


#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize+OnFocus, internal=true)]
pub struct NumericSelector<T>
where
    T: Number + 'static, 
{
    value: T,
    min: T,
    max: T,
    step: T,
    flags: Flags,
    format: Format,
    buttons: Buttons,
    txt: String,
    txtlen: u8,
    edit_mode: bool,
}
#[allow(private_bounds)]
impl<T> NumericSelector<T>
where
    T: Number + 'static,
{
    fn to_interval(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }

    /// Creates a new NumericSelector control with the specified layout, flags, and default format (Decimal)
    /// The flags can be a combination of the following values:
    /// * `Flags::HideButtons` - if set, the buttons will not be displayed
    /// * `Flags::ReadOnly` - if set, the control will be read-only
    /// 
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let ns = NumericSelector::<i32>::new(10, 0, 100, 1, Layout::new("x:1,y:1,w:20"), numericselector::Flags::None);
    /// 
    pub fn new(value: T, min: T, max: T, step: T, layout: Layout, flags: Flags) -> Self {
        Self::with_format(value, min, max, step, layout, flags, Format::Decimal)
    }


    /// Creates a new NumericSelector control with the specified layout, flags, and format
    /// The flags can be a combination of the following values:
    /// * `Flags::HideButtons` - if set, the buttons will not be displayed
    /// * `Flags::ReadOnly` - if set, the control will be read-only
    /// 
    /// The format can be one of the following values:
    /// * `Format::Decimal` - the value will be displayed as a decimal number
    /// * `Format::Percentage` - the value will be displayed as a percentage
    /// * `Format::Hex` - the value will be displayed as a hexadecimal number
    /// * `Format::DigitGrouping` - the value will be displayed with digit grouping
    /// * `Format::Size` - the value will be displayed as a size (bytes, KB, MB, GB, TB)
    /// 
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let ns = NumericSelector::<i32>::with_format(10, 0, 100, 1, Layout::new("x:1,y:1,w:20"), numericselector::Flags::None, numericselector::Format::Size);
    /// ```
    pub fn with_format(value: T, min: T, max: T, step: T, layout: Layout, flags: Flags, format: Format) -> Self {
        let v_min = if min < max { min } else { max };
        let v_max = if max > min { max } else { min };
        let v = Self::to_interval(value, v_min, v_max);
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            min: v_min,
            max: v_max,
            step,
            value: v,
            flags,
            format,
            txt: String::with_capacity(16),
            txtlen: 0,
            buttons: Buttons::new(),
            edit_mode: false,
        };
        obj.buttons.update_width(obj.size().width as u16);
        if flags.contains(Flags::HideButtons) {
            obj.set_size_bounds(3, 1, u16::MAX, 1);
        } else {
            obj.set_size_bounds(12, 1, u16::MAX, 1);
        }
        obj.update_string_representation();
        obj.update_button_status();
        obj
    }

    /// Returns the selected value
    #[inline(always)]
    pub fn value(&self) -> T {
        self.value
    }

    /// Sets the value of the control. If the value is outside the interval [min, max], it will be set to the closest limit
    pub fn set_value(&mut self, value: T) {
        self.value = Self::to_interval(value, self.min, self.max);
        self.update_button_status();
        self.update_string_representation()
    }
    #[inline(always)]
    fn update_button_status(&mut self) {
        if self.flags.contains(Flags::ReadOnly) {
            self.buttons.disable_buttons(true, true);
        } else {
            self.buttons.disable_buttons(self.value == self.min, self.value == self.max);
        }
    }
    fn update_string_representation(&mut self) {
        Number::write_to_string(&self.value, &mut self.txt, self.format);
        self.txtlen = self.txt.len() as u8;
    }
    fn increment(&mut self) {
        if self.flags.contains(Flags::ReadOnly) {
            return;
        }
        let mut new_value = self.value + self.step;
        if new_value < self.value {
            // overflow
            new_value = self.max;
        }
        new_value = Self::to_interval(new_value, self.min, self.max);
        if new_value != self.value {
            self.value = new_value;
            self.update_button_status();
            self.update_string_representation();
            self.emit_on_selection_changed_event();
        }
    }
    fn decrement(&mut self) {
        if self.flags.contains(Flags::ReadOnly) {
            return;
        }
        let mut new_value = self.value - self.step;
        if new_value > self.value {
            // underflow
            new_value = self.min;
        }
        new_value = Self::to_interval(new_value, self.min, self.max);
        if new_value != self.value {
            self.value = new_value;
            self.update_button_status();
            self.update_string_representation();
            self.emit_on_selection_changed_event();
        }
    }
    fn enter_edit_mode(&mut self) {
        if self.flags.contains(Flags::ReadOnly) {
            return;
        }
        if self.edit_mode {
            return;
        }
        // use normal representation for editing
        self.txt.clear();
        write!(self.txt, "{}", self.value).unwrap();
        self.txtlen = self.txt.len() as u8;
        self.edit_mode = true;
    }
    fn exit_edit_mode(&mut self, accept: bool) {
        if self.edit_mode {
            if accept {
                if let Ok(new_value) = self.txt.parse::<T>() {
                    let old_value = self.value;
                    self.set_value(new_value);
                    if self.value != old_value {
                        self.emit_on_selection_changed_event();
                    }
                }
            }
            self.edit_mode = false;
            self.update_string_representation();
        }
    }

    fn emit_on_selection_changed_event(&mut self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::NumericSelector(EventData {
                type_id: std::any::TypeId::of::<T>(),
            }),
        });
    }
}
impl<T> OnPaint for NumericSelector<T>
where
    T: Number + 'static,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let has_buttons = (!self.flags.contains(Flags::HideButtons)) && (!self.edit_mode);
        if has_buttons {
            self.buttons.paint(surface, theme, self.is_enabled());
        }
        let attr = match () {
            _ if !self.is_enabled() => theme.editor.inactive,
            _ if self.has_focus() => theme.editor.focused,
            _ if self.is_mouse_over() => theme.editor.hovered,
            _ => theme.editor.normal,
        };
        let w = self.size().width as i32;
        let (l, r) = if has_buttons { (4, w - 5) } else { (0, w - 1) };
        surface.fill_horizontal_line(l, 0, r, Character::with_attributes(' ', attr));
        if l + 2 <= r {
            if self.edit_mode {
                let chars_count = self.txt.len() as i32; // all are ascii
                let vis_chars = r - (l + 1);
                let mut format = TextFormatBuilder::new()
                    .position(l + 1, 0)
                    .attribute(attr)
                    .align(TextAlignament::Left)
                    .build();
                if chars_count <= vis_chars {
                    format.set_chars_count(chars_count as u16);
                    format.set_wrap_type(WrapType::SingleLineWrap(vis_chars as u16));
                    surface.write_text(&self.txt, &format);
                    surface.set_cursor(l + chars_count + 1, 0);
                } else {
                    let start = chars_count - vis_chars;
                    format.set_chars_count(vis_chars as u16);
                    format.set_wrap_type(WrapType::SingleLineWrap(vis_chars as u16));
                    surface.write_text(&self.txt[start as usize..], &format);
                    surface.set_cursor(r, 0);
                }
            } else {
                let format = TextFormatBuilder::new()
                    .position((l + r + 1) / 2, 0)
                    .attribute(attr)
                    .align(TextAlignament::Center)
                    .chars_count(self.txtlen as u16)
                    .wrap_type(WrapType::SingleLineWrap((r - (l + 1)) as u16))
                    .build();
                surface.write_text(&self.txt, &format);
            }
        }
    }
}
impl<T> OnKeyPressed for NumericSelector<T>
where
    T: Number + FromStr + 'static,
{
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        let readonly = self.flags.contains(Flags::ReadOnly);
        match key.value() {
            key!("Up") | key!("Right") => {
                self.exit_edit_mode(false);
                if !readonly {
                    self.increment();
                }
                return EventProcessStatus::Processed;
            }
            key!("Down") | key!("Left") => {
                self.exit_edit_mode(false);
                if !readonly {
                    self.decrement();
                }
                return EventProcessStatus::Processed;
            }
            key!("Home") => {
                self.exit_edit_mode(false);
                if !readonly {
                    self.set_value(self.min);
                }
                return EventProcessStatus::Processed;
            }
            key!("End") => {
                self.exit_edit_mode(false);
                if !readonly {
                    self.set_value(self.max);
                }
                return EventProcessStatus::Processed;
            }
            key!("Escape") => {
                if self.edit_mode {
                    self.exit_edit_mode(false);
                    return EventProcessStatus::Processed;
                }
                return EventProcessStatus::Ignored;
            }
            key!("Enter") => {
                if self.edit_mode {
                    self.exit_edit_mode(true);
                } else {
                    self.enter_edit_mode();
                }
                return EventProcessStatus::Processed;
            }
            key!("Backspace") => {
                self.enter_edit_mode();
                if (self.txtlen > 0) && (!readonly) {
                    self.txt.pop();
                    self.txtlen -= 1;
                }
                return EventProcessStatus::Processed;
            }
            _ => {}
        }
        if (character as u32) != 0 {
            let add_char = matches!(character, '0'..='9' | 'a'..='f' | 'A'..='F' | 'x' | 'X' | 'h' | 'H' | 'o' | 'O' | '.' | '_');
            if add_char && (!readonly) {
                self.enter_edit_mode();
                if self.txt.len() < u8::MAX as usize {
                    self.txt.push(character);
                    self.txtlen += 1;
                }
                return EventProcessStatus::Processed;
            }
        }
        EventProcessStatus::Ignored
    }
}
impl<T> OnMouseEvent for NumericSelector<T>
where
    T: Number + FromStr + 'static,
{
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        let mut response = EventProcessStatus::Ignored;
        // process buttons if visible
        if (!self.flags.contains_one(Flags::HideButtons | Flags::ReadOnly)) && (!self.edit_mode) {
            let bres = self.buttons.on_mouse_event(event);

            if bres.repaint {
                response = EventProcessStatus::Processed;
            }
            if bres.click_on_add {
                self.increment();
                response = EventProcessStatus::Processed;
            }
            if bres.click_on_sub {
                self.decrement();
                response = EventProcessStatus::Processed;
            }
            if !bres.forward_to_control {
                return response;
            }
        }
        // do other processing here
        response
    }
}
impl<T> OnResize for NumericSelector<T>
where
    T: Number + FromStr + 'static,
{
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.buttons.update_width(new_size.width as u16);
    }
}
impl<T> OnFocus for NumericSelector<T>
where
    T: Number + FromStr + 'static,
{
    fn on_focus(&mut self) {
        self.edit_mode = false;
    }

    fn on_lose_focus(&mut self) {
        self.edit_mode = false;
    }
}
