use super::structs::*;

extern "system" {
    #[warn(non_camel_case_types)]
    pub(crate) fn GetStdHandle(v: i32) -> HANDLE;
    #[warn(non_camel_case_types)]
    pub(crate) fn SetConsoleCursorPosition(handle: HANDLE, pos: COORD) -> BOOL;
    #[warn(non_camel_case_types)]
    pub(crate) fn SetConsoleCursorInfo(handle: HANDLE, info: &CONSOLE_CURSOR_INFO) -> BOOL;
    #[warn(non_camel_case_types)]
    pub(crate) fn GetConsoleMode(handle: HANDLE, mode_flags: &mut u32) -> BOOL;
    #[warn(non_camel_case_types)]
    pub(crate) fn SetConsoleMode(handle: HANDLE, mode_flags: u32) -> BOOL;
    #[warn(non_camel_case_types)]
    pub(crate) fn GetLastError() -> u32;
    #[warn(non_camel_case_types)]
    pub(crate) fn WriteConsoleOutputW(
        handle: HANDLE,
        lpBuffer: *const CHAR_INFO,
        dwBufferSize: COORD,
        dwBufferCoord: COORD,
        lpWriteRegion: &SMALL_RECT,
    );
    // #[warn(non_camel_case_types)]
    // pub(crate) fn WriteFile(handle: HANDLE, lpBuffer: *const u8, nNumberOfBytesToWrite: u32, lpNumberOfBytesWritten: &mut u32, lpOverlapped: *mut ()) -> BOOL;
    #[warn(non_camel_case_types)]
    pub(crate) fn GetConsoleScreenBufferInfo(handle: HANDLE, lpConsoleScreenBufferInfo: &mut CONSOLE_SCREEN_BUFFER_INFO) -> BOOL;

    #[warn(non_camel_case_types)]
    pub(crate) fn SetConsoleScreenBufferInfoEx(handle: HANDLE, lpConsoleScreenBufferInfoEx: &CONSOLE_SCREEN_BUFFER_INFOEX) -> BOOL;
    #[warn(non_camel_case_types)]
    pub(crate) fn GetConsoleScreenBufferInfoEx(handle: HANDLE, lpConsoleScreenBufferInfoEx: &mut CONSOLE_SCREEN_BUFFER_INFOEX) -> BOOL;

    #[warn(non_camel_case_types)]
    pub(crate) fn ReadConsoleInputW(handle: HANDLE, lpBuffer: *mut INPUT_RECORD, nLength: u32, lpNumberOfEventsRead: &mut u32) -> BOOL;

    #[warn(non_camel_case_types)]
    pub(crate) fn SetConsoleWindowInfo(handle: HANDLE, bAbsolute: BOOL, lpConsoleWindow: *const SMALL_RECT) -> BOOL;

    #[warn(non_camel_case_types)]
    pub(crate) fn SetConsoleScreenBufferSize(handle: HANDLE, new_size: COORD) -> BOOL;

    #[warn(non_camel_case_types)]
    pub(crate) fn SetConsoleTitleW(lpConsoleTitle: *const u16) -> BOOL;

    #[warn(non_camel_case_types)]
    pub(crate) fn OpenClipboard(hWndNewOwner: HANDLE) -> BOOL;

    #[warn(non_camel_case_types)]
    pub(crate) fn EmptyClipboard() -> BOOL;

    #[warn(non_camel_case_types)]
    pub(crate) fn CloseClipboard() -> BOOL;

    #[warn(non_camel_case_types)]
    pub(crate) fn SetClipboardData(uFormat: u32, hMem: HANDLE) -> HANDLE;

    #[warn(non_camel_case_types)]
    pub(crate) fn GetClipboardData(uFormat: u32) -> HANDLE;

    #[warn(non_camel_case_types)]
    pub(crate) fn IsClipboardFormatAvailable(uFormat: u32) -> BOOL;

    #[warn(non_camel_case_types)]
    pub(crate) fn GlobalAlloc(uFlags: u32, dwBytes: usize) -> HANDLE;

    #[warn(non_camel_case_types)]
    pub(crate) fn GlobalFree(hMem: HANDLE) -> HANDLE;

    #[warn(non_camel_case_types)]
    pub(crate) fn GlobalLock(hMem: HANDLE) -> *mut u8;

    #[warn(non_camel_case_types)]
    pub(crate) fn GlobalUnlock(hMem: HANDLE) -> BOOL;

}
