use crate::utils::NavigatorRoot;
#[derive(Debug)]
pub(crate) struct Root {
    pub(crate) name: String,
    pub(crate) size: u64,
    pub(crate) free_space: u64,
}
impl Root {
    pub(super) fn new(name: &str, size: u64, free_space: u64) -> Self {
        Self {
            name: name.to_string(),
            size,
            free_space,
        }
    }
    pub(super) fn from_csv_line(line: &str) -> Option<Self> {
        let mut parts = line.split(',');
        let entry_type = parts.next()?;
        let name = parts.next()?;
        let size = parts.next()?;
        let free_space = parts.next()?;
        match entry_type {
            "R"|"r"|"root" => {
                let size = size.parse().ok()?;
                let free_space = free_space.parse().ok()?;
                Some(Self {
                    name: name.to_string(),
                    size,
                    free_space,
                })
            },
            _ => None,
        }
    }
}
impl NavigatorRoot for Root {
    fn name(&self) -> &str {
        &self.name
    }
}