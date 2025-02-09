use std::path;
use std::path::PathBuf;
use super::{RootType, Root};

#[allow(clippy::upper_case_acronyms)]
type BOOL = u32;

extern "system" {
    #[warn(non_camel_case_types)]
    fn GetLogicalDrives() -> u32;
    fn GetDiskFreeSpaceExA(
        lpDirectoryName: *const u8,
        lpFreeBytesAvailableToCaller: *mut u64,
        lpTotalNumberOfBytes: *mut u64,
        lpTotalNumberOfFreeBytes: *mut u64,
    ) -> BOOL;
}

pub(super) fn get_os_roots() -> Vec<Root> {
    let result = unsafe { GetLogicalDrives() };
    let mut v = Vec::new();

    let mut a = 1;
    for i in 1..32 {
        if result & a != 0 {
            // TODO:
            let n = [64u8 + i as u8, b':', b'\\', 0];
            let mut free: u64 = 0;
            let mut total: u64 = 0;
            let success = unsafe {
                GetDiskFreeSpaceExA(n.as_ptr(), std::ptr::null_mut(), &mut total, &mut free) != 0
            };
            if success {
                v.push(Root {
                    path: format!("{}:\\", (64u8 + i as u8) as char),
                    size: total,
                    free_space: free,
                    root_type: RootType::Unknown, //GDT: use GetDriveTypeA to find the type !!!
                    name: String::new(), // GDT: use GetVolumeInformationA to get the name/label of the drive
                });
            }
        }
        a *= 2;
    }
    v
}

pub(super) fn get_os_absolute_path(path: &PathBuf) -> Option<PathBuf> {
    let buf = path.as_os_str().as_encoded_bytes();
    if buf.len() == 2 && buf[1] == b':' && ((buf[0] >= b'A' && buf[0] <= b'Z') || (buf[0] >= b'a' && buf[0] <= b'z')) {
        // special root form like "D:"
        let mut tmp = path.to_path_buf();
        tmp.push("\\");
        return Some(tmp);
    }

    if path.is_absolute() {
        return Some(path.to_path_buf());
    }
    path::absolute(path).ok()
}

pub(super) fn get_os_separator() -> char {
    '\\'
}

pub(super) fn is_fs_root(path: &str) -> bool {
    let buf = path.as_bytes();
    buf.len() >= 2 && buf[1] == b':' && ((buf[0] >= b'A' && buf[0] <= b'Z') || (buf[0] >= b'a' && buf[0] <= b'z'))
}