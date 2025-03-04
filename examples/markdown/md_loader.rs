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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_real_file() {
        // Assuming that "md_files/introduction.md" exists at the project root.
        let file_path = "markdown/md_files/introduction.md";
        let content = MarkdownLoader::load(file_path);
        assert!(content.is_some(), "The file {} should exist and be loaded", file_path);
        let content = content.unwrap();
        // Check for a known header (adjust as needed to match your file's content).
        assert!(content.contains("#"), "The file should contain markdown headers");
    }
}