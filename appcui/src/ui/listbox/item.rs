pub struct Item {
    pub(super) value: String,
    pub(super) count: u32,
    pub(super) left: u32,
}

impl Item {
    pub fn new(text: &str) -> Self {
        Item {
            value: String::from(text),
            count: text.chars().count() as u32,
            left: 0,
        }
    }   
}