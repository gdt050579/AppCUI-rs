use crate::params::Params;
pub(crate) trait Benchmark {
    fn init(&mut self) {}
    fn run(&mut self) {}
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn evaluate(&mut self, params: &Params, results: &mut Vec<BenchmarkResult>) {
        self.init();
        let start = std::time::Instant::now();
        for _ in 0..params.times {
            self.run();
        }
        let end = std::time::Instant::now();
        results.push(BenchmarkResult::new(
            self.name(),
            self.description(),
            end.duration_since(start).as_millis(),
            params.times as usize,
        ));
    }
}
pub(crate) struct BenchmarkList {
    list: Vec<Box<dyn Benchmark>>,
}
impl BenchmarkList {
    pub(crate) fn new() -> Self {
        Self { list: Vec::new() }
    }
    pub(crate) fn add(&mut self, benchmark: Box<dyn Benchmark>) {
        self.list.push(benchmark);
    }
    pub(crate) fn expand(&mut self, lst: BenchmarkList) {
        self.list.extend(lst.list);
    }
    pub(crate) fn list(&self) -> &[Box<dyn Benchmark>] {
        &self.list
    }
    pub(crate) fn list_mut(&mut self) -> &mut [Box<dyn Benchmark>] {
        &mut self.list
    }    
}
#[derive(Debug, Clone)]
pub(crate) struct BenchmarkResult {
    name: String,
    description: String,
    duration: u128,
    times: usize,
}
impl BenchmarkResult {
    pub(crate) fn new(name: &str, description: &str, duration: u128, times: usize) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            duration,
            times,
        }
    }
}
