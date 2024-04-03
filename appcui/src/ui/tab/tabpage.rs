use crate::prelude::*;

#[CustomControl(internal = true)]
pub(super) struct TabPage {}
impl TabPage {
    pub(super) fn new(visible: bool) -> Self {
        Self {
            base: ControlBase::with_status_flags(
                Layout::new("x:0,y:0,w:100%,h:100%,a:tl"),
                if visible {
                    StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput
                } else {
                    StatusFlags::Enabled | StatusFlags::AcceptInput
                },
            ),
        }
    }
}
