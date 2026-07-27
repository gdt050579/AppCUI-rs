use std::ops::{Div, Mul};

use crate::prelude::*;
use crate::ui::hslider::{events::EventData, Flags, Type};
use crate::ui::numericselector::Format;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct HSlider<T>
where
    T: Number + 'static + Into<f64> + TryFrom<f64>,
{
    value: T,
    min: T,
    max: T,
    step: T,
    page_step: T,
    hslider_type: Type,
    marker: char,
    marker_pos: u32,
    buffer_value: String,
    flags: Flags,
    pressed: bool,
}

impl<T> HSlider<T>
where
    T: Number + 'static + Into<f64> + TryFrom<f64>,
{
    pub fn new(min: T, max: T, step: T, hslider_type: Type, layout: Layout) -> Self {
        Self::inner_create(min, max, step, hslider_type, layout, StatusFlags::None)
    }
    fn inner_create(min: T, max: T, step: T, hslider_type: Type, layout: Layout, status: StatusFlags) -> Self {
        let mut hslider = HSlider {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput | status),
            value: min,
            min: min,
            max: max,
            step: step,
            page_step: step,
            hslider_type: hslider_type,
            marker: '●',
            marker_pos: 0,
            flags: Flags::None,
            pressed: false,
            buffer_value: String::new(),
        };
        hslider.value.write_to_string(&mut hslider.buffer_value, Format::Decimal);

        //hslider.set_size_bounds(1, 1, u16::MAX, 1);
        hslider
    }

    fn add_step_value(&mut self) {
        self.value = self.value + self.step;
        if self.value > self.max {
            self.value = self.max;
        }
        self.marker_pos = self.get_marker_x_position();
        self.value.write_to_string(&mut self.buffer_value, Format::Decimal);
    }

    fn sub_step_value(&mut self) {
        self.value = self.value - self.step;
        if self.value < self.min {
            self.value = self.min;
        }
        self.marker_pos = self.get_marker_x_position();
        self.value.write_to_string(&mut self.buffer_value, Format::Decimal);
    }

    fn set_marker_position_from_x(&mut self, x: i32) {
        let width = self.size().width as i32;
        self.marker_pos = x.clamp(0, width - 1) as u32;

        let min: f64 = self.min.into();
        let max: f64 = self.max.into();
        let pos: f64 = self.marker_pos as f64;

        let val = min + (pos / (width - 1) as f64) * (max - min);
        if let Ok(v) = T::try_from(val) {
            self.value = v;
            self.value.write_to_string(&mut self.buffer_value, Format::Decimal);
        }
    }

    fn get_marker_x_position(&self) -> u32 {
        let width = self.size().width as f64;
        if width <= 1.0 {
            return 0;
        }
        let value: f64 = self.value.into();
        let min: f64 = self.min.into();
        let max: f64 = self.max.into();
        if max - min == 0f64 {
            0
        } else {
            ((value - min) / (max - min) * (width - 1.0)).round() as u32
        }
    }

    fn paint_standard(&self, surface: &mut Surface, theme: &Theme) {
        let attr = match () {
            _ if !self.is_enabled() => theme.text.inactive,
            _ if self.has_focus() => theme.text.focused,
            _ if self.is_mouse_over() => theme.text.hovered,
            _ => theme.text.normal,
        };

        let attr1 = CharAttribute::new(Color::Green, Color::Transparent, CharFlags::None);
        let attr2 = CharAttribute::new(Color::Gray, Color::Transparent, CharFlags::None);
        let attr_marker = CharAttribute::new(Color::Red, Color::Transparent, CharFlags::None);

        let width = self.size().width;
        let size1 = self.marker_pos;
        let size2 = width.saturating_sub(self.marker_pos + 1);

        surface.fill_horizontal_line_with_size(0, 0, size1, Character::with_attributes('━', attr1));
        surface.write_char(self.marker_pos as i32, 0, Character::with_attributes(self.marker, attr_marker));
        surface.fill_horizontal_line_with_size(self.marker_pos as i32 + 1, 0, size2, Character::with_attributes('━', attr2));

        let format = TextFormatBuilder::new()
            .position(0, 1)
            .attribute(attr)
            .align(TextAlignment::Left)
            .chars_count(self.buffer_value.chars().count() as u16)
            .wrap_type(WrapType::SingleLineWrap(width as u16))
            .build();

        surface.write_text(&self.buffer_value, &format);
    }

    fn paint_progress_bar(&self, surface: &mut Surface, theme: &Theme) {
        // let attr = match () {
        //     _ if !self.is_enabled() => theme.text.inactive,
        //     _ if self.has_focus() => theme.text.focused,
        //     _ if self.is_mouse_over() => theme.text.hovered,
        //     _ => theme.text.normal,
        // };

        let attr1 = CharAttribute::new(Color::Transparent, Color::Green, CharFlags::None);
        let attr2 = CharAttribute::new(Color::Transparent, Color::Gray, CharFlags::None);
        let attr_text = CharAttribute::new(Color::Black, Color::Transparent, CharFlags::None);

        let width = self.size().width;
        let size1 = self.marker_pos + 1;
        let size2 = width.saturating_sub(self.marker_pos);

        surface.fill_horizontal_line_with_size(0, 0, size1, Character::with_attributes(' ', attr1));

        surface.fill_horizontal_line_with_size(size1 as i32, 0, size2, Character::with_attributes(' ', attr2));
        
        let pos = (size1 as i32 - self.buffer_value.len() as i32).max(0);
        let format = TextFormatBuilder::new()
            .position(pos, 0)
            .attribute(attr_text)
            .align(TextAlignment::Left)
            .chars_count(self.buffer_value.chars().count() as u16)
            .wrap_type(WrapType::SingleLineWrap(width as u16))
            .build();

        surface.write_text(&self.buffer_value, &format);
    }
}

impl<T> OnDefaultAction for HSlider<T>
where
    T: Number + 'static + Into<f64> + TryFrom<f64>,
{
    fn on_default_action(&mut self) {
        // self.raise_event(ControlEvent {
        //     emitter: self.handle,
        //     receiver: self.event_processor,
        //     data: ControlEventData::HyperLink(EventData),
        // });
    }
}
impl<T> OnKeyPressed for HSlider<T>
where
    T: Number + 'static + Into<f64> + TryFrom<f64>,
{
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Left") => {
                self.sub_step_value();
                EventProcessStatus::Processed
            }
            key!("Right") => {
                self.add_step_value();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl<T> OnPaint for HSlider<T>
where
    T: Number + 'static + Into<f64> + TryFrom<f64>,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        match &self.hslider_type {
            Type::Standard => self.paint_standard(surface, theme),
            Type::ProgressBar => self.paint_progress_bar(surface, theme),
        }
    }
}
impl<T> OnMouseEvent for HSlider<T>
where
    T: Number + 'static + Into<f64> + TryFrom<f64>,
{
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Pressed(mouse) => {
                self.pressed = true;
                self.set_marker_position_from_x(mouse.x);
                EventProcessStatus::Processed
            }
            MouseEvent::Released(_) => {
                self.pressed = false;
                EventProcessStatus::Processed
            }
            MouseEvent::Drag(mouse) => {
                self.set_marker_position_from_x(mouse.x);
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
