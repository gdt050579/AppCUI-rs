use std::ops::BitOrAssign;

const REPAINT_BIT: u8 = 1;
const UPDATE_VALUE_BIT: u8 = 2;
const PROCESSED_BY_COMPONENT_BIT: u8 = 4;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ProcessEventResult {
    value: u8,
}
impl ProcessEventResult {
    pub const Repaint: ProcessEventResult = ProcessEventResult {
        value: REPAINT_BIT | PROCESSED_BY_COMPONENT_BIT,
    };
    pub const Update: ProcessEventResult = ProcessEventResult {
        value: UPDATE_VALUE_BIT | PROCESSED_BY_COMPONENT_BIT,
    };
    pub const PassToControl: ProcessEventResult = ProcessEventResult { value: 0 };
    pub const PassToControlAndRepaint: ProcessEventResult = ProcessEventResult { value: REPAINT_BIT };
    pub const Processed: ProcessEventResult = ProcessEventResult {
        value: PROCESSED_BY_COMPONENT_BIT,
    };

    #[inline(always)]
    pub fn should_pass_to_control(&self) -> bool {
        (self.value & PROCESSED_BY_COMPONENT_BIT) == 0
    }
    #[inline(always)]
    pub fn should_repaint(&self) -> bool {
        (self.value & (REPAINT_BIT | UPDATE_VALUE_BIT)) != 0
    }
    #[inline(always)]
    pub fn should_update(&self) -> bool {
        (self.value & UPDATE_VALUE_BIT) != 0
    }
}
impl BitOrAssign for ProcessEventResult {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value
    }
}
