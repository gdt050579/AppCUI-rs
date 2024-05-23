pub struct Item {
    pub(super) value: String,
    pub(super) description: String,
}

impl Item {
    pub fn new(value: &str, description: &str) -> Self {
        let mut obj = Item {
            value: String::new(),
            description: String::new(),
        };
        obj.value.push_str(value);
        if !description.is_empty() {
            obj.description.push_str(description);
        }
        obj
    }
    pub fn from_string(value: String, description: String)->Self {
        Self {
            value,
            description,
        }
    }
}
