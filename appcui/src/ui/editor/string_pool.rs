pub(super) struct StringPool {
    strings: Vec<String>,
}

impl StringPool {
    pub(super) fn new() -> Self {
        Self { strings: Vec::new() }
    }
    pub(super) fn get(&mut self) -> String {
        self.strings.pop().unwrap_or_else(|| String::new())
    }
    pub(super) fn add(&mut self, text: String) {
        self.strings.push(text);
    }
}   