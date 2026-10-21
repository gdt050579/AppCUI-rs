use crate::prelude::*;
use crate::ui::linechart::LineChart;

#[test]
fn check_creation() {
    let _chart = LineChart::<i32>::new(layout!("x:1,y:1,w:10,h:5"), linechart::Flags::None);
}

#[test]
fn check_creation_with_flags() {
    let chart = LineChart::<f64>::new(layout!("x:1,y:1,w:10,h:5"), linechart::Flags::ShowLegend);
    assert!(chart.flags().contains(linechart::Flags::ShowLegend));
}

#[test]
fn check_creation_in_window() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
    ";
    App::new()
        .size(Size::new(30, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(LineChart::<u32>::new(layout!("d:f"), linechart::Flags::None));
            w
        })
        .run()
        .unwrap();
}
