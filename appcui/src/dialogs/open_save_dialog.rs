use crate::prelude::*;

use super::FileInfo;

#[Window(events = ToggleButtonEvents+ButtonEvents, internal: true)]
struct FileExplorer {
    list: Handle<ListView<FileInfo>>,
    details: Handle<ToggleButton>,
    columns: Handle<ToggleButton>,
    name: Handle<TextField>,
    b_ok: Handle<Button>,
    b_cancel: Handle<Button>,
    mask: Handle<ComboBox>,
}
impl FileExplorer {

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