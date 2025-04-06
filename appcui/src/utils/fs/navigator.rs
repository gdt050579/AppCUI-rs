use super::{Entry, EntryType, Root};
use crate::utils::NavigatorEntry;
use crate::prelude::*;
use chrono::DateTime;
use chrono::NaiveDateTime;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) struct Navigator {
    windows_model: bool,
}

impl crate::utils::Navigator<Entry, Root, PathBuf> for Navigator {
    #[cfg(target_os = "windows")]
    fn entries(&self, path: &PathBuf) -> Vec<Entry> {
        log!("FS","entries({})", path.display()); 
        if path.as_os_str().is_empty() {
            return vec![];
        }
        if let Some(normalized_root) = Self::normalize_windows_root(path) {
            Self::get_folder_listing(normalized_root.as_path()).unwrap_or_default()
        } else {
            Self::get_folder_listing(path).unwrap_or_default()
        }
    }

    #[cfg(target_family = "unix")]
    fn entries(&self, path: &PathBuf) -> Vec<Entry> {
        Self::get_folder_listing(path).unwrap_or_default()
    }

    fn roots(&self) -> Vec<Root> {
        super::get_os_roots()
    }

    #[cfg(target_os = "windows")]
    fn new() -> Self {
        Self { windows_model: true }
    }

    #[cfg(target_family = "unix")]
    fn new() -> Self {
        Self { windows_model: false }
    }

    fn join(&self, path: &PathBuf, entry: &Entry) -> Option<PathBuf> {
        if self.is_root(entry.name()) {
            Some(PathBuf::from(entry.name().replace('/', "\\").as_str()))
        } else {
            let mut components: Vec<&str> = path
                .components()
                .map(|c| c.as_os_str().to_str().unwrap())
                .filter(|c| *c != "\\")
                .collect();

            // println!("{:?}", components);
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
                // println!("Pushing {} to {}", c, s);
                // edge case for windows roots

                s.push_str(c);
            }
            Some(PathBuf::from(s))
        }
    }

    fn exists(&self, path: &PathBuf) -> Option<bool> {
        match path.try_exists() {
            Ok(v) => Some(v),
            _ => None,
        }
    }
    fn current_dir(&self) -> PathBuf {
        std::env::current_dir().unwrap_or_default()
    }
}

impl Navigator {
    fn get_folder_listing(path: &Path) -> std::io::Result<Vec<Entry>> {
        log!("FS","get_folder_listing({})", path.display());    
        let mut result: Vec<Entry> = vec![];
        // Read the directory entries
        for dir_entry in fs::read_dir(path)? {
            let entry = dir_entry?;
            let metadata = entry.metadata()?;
            if let Some(utf8path) = entry.file_name().to_str() {
                let entry = Self::get_entry_from_metadata(utf8path, &metadata)?;
                result.push(entry);
            }
        }
        result.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(result)
    }

    fn get_entry_from_metadata(path: &str, metadata: &std::fs::Metadata) -> std::io::Result<Entry> {
        let creation = metadata.created()?;
        let datetime = Self::system_time_to_naive_datetime(creation)?;
        let size = match metadata.is_dir() {
            false => metadata.len(),
            _ => 0,
        };
        Ok(Entry::new(
            path,
            size,
            datetime,
            if metadata.is_dir() { EntryType::Folder } else { EntryType::File },
        ))
    }

    fn system_time_to_naive_datetime(system_time: SystemTime) -> std::io::Result<NaiveDateTime> {
        if let Ok(ds) = system_time.duration_since(UNIX_EPOCH) {
            if let Some(date) = DateTime::from_timestamp(ds.as_secs() as i64, ds.subsec_nanos()) {
                return Ok(date.naive_utc());
            }
        }
        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid timestamp"))
    }

    fn is_root(&self, path: &str) -> bool {
        let buf = path.as_bytes();
        if self.windows_model {
            buf.len() >= 2 && buf[1] == b':' && ((buf[0] >= b'A' && buf[0] <= b'Z') || (buf[0] >= b'a' && buf[0] <= b'z'))
        } else {
            buf.len() == 1 && buf[0] == b'/'
        }
    }

    #[cfg(target_os = "windows")]
    fn normalize_windows_root(path: &Path) -> Option<PathBuf> {
        let buf = path.as_os_str().as_encoded_bytes();
        if buf.len() == 2 && buf[1] == b':' && ((buf[0] >= b'A' && buf[0] <= b'Z') || (buf[0] >= b'a' && buf[0] <= b'z')) {
            let mut tmp = path.to_path_buf();
            tmp.push("\\");
            return Some(tmp);
        }
        None
    }
}

impl Clone for Navigator {
    fn clone(&self) -> Self {
        Self {
            windows_model: self.windows_model,
        }
    }
}
