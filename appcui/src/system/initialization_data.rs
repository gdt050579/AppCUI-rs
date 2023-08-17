use EnumBitFlags::EnumBitFlags;

use crate::{graphics::Size, terminals::TerminalType, ui::{common::ControlManager, Desktop}};

#[EnumBitFlags(bits = 32)]
pub enum InitializationFlags {
    CommandBar = 0x01,
    Menu = 0x02,
}

pub(crate) struct InitializationData {
    pub(crate) flags: InitializationFlags,
    pub(crate) size: Option<Size>,
    pub(crate) terminal: TerminalType,
    pub(crate) debug_script: String,
    pub(crate) desktop_manager: ControlManager,
}

impl InitializationData {
    pub(crate) fn new(terminal: TerminalType, size: Option<Size>, flags: InitializationFlags) -> Self {
        Self {
            flags,
            size,
            terminal,
            debug_script: String::new(),
            desktop_manager: ControlManager::new(Desktop::new()),
        }
    }
    pub(crate) fn with_flags(flags: InitializationFlags) -> Self {
        Self {
            flags,
            size: None,
            terminal: TerminalType::Default,
            debug_script: String::new(),
            desktop_manager: ControlManager::new(Desktop::new())
        }
    }
}
impl Default for InitializationData {
    fn default() -> Self {
        Self {
            flags: InitializationFlags::None,
            size: None,
            terminal: TerminalType::Default,
            debug_script: String::new(),
            desktop_manager: ControlManager::new(Desktop::new())
        }
    }
}
