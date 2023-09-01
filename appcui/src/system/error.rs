#[repr(u8)]
#[derive(Debug,Copy,Clone)]
pub enum Error {
    InvalidSize,
    FailToGetStdInHandle,
    FailToGetStdOutHandle,
    GetConsoleModeFailed,
    SetConsoleModeFailed,
    GetConsoleScreenBufferInfoFailed,
    ScriptParsingError,
    AppAlreadyStarted,
}