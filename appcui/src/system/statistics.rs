use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Statistics {
    pub init_time: Duration,
    pub run_time: Duration,
    pub uninit_time: Duration,
    pub update_screen_time: Duration,
    pub update_screen_count: usize,
}
impl Statistics {
    pub(crate) const fn new() -> Self {
        Self {
            init_time: Duration::ZERO,
            run_time: Duration::ZERO,
            uninit_time: Duration::ZERO,
            update_screen_time: Duration::ZERO,
            update_screen_count: 0,
        }
    }
}
