use std::{cell::RefCell, sync::Mutex};

static LOG_FILE: Mutex<RefCell<Option<std::fs::File>>> = Mutex::new(RefCell::new(None));

#[cfg(debug_assertions)]
pub(crate) fn init_log_file(name: &str, append: bool) {
    use std::fs::OpenOptions;

    if let Ok(file) = OpenOptions::new().create(true).write(true).append(append).open(name) {        
        LOG_FILE.lock().unwrap().replace(Some(file));
        write_log_to_file("INFO", "Application started");
    }
}

#[cfg(debug_assertions)]
pub fn write_log_to_file(tag: &str, message: &str) {
    use std::io::Write;
    use chrono::Local;
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_entry = format!("[{}] [{}] {}\n", timestamp, tag, message);
    if let Ok(mut guard) = LOG_FILE.lock() {
        if let Some(file) = guard.get_mut() {
            file.write_all(log_entry.as_bytes()).unwrap();
            let _ = file.flush();
        }
    }
}

#[macro_export]
macro_rules! log {
    ($tag:expr, $fmt:expr, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            let msg = format!($fmt, $($arg)*);
            write_log_to_file($tag, &msg);
        }
    };
}
