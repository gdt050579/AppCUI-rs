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
    pub fn from_string(value: String, description: String) -> Self {
        Self { value, description }
    }
    pub fn set_value(&mut self, value: &str) {
        self.value.clear();
        self.value.push_str(value);
    }
    #[inline(always)]
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn set_description(&mut self, description: &str) {
        self.description.clear();
        self.description.push_str(description);
    }
    #[inline(always)]
    pub fn description(&self) -> &str {
        &self.description
    }
}
