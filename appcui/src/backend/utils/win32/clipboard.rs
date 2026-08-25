use super::{api, constants};

pub(crate) struct Clipboard;

impl Clipboard {
    pub(crate) fn text() -> Option<String> {
        unsafe {
            if api::OpenClipboard(0) == constants::FALSE {
                return None;
            }

            let hmem = api::GetClipboardData(constants::CF_UNICODETEXT);
            if hmem == 0 {
                api::CloseClipboard();
                return None;
            }
            let ptr = api::GlobalLock(hmem) as *mut u16;
            if ptr.is_null() {
                api::CloseClipboard();
                return None;
            }
            let byte_size = api::GlobalSize(hmem);
            let max_chars = byte_size / std::mem::size_of::<u16>();
            let mut len = 0usize;
            while len < max_chars && *ptr.add(len) != 0 {
                len += 1;
            }
            let s = String::from_utf16_lossy(std::slice::from_raw_parts(ptr as *const u16, len));
            api::GlobalUnlock(hmem);
            api::CloseClipboard();
            Some(s)
        }
    }

    pub(crate) fn set_text(text: &str) {
        if text.is_empty() {
            unsafe {
                if api::OpenClipboard(0) != constants::FALSE {
                    api::EmptyClipboard();
                    api::CloseClipboard();
                }
            }
        } else {
            unsafe {
                if api::OpenClipboard(0) == constants::FALSE {
                    return;
                }

                api::EmptyClipboard();

                let buffer: Vec<u16> = text.encode_utf16().collect();
                let len = buffer.len() + 1;

                let hmem = api::GlobalAlloc(constants::GMEM_MOVEABLE, len * 2);
                if hmem == 0 {
                    api::CloseClipboard();
                    return;
                }

                let mut ptr = api::GlobalLock(hmem) as *mut u16;
                if ptr.is_null() {
                    api::CloseClipboard();
                    api::GlobalFree(hmem);
                    return;
                }

                for value in buffer {
                    *ptr = value;
                    ptr = ptr.add(1);
                }
                *ptr = 0;

                api::GlobalUnlock(hmem);

                if api::SetClipboardData(constants::CF_UNICODETEXT, hmem) == 0 {
                    api::CloseClipboard();
                    api::GlobalFree(hmem);
                    return;
                }

                api::CloseClipboard();
            }
        }
    }

    pub(crate) fn has_text() -> bool {
        unsafe {
            (api::IsClipboardFormatAvailable(constants::CF_TEXT) != constants::FALSE)
                || (api::IsClipboardFormatAvailable(constants::CF_UNICODETEXT) != constants::FALSE)
        }
    }
}
