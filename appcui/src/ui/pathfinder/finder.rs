use crate::utils::GlyphParser;

use std::fs;
use std::path::Path;

pub(crate) struct Finder {
    cached_path: String,
    cached_items: Vec<String>,
    only_folders: bool,
}

impl Finder {
    pub fn new(only_folders: bool) -> Self {
        Self {
            cached_path: String::new(),
            cached_items: vec![],
            only_folders: only_folders,
        }
    }

    pub fn get_path_items(&mut self, path: &str) -> Vec<String> {
        let folder = Self::get_folder(path);
        if folder != self.cached_path {
            // create cache for this folder
            let folder_contents = Self::get_folder_listing(folder, self.only_folders);
            if folder_contents.is_ok() {
                self.cached_items = folder_contents.unwrap();
                self.cached_path = folder.to_string();
            }
        }
        Self::get_matching_paths(path, &self.cached_items)
    }

    fn get_folder(path: &str) -> &str {
        let mut end = path.len();
        while let Some((ch, sz)) = path.previous_glyph(end) {
            end -= sz as usize;
            if ch == '\\' || ch == '/' || end == 0 {
                break;
            }
        }
        &path[..end]
    }

    fn get_folder_listing(path: &str, only_folders: bool) -> Result<Vec<String>, std::io::Error> {
        let mut result: Vec<String> = vec![];
        let dir = Path::new(path);

        // Read the directory entries
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() || !only_folders {
                if let Some(utf8path) = path.to_str() {
                    // push only filename/foldername
                    // add terminating char for folders                    
                    result.push(utf8path.to_string());
                }
                else {
                    // TODO: support for non-utf8 paths
                }
            }
        }
        result.sort();
        Ok(result)
    }

    fn get_matching_paths(path: &str, items: &Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for s in items {
            if s.starts_with(path) {
                result.push(s.clone());
            }
        }
        result
    }
}

#[test]
fn test_get_folder() {
    assert_eq!(Finder::get_folder("C:\\salutare\\hell.exe"), "C:\\salutare");
    assert_eq!(Finder::get_folder("/mnt/salutare/hell.exe"), "/mnt/salutare");
    assert_eq!(Finder::get_folder("C:\\salutare\\"), "C:\\salutare");
    assert_eq!(Finder::get_folder("C:\\salutare"), "C:");
}

#[test]
fn test_get_folder_listing() {
    Finder::get_folder_listing("D:\\work\\bd", false);
}

