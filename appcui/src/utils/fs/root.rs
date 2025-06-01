use crate::utils::NavigatorRoot;
use crate::prelude::*;

#[derive(Debug,Eq,PartialEq)]
pub(crate) enum RootType {
    Fixed,
    Removable,
    Network,
    RamDisk,
    CdRom,
    Unknown,
}
impl RootType {
    pub(crate) fn new(value: &str) -> Option<Self> {
        match value {
            "fixed" | "disk" | "d" | "D" | "f" | "F" => Some(Self::Fixed),
            "removable" | "usb" | "U" | "u" => Some(Self::Removable),
            "network" | "net" | "n" | "N" => Some(Self::Network),
            "ramdisk" | "ram" | "r" | "R" => Some(Self::RamDisk),
            "cdrom" | "cd" | "c" | "C" => Some(Self::CdRom),
            "unknown" | "?" => Some(Self::Unknown),
            _ => None,            
        }
    }
    pub(crate) fn icon(&self) -> char {
        match self {
            Self::Fixed => '💻',
            Self::Removable => '🔌',
            Self::Network => '🖧',
            Self::RamDisk => '▦',
            Self::CdRom => '📀',
            Self::Unknown => '❓',
        }
    }
}

#[derive(Debug, ListItem)]
pub(crate) struct Root {
    #[Column(name = "&Path", width = 10, index = 1)]
    pub(crate) path: String,
    #[Column(name = "&Name", width = 15, index = 4)]
    pub(crate) name: String,
    #[Column(name = "&Size", width = 10, align = right, render = Size, index = 2)]
    pub(crate) size: u64,
    #[Column(name = "&Free", width = 10, align = right, render = Size, index = 3)]
    pub(crate) free_space: u64,
    pub(crate) root_type: RootType,
}
impl Root {
    // pub(super) fn new(path: &str, name: &str, size: u64, free_space: u64, root_type: RootType) -> Self {
    //     Self {
    //         path: path.to_string(),
    //         name: name.to_string(),
    //         size,
    //         free_space,
    //         root_type
    //     }
    // }
    pub(super) fn from_csv_line(line: &str) -> Option<Self> {
        let mut parts = line.split(',');
        let entry_type = parts.next()?;
        let path = parts.next()?;
        let size = parts.next()?;
        let free_space = parts.next()?;
        let name = parts.next()?;
        let type_str = parts.next()?;
        match entry_type {
            "R"|"r"|"root" => {
                let size = size.parse().ok()?;
                let free_space = free_space.parse().ok()?;
                let root_type = RootType::new(type_str)?;
                Some(Self {
                    path: path.to_string(),
                    name: name.to_string(),
                    size,
                    free_space,
                    root_type,
                })
            },
            _ => None,
        }
    }
}
impl NavigatorRoot for Root {
    #[cfg(test)]
    fn path(&self) -> &str {
        &self.path
    }
}