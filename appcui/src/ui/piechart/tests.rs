use crate::prelude::*;

fn chart() -> PieChart {
    PieChart::new(layout!("x:1,y:1,w:20,h:10"), piechart::Flags::None)
}

#[test]
fn check_creation() {
    let c = chart();
    assert_eq!(c.count(), 0);
    assert_eq!(c.value(0), None);
    assert_eq!(c.label(0), None);
}


#[test]
fn check_add_value() {
    let mut c = chart();
    c.add_value(10, "A", None);
    c.add_value(20, "", Some(CharAttribute::with_fore_color(Color::Red)));
    c.add_value(0, "Zero", None);
    assert_eq!(c.count(), 3);
    assert_eq!(c.value(0), Some(10));
    assert_eq!(c.label(0), Some("A"));
    assert_eq!(c.value(1), Some(20));
    assert_eq!(c.label(1), Some(""));
    assert_eq!(c.value(2), Some(0));
    assert_eq!(c.label(2), Some("Zero"));
    assert_eq!(c.value(3), None);
    assert_eq!(c.label(3), None);
}

#[test]
fn check_attributes_are_stored() {
    let mut c = chart();
    let red = CharAttribute::with_fore_color(Color::Red);
    c.add_value(1, "", None);
    c.add_value(2, "", Some(red));
    assert_eq!(c.slices[0].attr, None);
    assert_eq!(c.slices[1].attr, Some(red));
}

#[test]
fn check_large_values() {
    let mut c = chart();
    c.add_value(u64::MAX, "max", None);
    c.add_value(u64::MAX, "max2", None);
    assert_eq!(c.value(0), Some(u64::MAX));
    assert_eq!(c.value(1), Some(u64::MAX));
}

#[test]
fn check_set_value() {
    let mut c = chart();
    c.add_value(10, "A", None);
    c.add_value(20, "B", None);
    c.set_value(1, 99);
    assert_eq!(c.value(0), Some(10));
    assert_eq!(c.value(1), Some(99));
    assert_eq!(c.label(1), Some("B"));
    // out of range: no panic, nothing changes
    c.set_value(5, 1);
    assert_eq!(c.count(), 2);
}

#[test]
fn check_clear() {
    let mut c = chart();
    c.add_value(10, "A", None);
    c.add_value(20, "B", None);
    c.clear();
    assert_eq!(c.count(), 0);
    assert_eq!(c.value(0), None);
    c.add_value(5, "C", None);
    assert_eq!(c.count(), 1);
    assert_eq!(c.label(0), Some("C"));
}

#[test]
fn check_in_window() {
    let script = "
        Paint.Enable(false)
        Paint('empty chart')
    ";
    App::new()
        .size(Size::new(40, 12))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            let mut c = PieChart::new(layout!("d:f"), piechart::Flags::None);
            c.add_value(1, "A", None);
            w.add(c);
            w
        })
        .run()
        .unwrap();
}
