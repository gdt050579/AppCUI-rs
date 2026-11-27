use appcui::prelude::*;
use crate::benchmark::{Benchmark, BenchmarkList, BenchmarkResult};

struct TestDesktopCreate;

impl Benchmark for TestDesktopCreate {
    fn init(&mut self) {
    }
    fn run(&mut self) {
        let script = r#"
            Paint
        "#;
        let _ = App::new().debug_script(script).size(Size::new(100, 100)).run();
    }
    fn evaluate(&mut self, times: usize, results: &mut Vec<BenchmarkResult>) {
        let mut script = String::new();
        for _ in 0..times {
            script.push_str("Paint\n");
        }
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
        "Window::Create"
    }
    fn description(&self) -> &'static str {
        "Create a window"
    }
}

pub(crate) fn test_list() -> BenchmarkList {
    let mut benchmark_list = BenchmarkList::new();
    benchmark_list.add(Box::new(TestDesktopCreate { }));
    benchmark_list
}