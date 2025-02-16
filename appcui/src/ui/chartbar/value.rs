use crate::prelude::*;
use flat_string::FlatString;

pub struct Value {
    value: i32,
    label: FlatString<14>,
    color: Option<Color>,
}

impl Value {
    pub fn new(value: i32) -> Self {
        Value {
            value,
            label: FlatString::new(),
            color: None,
        }
    }
    pub fn with_label_and_color(value: i32, label: &str, color: Color) -> Self {
        Value {
            value,
            label: FlatString::from_str(label),
            color: Some(color),
        }
    }
    pub fn with_label(value: i32, label: &str) -> Self {
        Value {
            value,
            label: FlatString::from_str(label),
            color: None,
        }
    }
    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }
    pub fn set_label(&mut self, label: &str) {
        self.label.set(label);
    }

    pub fn color(&self) -> Option<Color> {
        self.color
    }
    pub fn label(&self) -> &str {
        if self.label.is_empty() {
            "?"
        } else {
            self.label.as_str()
        }
    }
    pub fn value(&self) -> i32 {
        self.value
    }

    pub(super) fn attr(&self, color: Option<Color>, default_color: Color) -> CharAttribute {
        CharAttribute::new(Color::Transparent, color.unwrap_or(self.color.unwrap_or(default_color)), CharFlags::None)
    }

    pub fn relative_size(&self, max_size: u32, min_value: i32, max_value: i32) -> u32 {
        let d = (max_value.saturating_sub(min_value)) as u32;
        let v = ((self.value - min_value) as u32).clamp(0, d);
        if d == 0 {
            0
        } else {
            v * max_size / d
        }
    }
}
