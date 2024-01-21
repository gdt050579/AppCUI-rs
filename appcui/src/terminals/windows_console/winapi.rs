use super::structs::*;

extern "system" {
    #[warn(non_camel_case_types)]
    pub(super) fn GetStdHandle(v: i32) -> HANDLE;
    #[warn(non_camel_case_types)]
    pub(super) fn SetConsoleCursorPosition(handle: HANDLE, pos: COORD) -> BOOL;
    #[warn(non_camel_case_types)]
    pub(super) fn SetConsoleCursorInfo(handle: HANDLE, info: &CONSOLE_CURSOR_INFO) -> BOOL;
    #[warn(non_camel_case_types)]
    pub(super) fn GetConsoleMode(handle: HANDLE, mode_flags: &mut u32) -> BOOL;
    #[warn(non_camel_case_types)]
    pub(super) fn SetConsoleMode(handle: HANDLE, mode_flags: u32) -> BOOL;
    #[warn(non_camel_case_types)]
    pub(super) fn GetLastError() -> u32;
    #[warn(non_camel_case_types)]
    pub(super) fn WriteConsoleOutputW(
        handle: HANDLE,
        lpBuffer: *const CHAR_INFO,
        dwBufferSize: COORD,
        dwBufferCoord: COORD,
        lpWriteRegion: &SMALL_RECT,
    );
    #[warn(non_camel_case_types)]
    pub(super) fn GetConsoleScreenBufferInfo(handle: HANDLE, lpConsoleScreenBufferInfo: &mut CONSOLE_SCREEN_BUFFER_INFO) -> BOOL;

    #[warn(non_camel_case_types)]
    pub(super) fn ReadConsoleInputW(handle: HANDLE, lpBuffer: *mut INPUT_RECORD, nLength: u32, lpNumberOfEventsRead: &mut u32) -> BOOL;

    #[warn(non_camel_case_types)]
    pub(super) fn SetConsoleWindowInfo(handle: HANDLE, bAbsolute: BOOL, lpConsoleWindow: *const SMALL_RECT)->BOOL;

    #[warn(non_camel_case_types)]
    pub(super) fn SetConsoleScreenBufferSize(handle: HANDLE, new_size: COORD) -> BOOL;

    #[warn(non_camel_case_types)]
    pub(super) fn SetConsoleTitleW(lpConsoleTitle: *const u16) -> BOOL;

}
