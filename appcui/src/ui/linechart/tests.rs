use std::ops::DerefMut;

use crate::prelude::*;
use crate::ui::linechart::LineChart;

#[test]
fn check_creation() {
    let _chart = LineChart::<i32>::new(layout!("x:1,y:1,w:10,h:5"), linechart::Flags::None);
}

#[test]
fn check_creation_with_flags() {
    let chart = LineChart::<f64>::new(layout!("x:1,y:1,w:10,h:5"), linechart::Flags::ShowZeroLineOnYAxis);
    assert!(chart.flags().contains(linechart::Flags::ShowZeroLineOnYAxis));
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

// ---------------------------------------------------------------- Series / SeriesBuilder

use crate::ui::linechart::{LineMode, Marker, Series, SeriesBuilder};
use crate::ui::linechart::linechart::PointPos;

#[test]
fn check_series_new() {
    let s = Series::<i32>::new("CPU");
    assert_eq!(s.label(), "CPU");
    assert_eq!(s.len(), 0);
    assert!(s.is_empty());
    assert_eq!(s.line(), None);
    assert_eq!(s.attr(), None);
    assert!(s.get(0).is_none());
}

#[test]
fn check_series_with_values() {
    let s = Series::with_values("CPU", [1, 2, 3]);
    assert_eq!(s.len(), 3);
    assert_eq!(s.iter().map(|b| b.value()).collect::<Vec<_>>(), vec![1, 2, 3]);
    let temps = [1.5, 2.5];
    let s = Series::<f64>::with_values("Temp", temps.iter());
    assert_eq!(s.get(1).map(|b| b.value()), Some(2.5));
}

#[test]
fn check_series_setters() {
    let mut s = Series::<u32>::new("A");
    s.set_label("B")
        .set_line(Some(LineMode::Braille))
        .set_attr(Some(charattr!("red")));
    assert_eq!(s.label(), "B");
    assert_eq!(s.line(), Some(LineMode::Braille));
    assert_eq!(s.attr(), Some(charattr!("red")));
    s.set_line(None).set_attr(None);
    assert_eq!(s.line(), None);
    assert_eq!(s.attr(), None);
}

#[test]
fn check_series_points() {
    let mut s = Series::<i32>::new("S");
    s.add(10);
    s.add(BarBuilder::new(20).label("Feb").attr(charattr!("green")).build());
    s.add_values([30, 40]);
    assert_eq!(s.len(), 4);
    assert_eq!(s.get(1).map(|b| b.label()), Some("Feb"));
    assert_eq!(s.get(1).and_then(|b| b.attr()), Some(charattr!("green")));

    assert!(s.insert(0, 5));
    assert!(s.insert(5, 50));
    assert!(!s.insert(10, 99));
    assert_eq!(s.iter().map(|b| b.value()).collect::<Vec<_>>(), vec![5, 10, 20, 30, 40, 50]);

    assert_eq!(s.set(2, 21).map(|b| b.value()), Some(20));
    assert!(s.set(100, 0).is_none());
    assert_eq!(s.delete(0).map(|b| b.value()), Some(5));
    assert!(s.delete(100).is_none());
    s.get_mut(0).unwrap().set_value(11);
    for b in s.iter_mut() {
        b.set_value(b.value() * 2);
    }
    assert_eq!(s.iter().map(|b| b.value()).collect::<Vec<_>>(), vec![22, 42, 60, 80, 100]);
}

#[test]
fn check_series_remove_first_and_clear() {
    let mut s = Series::with_values("S", [1, 2, 3, 4, 5]);
    s.remove_first(2);
    assert_eq!(s.iter().map(|b| b.value()).collect::<Vec<_>>(), vec![3, 4, 5]);
    s.remove_first(0);
    assert_eq!(s.len(), 3);
    s.remove_first(10);
    assert!(s.is_empty());
    s.add_values([7, 8]);
    s.set_line(Some(LineMode::None));
    s.clear();
    assert!(s.is_empty());
    assert_eq!(s.label(), "S");
    assert_eq!(s.line(), Some(LineMode::None));
}

#[test]
fn check_series_builder() {
    let s = SeriesBuilder::<i32>::new("RAM").build();
    assert_eq!(s.label(), "RAM");
    assert!(s.is_empty());
    assert_eq!(s.line(), None);
    assert_eq!(s.attr(), None);

    let s: Series<i32> = SeriesBuilder::new("RAM")
        .line(LineMode::Orthogonal(LineType::Double, OrthogonalDirection::HorizontalFirst))
        .attr(charattr!("aqua"))
        .values([1, 2])
        .values([3])
        .into();
    assert_eq!(s.line(), Some(LineMode::Orthogonal(LineType::Double, OrthogonalDirection::HorizontalFirst)));
    assert_eq!(s.attr(), Some(charattr!("aqua")));
    assert_eq!(s.iter().map(|b| b.value()).collect::<Vec<_>>(), vec![1, 2, 3]);
}

// ---------------------------------------------------------------- LineMode / Marker

#[test]
fn check_line_mode_default() {
    assert_eq!(LineMode::default(), LineMode::Line(LineType::Single));
}

#[test]
fn check_marker_default_and_width() {
    assert_eq!(Marker::default(), Marker::Small(BarPointType::Bullet));
    assert_eq!(Marker::None.width(), 1);
    assert_eq!(Marker::Small(BarPointType::Diamond).width(), 1);
    assert_eq!(Marker::Framed(BarFramedPointType::RoundSquare).width(), 5);
}

#[test]
fn check_marker_height() {
    assert_eq!(Marker::None.height(), 1);
    assert_eq!(Marker::Small(BarPointType::Bullet).height(), 1);
    assert_eq!(Marker::Framed(BarFramedPointType::Square).height(), 3);
}

#[test]
fn check_marker_from_draw_mode() {
    assert_eq!(Marker::from_draw_mode(BarDrawMode::Point(BarPointType::Bullet)), Some(Marker::Small(BarPointType::Bullet)));
    assert_eq!(
        Marker::from_draw_mode(BarDrawMode::FramedPoint(BarFramedPointType::BrailleCircle)),
        Some(Marker::Framed(BarFramedPointType::BrailleCircle))
    );
    // the 3x2 large point has no middle row for the value: it falls back to the default marker
    assert_eq!(Marker::from_draw_mode(BarDrawMode::LargePoint(BarLargePointType::Diamond)), None);
    assert_eq!(Marker::from_draw_mode(BarDrawMode::Fill(BarFillType::Solid)), None);
    assert_eq!(Marker::from_draw_mode(BarDrawMode::Line(LineType::Single)), None);
    assert_eq!(Marker::from_draw_mode(BarDrawMode::Cap(BarCapType::Solid)), None);
}

#[test]
fn check_frame_junctions_thick_and_double() {
    let thick = Some([('┨', '┠'), ('┫', '┣'), ('┨', '┠')]);
    assert_eq!(super::marker::frame_junctions(BarFramedPointType::ThickSquare), thick);
    assert_eq!(super::marker::frame_junctions(BarFramedPointType::ThickSquareBullet), thick);

    let double = Some([('╢', '╟'), ('╢', '╟'), ('╣', '╠')]);
    assert_eq!(super::marker::frame_junctions(BarFramedPointType::DoubleLineSquare), double);
    assert_eq!(super::marker::frame_junctions(BarFramedPointType::DoubleLineSquareBullet), double);
}

#[test]
fn check_horizontal_arm() {
    // (toward the left, toward the right): thin, thick, double
    let arms = [
        (true, "─┐┘┤┬┴┼╮╯-", "━┓┛┫┳┻╋", "═╗╝╣╦╩╬"),
        (false, "─┌└├┬┴┼╭╰-", "━┏┗┣┳┻╋", "═╔╚╠╦╩╬"),
    ];
    for (toward_left, thin, thick, double) in arms {
        for ch in thin.chars() {
            assert_eq!(super::marker::horizontal_arm(ch, toward_left), Some(0), "{ch} toward_left={toward_left}");
        }
        for ch in thick.chars() {
            assert_eq!(super::marker::horizontal_arm(ch, toward_left), Some(1), "{ch} toward_left={toward_left}");
        }
        for ch in double.chars() {
            assert_eq!(super::marker::horizontal_arm(ch, toward_left), Some(2), "{ch} toward_left={toward_left}");
        }
    }
    // a corner that opens the other way is not an arm in this direction
    for (ch, toward_left) in [('┐', false), ('┌', true), ('┓', false), ('┏', true), ('╗', false), ('╔', true)] {
        assert_eq!(super::marker::horizontal_arm(ch, toward_left), None, "{ch} toward_left={toward_left}");
    }
    assert_eq!(super::marker::horizontal_arm('│', true), None);
    assert_eq!(super::marker::horizontal_arm('│', false), None);
    assert_eq!(super::marker::horizontal_arm('x', true), None);
    assert_eq!(super::marker::horizontal_arm('x', false), None);
}

// ---------------------------------------------------------------- LineChart data API

/// Runs `f` while an application (and its theme) exists: changing the data of a chart repaints it.
fn in_app<F: FnOnce() + 'static>(f: F) {
    App::new()
        .size(Size::new(20, 5))
        .debug_script("Paint.Enable(false)")
        .window(move || {
            f();
            window!("Test,d:f")
        })
        .run()
        .unwrap();
}

