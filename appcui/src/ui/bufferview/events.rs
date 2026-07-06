use std::any::TypeId;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

/// Event handlers for [`super::BufferView`] notifications.
///
/// Implement this trait on a window or other event receiver to react when the user moves
/// the cursor or changes the selection inside a buffer view control.
pub trait GenericBufferViewEvents {
    /// Called when the current byte position changes inside a [`super::BufferView`].
    fn on_current_pos_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when the selected byte range changes inside a [`super::BufferView`].
    fn on_selection_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}


#[derive(Copy, Clone)]
pub(crate) enum BufferViewEventTypes {
    CurrentPosChanged,
    SelectionChanged,
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) event_type: BufferViewEventTypes,
    pub(crate) type_id: std::any::TypeId,
}
