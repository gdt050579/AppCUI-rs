use crate::benchmark::{Benchmark, BenchmarkList, BenchmarkResult};
use crate::params::Params;
use crate::utils;
use appcui::prelude::*;

fn run_nested_panels(params: &Params) {
    let script = utils::basic_paint_script(params.times, params.show_screen);
    let count = params.elements_count;
    let _ = App::new()
        .debug_script(&script)
        .size(params.screen_size)
        .window(move || {
            let mut w = window!("'Nested Panels',d:f");
            let mut panels = Vec::new();
            for index in 0..count {
                let name = format!("Panel {}", index + 1);
                panels.push(Panel::new(&name, Layout::fill()));
            }
            let mut last = None;
            for mut panel in panels {
                if let Some(last_panel) = last {
                    panel.add(last_panel);
                    last = Some(panel);
                } else {
                    last = Some(panel);
                }
            }
            w.add(last.unwrap());
            w
        })
        .run();
}

struct TestNestedPanels {
    size: Size,
}
impl Benchmark for TestNestedPanels {
    fn evaluate(&mut self, params: &Params, results: &mut Vec<BenchmarkResult>) {
        self.size = params.screen_size;
        run_nested_panels(params);
        utils::update_times_for_runtime_execution(self, params.times, results);
    }
    fn name(&self) -> &'static str {
        "Panel::Nested"
    }
    fn description(&self) -> &'static str {
        "Nested panels inside a window"
    }
    fn size(&self) -> Size {
        self.size
    }
}

pub(crate) fn test_list() -> BenchmarkList {
    let mut benchmark_list = BenchmarkList::new();
    benchmark_list.add(Box::new(TestNestedPanels { size: Size::new(1, 1) }));
    benchmark_list
}