fn empty_chart() -> LineChart<i32> {
    LineChart::<i32>::new(layout!("x:1,y:1,w:10,h:5"), linechart::Flags::None)
}

#[test]
fn check_chart_defaults() {
    let chart = empty_chart();
    assert_eq!(chart.series_count(), 0);
    assert_eq!(chart.points_count(), 0);
    assert!(chart.get_series(0).is_none());
    assert_eq!(chart.default_line(), LineMode::default());
    assert_eq!(chart.default_marker(), Marker::default());
    assert_eq!(chart.default_spacing(), 2);
}

#[test]
fn check_chart_set_defaults() {
    in_app(|| {
        let mut chart = empty_chart();
        chart.set_default_line(LineMode::Braille);
        chart.set_default_marker(Marker::Framed(BarFramedPointType::BrailleCircle));
        chart.set_default_spacing(0);
        assert_eq!(chart.default_line(), LineMode::Braille);
        assert_eq!(chart.default_marker(), Marker::Framed(BarFramedPointType::BrailleCircle));
        assert_eq!(chart.default_spacing(), 0);
    });
}

#[test]
fn check_chart_add_series() {
    in_app(|| {
        let mut chart = empty_chart();
        assert_eq!(chart.add_series(Series::with_values("CPU", [1, 2, 3])), 0);
        assert_eq!(chart.add_series(SeriesBuilder::new("RAM").line(LineMode::Braille).values([4, 5])), 1);
        assert_eq!(chart.add_series(Series::new("Disk")), 2);
        assert_eq!(chart.series_count(), 3);
        assert_eq!(chart.get_series(0).map(|s| s.label()), Some("CPU"));
        assert_eq!(chart.get_series(1).and_then(|s| s.line()), Some(LineMode::Braille));
        assert_eq!(chart.get_series(2).map(|s| s.len()), Some(0));
        assert!(chart.get_series(3).is_none());
    });
}

#[test]
fn check_chart_points_count_uneven() {
    in_app(|| {
        let mut chart = empty_chart();
        chart.add_series(Series::with_values("A", [1, 2]));
        chart.add_series(Series::with_values("B", [1, 2, 3, 4, 5]));
        chart.add_series(Series::new("C"));
        assert_eq!(chart.points_count(), 5);
        chart.update_series(1, |s| s.remove_first(4));
        assert_eq!(chart.points_count(), 2);
    });
}

#[test]
fn check_chart_add_value() {
    in_app(|| {
        let mut chart = empty_chart();
        // no series: the index is out of range and the chart stays empty
        chart.add_value(0, 1);
        assert_eq!(chart.series_count(), 0);
        assert_eq!(chart.points_count(), 0);

        chart.add_series(Series::new("A"));
        chart.add_series(Series::with_values("B", [7]));
        chart.add_value(0, 10);
        chart.add_value(0, BarBuilder::new(20).label("Feb").build());
        assert_eq!(chart.get_series(0).unwrap().iter().map(|b| b.value()).collect::<Vec<_>>(), vec![10, 20]);
        assert_eq!(chart.get_series(0).unwrap().get(1).map(|b| b.label()), Some("Feb"));
        // writing series 0 leaves series 1 unchanged
        assert_eq!(chart.get_series(1).unwrap().iter().map(|b| b.value()).collect::<Vec<_>>(), vec![7]);

        chart.add_value(1, 8);
        assert_eq!(chart.get_series(1).unwrap().iter().map(|b| b.value()).collect::<Vec<_>>(), vec![7, 8]);
        assert_eq!(chart.points_count(), 2);

        // past the last series: neither series grows
        chart.add_value(2, 99);
        chart.add_value(usize::MAX, 99);
        assert_eq!(chart.get_series(0).unwrap().len(), 2);
        assert_eq!(chart.get_series(1).unwrap().len(), 2);
        assert_eq!(chart.points_count(), 2);
    });
}

