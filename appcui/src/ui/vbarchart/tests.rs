use std::ops::DerefMut;

use crate::prelude::*;
use crate::ui::components::{BarBuilder, BarDrawMode, BarFillType};
use crate::ui::vbarchart::VBarChart;

fn chart_with_values(values: &[i32]) -> VBarChart<i32> {
    let mut chart = VBarChart::<i32>::new(layout!("x:1,y:1,w:10,h:5"), vbarchart::Flags::None);
    chart.add_bars(values);
    chart
}

#[test]
fn check_creation() {
    let _chart = VBarChart::<i32>::new(layout!("x:1,y:1,w:10,h:5"), vbarchart::Flags::None);
}

#[test]
fn check_selected_bar_default() {
    let chart = VBarChart::<i32>::new(layout!("x:1,y:1,w:10,h:5"), vbarchart::Flags::None);
    assert_eq!(chart.selected_bar(), None);
}

#[test]
fn check_get_bar() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xC6634698F528F8C1)
    ";
    App::new().size(Size::new(30, 10)).debug_script(script).window(|| {
        let chart = chart_with_values(&[1, 2, 3]);
        assert_eq!(chart.bars_count(), 3);
        assert_eq!(chart.get_bar(0).map(|b| b.value()), Some(1));
        assert_eq!(chart.get_bar(2).map(|b| b.value()), Some(3));
        assert!(chart.get_bar(3).is_none());
        let mut w = window!("Test,d:f");
        w.add(chart);
        w
    }).run().unwrap();
}

#[test]
fn check_modify_bar() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xEC95F56323529C27)
    ";
    App::new().size(Size::new(30, 10)).debug_script(script).window(|| {
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
    }).run().unwrap();
}

#[test]
fn check_modify_bar_clear_overrides() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x3827B3E29BBF11D0)
    ";
    App::new().size(Size::new(30, 10)).debug_script(script).window(|| {
        let mut chart = VBarChart::<i32>::new(layout!("x:1,y:1,w:10,h:5"), vbarchart::Flags::None);
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
    }).run().unwrap();
}

#[test]
fn check_update_bars_edit_insert_delete() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xFF36DED460639C63)
    ";
    App::new().size(Size::new(30, 10)).debug_script(script).window(|| {
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
    }).run().unwrap();
}

