use libc::statvfs;
use std::{fs::File, io::BufRead, io::BufReader};
use super::{RootType, Root};

pub(super) fn get_os_roots() -> Vec<Root> {
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

                    // Call statvfs to get filesystem stats
                    let mut stats: statvfs = unsafe { std::mem::zeroed() };
                    let c_mount_point = std::ffi::CString::new(mount_point).unwrap();

                    let result = unsafe { statvfs(c_mount_point.as_ptr(), &mut stats) };

                    if result == 0 {
                        // Calculate sizes
                        let total_size = stats.f_blocks as u64 * stats.f_frsize as u64;
                        let free_space = stats.f_bfree as u64 * stats.f_frsize as u64;

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

#[test]
fn test_os_roots() {
    let roots = get_os_roots();
    println!("{:?}", roots);
}