#[test]
fn check_chart_add_values() {
    in_app(|| {
        let mut chart = empty_chart();
        chart.add_series(Series::new("A"));
        chart.add_series(Series::new("B"));
        chart.add_value(0, 10);
        chart.add_value(0, BarBuilder::new(20).label("Feb").build());
        chart.add_values(1, [1, 2, 3]);
        let values = [4, 5];
        chart.add_values(1, values.iter());
        assert_eq!(chart.get_series(0).unwrap().iter().map(|b| b.value()).collect::<Vec<_>>(), vec![10, 20]);
        assert_eq!(chart.get_series(0).unwrap().get(1).map(|b| b.label()), Some("Feb"));
        assert_eq!(chart.get_series(1).unwrap().iter().map(|b| b.value()).collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
        assert_eq!(chart.points_count(), 5);
    });
}

#[test]
fn check_chart_invalid_series_index_is_ignored() {
    in_app(|| {
        let mut chart = empty_chart();
        // no series yet
        chart.add_value(0, 1);
        chart.add_values(0, [1, 2]);
        chart.update_series(0, |_| panic!("must not be called"));
        chart.update_all_series(|_, _| panic!("must not be called"));
        assert!(chart.remove_series(0).is_none());
        assert_eq!(chart.series_count(), 0);
        // with one series
        chart.add_series(Series::with_values("A", [1]));
        chart.add_value(5, 2);
        chart.add_values(1, [3, 4]);
        chart.update_series(1, |_| panic!("must not be called"));
        assert!(chart.remove_series(1).is_none());
        assert_eq!(chart.series_count(), 1);
        assert_eq!(chart.points_count(), 1);
    });
}

#[test]
fn check_chart_update_series() {
    in_app(|| {
        let mut chart = empty_chart();
        chart.add_series(Series::with_values("A", [1, 2, 3]));
        chart.update_series(0, |s| {
            s.set_label("CPU").set_attr(Some(charattr!("red")));
            s.get_mut(1).unwrap().set_value(20);
            s.add(4);
        });
        let s = chart.get_series(0).unwrap();
        assert_eq!(s.label(), "CPU");
        assert_eq!(s.attr(), Some(charattr!("red")));
        assert_eq!(s.iter().map(|b| b.value()).collect::<Vec<_>>(), vec![1, 20, 3, 4]);
    });
}

#[test]
fn check_chart_update_all_series() {
    in_app(|| {
        let mut chart = empty_chart();
        chart.add_series(Series::with_values("A", [1, 2, 3]));
        chart.add_series(Series::with_values("B", [10, 20, 30]));
        let sample = [4, 40];
        let mut visited = Vec::new();
        chart.update_all_series(|index, s| {
            visited.push(index);
            s.add(sample[index]);
            s.remove_first(1);
        });
        assert_eq!(visited, vec![0, 1]);
        assert_eq!(chart.get_series(0).unwrap().iter().map(|b| b.value()).collect::<Vec<_>>(), vec![2, 3, 4]);
        assert_eq!(chart.get_series(1).unwrap().iter().map(|b| b.value()).collect::<Vec<_>>(), vec![20, 30, 40]);
    });
}

#[test]
fn check_chart_remove_series() {
    in_app(|| {
        let mut chart = empty_chart();
        chart.add_series(Series::with_values("A", [1]));
        chart.add_series(Series::with_values("B", [1, 2, 3]));
        chart.add_series(Series::with_values("C", [1, 2]));
        let removed = chart.remove_series(1).unwrap();
        assert_eq!(removed.label(), "B");
        assert_eq!(removed.len(), 3);
        assert_eq!(chart.series_count(), 2);
        assert_eq!(chart.get_series(1).map(|s| s.label()), Some("C"));
        assert_eq!(chart.points_count(), 2);
    });
}

// ---------------------------------------------------------------- X-axis labels

#[test]
fn check_xaxis_labels_api() {
    in_app(|| {
        let mut c = empty_chart();
        assert_eq!(c.xaxis_labels_count(), 0);
        assert_eq!(c.xaxis_label(0), None);
        c.set_xaxis_labels(["Jan", "Feb", "", "Apr"]);
        assert_eq!(c.xaxis_labels_count(), 4);
        assert_eq!(c.xaxis_label(0), Some("Jan"));
        assert_eq!(c.xaxis_label(2), None); // empty label
        assert_eq!(c.xaxis_label(4), None);
        c.add_xaxis_label("May");
        c.add_xaxis_labels(vec![String::from("Jun"), String::from("Jul")]);
        assert_eq!(c.xaxis_labels_count(), 7);
        assert_eq!(c.xaxis_label(6), Some("Jul"));
        c.remove_first_xaxis_labels(2);
        assert_eq!(c.xaxis_label(0), None); // the former index 2 (empty)
        assert_eq!(c.xaxis_label(1), Some("Apr"));
        c.remove_first_xaxis_labels(100);
        assert_eq!(c.xaxis_labels_count(), 0);
        c.set_xaxis_labels(["a", "b"]);
        c.set_xaxis_labels(["x"]);
        assert_eq!(c.xaxis_labels_count(), 1);
        c.clear_xaxis_labels();
        assert_eq!(c.xaxis_labels_count(), 0);
    });
}

// ---------------------------------------------------------------- scale mapping

#[test]
fn check_value_to_level() {
    // 7 levels (0..=6) for the range 0..=60
    assert_eq!(LineChart::<i32>::value_to_level(0.0, 0.0, 60.0, 7), 0);
    assert_eq!(LineChart::<i32>::value_to_level(60.0, 0.0, 60.0, 7), 6);
    assert_eq!(LineChart::<i32>::value_to_level(30.0, 0.0, 60.0, 7), 3);
    assert_eq!(LineChart::<i32>::value_to_level(14.0, 0.0, 60.0, 7), 1);
    assert_eq!(LineChart::<i32>::value_to_level(16.0, 0.0, 60.0, 7), 2);
    // negative range
    assert_eq!(LineChart::<i32>::value_to_level(-10.0, -10.0, 10.0, 5), 0);
    assert_eq!(LineChart::<i32>::value_to_level(0.0, -10.0, 10.0, 5), 2);
    // clamped outside the range (Fixed scale)
    assert_eq!(LineChart::<i32>::value_to_level(100.0, 0.0, 50.0, 6), 5);
    assert_eq!(LineChart::<i32>::value_to_level(-5.0, 0.0, 50.0, 6), 0);
    // empty range -> middle
    assert_eq!(LineChart::<i32>::value_to_level(3.0, 3.0, 3.0, 7), 3);
    assert_eq!(LineChart::<i32>::value_to_level(3.0, 3.0, 3.0, 6), 2);
    // degenerate number of levels
    assert_eq!(LineChart::<i32>::value_to_level(3.0, 0.0, 10.0, 1), 0);
    assert_eq!(LineChart::<i32>::value_to_level(3.0, 0.0, 10.0, 0), 0);
}

// ---------------------------------------------------------------- painting (axes and markers)

fn run_paint_test<F>(script: &str, size: Size, build: F)
where
    F: Fn() -> LineChart<i32> + 'static,
{
    App::new()
        .size(size)
        .debug_script(script)
        .window(move || {
            let mut w = window!("Test,d:f");
            w.add(build());
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_paint_number_format() {
    let script = "
        Paint('1. Y-axis labels and the zero line use one decimal place')
        CheckHash(0x86694B47E9B6E2AD)
    ";
    run_paint_test(script, Size::new(40, 12), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::ShowZeroLineOnYAxis);
        c.add_series(Series::with_values("T", [-50, 50]));
        c.set_number_format(FormatNumber::new(10).decimals(1));
        c
    });
}

#[test]
fn check_paint_empty_chart() {
    let script = "
        Paint('1. Empty chart - only the Y axis and grid')
        CheckHash(0x4ADAAC3790E474DD)
    ";
    run_paint_test(script, Size::new(40, 12), || LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None));
}

#[test]
fn check_paint_single_series_from_zero() {
    let script = "
        Paint('1. One series, FromZero scale, no X axis')
        CheckHash(0xAB9FDCFB45E9FD26)
    ";
    run_paint_test(script, Size::new(40, 12), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
        c.add_series(Series::with_values("CPU", [10, 40, 25, 60, 5, 30]));
        c
    });
}

#[test]
fn check_paint_fit_data_with_index_labels() {
    let script = "
        Paint('1. FitData scale with negative values and index labels')
        CheckHash(0xC32CA9F078C4F11F)
    ";
    run_paint_test(script, Size::new(40, 12), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
        c.add_series(Series::with_values("T", [-20, -5, 10, 0, 15, -10]));
        c.set_scale(BarScale::FitData);
        c.set_xaxis_label_mode(linechart::XAxisLabelMode::Index(1));
        c
    });
}

#[test]
fn check_paint_multiple_series_xaxis_labels() {
    let script = "
        Paint('1. Three series of different lengths, X-axis labels set on the chart')
        CheckHash(0x9099A52F3E2A7F75)
    ";
    run_paint_test(script, Size::new(44, 14), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
        c.add_series(Series::with_values("A", [10, 20, 30]));
        c.add_series(SeriesBuilder::new("B").attr(charattr!("yellow")).values([50, 45, 35, 40, 55]));
        // only 5 names for 7 positions: the last two have no label
        c.set_xaxis_labels(["Jan", "Feb", "Mar", "Apr", "May"]);
        c.add_series(Series::with_values("C", [0, 5, 0, 5, 0, 5, 0]));
        c.set_default_spacing(4);
        c.set_xaxis_label_mode(linechart::XAxisLabelMode::Labels);
        c
    });
}

#[test]
fn check_paint_framed_markers_fixed_scale() {
    let script = "
        Paint('1. Framed markers, one point overridden with a small red marker, Fixed 0..50 (100 clamped to the top)')
        CheckHash(0x23841D1A4896ADA)
    ";
    run_paint_test(script, Size::new(44, 14), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
        c.set_default_marker(Marker::Framed(BarFramedPointType::RoundSquare));
        c.add_series(Series::with_values("A", [0, 10, 20, 30, 100]));
        c.update_series(0, |s| {
            s.get_mut(2)
                .unwrap()
                .set_draw_mode(BarDrawMode::Point(BarPointType::Diamond))
                .set_attr(charattr!("red"));
        });
        c.set_scale(BarScale::Fixed { min: 0, max: 50 });
        c.set_xaxis_label_mode(linechart::XAxisLabelMode::Index(0));
        c
    });
}

#[test]
fn check_paint_custom_spans_and_zero_line() {
    let script = "
        Paint('1. Custom spans (Q1, Q2) and the zero line')
        CheckHash(0x5357B05D6C46B0A3)
    ";
    run_paint_test(script, Size::new(44, 14), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::ShowZeroLineOnYAxis);
        c.add_series(Series::with_values("P", [-30, -10, 20, 40, 10, -20]));
        let spans = [BarSpan::new(0, 3, "Q1"), BarSpan::new(3, 3, "Q2")];
        c.set_xaxis_label_mode(linechart::XAxisLabelMode::Custom(&spans));
        c.set_yaxis_step(2);
        c
    });
}

