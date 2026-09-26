use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
/// Flags of a suggestion [`List`].
///
/// `ListFlags::None` keeps the trigger character in the text when an item is inserted.
pub enum ListFlags {
    /// Remove the trigger character on insertion, so `@` followed by `Ana` becomes `Ana`.
    RemoveTrigger = 0x01,
}

/// A suggestion list of a [`struct@super::MarkdownComposer`], opened by a trigger character.
///
/// Every item has a name, shown in the popup, and a value, inserted in the text. The two are
/// the same for a list built from names alone. Use [`super::MarkdownComposer::list`] and
/// [`super::MarkdownComposer::list_mut`] to reach the lists of a control.
pub struct List {
    trigger: char,
    items: Vec<String>,
    values: Vec<String>,
    flags: ListFlags,
}

impl List {
    pub(crate) fn with_items(trigger: char, items: &[&str], flags: ListFlags) -> Self {
        Self {
            trigger,
            items: items.iter().map(|item| item.to_string()).collect(),
            values: items.iter().map(|item| item.to_string()).collect(),
            flags,
        }
    }

    pub(crate) fn with_values(trigger: char, items: &[(&str, &str)], flags: ListFlags) -> Self {
        Self {
            trigger,
            items: items.iter().map(|(name, _)| name.to_string()).collect(),
            values: items.iter().map(|(_, value)| value.to_string()).collect(),
            flags,
        }
    }

    /// Adds an item that inserts its own name.
    pub fn add(&mut self, item: &str) {
        self.items.push(item.to_string());
        self.values.push(item.to_string());
    }

    /// Adds an item that shows `item` in the popup and inserts `value` in the text.
    pub fn add_value(&mut self, item: &str, value: &str) {
        self.items.push(item.to_string());
        self.values.push(value.to_string());
    }

    /// Removes the item at `index`. If the index is out of range, the list is not changed.
    pub fn remove(&mut self, index: u32) {
        if (index as usize) < self.items.len() {
            self.items.remove(index as usize);
            self.values.remove(index as usize);
        }
    }

    /// Removes all the items from the list.
    pub fn clear(&mut self) {
        self.items.clear();
        self.values.clear();
    }

    /// Returns the character that opens the list.
    pub fn trigger(&self) -> char {
        self.trigger
    }

    /// Returns the flags of the list.
    pub fn flags(&self) -> ListFlags {
        self.flags
    }

    /// Sets the flags of the list.
    pub fn set_flags(&mut self, flags: ListFlags) {
        self.flags = flags;
    }

    /// Returns the number of items in the list.
    pub fn len(&self) -> u32 {
        self.items.len() as u32
    }

    /// Returns **true** if the list has no items, **false** otherwise.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns the names of all the items in the list.
    pub fn items(&self) -> &[String] {
        &self.items
    }

    /// Returns the name of the item at `index`, or `None` if the index is out of range.
    pub fn item(&self, index: u32) -> Option<&str> {
        self.items.get(index as usize).map(|item| item.as_str())
    }

    /// Returns the value inserted by the item at `index`, or `None` if the index is out of range.
    pub fn value(&self, index: u32) -> Option<&str> {
        self.values.get(index as usize).map(|value| value.as_str())
    }
}
