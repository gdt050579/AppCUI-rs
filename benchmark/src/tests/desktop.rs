use appcui::prelude::*;
use crate::benchmark::{Benchmark, BenchmarkList, BenchmarkResult};

struct TestEmptyDesktop;

impl Benchmark for TestEmptyDesktop {
    fn evaluate(&mut self, times: usize, results: &mut Vec<BenchmarkResult>) {
        let mut script = "DisableChecks\nPaint.Enable(true)\n".to_string();
        script += "ForceRepaint\n".repeat(times).as_str();
        let _ = App::new().debug_script(&script).size(Size::new(100, 100)).run();
        let stats = App::statistics();
        results.push(BenchmarkResult::new(
            self.name(),
            self.description(),
            stats.run_time.as_millis(),
            times,
        ));
        results.push(BenchmarkResult::new(
            "Screen Update Time",
            "The time taken to update the screen",
            stats.update_screen_time.as_millis(),
            stats.update_screen_count,
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