#[test]
fn check_paint_no_yaxis_no_grid_spacing_marker_fallback() {
    let script = "
        Paint('1. Y axis and grid hidden, spacing 4, a Fill draw mode falls back to the default marker')
        CheckHash(0x712B05D6CB525D50)
    ";
    run_paint_test(script, Size::new(40, 10), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
        c.set_yaxis_visible(false);
        c.set_yaxis_show_grid(false);
        c.set_default_spacing(4);
        c.add_series(Series::with_values("A", [1, 3, 2, 4, 3]));
        c.add_series(Series::with_values("B", [BarBuilder::new(2).draw_mode(BarDrawMode::Fill(BarFillType::Solid)).build()]));
        c.set_default_marker(Marker::Small(BarPointType::Square));
        c
    });
}

#[test]
fn check_paint_equal_values_fit_data() {
    let script = "
        Paint('1. All values equal with FitData -> drawn in the middle')
        CheckHash(0x6D3A63551C6F553A)
    ";
    run_paint_test(script, Size::new(40, 12), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
        c.add_series(Series::with_values("A", [7, 7, 7, 7]));
        c.set_scale(BarScale::FitData);
        c
    });
}

#[test]
fn check_paint_disabled() {
    let script = "
        Paint('1. Disabled chart is drawn with the inactive color')
        CheckHash(0x1B891A0C20F4C2)
    ";
    run_paint_test(script, Size::new(40, 12), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
        c.add_series(Series::with_values("A", [10, 40, 25, 60]));
        c.set_enabled(false);
        c
    });
}

#[test]
fn check_paint_overflow_and_resize() {
    let script = "
        Paint('1. More points than fit - drawing stops at the right edge')
        CheckHash(0x9C11B031F492D8CF)
        Resize(60,12)
        Paint('2. After resize more points are visible')
        CheckHash(0x584A99CE4DB48950)
    ";
    run_paint_test(script, Size::new(30, 12), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
        c.add_series(Series::with_values("A", (0..20).map(|i| (i * 7) % 23)));
        c.set_xaxis_label_mode(linechart::XAxisLabelMode::Index(0));
        c
    });
}

// ---------------------------------------------------------------- painting (lines)

fn chart_with_line(line: LineMode) -> LineChart<i32> {
    let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
    c.set_default_spacing(4);
    c.set_default_line(line);
    c.add_series(Series::with_values("A", [10, 50, 30, 30, 0, 45, 20]));
    c
}

#[test]
fn check_paint_line_single() {
    let script = "
        Paint('1. LineMode::Line(Single)')
        CheckHash(0x9C0A0F72DC093657)
    ";
    run_paint_test(script, Size::new(50, 14), || chart_with_line(LineMode::Line(LineType::Single)));
}

#[test]
fn check_paint_line_double() {
    let script = "
        Paint('1. LineMode::Line(Double)')
        CheckHash(0xBE5B7C7C634F9B1D)
    ";
    run_paint_test(script, Size::new(50, 14), || chart_with_line(LineMode::Line(LineType::Double)));
}

#[test]
fn check_paint_line_ascii() {
    let script = "
        Paint('1. LineMode::Line(Ascii)')
        CheckHash(0xBEE31A7DA29C3BB6)
    ";
    run_paint_test(script, Size::new(50, 14), || chart_with_line(LineMode::Line(LineType::Ascii)));
}

#[test]
fn check_paint_line_braille() {
    let script = "
        Paint('1. LineMode::Braille')
        CheckHash(0xA9393F6C0FDC9AEF)
    ";
    run_paint_test(script, Size::new(50, 14), || chart_with_line(LineMode::Braille));
}

#[test]
fn check_paint_line_orthogonal() {
    let script = "
        Paint('1. LineMode::Orthogonal(Single, HorizontalUntilMiddle) - step chart')
        CheckHash(0xD7D08BF25DCAAD65)
    ";
    run_paint_test(script, Size::new(50, 14), || {
        chart_with_line(LineMode::Orthogonal(LineType::Single, OrthogonalDirection::HorizontalUntilMiddle))
    });
}

#[test]
fn check_paint_line_orthogonal_rounded_horizontal_first() {
    let script = "
        Paint('1. LineMode::Orthogonal(SingleRound, HorizontalFirst)')
        CheckHash(0xC73A5114BD664963)
    ";
    run_paint_test(script, Size::new(50, 14), || {
        chart_with_line(LineMode::Orthogonal(LineType::SingleRound, OrthogonalDirection::HorizontalFirst))
    });
}

#[test]
fn check_paint_line_none_scatter() {
    let script = "
        Paint('1. LineMode::None - markers only (scatter)')
        CheckHash(0xD21052872FB2C4CB)
    ";
    run_paint_test(script, Size::new(50, 14), || chart_with_line(LineMode::None));
}

#[test]
fn check_paint_lines_without_markers() {
    let script = "
        Paint('1. Marker::None - only the Braille line is visible')
        CheckHash(0x9D6407092B2B6EA4)
    ";
    run_paint_test(script, Size::new(50, 14), || {
        let mut c = chart_with_line(LineMode::Braille);
        c.set_default_marker(Marker::None);
        c
    });
}

#[test]
fn check_paint_mixed_series_lines() {
    let script = "
        Paint('1. Per-series line modes and colors, lines below every marker')
        CheckHash(0xFFBABC08B763249B)
    ";
    run_paint_test(script, Size::new(56, 16), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
        c.set_default_spacing(4);
        c.add_series(SeriesBuilder::new("CPU").line(LineMode::Braille).attr(charattr!("green")).values([10, 35, 20, 60, 40, 55, 30, 45]));
        c.add_series(
            SeriesBuilder::new("RAM")
                .line(LineMode::Line(LineType::Double))
                .attr(charattr!("yellow"))
                .values([40, 42, 44, 41, 43, 45]),
        );
        c.add_series(
            SeriesBuilder::new("Disk")
                .line(LineMode::Orthogonal(LineType::Single, OrthogonalDirection::HorizontalFirst))
                .attr(charattr!("aqua"))
                .values([5, 5, 15, 15, 25, 10, 10, 20]),
        );
        c.set_xaxis_label_mode(linechart::XAxisLabelMode::Index(1));
        c
    });
}

#[test]
fn check_paint_lines_framed_markers() {
    let script = "
        Paint('1. Lines are attached to the middle of the sides of framed markers, with junctions')
        CheckHash(0xBD2E31174AAC19E1)
    ";
    run_paint_test(script, Size::new(50, 16), || {
        let mut c = chart_with_line(LineMode::Line(LineType::Single));
        c.set_default_marker(Marker::Framed(BarFramedPointType::RoundSquareBullet));
        c.set_default_spacing(3);
        c
    });
}

#[test]
fn check_paint_lines_clipped_and_resize() {
    let script = "
        Paint('1. The last visible segment goes out of the plot area and is clipped')
        CheckHash(0x145A693FDE4E9F18)
        Resize(60,14)
        Paint('2. Wider - more segments are visible')
        CheckHash(0x5068AD751B88EAB5)
    ";
    run_paint_test(script, Size::new(32, 14), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
        c.set_default_spacing(3);
        c.add_series(Series::with_values("A", (0..15).map(|i| (i * 13) % 31)));
        c.set_xaxis_label_mode(linechart::XAxisLabelMode::Index(0));
        c
    });
}

// ---------------------------------------------------------------- scrolling

fn run_with_size(check: impl FnOnce(&mut LineChart<i32>) + 'static) {
    App::new()
        .size(Size::new(60, 15))
        .debug_script("Paint.Enable(false)")
        .window(move || {
            let mut chart = LineChart::<i32>::new(layout!("x:0,y:0,w:40,h:10"), linechart::Flags::None);
            chart.deref_mut().layout.update(80, 30);
            check(&mut chart);
            window!("Test,d:f")
        })
        .run()
        .unwrap();
}

#[test]
fn check_ensure_visible() {
    run_with_size(|chart| {
        chart.set_yaxis_visible(false);
        chart.add_series(Series::with_values("A", 1..=30));
        assert_eq!(chart.left_scroll, 0);
        assert_eq!(chart.first_visible_point, 0);
        // column 29 is at x = 2 + 29 * 3 = 89 -> scroll so that it ends one cell before the right edge
        // (the last column is kept free for the scroll bar)
        chart.ensure_visible(29);
        assert_eq!(chart.left_scroll, 51);
        assert_eq!(chart.first_visible_point, 17);
        // already visible -> no change
        chart.ensure_visible(20);
        assert_eq!(chart.left_scroll, 51);
        // back to the first point
        chart.ensure_visible(0);
        assert_eq!(chart.left_scroll, 2);
        assert_eq!(chart.first_visible_point, 0);
        // out of range -> no change
        chart.ensure_visible(100);
        assert_eq!(chart.left_scroll, 2);
    });
}

