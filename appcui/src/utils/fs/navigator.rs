use super::{Entry, EntryType, Root};
use crate::log;
use crate::utils::log::write_log_to_file;
use crate::utils::NavigatorEntry;
use chrono::DateTime;
use chrono::NaiveDateTime;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) struct Navigator {
}

impl crate::utils::Navigator<Entry, Root, PathBuf> for Navigator {
    fn entries(&self, path: &PathBuf) -> Option<Vec<Entry>> {
        if let Some(absoulte_path) = super::get_os_absolute_path(path) {
            log!("Debug", "Navigator::entries() Conversion to absoulte path {:?} -> {:?}", path, absoulte_path);
            if !absoulte_path.exists() {
                return None;
            }
            if let Ok(results) = Self::get_folder_listing(absoulte_path.as_path()) {
                return Some(results);
            }
        }
        None
    }

    fn roots(&self) -> Vec<Root> {
        super::get_os_roots()
    }

    fn new() -> Self {
        Self { }
    }

    fn join(&self, path: &PathBuf, entry: &Entry) -> Option<PathBuf> {
        if super::is_fs_root(entry.name()) {
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
                    s.push(super::get_os_separator());
                }
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
}

impl Clone for Navigator {
    fn clone(&self) -> Self {
        Self { }
    }
}
