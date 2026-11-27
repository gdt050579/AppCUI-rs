use appcui::prelude::*;
use crate::utils;
use crate::benchmark::{Benchmark, BenchmarkList, BenchmarkResult};

struct TestEmptyDesktop;

impl Benchmark for TestEmptyDesktop {
    fn evaluate(&mut self, times: usize, results: &mut Vec<BenchmarkResult>) {
        let script = utils::basic_paint_script(times);
        let _ = App::new().debug_script(&script).size(Size::new(1000, 1000)).run();
        utils::update_times_for_runtime_execution(self, times, results);
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