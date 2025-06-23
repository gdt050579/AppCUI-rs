use std::marker::PhantomData;

use crate::prelude::RuntimeManager;

use super::App;

/// The `Clipboard` structure provides system-wide clipboard functionality:
/// - Text-based clipboard operations
/// - Thread-safe clipboard access
/// - Platform-independent clipboard management
/// - Integration with terminal capabilities
pub struct Clipboard {
    _phantom: PhantomData<*mut ()>,
}

impl Clipboard {
    /// Clears the clipboard content
    pub fn clear() {
        if !App::is_created() {
            return;
        }
        RuntimeManager::get().backend_mut().set_clipboard_text("");
    }
    /// Checks if the clipboard has text
    pub fn has_text() -> bool {
        if !App::is_created() {
            return false;
        }
        RuntimeManager::get().backend().has_clipboard_text()
    }
    /// Sets the clipboard text
    pub fn set_text(text: &str) {
        if !App::is_created() {
            return;
        }
        RuntimeManager::get().backend_mut().set_clipboard_text(text);
    }
    /// Gets the clipboard text
    pub fn text() -> Option<String> {
        if !App::is_created() {
            return None;
        }
        RuntimeManager::get().backend_mut().clipboard_text()
    }
}
