pub(crate) struct MarkdownNavigator {
    history: Vec<String>,
    current: Option<String>,
}

impl MarkdownNavigator {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            current: None,
        }
    }

    pub fn open(&mut self, path: String) {
        if let Some(curr) = &self.current {
            self.history.push(curr.clone());
        }
        self.current = Some(path);
    }

    pub fn go_back(&mut self) -> Option<String>{
        if let Some(prev) = self.history.pop() {
            self.current = Some(prev);
        }
        self.current.clone()
    }
}