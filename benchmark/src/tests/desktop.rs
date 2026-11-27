use appcui::prelude::*;
use crate::benchmark::{Benchmark, BenchmarkList, BenchmarkResult};

struct TestEmptyDesktop;

impl Benchmark for TestEmptyDesktop {
    fn evaluate(&mut self, times: usize, results: &mut Vec<BenchmarkResult>) {
        let mut script = "DisableChecks\nPaint.Enable(false)\n".to_string();
        script += "Paint\n".repeat(times).as_str();
        let _ = App::new().debug_script(&script).size(Size::new(100, 100)).run();
        let stats = App::statistics();
        results.push(BenchmarkResult::new(
            self.name(),
            self.description(),
            stats.run_time.as_millis(),
            times,
        ));
    }
    fn name(&self) -> &'static str {
        "Desktop::ForceRepaint"
    }
    fn description(&self) -> &'static str {
        "An empty desktop"
    }
}

pub(crate) fn test_list() -> BenchmarkList {
    let mut benchmark_list = BenchmarkList::new();
    benchmark_list.add(Box::new(TestEmptyDesktop { }));
    benchmark_list
}