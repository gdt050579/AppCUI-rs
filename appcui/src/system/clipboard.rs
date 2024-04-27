use std::marker::PhantomData;

use crate::prelude::RuntimeManager;

use super::App;

pub struct Clipboard {
    _phantom: PhantomData<*mut ()>,
}

impl Clipboard {
    pub fn clear() {
        if !App::is_created() {
            return;
        }
        RuntimeManager::get().terminal_mut().set_clipboard_text("");
    }
    pub fn has_text() -> bool {
        if !App::is_created() {
            return false;
        }
        RuntimeManager::get().terminal().has_clipboard_text()
    }
    pub fn set_text(text: &str) {
        if !App::is_created() {
            return;
        }
        RuntimeManager::get().terminal_mut().set_clipboard_text(text);
    }
    pub fn text() -> Option<String> {
        if !App::is_created() {
            return None;
        }
        RuntimeManager::get().terminal_mut().get_clipboard_text()
    }
}
