use crate::benchmark::{Benchmark, BenchmarkList, BenchmarkResult};
use crate::utils;
use crate::params::Params;
use appcui::prelude::*;

fn run_overlapping_windows(params: &Params) {
    let script = utils::basic_paint_script(params.times, params.show_screen);
    let mut app = App::new().debug_script(&script).size(params.screen_size);
    for index in 0..params.elements_count {
        app = app.window(move || {
            let title = format!("Window {}", index);
            let layout = LayoutBuilder::new().left_anchor(index % 10).top_anchor(index % 10).right_anchor(0).bottom_anchor(0).build();
            Window::new(&title, layout, window::Flags::Sizeable)
        });
    }
    let _ = app.run();
}

struct TestWindowOverlapp {
    size: Size,
}
impl Benchmark for TestWindowOverlapp {
    fn evaluate(&mut self, params: &Params, results: &mut Vec<BenchmarkResult>) {
        self.size = params.screen_size;
        run_overlapping_windows(params);
        utils::update_times_for_runtime_execution(self, params.times, results);
    }
    fn name(&self) -> &'static str {
        "Window::Overlapping"
    }
    fn description(&self) -> &'static str {
        "Windows overlapping each other on a screen"
    }
    fn size(&self) -> Size {
        self.size
    }
}

pub(crate) fn test_list() -> BenchmarkList {
    let mut benchmark_list = BenchmarkList::new();
    benchmark_list.add(Box::new(TestWindowOverlapp { size: Size::new(1, 1) }));
    benchmark_list
}
