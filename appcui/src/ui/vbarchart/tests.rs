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
    let chart = chart_with_values(&[1, 2, 3]);
    assert_eq!(chart.bars_count(), 3);
    assert_eq!(chart.get_bar(0).map(|b| b.value()), Some(1));
    assert_eq!(chart.get_bar(2).map(|b| b.value()), Some(3));
    assert!(chart.get_bar(3).is_none());
}

#[test]
fn check_modify_bar() {
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
}

#[test]
fn check_modify_bar_clear_overrides() {
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
}

#[test]
fn check_update_bars_edit_insert_delete() {
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
}

#[test]
fn check_update_bars_set_clear_iter() {
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
}

#[test]
fn check_scale_sequences() {
    let script = "
        Paint.Enable(false)
        Paint('1. Mixed, positive and negative sequences')
        CheckHash(0xF5BB896CABAD0451)
    ";
    App::new().size(Size::new(80, 48)).debug_script(script).window(|| {
        let mut w = window!("Scales,a:c,w:76,h:44");
        w.add(label!("'-2, -1, 0, 1, 2',x:1,y:1,w:30"));
        let mut mixed = VBarChart::<i32>::new(layout!("x:1,y:2,w:72,h:12"), vbarchart::Flags::None);
        mixed.set_xaxis_label_mode(vbarchart::XAxisLabelMode::Index { start: 1 });
        mixed.add_bars([-2, -1, 0, 1, 2]);
        w.add(mixed);

        w.add(label!("'0, 1, 2',x:1,y:14,w:30"));
        let mut positive = VBarChart::<i32>::new(layout!("x:1,y:15,w:72,h:12"), vbarchart::Flags::None);
        positive.set_xaxis_label_mode(vbarchart::XAxisLabelMode::Index { start: 1 });
        positive.add_bars([0, 1, 2]);
        w.add(positive);

        w.add(label!("'-2, -1, 0',x:1,y:27,w:30"));
        let mut negative = VBarChart::<i32>::new(layout!("x:1,y:28,w:72,h:12"), vbarchart::Flags::None);
        negative.set_xaxis_label_mode(vbarchart::XAxisLabelMode::Index { start: 1 });
            negative.add_bars([-2, -1, 0]);
        w.add(negative);
        w
    }).run().unwrap();
}
