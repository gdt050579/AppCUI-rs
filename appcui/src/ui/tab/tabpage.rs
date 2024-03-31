use crate::prelude::*;

#[CustomControl(internal = true)]
pub(super) struct TabPage {}
impl TabPage {
    pub(super) fn new() -> Self {
        Self {
            base: ControlBase::with_status_flags(
                Layout::new("d:c,w:100%,h:100%"),
                StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput,
            ),
        }
    }
}
