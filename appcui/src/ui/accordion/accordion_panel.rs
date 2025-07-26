use crate::prelude::*;

#[CustomControl(internal = true)]
pub(super) struct AccordionPanel {}
impl AccordionPanel {
    pub(super) fn new(visible: bool) -> Self {
        Self {
            base: ControlBase::with_status_flags(
                LayoutBuilder::new().x(0).y(0).width(1.0f32).height(1.0f32).build(),
                if visible {
                    StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput
                } else {
                    StatusFlags::Enabled | StatusFlags::AcceptInput
                },
            ),
        }
    }
}
