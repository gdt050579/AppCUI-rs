use crate::utils::NavigatorEntry;

use super::{Entry, Root};
use std::path::PathBuf;
pub(crate) struct NavSimulator {
    data: String,
    windows_model: bool,
}
impl NavSimulator {
    pub(crate) fn with_csv(data: &str, windows_model: bool) -> Self {
        Self {
            data: data.to_string(),
            windows_model,
        }
    }
    fn is_root(&self, path: &str) -> bool {
        let buf = path.as_bytes();
        if self.windows_model {
            buf.len() >= 2 && buf[1] == b':' && ((buf[0] >= b'A' && buf[0] <= b'Z') || (buf[0] >= b'a' && buf[0] <= b'z'))
        } else {
            buf.len() == 1 && buf[0] == b'/'
        }
    }
}
impl crate::utils::Navigator<Entry, Root, PathBuf> for NavSimulator {
    fn entries(&self, path: &PathBuf) -> Vec<Entry> {
        let path = path.as_path().as_os_str().to_str().unwrap();
        let mut v = Vec::new();
        for line in self.data.lines() {
            if let Some(entry) = Entry::from_csv_line(line.trim(), path) {
                v.push(entry);
            }
        }
        v
    }
    fn roots(&self) -> Vec<Root> {
        let mut v = Vec::new();
        for line in self.data.lines() {
            if let Some(root) = Root::from_csv_line(line.trim()) {
                v.push(root);
            }
        }
        v
    }

    fn join(&self, path: &PathBuf, entry: &Entry) -> Option<PathBuf> {
        if self.is_root(entry.name()) {
            Some(PathBuf::from(entry.name()))
        } else {
            let mut components: Vec<&str> = path.components().map(|c| c.as_os_str().to_str().unwrap()).collect();
            for s in entry.name().split(|c| c == '/' || c == '\\') {
                match s {
                    ".." => {
                        if !components.is_empty() {
                            components.pop();
                        }
                    }
                    "." | "" => {
                        continue;
                    }
                    _ => {
                        components.push(s);
                    }
                }
            }
            let mut result = PathBuf::new();
            for component in components {
                result.push(component);
            }
            Some(result)
        }
    }

    fn exists(&self, path: &PathBuf) -> Option<bool> {
        if let Some(path) = path.as_os_str().to_str() {
            for line in self.data.lines() {
                let words = line.split(',').collect::<Vec<&str>>();
                if words.len() < 2 {
                    continue;
                }
                if words[1].trim() == path {
                    return Some(true);
                }
            }
            Some(false)
        } else {
            None
        }
    }
}
