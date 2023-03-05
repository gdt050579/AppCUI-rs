use EnumBitFlags::EnumBitFlags;

use crate::{graphics::Size, terminal::TerminalType};

#[EnumBitFlags(bits = 32)]
pub enum InitializationFlags {
    CommandBar = 0x01,
    Menu = 0x02,
}

pub(super) struct InitializationData {
    pub(super) flags: InitializationFlags,
    pub(super) size: Option<Size>,
    pub(super) terminal: TerminalType,
}

impl InitializationData {
    pub(super) fn new() -> Self {
        Self {
            flags: InitializationFlags::None,
            size: None,
            terminal: TerminalType::Default,
        }
    }
    pub(super) fn with_flags(flags: InitializationFlags) -> Self {
        Self {
            flags,
            size: None,
            terminal: TerminalType::Default,
        }
    }
}
