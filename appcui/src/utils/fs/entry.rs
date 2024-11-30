use chrono::NaiveDateTime;
use AppCUIProcMacro::ListViewItem;
use crate::utils::NavigatorEntry;
use crate::prelude::*;

#[derive(Debug, ListViewItem)]
pub(crate) struct Entry {
    #[Column(name : "&Name", width : 20)]
    pub(crate) name: String,
    #[Column(name : "&Size", width : 10, align: right)]
    pub(crate) size: u64,
    #[Column(name : "&Created", width : 15, align: center)]
    pub(crate) created: NaiveDateTime,
    pub(crate) folder: bool,
}

impl Entry {
    pub(super) fn new(name: &str, size: u64, created: NaiveDateTime, folder: bool) -> Self {
        Self {
            name: name.to_string(),
            size,
            created,
            folder,
        }
    }
    pub(super) fn from_csv_line(line: &str, path: &str) -> Option<Self> {
        // format is type,name,size,created
        let mut parts = line.split(',');
        let entry_type = parts.next()?;
        let full_path = parts.next()?;
        if full_path.starts_with(path) == false {
            return None;
        }
        if full_path.len() <= path.len() {
            return None;
        }
        for c in full_path.chars().skip(path.len()) {
            if (c == '/') || (c == '\\') {
                return None;
            }
        }
        let size = parts.next()?;
        let created = parts.next()?;
        let folder = match entry_type {
            "F"|"f"|"file" => false,
            "D"|"d"|"dir"|"folder" => true,
            _ => return None,
        };
        let size = size.parse().ok()?;
        let created = NaiveDateTime::parse_from_str(created, "%Y-%m-%d %H:%M:%S").ok()?;
        Some(Self {
            name: full_path[path.len()..].to_string(),
            size,
            created,
            folder,
        })
    }
}

impl NavigatorEntry for Entry {
    fn name(&self) -> &str {
        &self.name
    }
    fn is_container(&self) -> bool {
        self.folder
    }
}