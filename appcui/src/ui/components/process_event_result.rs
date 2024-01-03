#[derive(Copy,Clone,Eq,PartialEq)]
pub enum ProcessEventResult {
    Repaint,                
    Update,
    PassToControl,
    PassToControlAndRepaint,
    Processed
}
impl ProcessEventResult {
    #[inline(always)]
    pub fn should_pass_to_control(&self)->bool {
        match self {
            ProcessEventResult::PassToControl => true,
            ProcessEventResult::PassToControlAndRepaint => true,
            _ => false
        }
    }
}