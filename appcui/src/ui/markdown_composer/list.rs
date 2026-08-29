use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum ListFlags {
    RemoveTrigger = 0x01,
}

pub struct List {
    trigger: char,
    items: Vec<String>,
    values: Vec<String>,
    flags: ListFlags,
}

impl List {
    pub fn new(trigger: char, flags: ListFlags) -> Self {
        Self {
            trigger,
            items: Vec::new(),
            values: Vec::new(),
            flags,
        }
    }

    pub fn with_items(trigger: char, items: &[&str], flags: ListFlags) -> Self {
        Self {
            trigger,
            items: items.iter().map(|item| item.to_string()).collect(),
            values: items.iter().map(|item| item.to_string()).collect(),
            flags,
        }
    }

    pub fn with_values(trigger: char, items: &[(&str, &str)], flags: ListFlags) -> Self {
        Self {
            trigger,
            items: items.iter().map(|(name, _)| name.to_string()).collect(),
            values: items.iter().map(|(_, value)| value.to_string()).collect(),
            flags,
        }
    }

    pub fn add(&mut self, item: &str) {
        self.items.push(item.to_string());
        self.values.push(item.to_string());
    }

    pub fn add_value(&mut self, item: &str, value: &str) {
        self.items.push(item.to_string());
        self.values.push(value.to_string());
    }

    pub fn remove(&mut self, index: u32) {
        if (index as usize) < self.items.len() {
            self.items.remove(index as usize);
            self.values.remove(index as usize);
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.values.clear();
    }

    pub fn trigger(&self) -> char {
        self.trigger
    }

    pub fn flags(&self) -> ListFlags {
        self.flags
    }

    pub fn set_flags(&mut self, flags: ListFlags) {
        self.flags = flags;
    }

    pub fn len(&self) -> u32 {
        self.items.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn items(&self) -> &[String] {
        &self.items
    }

    pub fn item(&self, index: u32) -> Option<&str> {
        self.items.get(index as usize).map(|item| item.as_str())
    }

    pub fn value(&self, index: u32) -> Option<&str> {
        self.values.get(index as usize).map(|value| value.as_str())
    }
}
