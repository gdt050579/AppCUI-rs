use std::ops::DerefMut;

use crate::prelude::*;
use crate::ui::components::{BarBuilder, BarDrawMode, BarFillType};
use crate::ui::hbarchart::HBarChart;

fn chart_with_values(values: &[i32]) -> HBarChart<i32> {
    let mut chart = HBarChart::<i32>::new(layout!("x:1,y:1,w:10,h:5"), hbarchart::Flags::None);
    chart.add_bars(values);
    chart
}

#[test]
fn check_creation() {
    let chart = HBarChart::<i32>::new(layout!("x:1,y:1,w:10,h:5"), hbarchart::Flags::None);
    assert_eq!(chart.bars_count(), 0);
    assert_eq!(chart.selected_bar(), None);
}

#[test]
fn check_selected_bar_default() {
    let chart = HBarChart::<i32>::new(layout!("x:1,y:1,w:10,h:5"), hbarchart::Flags::None);
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

#[test]
fn check_get_bar() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x4431FDC5279246D8)
    ";
    App::new()
        .size(Size::new(30, 10))
        .debug_script(script)
        .window(|| {
            let chart = chart_with_values(&[1, 2, 3]);
            assert_eq!(chart.bars_count(), 3);
            assert_eq!(chart.get_bar(0).map(|b| b.value()), Some(1));
            assert_eq!(chart.get_bar(2).map(|b| b.value()), Some(3));
            assert!(chart.get_bar(3).is_none());
            let mut w = window!("Test,d:f");
            w.add(chart);
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_modify_bar() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x53F6E6C54E2ECDC4)
    ";
    App::new()
        .size(Size::new(30, 10))
        .debug_script(script)
        .window(|| {
            let mut chart = chart_with_values(&[1, 2, 3]);
            chart.update_bar(1, |bar| {
                bar.set_value(20);
                bar.set_label("Apr");
                bar.set_thickness(4);
                bar.set_spacing(2);
                bar.set_draw_mode(BarDrawMode::Fill(BarFillType::Custom('x')));
                bar.set_attr(charattr!("red"));
            });
            let bar = chart.get_bar(1).unwrap();
            assert_eq!(bar.value(), 20);
            assert_eq!(bar.label(), "Apr");
            assert_eq!(bar.thickness(), Some(4));
            assert_eq!(bar.spacing(), Some(2));
            assert_eq!(bar.draw_mode(), Some(BarDrawMode::Fill(BarFillType::Custom('x'))));
            assert_eq!(bar.attr(), Some(charattr!("red")));
            chart.update_bar(10, |bar| {
                bar.set_value(0);
            });
            assert_eq!(chart.get_bar(1).map(|b| b.value()), Some(20));
            let mut w = window!("Test,d:f");
            w.add(chart);
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_modify_bar_clear_overrides() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xA8581E2847D0C238)
    ";
    App::new()
        .size(Size::new(30, 10))
        .debug_script(script)
        .window(|| {
            let mut chart = HBarChart::<i32>::new(layout!("x:1,y:1,w:10,h:5"), hbarchart::Flags::None);
            chart.add_bar(BarBuilder::new(1).thickness(5).spacing(3).label("A").build());
            chart.update_bar(0, |bar| {
                bar.clear_thickness();
                bar.clear_spacing();
                bar.clear_attr();
                bar.clear_draw_mode();
                bar.set_label("");
            });
            let bar = chart.get_bar(0).unwrap();
            assert!(bar.thickness().is_none());
            assert!(bar.spacing().is_none());
            assert!(bar.attr().is_none());
            assert!(bar.draw_mode().is_none());
            assert_eq!(bar.label(), "");
            let mut w = window!("Test,d:f");
            w.add(chart);
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_update_bars_edit_insert_delete() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xBD5036AAF0D40D36)
    ";
    App::new()
        .size(Size::new(30, 10))
        .debug_script(script)
        .window(|| {
            let mut chart = chart_with_values(&[1, 2, 3]);
            chart.update_bars(|bars| {
                bars.get_mut(0).unwrap().set_value(10);
                assert!(bars.insert(1, 15));
                assert_eq!(bars.delete(3).map(|b| b.value()), Some(3));
                assert!(!bars.insert(10, 99));
                assert!(bars.delete(10).is_none());
            });
            assert_eq!(chart.bars_count(), 3);
            assert_eq!(chart.get_bar(0).map(|b| b.value()), Some(10));
            assert_eq!(chart.get_bar(1).map(|b| b.value()), Some(15));
            assert_eq!(chart.get_bar(2).map(|b| b.value()), Some(2));
            let mut w = window!("Test,d:f");
            w.add(chart);
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_update_bars_set_clear_iter() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xCF21CB17AD3D2931)
    ";
    App::new()
        .size(Size::new(30, 10))
        .debug_script(script)
        .window(|| {
            let mut chart = chart_with_values(&[1, 2, 3]);
            chart.update_bars(|bars| {
                let old = bars.set(1, BarBuilder::new(8).label("B").build());
                assert_eq!(old.map(|b| b.value()), Some(2));
                bars.add(4);
                bars.add_bars([5, 6]);
                let values: Vec<i32> = bars.iter().map(|b| b.value()).collect();
                assert_eq!(values, vec![1, 8, 3, 4, 5, 6]);
                for bar in bars.iter_mut() {
                    bar.set_value(bar.value() * 2);
                }
                assert!(!bars.is_empty());
                bars.clear();
                assert!(bars.is_empty());
                assert_eq!(bars.len(), 0);
            });
            assert_eq!(chart.bars_count(), 0);
            assert!(chart.get_bar(0).is_none());
            let mut w = window!("Test,d:f");
            w.add(chart);
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_scale_sequences() {
    let script = "
        Paint.Enable(false)
        Paint('1. Mixed, positive and negative sequences')
        CheckHash(0x9C357508FF704640)
    ";
    App::new()
        .size(Size::new(80, 48))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Scales,a:c,w:76,h:44");
            w.add(label!("'-2, -1, 0, 1, 2',x:1,y:1,w:30"));
            let mut mixed = HBarChart::<i32>::new(layout!("x:1,y:2,w:72,h:12"), hbarchart::Flags::None);
            mixed.set_yaxis_label_mode(hbarchart::YAxisLabelMode::Index(1));
            mixed.add_bars([-2, -1, 0, 1, 2]);
            w.add(mixed);

            w.add(label!("'0, 1, 2',x:1,y:14,w:30"));
            let mut positive = HBarChart::<i32>::new(layout!("x:1,y:15,w:72,h:12"), hbarchart::Flags::None);
            positive.set_yaxis_label_mode(hbarchart::YAxisLabelMode::Index(1));
            positive.add_bars([0, 1, 2]);
            w.add(positive);

            w.add(label!("'-2, -1, 0',x:1,y:27,w:30"));
            let mut negative = HBarChart::<i32>::new(layout!("x:1,y:28,w:72,h:12"), hbarchart::Flags::None);
            negative.set_yaxis_label_mode(hbarchart::YAxisLabelMode::Index(1));
            negative.add_bars([-2, -1, 0]);
            w.add(negative);
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_fill_default() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x67DD844076BF71C5)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!("type: i32, d:f, dm: Fill, ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_fill_solid() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x67DD844076BF71C5)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Fill(Solid), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_fill_shade75() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xBF272393E61ED5A6)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Fill(Shade75), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_fill_shade50() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x28255EDA71C523CF)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Fill(Shade50), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_fill_shade25() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x75C4ADD0F5FDBA4C)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Fill(Shade25), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_fill_braille() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x8D376918D57CF519)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Fill(Braille), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_fill_checkerboard() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x544351EA3D78F0B7)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Fill(Checkerboard), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_fill_grid() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xDD7638957E1BC5A1)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Fill(Grid), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_fill_grid_double() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x3E558A7E1FE2A91)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Fill(GridDouble), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_fill_cross_hatch() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x1960778DE361CC86)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Fill(CrossHatch), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_fill_dashed() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x994EBDE81D6460D2)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Fill(Dashed), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_fill_diagonal_up() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x19D7A87AEA819FEC)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Fill(DiagonalUp), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_fill_diagonal_down() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xC5AD92FC1C52DF6F)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Fill(DiagonalDown), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_fill_notched() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xC610288C0A1B8C52)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Fill(Notched), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_fill_custom() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x75A244F2C19A441D)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Fill('#'), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_line_single() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xE489B83D0DD86891)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Line(Single), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_line_double() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x14596E0BE8C619C5)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Line(Double), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_line_single_thick() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xCD818A99A1BF748E)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Line(SingleThick), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_line_braille() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xB142E13CAF00CDED)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Line(Braille), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_line_ascii() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x90538AD5876EDA4E)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Line(Ascii), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_line_border_ascii_round_and_single_round() {
    let script = "
        Paint.Enable(false)
        Paint('1. Border, AsciiRound and SingleRound lines')
        CheckHash(0x626577312DB18D1C)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, flags: ShowZeroLineOnXAxis, ylabels:Index(1), values: [{8, dm: Line(Border)}, {6, dm: Line(AsciiRound)}, {4, dm: Line(SingleRound)}, {-4, dm: Line(Border)}, {-6, dm: Line(AsciiRound)}, {-8, dm: Line(SingleRound)}]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_rectangle_single() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xCDF658CDB8538C98)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Rectangle(Single), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_rectangle_double() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x8EE78A2CC1AD226B)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Rectangle(Double), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_rectangle_single_thick() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x9987E0D6C71E4121)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Rectangle(Thick), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_rectangle_braille() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x8E101117D60341F1)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Rectangle(Braille), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_rectangle_ascii() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x8D0FAA5AACCBB0DA)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Rectangle(Ascii), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_rectangle_single_round() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xE45F4AF318349293)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Rectangle(SingleRound), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_rectangle_ascii_round() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x274F1A55662608F1)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Rectangle(AsciiRound), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_filled_rectangle_single() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x5E9923ED29DE1CBA)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: FilledRectangle(Single), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_filled_rectangle_double() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x6259D2929A2E26F1)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: FilledRectangle(Double), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_filled_rectangle_single_thick() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x92B9747FC5535BBE)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: FilledRectangle(SingleThick), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_filled_rectangle_braille() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x2DB32175A49ADF8E)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: FilledRectangle(Braille), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_filled_rectangle_ascii() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xC3D488829420A3CE)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: FilledRectangle(Ascii), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_filled_rectangle_single_round() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xFB80B4842AB41CDD)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: FilledRectangle(Round), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_filled_rectangle_ascii_round() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xE1626E43038BCD15)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: FilledRectangle(AsciiRound), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_point_bullet() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x30A37F0571E22898)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Point(Bullet), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_point_diamond() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x63C72BA89C484909)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Point(Diamond), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_point_square() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x990E83CB5A8F709F)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Point(Square), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_point_custom() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x542C53CB9F0CF85C)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Point('X'), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_point_all_negative_values() {
    let script = "
        Paint.Enable(false)
        Paint('1. Points for negative values')
        CheckHash(0x7EF09AFBD73CEBD2)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Point(Bullet), flags: ShowZeroLineOnXAxis, ylabels:Index(1), values: [-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_large_point_round_square() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xD0F05A138BE8B577)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: LargePoint(RoundSquare), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_large_point_square() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xE45D750C4FFC1D4)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: LargePoint(Square), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_large_point_double_line_square() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xFBC55DCF3450AC1F)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: LargePoint(DoubleLineSquare), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_large_point_thick_square() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x58585E268A5958E5)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: LargePoint(ThickSquare), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_large_point_circle() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x50E621566336EC98)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: LargePoint(Circle), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_large_point_diamond() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x9B27A70F5EA3F658)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: LargePoint(Diamond), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_large_point_custom() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xFB28EDBD9BB05DF)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: LargePoint('X'), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_cap_solid() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xF6A10B141349AA67)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Cap(solid), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_cap_shade75() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x83978187E97EED9C)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Cap(Shade75), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_cap_shade50() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x89BAD71A4347A745)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Cap(Shade50), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_cap_shade25() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xD0173431EE859ADE)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Cap(Shade25), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_cap_braille() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x6EEA726329394FBB)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Cap(Braille), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_cap_single_line() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xAAECD77C3D776055)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Cap(SingleLine), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_cap_double_line() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xA59B601E6C22FA9E)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Cap(DoubleLine), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_cap_thick_line() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xAA3CE9B394D708AC)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Cap(ThickLine), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_cap_custom() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x85331169D036F23C)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Cap('x'), ylabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_draw_mode_bar_width_3() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x4BB59C4E6B17B804)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dm: Fill('x'), space:2, dbw: 3, ylabels:Index(0), values: [1, 2, 3,4,5,6,7,8,9,10]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_bars_with_different_thickness_spacing_and_draw_mode() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xAB1E4E83CA5DA96D)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!("type: i32, d:f, ylabels:Index(1), values: [{1, w:1, s:1, dm: Line(Single)}, {2, w:2, s:2, dm: Rectangle(Single)}, {3, w:3, s:3, dm: FilledRectangle(Single)}, {4, w:4, s:4, dm: Fill(Shade75)}, {5, w:5, s:5, dm: Cap(Solid)}]"));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_bars_with_different_thickness_spacing_and_draw_mode_custom_color() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xC85A0438E9A5999D)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!("type: i32, d:f, barcolor: red, ylabels:Index(1), values: [{1, w:1, s:1, dm: Line(Single)}, {2, w:2, s:2, dm: Rectangle(Single)}, {3, w:3, s:3, dm: FilledRectangle(Single)}, {4, w:4, s:4, dm: Fill(Shade75)}, {5, w:5, s:5, dm: Cap(Solid)}]"));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_bars_with_different_colors_and_default_size() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x88B3AF7CDCFD9724)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!("type: i32, d:f, dbw: 3, space: 2, ylabels:Index(1), values: [{1, attr: red}, {2, attr: green}, {3, attr: aqua}, {4, attr: yellow}, {5, attr: pink}]"));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_ylabel_groups_two_quarters() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x7F5F3B5E0E031617)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dbw: 3, ylabels: [{0, 3, '1st quarter'}, {3, 3, '2nd quarter'}], values: [1, 2, 3, {4, s: 6}, 5, 6]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_scale_fit_data() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x42FC26315EDE7A9)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dbw: 3, scale: FitData, ylabels:Index(1), values: [1, 2, 3, 4, 5, 6]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_scale_fixed() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xE43D085601A7380B)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dbw: 3, scale: Fixed(0, 12), ylabels:Index(1), values: [1, 2, 3, 4, 5, 6]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_scale_from_zero_min_range() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xA50BE4217C66D74D)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dbw: 3, scale: FromZeroMinRange(-6, 6), ylabels:Index(1), values: [1, 2, 3, 4, 5, 6]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_bar_label_tooltips() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x96E229C058CC96B1)
        Mouse.Move(12,2)
        Paint('2. Hover A = 2')
        CheckHash(0x7CEBD6075D324986)
        Mouse.Move(12,4)
        Paint('3. Hover B = 4')
        CheckHash(0x34538B80C938EF1F)
        Mouse.Move(12,6)
        Paint('4. Hover C = 6')
        CheckHash(0x7C5946CFDDCEBC76)
        Mouse.Move(12,8)
        Paint('5. Hover D = 8')
        CheckHash(0x541594CCA3EA847F)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, ylabels: BarLabels, values: [{2, label: A}, {4, label: B}, {6, label: C}, {8, label: D}]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_mouse_wheel_scroll() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x5C52049822B6E068)
        Mouse.Wheel(4,6,down,8)
        Paint('2. Scrolled down')
        CheckHash(0xCDC0F384ECCAB732)
        Mouse.Wheel(4,6,up,4)
        Paint('3. Scrolled up')
        CheckHash(0x60AB981015982D62)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!("type: i32, d:f, dbw: 3, flags: ScrollBars, ylabels:Index(1), values: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]"));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_mouse_drag_scrollbar() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x5C52049822B6E068)
        Mouse.Drag(59,2,59,10)
        Paint('2. Dragged scrollbar down')
        CheckHash(0x506A7C60F729A250)
        Mouse.Drag(59,10,59,4)
        Paint('3. Dragged scrollbar up')
        CheckHash(0x6E04D7223CB29CF6)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!("type: i32, d:f, dbw: 3, flags: ScrollBars, ylabels:Index(1), values: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]"));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_key_scroll() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x5C52049822B6E068)
        Key.Pressed(Down,8)
        Paint('2. Scrolled down')
        CheckHash(0xCDC0F384ECCAB732)
        Key.Pressed(Up,4)
        Paint('3. Scrolled up')
        CheckHash(0x60AB981015982D62)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!("type: i32, d:f, dbw: 3, flags: ScrollBars, ylabels:Index(1), values: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]"));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_key_home_end_and_ctrl_arrows() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x5C52049822B6E068)
        Key.Pressed(End)
        Paint('2. End')
        CheckHash(0xAE45D5CB2645B665)
        Key.Pressed(Home)
        Paint('3. Home')
        CheckHash(0x5C52049822B6E068)
        Key.Pressed(Ctrl+Down,3)
        Paint('4. Ctrl+Down three bars')
        CheckHash(0xF45A0B6A1CE4919)
        Key.Pressed(Ctrl+Up)
        Paint('5. Ctrl+Up one bar')
        CheckHash(0x6782C9A33645054E)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!("type: i32, d:f, dbw: 3, flags: ScrollBars, ylabels:Index(1), values: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]"));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_click_select_and_clear() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xAFFE57BD7FF0A2)
        Mouse.Click(6,2,left)
        Paint('2. Selected first bar')
        CheckHash(0x90FED95E18F88EF8)
        Mouse.Click(6,4,left)
        Paint('3. Selected second bar')
        CheckHash(0xD9F7488DE04013AC)
        Mouse.Click(6,6,left)
        Paint('4. Selected third bar')
        CheckHash(0x3F24A2C29DBB531E)
        Mouse.Click(20,3,left)
        Paint('5. Selection cleared')
        CheckHash(0xAFFE57BD7FF0A2)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!("type: i32, d:f, values: [1, 2, 3]"));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_click_select_and_clear_dim_bars() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xAFFE57BD7FF0A2)
        Mouse.Click(6,2,left)
        Paint('2. Selected first bar')
        CheckHash(0x1E269FD00C73AF0C)
        Mouse.Click(6,4,left)
        Paint('3. Selected second bar')
        CheckHash(0x3EA89B7D7344B6E8)
        Mouse.Click(6,6,left)
        Paint('4. Selected third bar')
        CheckHash(0xE010C563951E1E6E)
        Mouse.Click(20,3,left)
        Paint('5. Selection cleared')
        CheckHash(0xAFFE57BD7FF0A2)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!("type: i32, d:f, flags: DimBarsOnSelection, values: [1, 2, 3]"));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_numeric_format_prefix_suffix_and_base() {
    let script = "
        Paint.Enable(false)
        Paint('1. Hex labels with prefix and suffix')
        CheckHash(0x5777AD6090C5D853)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dbw: 3, step: 10, nf: {hex, prefix: '0x', suffix: h}, values: [16, 160, 255]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_chart_without_x_and_y_labels() {
    let script = "
        Paint.Enable(false)
        Paint('1. No X or Y labels')
        CheckHash(0x454AE68A7E0F4489)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            let mut chart = hbarchart!("type: i32, d:f, dbw: 3, ylabels: None, values: [1, 2, 3, 4, 5, 6]");
            chart.set_xaxis_visible(false);
            chart.set_xaxis_show_grid(false);
            w.add(chart);
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_bar_events_update_label() {
    #[Window(events = HBarChartEvents<i32>, internal: true)]
    struct MyWin {
        info: Handle<Label>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut win = Self {
                base: window!("Test,d:f"),
                info: Handle::None,
            };
            win.info = win.add(label!("'No selection',x:0,y:0,w:40"));
            win.add(hbarchart!(
                "type: i32, x:0, y:1, w:58, h:12, values: [{2, label: A}, {4, label: B}, {6, label: C}]"
            ));
            win
        }
    }
    impl HBarChartEvents<i32> for MyWin {
        fn on_bar_selected(&mut self, handle: Handle<HBarChart<i32>>, index: u32) -> EventProcessStatus {
            let text = if let Some(chart) = self.control(handle) {
                if let Some(bar) = chart.get_bar(index as usize) {
                    format!("{} = {}", bar.label(), bar.value())
                } else {
                    format!("bar {index}")
                }
            } else {
                String::from("?")
            };
            let info = self.info;
            if let Some(label) = self.control_mut(info) {
                label.set_caption(&text);
            }
            EventProcessStatus::Processed
        }
        fn on_clear_selection(&mut self, _: Handle<HBarChart<i32>>) -> EventProcessStatus {
            let info = self.info;
            if let Some(label) = self.control_mut(info) {
                label.set_caption("No selection");
            }
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. No selection')
        CheckHash(0xF00F293DCC266B7)
        Mouse.Click(6,3,left)
        Paint('2. Selected A = 2')
        CheckHash(0x250654D90C34B7D1)
        Mouse.Click(6,5,left)
        Paint('3. Selected B = 4')
        CheckHash(0x143E7DEDB7B65A04)
        Mouse.Click(6,7,left)
        Paint('4. Selected C = 6')
        CheckHash(0x63232452F3116D55)
        Mouse.Click(11,4,left)
        Paint('5. Selection cleared')
        CheckHash(0xF00F293DCC266B7)
    ";
    App::new().size(Size::new(60, 15)).debug_script(script).window(MyWin::new).run().unwrap();
}

#[test]
fn check_show_zero_line_on_x_axis() {
    let script = "
        Paint.Enable(false)
        Paint('1. Zero line')
        CheckHash(0x54AA645C13A994A5)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: i32, d:f, dbw: 3, flags: ShowZeroLineOnXAxis, ylabels:Index(1), values: [-4, -2, 2, 4]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_resize_window_adjusts_bars() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial window')
        CheckHash(0x51EE47CF2BC13CC)
        Mouse.Drag(37,13,55,22)
        Paint('2. Window grown')
        CheckHash(0x8B21535BCDECFAE4)
        Mouse.Drag(55,22,30,10)
        Paint('3. Window shrunk')
        CheckHash(0x8ECC4B29317EF486)
    ";
    App::new()
        .size(Size::new(80, 30))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Chart,x:2,y:2,w:36,h:12,flags: Sizeable");
            w.add(hbarchart!("type: i32, d:f, dbw: 3, values: [1, 2, 3, 4, 5]"));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_long_labels_scroll_to_end() {
    let script = "
        Paint.Enable(false)
        Paint('1. Start')
        CheckHash(0xEA34C914A5E2137F)
        Key.Pressed(End)
        Paint('2. Scrolled to the end')
        CheckHash(0x4FF673A239FD87C7)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!("type: i32, d:f, dbw: 5, space: 3, flags: ScrollBars, ylabels: BarLabels, values: [{1, label: 'AlphaOne'}, {2, label: 'BravoTwo'}, {3, label: 'CharlieX'}, {4, label: 'DeltaOne'}, {5, label: 'EchoFive'}, {6, label: 'FoxtrotX'}, {7, label: 'GolfNine'}, {8, label: 'HotelTen'}, {9, label: 'IndiaBar'}, {10, label: 'JulietXX'}, {11, label: 'KiloBars'}, {12, label: 'LimaTest'}, {13, label: 'MikeData'}, {14, label: 'November'}, {15, label: 'OscarBar'}, {16, label: 'PapaTest'}, {17, label: 'QuebecXX'}, {18, label: 'RomeoBar'}, {19, label: 'SierraXX'}, {20, label: 'TangoEnd'}]"));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_update_bar() {
    #[Window(events = ButtonEvents, internal: true)]
    struct MyWin {
        chart: Handle<HBarChart<i32>>,
        updated: bool,
    }
    impl MyWin {
        fn new() -> Self {
            let mut win = Self {
                base: window!("Test,d:f"),
                chart: Handle::None,
                updated: false,
            };
            win.add(button!("Update,x:0,y:0,w:10"));
            win.chart = win.add(hbarchart!(
                "type: i32, x:0, y:1, w:58, h:12, ylabels: BarLabels, values: [{2, label: Low}, {4, label: Mid}, {6, label: High}]"
            ));
            win
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
            let chart = self.chart;
            let already = self.updated;
            let Some(chart) = self.control_mut(chart) else {
                return EventProcessStatus::Ignored;
            };
            if !already {
                chart.update_bar(1, |bar| {
                    bar.set_value(12);
                    bar.set_label("Updated");
                    bar.set_thickness(4);
                    bar.set_spacing(3);
                    bar.set_draw_mode(BarDrawMode::Fill(BarFillType::Shade50));
                    bar.set_attr(charattr!("red"));
                });
                let bar = chart.get_bar(1).unwrap();
                assert_eq!(bar.value(), 12);
                assert_eq!(bar.label(), "Updated");
                assert_eq!(bar.thickness(), Some(4));
                assert_eq!(bar.spacing(), Some(3));
                assert_eq!(bar.draw_mode(), Some(BarDrawMode::Fill(BarFillType::Shade50)));
                assert_eq!(bar.attr(), Some(charattr!("red")));
                assert_eq!(chart.get_bar(0).map(|b| b.value()), Some(2));
                assert_eq!(chart.get_bar(2).map(|b| b.value()), Some(6));
            } else {
                chart.update_bar(99, |bar| {
                    bar.set_value(0);
                });
                assert_eq!(chart.bars_count(), 3);
                assert_eq!(chart.get_bar(1).map(|b| b.value()), Some(12));
                assert_eq!(chart.get_bar(1).map(|b| b.label()), Some("Updated"));
            }
            self.updated = true;
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Before update')
        CheckHash(0x2B6145C504DE0114)
        Mouse.Click(5,1,left)
        Paint('2. Middle bar updated')
        CheckHash(0xC81CE898763D2C4B)
        Mouse.Click(5,1,left)
        Paint('3. Out of range leaves the chart unchanged')
        CheckHash(0xC81CE898763D2C4B)
    ";
    App::new().size(Size::new(60, 15)).debug_script(script).window(MyWin::new).run().unwrap();
}

#[test]
fn check_set_xaxis_step() {
    #[Window(events = ButtonEvents, internal: true)]
    struct MyWin {
        chart: Handle<HBarChart<i32>>,
        stage: u8,
    }
    impl MyWin {
        fn new() -> Self {
            let mut win = Self {
                base: window!("Test,d:f"),
                chart: Handle::None,
                stage: 0,
            };
            win.add(button!("Step,x:0,y:0,w:10"));
            win.chart = win.add(hbarchart!(
                "type: i32, x:0, y:1, w:58, h:12, dbw: 3, ylabels:Index(1), values: [1, 2, 3, 4, 5]"
            ));
            win
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
            let chart = self.chart;
            let stage = self.stage;
            let Some(chart) = self.control_mut(chart) else {
                return EventProcessStatus::Ignored;
            };
            match stage {
                0 => chart.set_xaxis_step(6),
                _ => chart.set_xaxis_step(0),
            }
            self.stage = stage + 1;
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Default step')
        CheckHash(0xA4D19A0DE57E1318)
        Mouse.Click(5,1,left)
        Paint('2. Step 6')
        CheckHash(0xC6FD486FC81AD323)
        Mouse.Click(5,1,left)
        Paint('3. Step 0 clamps to 1')
        CheckHash(0x93CD7F50674A9EAA)
    ";
    App::new().size(Size::new(60, 15)).debug_script(script).window(MyWin::new).run().unwrap();
}

#[test]
fn check_f32_fractional_values() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xF3D38C480444338D)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: f32, d:f, dbw: 3, ylabels:Index(1), values: [0.5, 1.25, 2.75, 4.0, -1.5, 3.5]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_f64_fractional_values() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x6335DAE755E51FF9)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!(
                "type: f64, d:f, dbw: 3, ylabels:Index(1), values: [0.25, 1.5, 3.125, 6.5, -2.75, 4.25]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_i16_signed_values() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xEF5DDCC38110474D)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(hbarchart!("type: i16, d:f, dbw: 3, ylabels:Index(1), values: [-20, -5, 0, 8, 15, 30]"));
            w
        })
        .run()
        .unwrap();
}

fn run_ensure_visible(check: impl FnOnce(&mut HBarChart<i32>) + Send + 'static) {
    let script = "
        Paint.Enable(false)
        Paint('done')
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(move || {
            let mut chart = HBarChart::<i32>::new(layout!("x:0,y:0,w:40,h:10"), hbarchart::Flags::None);
            chart.deref_mut().layout.update(80, 30);
            check(&mut chart);
            window!("Test,d:f")
        })
        .run()
        .unwrap();
}

#[test]
fn check_ensure_visible_scrolls_far_bar_into_view_and_leaves_visible_bars() {
    run_ensure_visible(|chart| {
        chart.set_xaxis_visible(false);
        chart.add_bars(1..=30);
        assert_eq!(chart.top_scroll, 0);

        chart.ensure_visible(29);
        assert_eq!(chart.top_scroll, 50);
        assert_eq!(chart.first_visible_bar, 24);

        chart.ensure_visible(25);
        assert_eq!(chart.top_scroll, 50);

        chart.ensure_visible(0);
        assert_eq!(chart.top_scroll, 1);

        chart.ensure_visible(0);
        assert_eq!(chart.top_scroll, 1);
        chart.ensure_visible(100);
        assert_eq!(chart.top_scroll, 1);
    });
}

#[test]
fn check_ensure_visible_reveals_a_partially_clipped_bar() {
    run_ensure_visible(|chart| {
        chart.set_xaxis_visible(false);
        chart.set_default_bar_width(8);
        chart.add_bars(1..=10);
        chart.ensure_visible(4);
        assert_eq!(chart.top_scroll, 35);
    });
}

#[test]
fn check_ensure_visible_accounts_for_the_x_axis_margin() {
    run_ensure_visible(|chart| {
        chart.add_bars(1..=25);
        chart.ensure_visible(20);
        assert_eq!(chart.top_scroll, 34);
    });
}

#[test]
fn check_ensure_visible_tall_bar_scrolls_only_until_it_overlaps_the_plot() {
    run_ensure_visible(|chart| {
        chart.set_xaxis_visible(false);
        chart.set_default_bar_width(50);
        chart.add_bar(1);
        chart.ensure_visible(0);
        assert_eq!(chart.top_scroll, 0);

        chart.top_scroll = 100;
        chart.ensure_visible(0);
        assert_eq!(chart.top_scroll, 41);

        chart.set_default_bar_spacing(45);
        chart.update_bar(0, |_| {});
        chart.top_scroll = 0;
        chart.ensure_visible(0);
        assert_eq!(chart.top_scroll, 45);
    });
}

#[test]
fn check_ensure_visible_out_of_range_index_does_nothing() {
    run_ensure_visible(|chart| {
        chart.ensure_visible(0);
        assert_eq!(chart.top_scroll, 0);
        chart.add_bar(1);
        chart.top_scroll = 3;
        chart.ensure_visible(4);
        assert_eq!(chart.top_scroll, 3);
    });
}
