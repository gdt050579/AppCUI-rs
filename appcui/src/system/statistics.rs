use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Statistics {
    pub init_time: Duration,
    pub run_time: Duration,
    pub uninit_time: Duration,
}
impl Statistics {
    pub(crate) const fn new() -> Self {
        Self {
            init_time: Duration::ZERO,
            run_time: Duration::ZERO,
            uninit_time: Duration::ZERO,
        }
    }
}
