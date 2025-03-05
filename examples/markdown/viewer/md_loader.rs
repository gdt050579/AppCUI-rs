use std::fs;
use std::path::Path;

pub struct MarkdownLoader;

impl MarkdownLoader {
    pub fn load(path: &str) -> Option<String> {
        let file_path = Path::new(path);
        if file_path.exists() && file_path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            fs::read_to_string(file_path).ok()
        } else {
            None
        }
    }
}