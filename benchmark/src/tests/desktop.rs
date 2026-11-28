use appcui::prelude::*;
use crate::params::Params;
use crate::utils;
use crate::benchmark::{Benchmark, BenchmarkList, BenchmarkResult};

struct TestEmptyDesktop {
    size: Size,
}

impl Benchmark for TestEmptyDesktop {
    fn evaluate(&mut self, params: &Params, results: &mut Vec<BenchmarkResult>) {
        let script = utils::basic_paint_script(params.times);
        self.size = params.screen_size;
        let _ = App::new().debug_script(&script).size(params.screen_size).run();
        utils::update_times_for_runtime_execution(self, params.times, results);
    }
    fn name(&self) -> &'static str {
        "Desktop::ForceRepaint"
    }
    fn description(&self) -> &'static str {
        "An empty desktop"
    }
    fn size(&self) -> Size {
        self.size
    }
}

pub(crate) fn test_list() -> BenchmarkList {
    let mut benchmark_list = BenchmarkList::new();
    benchmark_list.add(Box::new(TestEmptyDesktop { size: Size::new(1, 1) }));
    benchmark_list
}