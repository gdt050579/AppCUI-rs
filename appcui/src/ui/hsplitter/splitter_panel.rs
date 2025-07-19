use crate::prelude::*;

#[CustomControl(internal = true)]
pub(super) struct SplitterPanel {}
impl SplitterPanel {
    pub(super) fn new() -> Self {
        Self {
            base: ControlBase::with_status_flags(
                LayoutBuilder::new().x(0).y(0).width(1.0f32).height(1.0f32).build(),
                StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput,
            ),
        }
    }
}