#[test]
fn check_ensure_visible_with_yaxis_margin() {
    run_with_size(|chart| {
        chart.add_series(Series::with_values("A", 1..=30));
        // plot width is 40 - 8 - 1 = 31
        chart.ensure_visible(20);
        assert_eq!(chart.left_scroll, 32);
    });
}

#[test]
fn check_ensure_visible_branches() {
    run_with_size(|chart| {
        chart.set_yaxis_visible(false);
        // index is past the end (the chart has no points): return before touching the scroll
        chart.left_scroll = 4;
        chart.ensure_visible(0);
        assert_eq!(chart.left_scroll, 4);

        chart.add_series(Series::with_values("A", [1, 2, 3]));
        // index == points_count is also out of range. update_layout would clamp 4 down to 0,
        // so an unchanged scroll shows that return happens first
        chart.left_scroll = 4;
        chart.ensure_visible(3);
        assert_eq!(chart.left_scroll, 4);

        // width 1 and no Y axis: the plot width is 0, so a valid index still does not scroll
        chart.set_size(1, 10);
        chart.deref_mut().layout.update(80, 30);
        assert_eq!(chart.size().width, 1);
        chart.left_scroll = 7;
        chart.ensure_visible(1);
        assert_eq!(chart.left_scroll, 7);

        // a column that fits: already visible, then off the right, then off the left
        chart.set_size(40, 10);
        chart.deref_mut().layout.update(80, 30);
        chart.set_default_spacing(2);
        chart.add_values(0, 4..=30);
        assert_eq!(chart.points_count(), 30);
        chart.left_scroll = 0;
        chart.ensure_visible(0);
        assert_eq!(chart.left_scroll, 0);
        chart.ensure_visible(29);
        assert_eq!(chart.left_scroll, 51);
        assert_eq!(chart.first_visible_point, 17);
        chart.ensure_visible(20);
        assert_eq!(chart.left_scroll, 51);
        chart.ensure_visible(0);
        assert_eq!(chart.left_scroll, 2);
        assert_eq!(chart.first_visible_point, 0);

        // framed markers are 5 cells wide, the plot is 3: the column does not fit
        chart.set_size(4, 10);
        chart.deref_mut().layout.update(80, 30);
        chart.set_default_spacing(0);
        chart.set_default_marker(Marker::Framed(BarFramedPointType::Square));
        assert_eq!(chart.size().width, 4);
        // the column ends exactly on the left edge of the view -> scroll to its right edge
        chart.left_scroll = 5;
        chart.ensure_visible(0);
        assert_eq!(chart.left_scroll, 2);
        assert_eq!(chart.first_visible_point, 0);
        // the same column now overlaps the view -> leave the scroll alone
        chart.ensure_visible(0);
        assert_eq!(chart.left_scroll, 2);
        assert_eq!(chart.first_visible_point, 0);
        // the next column starts exactly on the right edge of the view
        chart.ensure_visible(1);
        assert_eq!(chart.left_scroll, 5);
        assert_eq!(chart.first_visible_point, 1);
    });
}

#[test]
fn check_scroll_is_clamped_when_data_shrinks() {
    run_with_size(|chart| {
        chart.set_yaxis_visible(false);
        chart.add_series(Series::with_values("A", 1..=30));
        chart.ensure_visible(29);
        assert_eq!(chart.left_scroll, 51);
        chart.update_all_series(|_, s| s.remove_first(20));
        // 10 points: 2 + 10 * 3 + 1 = 33 cells -> fits in 40, no scroll
        assert_eq!(chart.left_scroll, 0);
        assert_eq!(chart.first_visible_point, 0);
        chart.add_values(0, 1..=20);
        assert_eq!(chart.left_scroll, 0);
    });
}

fn scroll_chart() -> LineChart<i32> {
    let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::ScrollBars);
    c.set_default_spacing(3);
    c.add_series(Series::with_values("A", (0..30).map(|i| (i * 13) % 31)));
    c.set_xaxis_label_mode(linechart::XAxisLabelMode::Index(0));
    c
}

#[test]
fn check_scroll_keys() {
    let script = "
        Paint('1. Initial state - focused, the scroll bar is visible')
        CheckHash(0x7CC8E40FC08C4D3)
        Key.Pressed(Right,5)
        Paint('2. Scrolled 5 cells to the right')
        CheckHash(0x94D52E9D5E262DB)
        Key.Pressed(Left,2)
        Paint('3. Back 2 cells')
        CheckHash(0x9E83D75CB1963E2F)
        Key.Pressed(End)
        Paint('4. End - last point is visible')
        CheckHash(0x2DA20FD275F9A994)
        Key.Pressed(Right)
        Paint('5. Right at the end does nothing')
        CheckHash(0x2DA20FD275F9A994)
        Key.Pressed(Home)
        Paint('6. Home - back to the first point')
        CheckHash(0x7CC8E40FC08C4D3)
    ";
    run_paint_test(script, Size::new(40, 12), scroll_chart);
}

#[test]
fn check_scroll_ctrl_arrows() {
    let script = "
        Paint('1. Initial state')
        CheckHash(0x7CC8E40FC08C4D3)
        Key.Pressed(Ctrl+Right,3)
        Paint('2. Three points to the right (point 3 is the first one)')
        CheckHash(0xFD3E5C68A8170FF6)
        Key.Pressed(Right)
        Paint('3. One more cell - no longer aligned to point 3')
        CheckHash(0x518617DB521B53F5)
        Key.Pressed(Ctrl+Left)
        Paint('4. Ctrl+Left aligns point 3 again')
        CheckHash(0xFD3E5C68A8170FF6)
        Key.Pressed(Ctrl+Left)
        Paint('5. Ctrl+Left goes to point 2')
        CheckHash(0x62DD95D0C1F4C38A)
    ";
    run_paint_test(script, Size::new(40, 12), scroll_chart);
}

#[test]
fn check_scroll_mouse_wheel() {
    let script = "
        Paint('1. Initial state')
        CheckHash(0x7CC8E40FC08C4D3)
        Mouse.Wheel(20,5,down,6)
        Paint('2. Wheel down/right - 6 cells')
        CheckHash(0x75A2840F1EF3E2AB)
        Mouse.Wheel(20,5,up,2)
        Paint('3. Wheel up/left - 2 cells')
        CheckHash(0xCB1C915A35F0EBD4)
    ";
    run_paint_test(script, Size::new(40, 12), scroll_chart);
}

#[test]
fn check_scroll_mouse_drag_scrollbar() {
    let script = "
        Paint('1. Initial state')
        CheckHash(0x7CC8E40FC08C4D3)
        Mouse.Drag(2,11,25,11)
        Paint('2. Scroll bar dragged to the right')
        CheckHash(0x1CE91EBF0688A664)
        Mouse.Drag(25,11,10,11)
        Paint('3. Scroll bar dragged back')
        CheckHash(0xC5196E89CEBDBD59)
    ";
    run_paint_test(script, Size::new(40, 12), scroll_chart);
}

#[test]
fn check_scroll_without_scrollbars_flag() {
    let script = "
        Paint('1. No ScrollBars flag - no scroll bar, the bottom margin is not used')
        CheckHash(0x83B104EC8A2615D)
        Key.Pressed(Right,4)
        Paint('2. Still scrolls with the keyboard')
        CheckHash(0x293017EDA399A92E)
    ";
    run_paint_test(script, Size::new(40, 12), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
        c.set_default_spacing(3);
        c.add_series(Series::with_values("A", (0..30).map(|i| (i * 13) % 31)));
        c.set_xaxis_label_mode(linechart::XAxisLabelMode::Index(0));
        c
    });
}

