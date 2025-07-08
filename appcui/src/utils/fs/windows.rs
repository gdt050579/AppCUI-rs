use super::{Root, RootType};

#[allow(clippy::upper_case_acronyms)]
type BOOL = u32;

extern "system" {
    #[warn(non_camel_case_types)]
    fn GetLogicalDrives() -> u32;
    #[warn(non_camel_case_types)]
    fn GetDiskFreeSpaceExA(
        lpDirectoryName: *const u8,
        lpFreeBytesAvailableToCaller: *mut u64,
        lpTotalNumberOfBytes: *mut u64,
        lpTotalNumberOfFreeBytes: *mut u64,
    ) -> BOOL;
    #[warn(non_camel_case_types)]
    fn GetDriveTypeA(lpRootPathName: *const u8) -> u32;
}

// const DRIVE_UNKNOWN: u32 = 0;
// const DRIVE_NO_ROOT_DIR: u32 = 1;
const DRIVE_REMOVABLE: u32 = 2;
const DRIVE_FIXED: u32 = 3;
const DRIVE_REMOTE: u32 = 4;
const DRIVE_CDROM: u32 = 5;
const DRIVE_RAMDISK: u32 = 6;

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
            let success = unsafe { GetDiskFreeSpaceExA(n.as_ptr(), std::ptr::null_mut(), &mut total, &mut free) != 0 };
            let drive_type = unsafe { GetDriveTypeA(n.as_ptr()) };
            let root_type = match drive_type {
                DRIVE_CDROM => RootType::CdRom,
                DRIVE_FIXED => RootType::Fixed,
                DRIVE_REMOVABLE => RootType::Removable,
                DRIVE_REMOTE => RootType::Network,
                DRIVE_RAMDISK => RootType::RamDisk,
                _ => RootType::Unknown,
            };
            if success {
                v.push(Root {
                    path: format!("{}:\\", (64u8 + i as u8) as char),
                    size: total,
                    free_space: free,
                    root_type,
                    name: String::new(),          // GDT: use GetVolumeInformationA to get the name/label of the drive
                });
            }
        }
        a *= 2;
    }
    v
}

#[test]
fn test_os_roots() {
    let roots = get_os_roots();
    println!("{roots:?}");
}
