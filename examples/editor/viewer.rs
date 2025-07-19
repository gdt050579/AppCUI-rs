use std::path::Path;
use appcui::prelude::*;

#[Window()]
pub struct Viewer {
}

impl Viewer {
    pub fn new(path: &Path, text_print: &str) -> Self {
        let mut w = Self {
            base: Window::new(path.to_str().unwrap_or("???"), Layout::new("a:c,w:50%,h:50%"), window::Flags::Sizeable),
        };
        let ta = TextArea::new(text_print, Layout::new("a:c,w:100%,h:100%"), textarea::Flags::ShowLineNumber | textarea::Flags::ScrollBars);
        w.add(ta);
        w
    }
}