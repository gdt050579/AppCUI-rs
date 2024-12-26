
use crate::prelude::*;
use flat_string::FlatString;

pub struct Value
{
    value: i32,
    label: FlatString<14>,
    attribute: CharAttribute,
    
}

impl Value
{
    pub fn new(value: i32 ) -> Self
    {
        let v = Value
        {
            value,
            label: FlatString::new(),
            attribute: CharAttribute::new(Color::White, Color::White, CharFlags::None),
        };
        v
    }
    pub fn with_label_color(value: i32,label: &str, color: CharAttribute) -> Self
    {
        let v = Value
        {
            value,
            label: FlatString::from_str(label),
            attribute: color,
        };
        v
    }
    pub fn set_color(&mut self, color: CharAttribute) { self.attribute = color; }
    pub fn set_value(&mut self, value: i32) { self.value = value; }
    pub fn set_label(&mut self, label: &str) { self.label.set(label); }
    
    pub fn attr(&self) -> CharAttribute {self.attribute}
    pub fn label(&self) -> &str { self.label.as_str()}
    pub fn value(&self) -> i32 { self.value }
   
    pub fn relative_size(&self, max_size: u32, min_value: i32, max_value: i32) -> u32 {
        let d = (max_value - min_value).max(1) as u32;
        let v = ((self.value - min_value) as u32).clamp(0, max_size);
        v*max_size/d
    }

}