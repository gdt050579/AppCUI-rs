use chrono::NaiveDateTime;
use super::{Navigator, NavigatorEntry, NavigatorRoot};

pub(crate) struct MOCNavigator {
    data: String,   
}

#[derive(Debug)]
pub(crate) struct MOCNavigatorEntry {
    pub(crate) name: String,
    pub(crate) size: u64,
    pub(crate) created: NaiveDateTime,
    pub(crate) folder: bool,
}
impl MOCNavigatorEntry {
    fn from_csv_line(line: &str, path: &str) -> Option<Self> {
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


#[derive(Debug)]
pub(crate) struct MOCNavigatorRoot {
    pub(crate) name: String,
}
impl MOCNavigatorRoot {
    fn from_csv_line(line: &str) -> Option<Self> {
        let mut parts = line.split(',');
        let entry_type = parts.next()?;
        let name = parts.next()?;
        match entry_type {
            "R"|"r"|"root" => Some(Self { name: name.to_string() }),
            _ => None,
        }
    }
}

impl MOCNavigator {
    pub(crate) fn with_csv(data: &str) -> Self {
        Self {
            data: data.to_string(),
        }
    }
}   
impl Navigator<MOCNavigatorEntry, MOCNavigatorRoot> for MOCNavigator {
    fn entries(&self, path: &str) -> Vec<MOCNavigatorEntry> {
        let mut v = Vec::new();
        for line in self.data.lines() {
            if let Some(entry) = MOCNavigatorEntry::from_csv_line(line.trim(), path) {
                v.push(entry);
            }
        }  
        v 
    }
    fn roots(&self) -> Vec<MOCNavigatorRoot> {
        let mut v = Vec::new();
        for line in self.data.lines() {
            if let Some(root) = MOCNavigatorRoot::from_csv_line(line.trim()) {
                v.push(root);
            }
        }
        v
    }
}
impl NavigatorEntry for MOCNavigatorEntry {
    fn name(&self) -> &str {
        &self.name
    }
    fn is_container(&self) -> bool {
        self.folder
    }
}
impl NavigatorRoot for MOCNavigatorRoot {
    fn name(&self) -> &str {
        &self.name
    }
}