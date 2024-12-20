
use crate::prelude::*;


pub struct Value<T: Copy>
{
    value: T,
    label: String,
    attribute: CharAttribute,
    
}

impl<T: Copy> Value<T>
{
    pub fn new(value: T ) -> Self
    {
        let v = Value
        {
            value,
            label: String::new(),
            attribute: CharAttribute::new(Color::White, Color::White, CharFlags::None),
        };
        v
    }
    pub fn with_label_color_width(value: T,label: &str, color: CharAttribute) -> Self
    {
        let v = Value
        {
            value,
            label: String::from(label),
            attribute: color,
        };
        v
    }
    pub fn set_color(&mut self, color: CharAttribute) { self.attribute = color; }
    pub fn set_value(&mut self, value: T) { self.value = value; }
    pub fn set_label(&mut self, label: &str) { self.label = String::from(label); }
    
    pub fn attr(&self) -> CharAttribute {self.attribute}
    pub fn label(&self) -> &str {&self.label.as_str()}
    pub fn value(&self) -> T { self.value }
    pub fn relative_size(&self, max_size: u32, min_value: T, max_value: T) -> u32 {
       let d = (max_value - min_value) as u32;
       let v = ((self.value - min_value) as u32).clamp(0, max_size);
       v*max_size/d
    }

    // pub fn update_top_pos(&mut self, new_top: usize)
    // {
    //     if new_top != self.m_top
    //     {

    //     }
    // }

}