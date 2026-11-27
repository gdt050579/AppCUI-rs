use crate::benchmark::{Benchmark, BenchmarkResult};
use appcui::prelude::*;

pub(crate) fn basic_paint_script(times: u32) -> String {
    let mut script = "DisableChecks\nPaint.Enable(true)\n".to_string();
    script += "ForceRepaint\n".repeat(times as usize).as_str();
    script
}

pub(crate) fn update_times_for_runtime_execution<T>(obj: &mut T, times: u32, results: &mut Vec<BenchmarkResult>)
where
    T: Benchmark,
{
    let stats = App::statistics();
    results.push(BenchmarkResult::new(
        obj.name(),
        obj.description(),
        stats.run_time.as_millis(),
        times as usize,
    ));
    results.push(BenchmarkResult::new(
        format!("{} (update screen)", obj.name()).as_str(),
        "The time taken to update the screen",
        stats.update_screen_time.as_millis(),
        stats.update_screen_count,
    ));
}