#[test]
fn check_scroll_custom_spans_and_resize() {
    let script = "
        Paint('1. Custom spans')
        CheckHash(0x19BD1F64CC06CB53)
        Key.Pressed(Right,10)
        Paint('2. Scrolled - the first span is partially hidden')
        CheckHash(0x70678DB95B5289D9)
        Resize(80,12)
        Paint('3. Everything fits after resize - scroll goes back to 0')
        CheckHash(0x37E915FBA8C5DF76)
    ";
    run_paint_test(script, Size::new(40, 12), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::ScrollBars);
        c.add_series(Series::with_values("A", (0..18).map(|i| (i * 7) % 11)));
        c.set_default_spacing(2);
        let spans = [BarSpan::new(0, 6, "First"), BarSpan::new(6, 6, "Second"), BarSpan::new(12, 6, "Third")];
        c.set_xaxis_label_mode(linechart::XAxisLabelMode::Custom(&spans));
        c
    });
}

// ---------------------------------------------------------------- hovering

#[test]
fn check_point_index_at() {
    run_with_size(|chart| {
        // 40x10 control, Y axis visible (left margin 8), spacing 2, column width 1 -> one point every 3 cells
        chart.add_series(Series::with_values("A", 1..=30));
        assert_eq!(chart.point_index_at(7, 5), None); // on the Y axis
        assert_eq!(chart.point_index_at(8, 5), None); // first spacing cell, closer to nothing
        assert_eq!(chart.point_index_at(9, 5), Some(0));
        assert_eq!(chart.point_index_at(10, 5), Some(0)); // the marker of point 0
        assert_eq!(chart.point_index_at(11, 5), Some(0));
        assert_eq!(chart.point_index_at(12, 5), Some(1));
        assert_eq!(chart.point_index_at(39, 0), Some(10));
        assert_eq!(chart.point_index_at(40, 5), None); // outside the control
        assert_eq!(chart.point_index_at(10, 9), Some(0)); // last plot row (no X axis)
        assert_eq!(chart.point_index_at(10, 10), None); // below the plot
        assert_eq!(chart.point_index_at(10, -1), None);
        // scrolled
        chart.ensure_visible(29);
        let scroll = chart.left_scroll;
        assert_eq!(chart.point_index_at(10 - scroll + 3 * 20, 5), Some(20));
    });
}

#[test]
fn check_point_index_at_beyond_last_point() {
    run_with_size(|chart| {
        chart.add_series(Series::with_values("A", [1, 2, 3]));
        assert_eq!(chart.point_index_at(17, 5), Some(2));
        assert_eq!(chart.point_index_at(18, 5), None);
        assert_eq!(chart.point_index_at(30, 5), None);
    });
}

#[test]
fn check_point_index_at_empty_chart() {
    run_with_size(|chart| {
        assert_eq!(chart.point_index_at(10, 5), None);
        chart.add_series(Series::new("A"));
        assert_eq!(chart.point_index_at(10, 5), None);
    });
}

#[test]
fn check_show_point_tooltip_branches() {
    run_with_size(|chart| {
        // no series, and a series index past the end: the previous text stays
        chart.tooltip_text = String::from("kept");
        chart.show_point_tooltip(PointPos::new(0, 0));
        chart.show_point_tooltip(PointPos::new(u32::MAX, 0));
        assert_eq!(chart.tooltip_text, "kept");

        chart.add_series(Series::with_values("", [BarBuilder::new(1000).build()]));
        chart.add_series(Series::with_values("RAM", [20, 22]));
        chart.add_series(Series::with_values("CPU", [BarBuilder::new(55).label("peak").build()]));
        // the point index is past the end of that series
        chart.show_point_tooltip(PointPos::new(0, 1));
        chart.show_point_tooltip(PointPos::new(2, 1));
        assert_eq!(chart.tooltip_text, "kept");

        // value only: no series name, no X label, no note
        chart.show_point_tooltip(PointPos::new(0, 0));
        assert_eq!(chart.tooltip_text, "1,000");
        // series name, still no X label and no note
        chart.show_point_tooltip(PointPos::new(1, 0));
        assert_eq!(chart.tooltip_text, "RAM\n20");

        // an empty label counts as no label; the next position has a name
        chart.set_xaxis_labels(["", "Feb"]);
        chart.show_point_tooltip(PointPos::new(0, 0));
        assert_eq!(chart.tooltip_text, "1,000");
        chart.show_point_tooltip(PointPos::new(1, 1));
        assert_eq!(chart.tooltip_text, "RAM\nFeb: 22");

        chart.set_xaxis_labels(["Jan"]);
        // name, X label and note
        chart.show_point_tooltip(PointPos::new(2, 0));
        assert_eq!(chart.tooltip_text, "CPU\nJan: 55\npeak");
        // note without a series name
        chart.update_series(0, |s| {
            s.set_label("");
            s.get_mut(0).unwrap().set_label("note");
        });
        chart.clear_xaxis_labels();
        chart.show_point_tooltip(PointPos::new(0, 0));
        assert_eq!(chart.tooltip_text, "1,000\nnote");

        // the formatted number does not fit in the buffer, so that part is empty
        let suffix = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
        assert!(suffix.len() > 64);
        chart.set_number_format(FormatNumber::new(10).suffix(suffix));
        chart.show_point_tooltip(PointPos::new(1, 0));
        assert_eq!(chart.tooltip_text, "RAM\n");
        chart.set_xaxis_labels(["Jan"]);
        chart.show_point_tooltip(PointPos::new(2, 0));
        assert_eq!(chart.tooltip_text, "CPU\nJan: \npeak");
    });
}

fn hover_chart(flags: linechart::Flags) -> LineChart<i32> {
    let mut c = LineChart::<i32>::new(layout!("d:f"), flags);
    c.set_default_spacing(4);
    // the Apr point of CPU has a note
    c.add_series(Series::with_values("CPU", [BarBuilder::new(35).build(), 42.into(), 38.into(), BarBuilder::new(55).label("peak").build()]));
    c.add_series(Series::with_values("RAM", [20, 22, 25, 24, 30, 33, 31, 36, 40]));
    c.add_series(Series::with_values("", [5, 5, 10]));
    c.set_xaxis_labels(["Jan", "Feb", "Mar", "Apr"]);
    c.set_xaxis_label_mode(linechart::XAxisLabelMode::Labels);
    c
}

#[test]
fn check_point_at() {
    run_with_size(|chart| {
        // 40x10, no X axis: 9 levels, value 0 -> row 9, value 10 -> row 1; columns at x = 10, 13, ...
        chart.add_series(Series::with_values("A", [0, 10]));
        chart.add_series(Series::with_values("B", [10, 0]));
        assert_eq!(chart.point_at(10, 9), Some(PointPos::new(0, 0)));
        assert_eq!(chart.point_at(10, 8), Some(PointPos::new(0, 0))); // one row away
        assert_eq!(chart.point_at(11, 8), Some(PointPos::new(0, 0))); // and inside the column
        assert_eq!(chart.point_at(10, 7), None); // two rows away from every marker
        assert_eq!(chart.point_at(10, 1), Some(PointPos::new(1, 0)));
        assert_eq!(chart.point_at(10, 0), Some(PointPos::new(1, 0)));
        assert_eq!(chart.point_at(13, 1), Some(PointPos::new(0, 1)));
        assert_eq!(chart.point_at(13, 9), Some(PointPos::new(1, 1)));
        assert_eq!(chart.point_at(4, 9), None); // Y axis
        // overlapping markers: the series drawn last wins
        chart.add_series(Series::with_values("C", [0]));
        assert_eq!(chart.point_at(10, 9), Some(PointPos::new(2, 0)));
        assert_eq!(chart.point_at(13, 9), Some(PointPos::new(1, 1)));
    });
}

#[test]
fn check_point_at_framed_markers() {
    run_with_size(|chart| {
        // framed markers (5x3): 7 levels; value 0 -> middle row 8 (rows 7..9), value 10 -> middle row 2 (rows 1..3)
        // columns are 5 wide, one every 7 cells: column 0 at x = 10..14, column 1 at x = 17..21
        chart.set_default_marker(Marker::Framed(BarFramedPointType::Square));
        chart.add_series(Series::with_values("A", [0, 10]));
        assert_eq!(chart.point_at(10, 9), Some(PointPos::new(0, 0)));
        assert_eq!(chart.point_at(14, 7), Some(PointPos::new(0, 0)));
        assert_eq!(chart.point_at(12, 6), Some(PointPos::new(0, 0))); // one row above the frame
        assert_eq!(chart.point_at(12, 5), None);
        assert_eq!(chart.point_at(19, 3), Some(PointPos::new(0, 1)));
        assert_eq!(chart.point_at(19, 4), Some(PointPos::new(0, 1)));
        assert_eq!(chart.point_at(19, 5), None);
        assert_eq!(chart.point_at(19, 0), Some(PointPos::new(0, 1))); // the empty row above the plot
    });
}

