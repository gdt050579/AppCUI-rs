#[cfg(not(target_arch = "wasm32"))]
use std::{cell::RefCell, sync::Mutex};

#[cfg(not(target_arch = "wasm32"))]
static LOG_FILE: Mutex<RefCell<Option<std::fs::File>>> = Mutex::new(RefCell::new(None));

#[cfg(all(not(target_arch = "wasm32"), debug_assertions))]
pub(crate) fn init_log_file(name: &str, append: bool) {
    use std::fs::OpenOptions;

    if let Ok(file) = OpenOptions::new().create(true).write(true).append(append).open(name) {
        LOG_FILE.lock().unwrap().replace(Some(file));
        write_log_to_file("INFO", "Application started");
    }
}

#[cfg(debug_assertions)]
pub fn write_log_to_file(tag: &str, message: &str) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use chrono::Local;
        use std::io::Write;
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!("[{}] [{}] {}\n", timestamp, tag, message);
        if let Ok(mut guard) = LOG_FILE.lock() {
            if let Some(file) = guard.get_mut() {
                file.write_all(log_entry.as_bytes()).unwrap();
                let _ = file.flush();
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        use js_sys::Date;
        let ts = Date::new_0().to_iso_string().as_string().unwrap_or_else(|| "unknown time".into());
        web_sys::console::log_1(&format!("[{}] [{}] {}", ts, tag, message).into());
    }
}

#[cfg(all(target_arch = "wasm32", debug_assertions))]
pub(crate) fn init_log_file(name: &str, append: bool) {
    web_sys::console::log_1(&format!("📝 wasm log initialized: {}", name).into());
}

#[macro_export]
macro_rules! log {
    ($tag:literal, $fmt:literal) => {
        #[cfg(debug_assertions)]
        {
            write_log_to_file($tag,$fmt);
        }
    };
    ($tag:literal, $fmt:literal, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            let msg = format!($fmt, $($arg)*);
            write_log_to_file($tag, &msg);
        }
    };
}
