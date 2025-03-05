use appcui::{prelude::*, ui::window::Flags};
mod markdown_navigator;
mod md_loader;
use markdown_navigator::MarkdownNavigator;
use md_loader::MarkdownLoader;

#[Window(events = MarkdownEvents)]
pub struct Viewer {
    base_path: String,
    navigator: MarkdownNavigator,
}

impl Viewer {
    pub fn new() -> Self {
        let base_path = "D:/rust/AppCUI-rs/examples/markdown/md_files".to_string();
        let initial_path = format!("{}/{}", base_path, "introduction.md");

        let window = Window::new(&initial_path, Layout::new("d:c, w:50, h: 15"), Flags::Sizeable);
        let mut w: Viewer = Self {
            base: window,
            base_path,
            navigator: MarkdownNavigator::new(),
        };
        
        let content = MarkdownLoader::load(&initial_path);
        
        if let Some(text) = content {
            w.navigator.open("introduction.md".to_string());
            let m: Markdown = Markdown::new(&text, Layout::new("d: c"), markdown::Flags::ScrollBars);
            w.add(m);
        }
        w
    }

    fn full_path(&self, filename: &str) -> String {
        format!("{}/{}", self.base_path, filename)
    }
}

impl MarkdownEvents for Viewer {
    fn on_external_link(&mut self, _handle: Handle<Markdown>, link: &str) -> EventProcessStatus {
        let full_path = self.full_path(link);
        if let Some(file_content) = MarkdownLoader::load(&full_path) {
            self.base.set_title(&full_path);
            self.navigator.open(link.to_string());

            if let Some(md) = self.control_mut(_handle) {
                md.set_content(&file_content);
            }
        } else {
            if let Some(md) = self.control_mut(_handle) {
                md.set_content(&format!("Could not load file at {}", full_path));
            }
        }
        EventProcessStatus::Processed
    }

    fn on_backspace_navigation(&mut self, _handle: Handle<Markdown>) -> EventProcessStatus {
        if let Some(current_path) = self.navigator.go_back() {
            let full_path = self.full_path(&current_path);
            if let Some(file_content) = MarkdownLoader::load(&full_path) {
                if let Some(md) = self.control_mut(_handle) {
                    md.set_content(&file_content);
                }
            } else {
                if let Some(md) = self.control_mut(_handle) {
                    md.set_content(&format!("Could not load file at {}", current_path));
                }
            }
        }
        EventProcessStatus::Processed
    }
}
