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
fn check_marker_to_draw_mode() {
    assert_eq!(Marker::None.draw_mode(), None);
    assert_eq!(Marker::Small(BarPointType::Square).draw_mode(), Some(BarDrawMode::Point(BarPointType::Square)));
    assert_eq!(
        Marker::Large(BarLargePointType::Circle).draw_mode(),
        Some(BarDrawMode::LargePoint(BarLargePointType::Circle))
    );
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
