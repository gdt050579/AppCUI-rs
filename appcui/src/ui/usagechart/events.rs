use crate::{system::Handle, ui::common::traits::EventProcessStatus, ui::usagechart::UsageChart};

pub trait UsageChartEvents {
    fn on_usage_segment_selected(&mut self, _handle: Handle<UsageChart>, _index: usize) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_clear_selection(&mut self, _handle: Handle<UsageChart>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) index: Option<u32>,
}