#[test]
fn check_hover_point_tooltip() {
    let script = "
        Paint('1. Initial state')
        CheckHash(0xAE1B20625165EBED)
        Mouse.Move(13,7)
        Paint('2. On CPU/Jan: CPU, Jan: 35')
        CheckHash(0xE103AA87C014F15B)
        Mouse.Move(13,8)
        Paint('3. One row below - still CPU/Jan')
        CheckHash(0xE103AA87C014F15B)
        Mouse.Move(13,9)
        Paint('4. Two rows away from every marker - no tooltip')
        CheckHash(0xAE1B20625165EBED)
        Mouse.Move(13,11)
        Paint('5. On RAM/Jan - RAM, Jan: 20')
        CheckHash(0xB3FA0029AAB18A3B)
        Mouse.Move(13,15)
        Paint('6. Unnamed series - only Jan: 5')
        CheckHash(0xD66C0031112C86A6)
        Mouse.Move(28,2)
        Paint('7. On CPU/Apr - CPU, Apr: 55 and its note (peak)')
        CheckHash(0x1F32DA6A770ADF7C)
    ";
    run_paint_test(script, Size::new(46, 20), || hover_chart(linechart::Flags::ScrollBars));
}

#[test]
fn check_hover_group_tooltip_and_highlight() {
    let script = "
        Paint('1. Initial state')
        CheckHash(0xAE1B20625165EBED)
        Mouse.Move(13,6)
        Paint('2. Group hover on Jan: highlighted column, label, CPU, RAM and the unnamed series')
        CheckHash(0x5E3379651839428D)
        Mouse.Move(16,6)
        Paint('3. Between two points - the nearest one (Feb)')
        CheckHash(0x9F04CE742CF22598)
        Mouse.Move(28,2)
        Paint('4. Apr - CPU shows its note (peak), the third series has no point here')
        CheckHash(0x3C5C5E419771706D)
        Mouse.Move(38,9)
        Paint('5. Point 6 - only RAM, no label')
        CheckHash(0x7545196A4034462)
        Mouse.Move(5,6)
        Paint('6. Over the Y axis labels - no tooltip, no highlight')
        CheckHash(0xAE1B20625165EBED)
    ";
    run_paint_test(script, Size::new(46, 20), || hover_chart(linechart::Flags::ScrollBars | linechart::Flags::GroupHover));
}

#[test]
fn check_hover_hidden_on_scroll() {
    let script = "
        Paint('1. Initial state')
        CheckHash(0xE6D3D1C59FD0CC8D)
        Mouse.Move(18,6)
        Paint('2. Group hover on Feb')
        CheckHash(0x75246F8B9BB06D94)
        Key.Pressed(Right,2)
        Paint('3. Scrolling hides the tooltip and the highlight')
        CheckHash(0x4B72229E48E5B586)
        Mouse.Move(19,6)
        Paint('4. Moving the mouse shows it again')
        CheckHash(0x39E7D07D258F972)
    ";
    run_paint_test(script, Size::new(40, 20), || hover_chart(linechart::Flags::ScrollBars | linechart::Flags::GroupHover));
}

// ---------------------------------------------------------------- selection

#[test]
fn check_select_point_api() {
    run_with_size(|chart| {
        chart.add_series(Series::with_values("A", 1..=10));
        chart.add_series(Series::with_values("B", 1..=3));
        assert_eq!(chart.selected_point(), None);
        chart.select_point(1, 2);
        assert_eq!(chart.selected_point(), Some((1, 2)));
        // points that do not exist clear the selection
        chart.select_point(1, 3);
        assert_eq!(chart.selected_point(), None);
        chart.select_point(5, 0);
        assert_eq!(chart.selected_point(), None);
        chart.select_point(0, 9);
        assert_eq!(chart.selected_point(), Some((0, 9)));
        chart.clear_selection();
        assert_eq!(chart.selected_point(), None);
    });
}

#[test]
fn check_selection_follows_data_changes() {
    run_with_size(|chart| {
        chart.add_series(Series::with_values("A", 1..=10));
        chart.add_series(Series::with_values("B", 1..=10));
        chart.add_series(Series::with_values("C", 1..=10));
        chart.select_point(2, 7);
        // editing a point keeps the selection
        chart.update_series(2, |s| {
            s.get_mut(7).unwrap().set_draw_mode(BarDrawMode::FramedPoint(BarFramedPointType::Filled));
        });
        assert_eq!(chart.selected_point(), Some((2, 7)));
        // removing an earlier series shifts the selected series index
        chart.remove_series(0);
        assert_eq!(chart.selected_point(), Some((1, 7)));
        // removing the selected series clears the selection
        chart.remove_series(1);
        assert_eq!(chart.selected_point(), None);
        // shrinking the series below the selected index clears it too
        chart.select_point(0, 7);
        chart.update_series(0, |s| s.remove_first(2));
        assert_eq!(chart.selected_point(), Some((0, 7)));
        chart.update_series(0, |s| s.remove_first(1));
        assert_eq!(chart.selected_point(), None);
    });
}