#[test]
fn check_update_bars_set_clear_iter() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xA4F74EB3CD5493CC)
    ";
    App::new().size(Size::new(30, 10)).debug_script(script).window(|| {
        let mut chart = chart_with_values(&[1, 2, 3]);
        chart.update_bars(|bars| {
            let old = bars.set(1, BarBuilder::new(8).label("B").build());
            assert_eq!(old.map(|b| b.value()), Some(2));
            bars.add(4);
            bars.add_bars(&[5, 6]);
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
    }).run().unwrap();
}

#[test]
fn check_scale_sequences() {
    let script = "
        Paint.Enable(false)
        Paint('1. Mixed, positive and negative sequences')
        CheckHash(0xF5BB896CABAD0451)
    ";
    App::new()
        .size(Size::new(80, 48))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Scales,a:c,w:76,h:44");
            w.add(label!("'-2, -1, 0, 1, 2',x:1,y:1,w:30"));
            let mut mixed = VBarChart::<i32>::new(layout!("x:1,y:2,w:72,h:12"), vbarchart::Flags::None);
            mixed.set_xaxis_label_mode(vbarchart::XAxisLabelMode::Index (1));
            mixed.add_bars([-2, -1, 0, 1, 2]);
            w.add(mixed);

            w.add(label!("'0, 1, 2',x:1,y:14,w:30"));
            let mut positive = VBarChart::<i32>::new(layout!("x:1,y:15,w:72,h:12"), vbarchart::Flags::None);
            positive.set_xaxis_label_mode(vbarchart::XAxisLabelMode::Index (1));
            positive.add_bars([0, 1, 2]);
            w.add(positive);

            w.add(label!("'-2, -1, 0',x:1,y:27,w:30"));
            let mut negative = VBarChart::<i32>::new(layout!("x:1,y:28,w:72,h:12"), vbarchart::Flags::None);
            negative.set_xaxis_label_mode(vbarchart::XAxisLabelMode::Index (1));
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
        CheckHash(0x85CD48D7CD7F037B)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Fill, xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x85CD48D7CD7F037B)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Fill(Solid), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x45CB129DB8DA0560)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Fill(Shade75), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x58497164B9FD02D5)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Fill(Shade50), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x443A53EEA78A57BE)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Fill(Shade25), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x75BA1E461E04E54F)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Fill(Braille), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xA81B11F8A62395ED)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Fill(Checkerboard), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x5E9B68A79C148137)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Fill(Grid), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xFB8A6A73B64A5FE7)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Fill(GridDouble), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xE19205C0096ACF00)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Fill(CrossHatch), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x7E20391F083474CC)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Fill(Dashed), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x3BC6D4C2B771C65E)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Fill(DiagonalUp), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x92077637429F54B5)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Fill(DiagonalDown), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xDC247D8D1E62934C)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Fill(Notched), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xEC8659D167C0151B)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Fill('#'), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x18C0C9F6D4955E99)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Line(Single), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xCE10FD14527A464E)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Line(Double), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x5F643110C9121870)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Line(SingleThick), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xD36AF1285FB964F)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Line(Braille), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xBD3F2ADBD9F5C874)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Line(Ascii), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xAC18B26C12CC2757)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Rectangle(Single), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x873CF06C7335C450)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Rectangle(Double), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xB22FC06EBDAD9FA7)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Rectangle(Thick), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x36D2C7553979A8C3)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Rectangle(Braille), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x205E17DD8FB534D3)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Rectangle(Ascii), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xB0FECD5D6FB3C93C)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Rectangle(SingleRound), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xC3621BA36E2847A0)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Rectangle(AsciiRound), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x772FD5491D6C0338)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: FilledRectangle(Single), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x91E87CB3F9E9EF57)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: FilledRectangle(Double), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x401E5894612ECF1D)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: FilledRectangle(SingleThick), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xE3EF8DDA1E2085E1)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: FilledRectangle(Braille), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x369465A76E1991DA)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: FilledRectangle(Ascii), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xD6183A420EC78B77)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: FilledRectangle(Round), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x70E801F728FD41C1)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: FilledRectangle(AsciiRound), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x30E3E0696C506B08)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Point(Bullet), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x2E121D27F95BB5C0)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Point(Diamond), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x8874B6F46489A98)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Point(Square), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x5BA6BF6A6CEC82B0)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Point('X'), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x3425ADAD9D2FB557)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: LargePoint(RoundSquare), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xEC5972B182D53BE4)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: LargePoint(Square), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xA22A212624C7A65F)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: LargePoint(DoubleLineSquare), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xDA9EFDFCAD295A7D)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: LargePoint(ThickSquare), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xAD96F2D374C8B6B0)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: LargePoint(Circle), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x39C0EF4342D47FC8)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: LargePoint(Diamond), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x9DE76BC8AAB8B4F7)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: LargePoint('X'), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xFAA4C28B7B1DF0D8)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Cap(solid), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xF449BF652D92DFA0)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Cap(Shade75), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x7A639FAAAE68E8F8)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Cap(Shade50), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x24510693ADE1A910)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Cap(Shade25), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x6E18F77D48D9BB98)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Cap(Braille), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xCB66D9AC4FFA5298)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Cap(SingleLine), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xECB76DA72F288798)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Cap(DoubleLine), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xDB71695D840DB030)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Cap(ThickLine), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x19DAEF20767A96F0)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Cap('x'), xlabels:Index(1), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0x921C3CC2DF3C4B26)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dm: Fill('x'), space:2, dbw: 3, xlabels:Index(0), values: [1, 2, 3,4,5,6,7,8,9,10]"));
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
        CheckHash(0xF69A7D74E1334CF5)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, xlabels:Index(1), values: [{1, w:1, s:1, dm: Line(Single)}, {2, w:2, s:2, dm: Rectangle(Single)}, {3, w:3, s:3, dm: FilledRectangle(Single)}, {4, w:4, s:4, dm: Fill(Shade75)}, {5, w:5, s:5, dm: Cap(Solid)}]"));
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
        CheckHash(0x5D09324D414BF45A)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, barcolor: red, xlabels:Index(1), values: [{1, w:1, s:1, dm: Line(Single)}, {2, w:2, s:2, dm: Rectangle(Single)}, {3, w:3, s:3, dm: FilledRectangle(Single)}, {4, w:4, s:4, dm: Fill(Shade75)}, {5, w:5, s:5, dm: Cap(Solid)}]"));
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
        CheckHash(0x14A769DFEE1BC29C)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dbw: 3, space: 2, xlabels:Index(1), values: [{1, attr: red}, {2, attr: green}, {3, attr: aqua}, {4, attr: yellow}, {5, attr: pink}]"));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_xlabel_groups_two_quarters() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x8F5193F84D4FE769)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dbw: 3, xlabels: [{0, 3, '1st quarter'}, {3, 3, '2nd quarter'}], values: [1, 2, 3, {4, s: 6}, 5, 6]"));
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
        CheckHash(0xF3DA42C05889012D)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dbw: 3, scale: FitData, xlabels:Index(1), values: [1, 2, 3, 4, 5, 6]"));
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
        CheckHash(0x46DA183510415331)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dbw: 3, scale: Fixed(0, 12), xlabels:Index(1), values: [1, 2, 3, 4, 5, 6]"));
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
        CheckHash(0xDF2E54B23467D7C5)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dbw: 3, scale: FromZeroMinRange(-6, 6), xlabels:Index(1), values: [1, 2, 3, 4, 5, 6]"));
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
        CheckHash(0xFAA3CD0179B96822)
        Mouse.Move(10,11)
        Paint('2. Hover A = 2')
        CheckHash(0x8D36014F9AE18CB7)
        Mouse.Move(12,11)
        Paint('3. Hover B = 4')
        CheckHash(0xD60F6BD5AA214409)
        Mouse.Move(14,11)
        Paint('4. Hover C = 6')
        CheckHash(0x9F2B5C7F385DEF69)
        Mouse.Move(16,11)
        Paint('5. Hover D = 8')
        CheckHash(0x5B013B2D1404F0E9)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, xlabels: BarLabels, values: [{2, label: A}, {4, label: B}, {6, label: C}, {8, label: D}]"));
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
        CheckHash(0xF59528ABE265C002)
        Mouse.Wheel(4,6,right,8)
        Paint('2. Scrolled right')
        CheckHash(0x6B97F9D02820A0C2)
        Mouse.Wheel(4,6,left,4)
        Paint('3. Scrolled left')
        CheckHash(0xE9B29B25C670DA91)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dbw: 3, flags: ScrollBars, xlabels:Index(1), values: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]"));
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
        CheckHash(0xF59528ABE265C002)
        Mouse.Drag(2,14,30,14)
        Paint('2. Dragged scrollbar to the right')
        CheckHash(0x9347883BB5DC8C06)
        Mouse.Drag(30,14,10,14)
        Paint('3. Dragged scrollbar to the left')
        CheckHash(0x428C533B4DA0EA5)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dbw: 3, flags: ScrollBars, xlabels:Index(1), values: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]"));
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
        CheckHash(0xF59528ABE265C002)
        Key.Pressed(Right,8)
        Paint('2. Scrolled right')
        CheckHash(0x6B97F9D02820A0C2)
        Key.Pressed(Left,4)
        Paint('3. Scrolled left')
        CheckHash(0xE9B29B25C670DA91)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dbw: 3, flags: ScrollBars, xlabels:Index(1), values: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]"));
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
        CheckHash(0xF59528ABE265C002)
        Key.Pressed(End)
        Paint('2. End')
        CheckHash(0xB278FD604DBFA4D3)
        Key.Pressed(Home)
        Paint('3. Home')
        CheckHash(0xF59528ABE265C002)
        Key.Pressed(Ctrl+Right,3)
        Paint('4. Ctrl+Right three bars')
        CheckHash(0x38684DB1942FD4B4)
        Key.Pressed(Ctrl+Left)
        Paint('5. Ctrl+Left one bar')
        CheckHash(0x2BAB93E20FEF333F)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dbw: 3, flags: ScrollBars, xlabels:Index(1), values: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]"));
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
        CheckHash(0xD77819D88E41ACD4)
        Mouse.Click(10,13,left)
        Paint('2. Selected first bar')
        CheckHash(0x6EA5F33C0AC6ED36)
        Mouse.Click(12,13,left)
        Paint('3. Selected second bar')
        CheckHash(0x458A2B65CBC8CBE6)
        Mouse.Click(14,13,left)
        Paint('4. Selected third bar')
        CheckHash(0xD54ADB3C69487562)
        Mouse.Click(30,6,left)
        Paint('5. Selection cleared')
        CheckHash(0xD77819D88E41ACD4)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, values: [1, 2, 3]"));
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
        CheckHash(0xD77819D88E41ACD4)
        Mouse.Click(10,13,left)
        Paint('2. Selected first bar')
        CheckHash(0x1F613B4C22330FE9)
        Mouse.Click(12,13,left)
        Paint('3. Selected second bar')
        CheckHash(0x27D71808463260C9)
        Mouse.Click(14,13,left)
        Paint('4. Selected third bar')
        CheckHash(0xF011FE462A387B4A)
        Mouse.Click(30,6,left)
        Paint('5. Selection cleared')
        CheckHash(0xD77819D88E41ACD4)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, flags: DimBarsOnSelection, values: [1, 2, 3]"));
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
        CheckHash(0xE74870A80BE1C82F)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dbw: 3, yw: 10, nf: {hex, prefix: '0x', suffix: h}, values: [16, 160, 255]"));
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
        CheckHash(0x3E1D4BA8E785AEE1)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            let mut chart = vbarchart!("type: i32, d:f, dbw: 3, xlabels: None, values: [1, 2, 3, 4, 5, 6]");
            chart.set_yaxis_visible(false);
            chart.set_yaxis_show_grid(false);
            w.add(chart);
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_bar_events_update_label() {
    #[Window(events = VBarChartEvents<i32>, internal: true)]
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
            win.add(vbarchart!("type: i32, x:0, y:1, w:58, h:12, values: [{2, label: A}, {4, label: B}, {6, label: C}]"));
            win
        }
    }
    impl VBarChartEvents<i32> for MyWin {
        fn on_bar_selected(&mut self, handle: Handle<VBarChart<i32>>, index: u32) -> EventProcessStatus {
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
        fn on_clear_selection(&mut self, _: Handle<VBarChart<i32>>) -> EventProcessStatus {
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
        CheckHash(0xE7937465380314C2)
        Mouse.Click(10,13,left)
        Paint('2. Selected A = 2')
        CheckHash(0x22DF43A4418E09C8)
        Mouse.Click(12,13,left)
        Paint('3. Selected B = 4')
        CheckHash(0xFB6F7C1791142471)
        Mouse.Click(14,13,left)
        Paint('4. Selected C = 6')
        CheckHash(0xFF5ED28DABE6DF3A)
        Mouse.Click(30,6,left)
        Paint('5. Selection cleared')
        CheckHash(0xE7937465380314C2)
    ";
    App::new().size(Size::new(60, 15)).debug_script(script).window(MyWin::new).run().unwrap();
}

#[test]
fn check_show_zero_line_on_y_axis() {
    let script = "
        Paint.Enable(false)
        Paint('1. Zero line')
        CheckHash(0xF194B477BFD54751)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dbw: 3, flags: ShowZeroLineOnYAxis, xlabels:Index(1), values: [-4, -2, 2, 4]"));
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
        CheckHash(0x2CF6307B934FF0AA)
        Mouse.Drag(37,13,55,22)
        Paint('2. Window grown')
        CheckHash(0x80336B48FAF584B6)
        Mouse.Drag(55,22,30,10)
        Paint('3. Window shrunk')
        CheckHash(0x7FE53C6E3285A6E)
    ";
    App::new()
        .size(Size::new(80, 30))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Chart,x:2,y:2,w:36,h:12,flags: Sizeable");
            w.add(vbarchart!("type: i32, d:f, dbw: 3, values: [1, 2, 3, 4, 5]"));
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
        CheckHash(0x44CEB6A287C691EA)
        Key.Pressed(End)
        Paint('2. Scrolled to the end')
        CheckHash(0xB07456F44A5062E3)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i32, d:f, dbw: 5, space: 3, flags: ScrollBars, xlabels: BarLabels, values: [{1, label: 'AlphaOne'}, {2, label: 'BravoTwo'}, {3, label: 'CharlieX'}, {4, label: 'DeltaOne'}, {5, label: 'EchoFive'}, {6, label: 'FoxtrotX'}, {7, label: 'GolfNine'}, {8, label: 'HotelTen'}, {9, label: 'IndiaBar'}, {10, label: 'JulietXX'}, {11, label: 'KiloBars'}, {12, label: 'LimaTest'}, {13, label: 'MikeData'}, {14, label: 'November'}, {15, label: 'OscarBar'}, {16, label: 'PapaTest'}, {17, label: 'QuebecXX'}, {18, label: 'RomeoBar'}, {19, label: 'SierraXX'}, {20, label: 'TangoEnd'}]"));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_update_bar() {
    #[Window(events = ButtonEvents, internal: true)]
    struct MyWin {
        chart: Handle<VBarChart<i32>>,
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
            win.chart = win.add(vbarchart!("type: i32, x:0, y:1, w:58, h:12, xlabels: BarLabels, values: [{2, label: Low}, {4, label: Mid}, {6, label: High}]"));
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
        CheckHash(0x7DE4ACCC00409F3D)
        Mouse.Click(5,1,left)
        Paint('2. Middle bar updated')
        CheckHash(0x406AA96E445D18DC)
        Mouse.Click(5,1,left)
        Paint('3. Out of range leaves the chart unchanged')
        CheckHash(0x406AA96E445D18DC)
    ";
    App::new().size(Size::new(60, 15)).debug_script(script).window(MyWin::new).run().unwrap();
}

#[test]
fn check_set_yaxis_step() {
    #[Window(events = ButtonEvents, internal: true)]
    struct MyWin {
        chart: Handle<VBarChart<i32>>,
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
            win.chart = win.add(vbarchart!("type: i32, x:0, y:1, w:58, h:12, dbw: 3, xlabels:Index(1), values: [1, 2, 3, 4, 5]"));
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
                0 => chart.set_yaxis_step(6),
                _ => chart.set_yaxis_step(0),
            }
            self.stage = stage + 1;
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Default step')
        CheckHash(0xCBFE97E0C60920C5)
        Mouse.Click(5,1,left)
        Paint('2. Step 6')
        CheckHash(0x6DE2F9CE0E25899)
        Mouse.Click(5,1,left)
        Paint('3. Step 0 clamps to 1')
        CheckHash(0xBE1C28CFCB4404A6)
    ";
    App::new().size(Size::new(60, 15)).debug_script(script).window(MyWin::new).run().unwrap();
}

#[test]
fn check_f32_fractional_values() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x597A61F294DAF6AA)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: f32, d:f, dbw: 3, xlabels:Index(1), values: [0.5, 1.25, 2.75, 4.0, -1.5, 3.5]"));
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
        CheckHash(0xAA09ED305ED29429)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: f64, d:f, dbw: 3, xlabels:Index(1), values: [0.25, 1.5, 3.125, 6.5, -2.75, 4.25]"));
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
        CheckHash(0xB86B29C8659DB186)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(vbarchart!("type: i16, d:f, dbw: 3, xlabels:Index(1), values: [-20, -5, 0, 8, 15, 30]"));
            w
        })
        .run()
        .unwrap();
}

