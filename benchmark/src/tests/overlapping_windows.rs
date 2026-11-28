use crate::benchmark::{Benchmark, BenchmarkList, BenchmarkResult};
use crate::utils;
use crate::params::Params;
use appcui::prelude::*;

fn run_overlapping_windows(count: i32, params: &Params) {
    let script = utils::basic_paint_script(params.times, params.show_screen);
    let mut app = App::new().debug_script(&script).size(params.screen_size);
    for index in 0..count {
        app = app.window(move || {
            let title = format!("Window {}", index);
            let layout = LayoutBuilder::new().left_anchor(index % 10).top_anchor(index % 10).right_anchor(0).bottom_anchor(0).build();
            Window::new(&title, layout, window::Flags::Sizeable)
        });
    }
    let _ = app.run();
}

struct Test900WindowOverlapp {
    size: Size,
}
impl Benchmark for Test900WindowOverlapp {
    fn evaluate(&mut self, params: &Params, results: &mut Vec<BenchmarkResult>) {
        self.size = params.screen_size;
        run_overlapping_windows(900, params);
        utils::update_times_for_runtime_execution(self, params.times, results);
    }
    fn name(&self) -> &'static str {
        "Window::900 Overlapping"
    }
    fn description(&self) -> &'static str {
        "900 windows overlapping each other on a 1000x1000 screen"
    }
    fn size(&self) -> Size {
        self.size
    }
}

struct Test100WindowOverlapp {
    size: Size,
}
impl Benchmark for Test100WindowOverlapp {
    fn evaluate(&mut self, params: &Params, results: &mut Vec<BenchmarkResult>) {
        self.size = params.screen_size;
        run_overlapping_windows(100, params);
        utils::update_times_for_runtime_execution(self, params.times, results);
    }
    fn name(&self) -> &'static str {
        "Window::100 Overlapping"
    }
    fn description(&self) -> &'static str {
        "100 windows overlapping each other on a 1000x1000 screen"
    }
    fn size(&self) -> Size {
        self.size
    }
}
struct Test10WindowOverlapp {
    size: Size,
}
impl Benchmark for Test10WindowOverlapp {
    fn evaluate(&mut self, params: &Params, results: &mut Vec<BenchmarkResult>) {
        self.size = params.screen_size;
        run_overlapping_windows(10, params);
        utils::update_times_for_runtime_execution(self, params.times, results);
    }
    fn name(&self) -> &'static str {
        "Window::10 Overlapping"
    }
    fn description(&self) -> &'static str {
        "10 windows overlapping each other on a 1000x1000 screen"
    }
    fn size(&self) -> Size {
        self.size
    }
}

pub(crate) fn test_list() -> BenchmarkList {
    let mut benchmark_list = BenchmarkList::new();
    benchmark_list.add(Box::new(Test900WindowOverlapp { size: Size::new(1, 1) }));
    benchmark_list.add(Box::new(Test100WindowOverlapp { size: Size::new(1, 1) }));
    benchmark_list.add(Box::new(Test10WindowOverlapp { size: Size::new(1, 1) }));
    benchmark_list
}
