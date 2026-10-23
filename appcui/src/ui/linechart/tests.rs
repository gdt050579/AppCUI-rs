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
    assert_eq!(Marker::Large(BarLargePointType::RoundSquare).width(), 3);
}

#[test]
fn check_marker_height() {
    assert_eq!(Marker::None.height(), 1);
    assert_eq!(Marker::Small(BarPointType::Bullet).height(), 1);
    assert_eq!(Marker::Large(BarLargePointType::Square).height(), 2);
}

#[test]
fn check_marker_from_draw_mode() {
    assert_eq!(Marker::from_draw_mode(BarDrawMode::Point(BarPointType::Bullet)), Some(Marker::Small(BarPointType::Bullet)));
    assert_eq!(
        Marker::from_draw_mode(BarDrawMode::LargePoint(BarLargePointType::Diamond)),
        Some(Marker::Large(BarLargePointType::Diamond))
    );
    assert_eq!(Marker::from_draw_mode(BarDrawMode::Fill(BarFillType::Solid)), None);
    assert_eq!(Marker::from_draw_mode(BarDrawMode::Line(LineType::Single)), None);
    assert_eq!(Marker::from_draw_mode(BarDrawMode::Cap(BarCapType::Solid)), None);
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
        chart.set_default_marker(Marker::Large(BarLargePointType::Circle));
        chart.set_default_spacing(0);
        assert_eq!(chart.default_line(), LineMode::Braille);
        assert_eq!(chart.default_marker(), Marker::Large(BarLargePointType::Circle));
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
fn check_paint_multiple_series_point_labels() {
    let script = "
        Paint('1. Three series of different lengths, PointLabels taken from the second series')
        CheckHash(0x9099A52F3E2A7F75)
    ";
    run_paint_test(script, Size::new(44, 14), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
        c.add_series(Series::with_values("A", [10, 20, 30]));
        c.add_series(
            SeriesBuilder::new("B")
                .attr(charattr!("yellow"))
                .values([
                    BarBuilder::new(50).label("Jan").build(),
                    BarBuilder::new(45).label("Feb").build(),
                    BarBuilder::new(35).label("Mar").build(),
                    BarBuilder::new(40).label("Apr").build(),
                    BarBuilder::new(55).label("May").build(),
                ]),
        );
        c.add_series(Series::with_values("C", [0, 5, 0, 5, 0, 5, 0]));
        c.set_default_spacing(4);
        c.set_xaxis_label_mode(linechart::XAxisLabelMode::PointLabels);
        c
    });
}

#[test]
fn check_paint_large_markers_fixed_scale() {
    let script = "
        Paint('1. Large markers, one point overridden with a small red marker, Fixed 0..50 (100 clamped to the top)')
        CheckHash(0xCFF5CA694635AB32)
    ";
    run_paint_test(script, Size::new(44, 14), || {
        let mut c = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
        c.set_default_marker(Marker::Large(BarLargePointType::RoundSquare));
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
fn check_paint_lines_large_markers() {
    let script = "
        Paint('1. Lines between large markers start from the marker centers')
        CheckHash(0x22C452CF385EAC92)
    ";
    run_paint_test(script, Size::new(50, 16), || {
        let mut c = chart_with_line(LineMode::Line(LineType::Single));
        c.set_default_marker(Marker::Large(BarLargePointType::Circle));
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
