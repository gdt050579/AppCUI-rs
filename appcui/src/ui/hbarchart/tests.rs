use crate::prelude::*;
use crate::ui::hbarchart::HBarChart;

#[test]
fn check_creation() {
    let chart = HBarChart::<i32>::new(layout!("x:1,y:1,w:10,h:5"), hbarchart::Flags::None);
    assert_eq!(chart.bars_count(), 0);
    assert_eq!(chart.selected_bar(), None);
}

#[test]
fn check_macro_and_bar_api() {
    let script = "Paint.Enable(false)";
    App::new().size(Size::new(40, 12)).debug_script(script).window(|| {
        let mut chart = hbarchart!(
            "type: i32, x:1, y:1, w:30, h:10, flags: ScrollBars+ShowZeroLineOnXAxis, scale: Fixed(0, 100), dbw: 2, space: 1, dm: Fill(Shade50), ylabels: Index(1), xw: 6, step: 2, values: [1, {2, label: Mar}]"
        );
        assert_eq!(chart.bars_count(), 2);
        assert_eq!(chart.get_bar(1).map(|bar| bar.label()), Some("Mar"));
        assert_eq!(chart.get_bar(1).map(|bar| bar.value()), Some(2));
        chart.add_bar(4);
        chart.update_bar(0, |bar| {
            bar.set_value(10);
        });
        chart.update_bars(|bars| {
            assert!(bars.insert(0, 0));
            assert!(bars.delete(1).is_some());
        });
        assert_eq!(chart.bars_count(), 3);
        assert_eq!(chart.get_bar(0).map(|bar| bar.value()), Some(0));
        chart.set_bars_scale(hbarchart::BarScale::FromZero);
        chart.set_default_bar_width(3);
        chart.set_default_bar_spacing(2);
        chart.set_xaxis_width(8);
        chart.set_xaxis_step(2);
        chart.set_xaxis_visible(true);
        chart.set_xaxis_show_grid(true);
        chart.ensure_visible(2);
        chart.ensure_visible(99);
        let mut w = window!("Test,d:f");
        w.add(chart);
        w
    }).run().unwrap();
    check_event_trait_is_accepted();
}

fn check_event_trait_is_accepted() {
    #[Window(events = HBarChartEvents<i32>, internal: true)]
    struct MyWin {}
    impl MyWin {
        fn new() -> Self {
            let mut win = Self { base: window!("Test,d:f") };
            win.add(hbarchart!("i32,d:f,values:[1,2,3]"));
            win
        }
    }
    impl HBarChartEvents<i32> for MyWin {
        fn on_bar_selected(&mut self, _handle: Handle<HBarChart<i32>>, _index: u32) -> EventProcessStatus {
            EventProcessStatus::Processed
        }
        fn on_clear_selection(&mut self, _handle: Handle<HBarChart<i32>>) -> EventProcessStatus {
            EventProcessStatus::Processed
        }
    }
    let script = "Paint.Enable(false)";
    App::new().size(Size::new(40, 12)).debug_script(script).window(MyWin::new).run().unwrap();
}
