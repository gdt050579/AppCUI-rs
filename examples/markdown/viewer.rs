use appcui::{prelude::*, ui::window::Flags};
mod markdown_navigator;
mod md_loader;
use markdown_navigator::MarkdownNavigator;
use md_loader::MarkdownLoader;

#[Window(events = MarkdownEvents+ToolBarEvents)]
pub struct Viewer {
    base_path: String,
    navigator: MarkdownNavigator,
    h_md: Handle<Markdown>,
}

impl Viewer {
    pub fn new(base_path: String) -> Self {
        let initial_path = format!(r"{}\{}", base_path, "introduction.md");

        let window = Window::new(&initial_path, Layout::new("d:c, w:50, h: 15"), Flags::Sizeable);
        let mut w: Viewer = Self {
            base: window,
            base_path,
            navigator: MarkdownNavigator::new(),
            h_md: Handle::None
        };
        
        let content = MarkdownLoader::load(&initial_path);
        
        if let Some(text) = content {
            w.navigator.open("introduction.md".to_string());
            let m: Markdown = Markdown::new(&text, Layout::new("d: c"), markdown::Flags::ScrollBars);
            w.h_md = w.add(m);
        }
        let group = w.toolbar().create_group(toolbar::GroupPosition::TopLeft);
        w.toolbar().add(group, window::toolbar::Button::new("<--"));
        w
    }

    fn full_path(&self, filename: &str) -> String {
        format!(r"{}\{}", self.base_path, filename)
    }

    fn go_back(&mut self) {
        let handle = self.h_md;
        if let Some(current_path) = self.navigator.go_back() {
            let full_path = self.full_path(&current_path);
            if let Some(file_content) = MarkdownLoader::load(&full_path) {
                if let Some(md) = self.control_mut(handle) {
                    md.set_content(&file_content);
                }
            } else if let Some(md) = self.control_mut(handle) {
                md.set_content(&format!("Could not load file at {}", current_path));
            }
        }
    }
}


impl ToolBarEvents for Viewer {
    fn on_button_clicked(&mut self, _: Handle<toolbar::Button>) -> EventProcessStatus {
        self.go_back();
        EventProcessStatus::Processed
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
        } else if let Some(md) = self.control_mut(_handle) {
            md.set_content(&format!("Could not load file at {}", full_path));
        }
        EventProcessStatus::Processed
    }

    fn on_backspace_navigation(&mut self, _handle: Handle<Markdown>) -> EventProcessStatus {
        self.go_back();
        EventProcessStatus::Processed
    }
}
