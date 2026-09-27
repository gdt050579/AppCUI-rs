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
        let log_entry = format!("[{timestamp}] [{tag}] {message}\n");
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

/// Writes a tagged log message in debug builds.
///
/// Call [`log_file`](crate::system::MultiWindowAppBuilder::log_file) when building
/// the application to choose the output file (native) or to initialize logging (WASM).
/// In **debug** builds the message is formatted and written; in **release** builds this
/// macro expands to nothing (no formatting, no I/O).
///
/// Native targets append a line of the form `[YYYY-MM-DD HH:MM:SS] [tag] message`
/// to the configured file. WASM targets write the same format to the browser console.
/// If logging was never enabled, the call is a no-op.
///
/// # Examples
///
/// ```rust
/// use appcui::prelude::*;
/// use appcui::log;
///
/// log!("INFO", "Application started");
/// let x = 10;
/// log!("INFO", "The value of x is: {}", x);
/// ```
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
