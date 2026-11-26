pub(crate) trait Benchmark {
    fn init(&mut self);
    fn run(&mut self);
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
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
    pub(crate) fn run(&mut self, name: &str, times: usize) {
        for b in &mut self.list {
            if b.name().to_lowercase().contains(&name.to_lowercase()) {
                let mut total_time = 0;
                b.init();
                let start = std::time::Instant::now();
                for _ in 0..times {
                    b.run();
                }
                let end = std::time::Instant::now();
                total_time += end.duration_since(start).as_millis();
                println!("{}: {}ms", b.name(), total_time / times as u128);
            }
        }
    }
}