use libc::{statvfs, statvfs64};
use std::{ffi::CStr, fs::File, io::BufRead, io::BufReader};

pub(super) fn get_os_roots() -> Vec<Root> {
    // println!("{:<30} {:<15} {:<15} {:<15}", "Mount Point", "Total Size", "Free Space", "Used Space");

    // Open the /etc/mtab file to read mounted file systems
    if let Ok(file) = File::open("/etc/mtab") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                // Each line represents a mounted filesystem
                let fields: Vec<&str> = line.split_whitespace().collect();

                // The second field is the mount point
                if fields.len() > 1 {
                    let mount_point = fields[1];

                    // Call statvfs to get filesystem stats
                    let mut stats: statvfs = unsafe { std::mem::zeroed() };
                    let c_mount_point = std::ffi::CString::new(mount_point).unwrap();

                    let result = unsafe { statvfs64(c_mount_point.as_ptr(), &mut stats) };

                    if result == 0 {
                        // Calculate sizes
                        let total_size = stats.f_blocks * stats.f_frsize as u64;
                        let free_space = stats.f_bfree * stats.f_frsize as u64;
                        let used_space = total_size - free_space;

                        // println!(
                        //     "{:<30} {:<15} {:<15} {:<15}",
                        //     mount_point,
                        //     format!("{} MB", total_size / 1_048_576),
                        //     format!("{} MB", free_space / 1_048_576),
                        //     format!("{} MB", used_space / 1_048_576)
                        // );
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to open /etc/mtab");
    }
}