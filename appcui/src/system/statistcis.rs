pub struct Statistics {
    pub init_tim: Duration,
    pub run_time: Duration,
}
impl Statistics {
    pub(crate) fn new() -> Self {
        Self { init_time: Duration::from_secs(0), run_time: Duration::from_secs(0) }
    }
}