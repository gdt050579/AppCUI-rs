use crate::prelude::*;
use crate::utils::NavigatorEntry;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub(crate) struct Entry {
    pub(crate) name: String,
    pub(crate) size: u64,
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
            "F" | "f" | "file" => false,
            "D" | "d" | "dir" | "folder" => true,
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

impl listview::ListItem for Entry {
    const COLUMNS_COUNT: u16 = 3;

    fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
        match column_index {
            0 => Some(listview::RenderMethod::Text(&self.name)),
            1 => {
                if self.folder {
                    Some(listview::RenderMethod::Ascii("Folder"))
                } else {
                    Some(listview::RenderMethod::Size(self.size, listview::SizeFormat::AutoWithDecimals))
                }
            }
            2 => Some(listview::RenderMethod::DateTime(self.created, listview::DateTimeFormat::Short)),
            _ => None,
        }
    }

    fn column(index: u16) -> Column {
        match index {
            0 => Column::new("&Name", 28, TextAlignament::Left),
            1 => Column::new("&Size", 12, TextAlignament::Right),
            2 => Column::new("&Created", 16, TextAlignament::Center),
            _ => Column::new("", 10, TextAlignament::Left),
        }
    }

    fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
        match column_index {
            0 => self.name.cmp(&other.name),
            1 => self.size.cmp(&other.size),
            2 => self.created.cmp(&other.created),
            _ => std::cmp::Ordering::Equal
        }
    }
}
