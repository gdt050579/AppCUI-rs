use crate::prelude::*;
use super::FileInfo;

#[Window(events = ToggleButtonEvents+ButtonEvents, internal: true)]
pub(super) struct FileExplorer {
    list: Handle<ListView<FileInfo>>,
    details: Handle<ToggleButton>,
    columns: Handle<ToggleButton>,
    name: Handle<TextField>,
    b_ok: Handle<Button>,
    b_cancel: Handle<Button>,
    mask: Handle<ComboBox>,
}
impl FileExplorer {
    pub(super) fn new(title: &str)->Self {
        let mut w = Self {
            base: Window::new(title, Layout::new("d:c,w:70,h:20"), window::Flags::Sizeable),
            list: Handle::None,
            details: Handle::None,
            columns: Handle::None,
            name: Handle::None,
            b_ok: Handle::None,
            b_cancel: Handle::None,
            mask: Handle::None,            
        };
        w.set_size_bounds(40, 10, u16::MAX, u16::MAX);
        w       
    }
}
impl ButtonEvents for FileExplorer {
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl ToggleButtonEvents for FileExplorer {
    fn on_selection_changed(&mut self, _handle: Handle<ToggleButton>, _selected: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}