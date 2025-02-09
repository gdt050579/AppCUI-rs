use crate::utils::NavigatorEntry;

use super::{Entry, Root};
use std::path::PathBuf;
pub(crate) struct NavSimulator {
    data: String,
    windows_model: bool,
    current_dir: String,
}
impl NavSimulator {
    #[cfg(test)]
    pub(crate) fn with_csv(data: &str, windows_model: bool, current_dir: &str) -> Self {
        Self {
            data: data.to_string(),
            windows_model,
            current_dir: current_dir.to_string(),
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
    fn entries(&self, path: &PathBuf) -> Option<Vec<Entry>> {
        let path = path.as_path().as_os_str().to_str().unwrap();
        let mut v = Vec::new();
        for line in self.data.lines() {
            if let Some(entry) = Entry::from_csv_line(line.trim(), path) {
                v.push(entry);
            }
        }
        Some(v)
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
    fn new() -> Self {
        Self { data: String::new(), windows_model: true, current_dir: String::new() }
    }

    fn join(&self, path: &PathBuf, entry: &Entry) -> Option<PathBuf> {
        if self.is_root(entry.name()) {
            Some(PathBuf::from(entry.name().replace('/', "\\").as_str()))
        } else {
            let mut components: Vec<&str> = path.components().map(|c| c.as_os_str().to_str().unwrap()).filter(|c| *c != "\\").collect();
            for s in entry.name().split(['/', '\\']) {
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
            let mut s = String::with_capacity(256);
            for c in components {
                if !s.is_empty() {
                    if self.windows_model {
                        s.push('\\');
                    } else {
                        s.push('/');
                    }
                }
                s.push_str(c);
            }
            Some(PathBuf::from(s))
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
    fn current_dir(&self) -> PathBuf {
        PathBuf::from(&self.current_dir)
    }
}

impl Clone for NavSimulator {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            windows_model: self.windows_model,
            current_dir: self.current_dir.clone(),
        }
    }
}