fn run_ensure_visible(check: impl FnOnce(&mut VBarChart<i32>) + Send + 'static) {
    let script = "
        Paint.Enable(false)
        Paint('done')
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(move || {
            let mut chart = VBarChart::<i32>::new(layout!("x:0,y:0,w:40,h:10"), vbarchart::Flags::None);
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
        chart.set_yaxis_visible(false);
        chart.add_bars(1..=30);
        assert_eq!(chart.left_scroll, 0);

        chart.ensure_visible(29);
        assert_eq!(chart.left_scroll, 20);
        assert_eq!(chart.first_visible_bar, 9);

        chart.ensure_visible(10);
        assert_eq!(chart.left_scroll, 20);

        chart.ensure_visible(0);
        assert_eq!(chart.left_scroll, 1);

        chart.ensure_visible(0);
        assert_eq!(chart.left_scroll, 1);
        chart.ensure_visible(100);
        assert_eq!(chart.left_scroll, 1);
    });
}

#[test]
fn check_ensure_visible_reveals_a_partially_clipped_bar() {
    run_ensure_visible(|chart| {
        chart.set_yaxis_visible(false);
        chart.set_default_bar_width(8);
        chart.add_bars(1..=10);
        chart.ensure_visible(4);
        assert_eq!(chart.left_scroll, 5);
    });
}

