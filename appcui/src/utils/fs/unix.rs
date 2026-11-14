use super::{Root, RootType};
use std::{fs::File, io::BufRead, io::BufReader};

/// Returns `(total_size, free_space)` in bytes for the file system mounted at `mount_point`,
/// or `None` if `statvfs()` fails.
///
/// `struct statvfs` has a different layout on each OS / architecture / C library, so instead of
/// declaring it we let `statvfs()` fill a large, 8-byte aligned buffer and read the three fields
/// we need (`f_frsize`, `f_blocks`, `f_bfree`) at the offsets used by the current target.
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
fn disk_space(mount_point: &str) -> Option<(u64, u64)> {
    use std::ffi::{c_char, c_int, CString};

    // Offsets in `struct statvfs` on 64-bit Linux (same for glibc and musl):
    //   f_bsize: c_ulong @0, f_frsize: c_ulong @8, f_blocks: u64 @16, f_bfree: u64 @24 (112 bytes in total)
    const F_FRSIZE: usize = 8;
    const F_BLOCKS: usize = 16;
    const F_BFREE: usize = 24;

    /// Bigger than any `struct statvfs`, and aligned like it (its fields are 8-byte integers)
    #[repr(C, align(8))]
    struct StatVfsBuffer([u8; 1024]);

    extern "C" {
        fn statvfs(path: *const c_char, buf: *mut StatVfsBuffer) -> c_int;
    }

    let path = CString::new(mount_point).ok()?;
    let mut buf = StatVfsBuffer([0; 1024]);
    if unsafe { statvfs(path.as_ptr(), &mut buf) } != 0 {
        return None;
    }
    let read_u64 = |offset: usize| u64::from_ne_bytes(buf.0[offset..offset + 8].try_into().unwrap());

    let block_size = read_u64(F_FRSIZE);
    Some((read_u64(F_BLOCKS).saturating_mul(block_size), read_u64(F_BFREE).saturating_mul(block_size)))
}

/// Disk sizes are not read on this platform yet (macOS, 32-bit Linux): report 0 for both values.
#[cfg(not(any(all(target_os = "linux", target_pointer_width = "64"), target_arch = "wasm32")))]
fn disk_space(_mount_point: &str) -> Option<(u64, u64)> {
    Some((0, 0))
}

pub(super) fn get_os_roots() -> Vec<Root> {
    #[cfg(target_arch = "wasm32")]
    {
        // WASM doesn't have access to the file system, return empty roots
        return Vec::new();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut roots = Vec::new();

        // Open the /etc/mtab file to read mounted file systems
        if let Ok(file) = File::open("/etc/mtab") {
            let reader = BufReader::new(file);

            for line in reader.lines() {
                if let Ok(line) = line {
                    // Each line represents a mounted filesystem
                    let fields: Vec<&str> = line.split_whitespace().collect();

                    // The second field is the mount point
                    if fields.len() > 1 {
                        let device_name = fields[0];
                        let mount_point = fields[1];

                        if let Some((total_size, free_space)) = disk_space(mount_point) {
                            let root_type = if mount_point.starts_with("/media") || mount_point.starts_with("/mnt") {
                                RootType::Removable
                            } else if mount_point.starts_with("/dev") {
                                RootType::RamDisk
                            } else if mount_point.starts_with("/run") {
                                RootType::Network
                            } else {
                                RootType::Fixed
                            };

                            roots.push(Root {
                                path: mount_point.to_string(),
                                size: total_size,
                                free_space,
                                root_type,
                                name: device_name.to_string(),
                            });
                        }
                    }
                }
            }
        } else {
            eprintln!("Failed to open /etc/mtab");
        }
        roots
    }
}

#[test]
fn test_os_roots() {
    let roots = get_os_roots();
    println!("{:?}", roots);
}
