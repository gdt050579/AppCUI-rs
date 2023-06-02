#[repr(u8)]
#[derive(Debug,Copy,Clone)]
pub enum Error {
    InvalidSize,
    FailToGetStdInOutHandler,
    GetConsoleModeFailed,
    SetConsoleModeFailed,
    GetConsoleScreenBufferInfoFailed,
    ScriptParsingError,
    AppAlreadyStarted,
}