#[test]
fn check_ensure_visible_accounts_for_the_y_axis_margin() {
    run_ensure_visible(|chart| {
        chart.add_bars(1..=25);
        chart.ensure_visible(20);
        assert_eq!(chart.left_scroll, 10);
    });
}

#[test]
fn check_ensure_visible_wide_bar_scrolls_only_until_it_overlaps_the_plot() {
    run_ensure_visible(|chart| {
        chart.set_yaxis_visible(false);
        chart.set_default_bar_width(50);
        chart.add_bar(1);
        chart.ensure_visible(0);
        assert_eq!(chart.left_scroll, 0);

        chart.left_scroll = 100;
        chart.ensure_visible(0);
        assert_eq!(chart.left_scroll, 11);

        chart.set_default_bar_spacing(45);
        chart.update_bar(0, |_| {});
        chart.left_scroll = 0;
        chart.ensure_visible(0);
        assert_eq!(chart.left_scroll, 45);
    });
}

#[test]
fn check_ensure_visible_out_of_range_index_does_nothing() {
    run_ensure_visible(|chart| {
        chart.ensure_visible(0);
        assert_eq!(chart.left_scroll, 0);
        chart.add_bar(1);
        chart.left_scroll = 3;
        chart.ensure_visible(4);
        assert_eq!(chart.left_scroll, 3);
    });
}