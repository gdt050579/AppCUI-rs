use crate::benchmark::{Benchmark, BenchmarkList, BenchmarkResult};
use crate::utils;
use appcui::prelude::*;

fn run_overlapping_windows(count: i32, times: usize) {
    let script = utils::basic_paint_script(times);
    let mut app = App::new().debug_script(&script).size(Size::new(1000, 1000));
    for index in 0..count {
        app = app.window(move || {
            let title = format!("Window {}", index);
            let layout = LayoutBuilder::new().left_anchor(index).top_anchor(index).right_anchor(0).bottom_anchor(0).build();
            Window::new(&title, layout, window::Flags::Sizeable)
        });
    }
    let _ = app.run();
}

struct Test900WindowOverlapp;
impl Benchmark for Test900WindowOverlapp {
    fn evaluate(&mut self, times: usize, results: &mut Vec<BenchmarkResult>) {
        run_overlapping_windows(900, times);
        utils::update_times_for_runtime_execution(self, times, results);
    }
    fn name(&self) -> &'static str {
        "Window::900 Overlapping"
    }
    fn description(&self) -> &'static str {
        "900 windows overlapping each other on a 1000x1000 screen"
    }
}

struct Test100WindowOverlapp;
impl Benchmark for Test100WindowOverlapp {
    fn evaluate(&mut self, times: usize, results: &mut Vec<BenchmarkResult>) {
        run_overlapping_windows(100, times);
        utils::update_times_for_runtime_execution(self, times, results);
    }
    fn name(&self) -> &'static str {
        "Window::100 Overlapping"
    }
    fn description(&self) -> &'static str {
        "100 windows overlapping each other on a 1000x1000 screen"
    }
}
struct Test10WindowOverlapp;
impl Benchmark for Test10WindowOverlapp {
    fn evaluate(&mut self, times: usize, results: &mut Vec<BenchmarkResult>) {
        run_overlapping_windows(10, times);
        utils::update_times_for_runtime_execution(self, times, results);
    }
    fn name(&self) -> &'static str {
        "Window::10 Overlapping"
    }
    fn description(&self) -> &'static str {
        "10 windows overlapping each other on a 1000x1000 screen"
    }
}

pub(crate) fn test_list() -> BenchmarkList {
    let mut benchmark_list = BenchmarkList::new();
    benchmark_list.add(Box::new(Test900WindowOverlapp {}));
    benchmark_list.add(Box::new(Test100WindowOverlapp {}));
    benchmark_list.add(Box::new(Test10WindowOverlapp {}));
    benchmark_list
}
