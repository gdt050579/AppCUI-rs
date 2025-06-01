#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub(crate) enum Command {
    None,
    Start(u32),
    Stop,
    Pause,
    Resume,
    SetInterval(u32),
}

impl Command {
    pub(super) fn iterval(&self) -> Option<u32> {
        match self {
            Command::Start(interval) => Some(*interval),
            Command::SetInterval(interval) => Some(*interval),
            _ => None,
        }
    }
}
