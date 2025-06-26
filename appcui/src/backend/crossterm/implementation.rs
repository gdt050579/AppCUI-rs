use std::sync::mpsc::Sender;
use crate::{
    backend::Backend,
    system::{Error, SystemEvent},
};
use crossterm::{
    execute,
    style::Print,
    terminal::{enable_raw_mode, disable_raw_mode},
};
use super::input::Input;

pub(crate) struct CrossTerm {}

impl CrossTerm {
    pub(crate) fn new(builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Self, Error> {
        todo!()
    }
}

impl Backend for CrossTerm {
    fn update_screen(&mut self, surface: &crate::prelude::Surface) {
        todo!()
    }

    fn on_resize(&mut self, new_size: crate::prelude::Size) {
        todo!()
    }

    fn size(&self) -> crate::prelude::Size {
        todo!()
    }

    fn clipboard_text(&self) -> Option<String> {
        todo!()
    }

    fn set_clipboard_text(&mut self, text: &str) {
        todo!()
    }

    fn has_clipboard_text(&self) -> bool {
        todo!()
    }

    fn is_single_threaded(&self) -> bool {
        todo!()
    }
}