#[test]
fn check_selection_events() {
    #[Window(events = LineChartEvents<i32>, internal: true)]
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
            let mut chart = LineChart::<i32>::new(layout!("x:0,y:1,w:44,h:12"), linechart::Flags::None);
            chart.set_default_spacing(4);
            chart.add_series(Series::with_values("CPU", [35, 42, 38]));
            chart.add_series(Series::with_values("RAM", [20, 22, 25, 24]));
            win.add(chart);
            win
        }
    }
    impl LineChartEvents<i32> for MyWin {
        fn on_point_selected(&mut self, handle: Handle<LineChart<i32>>, series_index: u32, point_index: u32) -> EventProcessStatus {
            let text = if let Some(chart) = self.control(handle) {
                let series = chart.get_series(series_index as usize).unwrap();
                let value = series.get(point_index as usize).unwrap().value();
                format!("{} #{} = {}", series.label(), point_index, value)
            } else {
                String::from("?")
            };
            let info = self.info;
            if let Some(label) = self.control_mut(info) {
                label.set_caption(&text);
            }
            EventProcessStatus::Processed
        }
        fn on_clear_selection(&mut self, _: Handle<LineChart<i32>>) -> EventProcessStatus {
            let info = self.info;
            if let Some(label) = self.control_mut(info) {
                label.set_caption("No selection");
            }
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint('1. No selection')
        CheckHash(0x216EB2B84EB445E9)
        Mouse.Click(13,5,left)
        Paint('2. Click on CPU/Jan -> CPU #0 = 35')
        CheckHash(0x770DCC419FCDB2D5)
        Key.Pressed(Right)
        Paint('3. Right -> CPU #1 = 42')
        CheckHash(0xC49F7666A9EC8FA8)
        Key.Pressed(Down)
        Paint('4. Down -> RAM #1 = 22')
        CheckHash(0x1C4EA3F8654265C6)
        Key.Pressed(Right,2)
        Paint('5. Right x2 -> RAM #3 = 24')
        CheckHash(0x47B0B11A9FD570E4)
        Key.Pressed(Up)
        Paint('6. Up - CPU has no point #3 - nothing changes')
        CheckHash(0x47B0B11A9FD570E4)
        Key.Pressed(Right)
        Paint('7. Right on the last RAM point - nothing changes')
        CheckHash(0x47B0B11A9FD570E4)
        Key.Pressed(Left)
        Paint('8. Left -> RAM #2 = 25')
        CheckHash(0xB5940221BAEB0A2)
        Key.Pressed(Up)
        Paint('9. Up -> CPU #2 = 38')
        CheckHash(0xCE2E3E9874C330C1)
        Key.Pressed(Escape)
        Paint('10. Escape clears the selection')
        CheckHash(0x216EB2B84EB445E9)
        Mouse.Click(13,8,left)
        Paint('11. Click on RAM/Jan -> RAM #0 = 20')
        CheckHash(0x2791C9864E3A534F)
        Mouse.Click(13,11,left)
        Paint('12. Click far from every marker clears the selection')
        CheckHash(0x216EB2B84EB445E9)
    ";
    App::new().size(Size::new(46, 16)).debug_script(script).window(MyWin::new).run().unwrap();
}

fn selection_chart(flags: linechart::Flags) -> LineChart<i32> {
    let mut c = LineChart::<i32>::new(layout!("d:f"), flags);
    c.set_default_spacing(4);
    c.add_series(SeriesBuilder::new("CPU").line(LineMode::Braille).values([10, 35, 20, 60, 40, 55]));
    c.add_series(SeriesBuilder::new("RAM").values([40, 42, 44, 41, 43, 45]));
    c.update_series(1, |s| {
        s.get_mut(4).unwrap().set_draw_mode(BarDrawMode::FramedPoint(BarFramedPointType::RoundSquareBullet));
    });
    c.set_xaxis_label_mode(linechart::XAxisLabelMode::Index(1));
    c
}

#[test]
fn check_selection_frame() {
    let script = "
        Paint('1. No selection')
        CheckHash(0x8E2C4525A477BAB)
        Mouse.Click(24,5,left)
        Paint('2. RAM point 2 selected - only its marker is framed')
        CheckHash(0xBDD46EDAED66D1E)
        Mouse.Click(51,5,left)
        Paint('3. The framed RAM point 5 selected')
        CheckHash(0xF181A28BDB2C5B03)
    ";
    run_paint_test(script, Size::new(70, 14), || selection_chart(linechart::Flags::None));
}

#[test]
fn check_selection_dim_points() {
    let script = "
        Paint('1. No selection - nothing is dimmed')
        CheckHash(0x8E2C4525A477BAB)
        Mouse.Click(42,3,left)
        Paint('2. CPU point 4 selected - everything else (RAM included) is dimmed')
        CheckHash(0x9008D1B8A60746CF)
        Key.Pressed(Escape)
        Paint('3. Escape - back to normal')
        CheckHash(0x8E2C4525A477BAB)
    ";
    run_paint_test(script, Size::new(70, 14), || selection_chart(linechart::Flags::DimPointsOnSelection));
}

#[test]
fn check_selection_keyboard_scrolls_into_view() {
    let script = "
        Paint('1. Initial state')
        CheckHash(0x8204DE6F8A55998D)
        Mouse.Click(13,10,left)
        Paint('2. Point 0 selected')
        CheckHash(0x70050F4378B5B036)
        Key.Pressed(Right,8)
        Paint('3. Point 8 selected - the chart scrolled to show it')
        CheckHash(0x204DB2CCF00F9C9)
        Key.Pressed(Home)
        Paint('4. Home still scrolls (the selection stays on point 8, out of view)')
        CheckHash(0x570FBB86E248BC12)
    ";
    run_paint_test(script, Size::new(40, 14), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::ScrollBars | linechart::Flags::DimPointsOnSelection);
        c.set_default_spacing(4);
        c.add_series(Series::with_values("A", (0..20).map(|i| (i * 7) % 13)));
        c.set_xaxis_label_mode(linechart::XAxisLabelMode::Index(0));
        c
    });
}

// ---------------------------------------------------------------- linechart! macro

#[test]
fn check_macro_data() {
    in_app(|| {
        let c = linechart!(
            "type: i32, d:f, flags: ScrollBars+GroupHover,
            line: Braille, marker: Framed(RoundSquareBullet), spacing: 3,
            labels: [Jan, Feb, 'Mar 2026'],
            series: [
                {CPU, values: [10, 20, {30, label: peak, dm: Point(Diamond)}], attr: green},
                {name: RAM, values: [5, 6], line: Orthogonal(Double, VerticalFirst)},
                {}
            ]"
        );
        assert!(c.flags().contains(linechart::Flags::ScrollBars | linechart::Flags::GroupHover));
        assert_eq!(c.default_line(), LineMode::Braille);
        assert_eq!(c.default_marker(), Marker::Framed(BarFramedPointType::RoundSquareBullet));
        assert_eq!(c.default_spacing(), 3);
        assert_eq!(c.series_count(), 3);
        assert_eq!(c.points_count(), 3);
        assert_eq!(c.xaxis_labels_count(), 3);
        assert_eq!(c.xaxis_label(2), Some("Mar 2026"));
        let cpu = c.get_series(0).unwrap();
        assert_eq!(cpu.label(), "CPU");
        assert_eq!(cpu.line(), None);
        assert_eq!(cpu.attr(), Some(CharAttribute::with_fore_color(Color::Green)));
        assert_eq!(cpu.iter().map(|b| b.value()).collect::<Vec<_>>(), vec![10, 20, 30]);
        assert_eq!(cpu.get(2).unwrap().label(), "peak");
        assert_eq!(cpu.get(2).unwrap().draw_mode(), Some(BarDrawMode::Point(BarPointType::Diamond)));
        let ram = c.get_series(1).unwrap();
        assert_eq!(ram.label(), "RAM");
        assert_eq!(ram.line(), Some(LineMode::Orthogonal(LineType::Double, OrthogonalDirection::VerticalFirst)));
        assert_eq!(ram.attr(), None);
        let empty = c.get_series(2).unwrap();
        assert_eq!(empty.label(), "");
        assert!(empty.is_empty());
    });
}

#[test]
fn check_macro_line_and_marker_forms() {
    in_app(|| {
        let c = linechart!("i32, d:f, line: Line, marker: Point");
        assert_eq!(c.default_line(), LineMode::Line(LineType::Single));
        assert_eq!(c.default_marker(), Marker::Small(BarPointType::Bullet));
        let c = linechart!("i32, d:f, line: Line(Thick), marker: Small('x')");
        assert_eq!(c.default_line(), LineMode::Line(LineType::SingleThick));
        assert_eq!(c.default_marker(), Marker::Small(BarPointType::Custom('x')));
        let c = linechart!("i32, d:f, line: Orthogonal, marker: FramedPoint");
        assert_eq!(c.default_line(), LineMode::Orthogonal(LineType::Single, OrthogonalDirection::Auto));
        assert_eq!(c.default_marker(), Marker::Framed(BarFramedPointType::RoundSquare));
        let c = linechart!("i32, d:f, line: ortho(round), marker: framed('#')");
        assert_eq!(c.default_line(), LineMode::Orthogonal(LineType::SingleRound, OrthogonalDirection::Auto));
        assert_eq!(c.default_marker(), Marker::Framed(BarFramedPointType::Custom('#')));
        let c = linechart!("i32, d:f, line: None, marker: None");
        assert_eq!(c.default_line(), LineMode::None);
        assert_eq!(c.default_marker(), Marker::None);
    });
}

#[test]
fn check_macro_paint_labels() {
    let script = "
        Paint.Enable(false)
        Paint('1. Two series, month labels')
        CheckHash(0x1184D4D5A640B8CC)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(linechart!(
                "type: i32, d:f, scale: Fixed(0, 50), spacing: 4,
                labels: [Jan, Feb, Mar, Apr, May],
                series: [
                    {A, values: [10, 30, 20, 45, 25]},
                    {B, values: [5, 10, 15, 20, 25], line: Braille}
                ]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_macro_paint_framed_index_axis() {
    let script = "
        Paint.Enable(false)
        Paint('1. Framed markers, index axis, no grid')
        CheckHash(0xADC73CE38F58DD75)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(linechart!(
                "type: i32, d:f, scale: Fixed(0, 40), marker: Framed, spacing: 4, xlabels: Index(1), grid: false, yw: 4,
                series: [{A, values: [10, 30, 20, 35]}]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_macro_paint_spans_no_yaxis() {
    let script = "
        Paint.Enable(false)
        Paint('1. Span labels, no Y axis')
        CheckHash(0xE63F6C2E899700F9)
    ";
    App::new()
        .size(Size::new(60, 15))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            w.add(linechart!(
                "type: i32, d:f, scale: fit, yaxis: false, labels: [a, b, c, d],
                xlabels: [{0, 2, H1}, {2, 2, H2}],
                series: [{A, values: [1, 4, 2, 5], line: Orthogonal(Single, HorizontalUntilMiddle)}]"
            ));
            w
        })
        .run()
        .unwrap();
}
