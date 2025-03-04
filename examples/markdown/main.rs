use appcui::prelude::*;
mod markdown_navigator;
mod md_loader;
use markdown_navigator::MarkdownNavigator;
use md_loader::MarkdownLoader;

#[Window(events = MarkdownEvents)]
struct my_win {
    base_path: String,
    navigator: MarkdownNavigator,
}

impl my_win {
    fn new() -> Self {
        let base_path = "examples/markdown/md_files".to_owned();
        let initial_path = format!("{}/{}", base_path, "introduction.md");

        let mut navigator = MarkdownNavigator::new();
        navigator.open(initial_path);

        let mut w = Self {
            base: window!("Markdown,d:c,w:50,h:15,flags:sizeable"),
            base_path,
            navigator,
        };

        let full_path = w.full_path("introduction.md");
        let content = MarkdownLoader::load(&full_path);
        if let Some(text) = content {
            let m = Markdown::new(&text, Layout::new("d: c"), markdown::Flags::ScrollBars);
            w.add(m);
        }
        w
    }

    fn full_path(&self, filename: &str) -> String {
        format!("{}/{}", self.base_path, filename)
    }
}

impl MarkdownEvents for my_win {
    fn on_external_link(&mut self, _handle: Handle<Markdown>, link: &str) -> EventProcessStatus {
        let full_path = self.full_path(link);
        if let Some(file_content) = MarkdownLoader::load(&full_path) {
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

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(my_win::new());
    app.run();
    Ok(())
}
