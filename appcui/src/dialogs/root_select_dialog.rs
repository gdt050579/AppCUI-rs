use crate::prelude::*;
use std::path::PathBuf;
use crate::utils::fs::Root; 

#[ModalWindow(events: ButtonEvents+ListViewEvents<Root>+WindowEvents, response: PathBuf, internal: true)]
pub(super) struct RootSelectDialog {
    list: Handle<ListView<Root>>,
    b_ok: Handle<Button>,
    b_cancel: Handle<Button>,
}

impl WindowEvents for RootSelectDialog {

}

impl ButtonEvents for RootSelectDialog {
}

impl ListViewEvents<Root> for RootSelectDialog {

}