pub struct List {
    trigger: char,
    items: Vec<String>,
}

impl List {
    pub fn new(trigger: char) -> Self {
        Self {
            trigger,
            items: Vec::new(),
        }
    }

    pub fn with_items(trigger: char, items: &[&str]) -> Self {
        Self {
            trigger,
            items: items.iter().map(|item| item.to_string()).collect(),
        }
    }

    pub fn add(&mut self, item: &str) {
        self.items.push(item.to_string());
    }

    pub fn remove(&mut self, index: u32) {
        if (index as usize) < self.items.len() {
            self.items.remove(index as usize);
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn trigger(&self) -> char {
        self.trigger
    }

    pub fn len(&self) -> u32 {
        self.items.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn item(&self, index: u32) -> Option<&str> {
        self.items.get(index as usize).map(|item| item.as_str())
    }
}
