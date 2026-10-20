use crate::prelude::*;

fn chart() -> PieChart {
    PieChart::new(layout!("x:1,y:1,w:20,h:10"), piechart::Flags::None)
}

fn in_app(test: impl FnOnce() + 'static) {
    App::new()
        .size(Size::new(40, 12))
        .debug_script("Paint.Enable(false)")
        .window(|| {
            test();
            window!("Test,d:f")
        })
        .run()
        .unwrap();
}

#[test]
fn check_creation() {
    in_app(|| {
        let c = chart();
        assert_eq!(c.count(), 0);
        assert_eq!(c.value(0), None);
        assert_eq!(c.label(0), None);
    });
}


#[test]
fn check_add_value() {
    in_app(|| {
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
    });
}

#[test]
fn check_attributes_are_stored() {
    in_app(|| {
        let mut c = chart();
        let red = CharAttribute::with_fore_color(Color::Red);
        c.add_value(1, "", None);
        c.add_value(2, "", Some(red));
        assert_eq!(c.slices[0].attr, None);
        assert_eq!(c.slices[1].attr, Some(red));
    });
}

#[test]
fn check_large_values() {
    in_app(|| {
        let mut c = chart();
        c.add_value(u64::MAX, "max", None);
        c.add_value(u64::MAX, "max2", None);
        assert_eq!(c.value(0), Some(u64::MAX));
        assert_eq!(c.value(1), Some(u64::MAX));
    });
}

#[test]
fn check_set_value() {
    in_app(|| {
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
    });
}

#[test]
fn check_clear() {
    in_app(|| {
        let mut c = chart();
        c.add_value(10, "A", None);
        c.add_value(20, "B", None);
        c.clear();
        assert_eq!(c.count(), 0);
        assert_eq!(c.value(0), None);
        c.add_value(5, "C", None);
        assert_eq!(c.count(), 1);
        assert_eq!(c.label(0), Some("C"));
    });
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

// ---------------------------------------------------------------------------
// step 2: proportions
// ---------------------------------------------------------------------------
use super::piechart::{largest_remainder, update_proportions, FULL, PERCENT_UNITS};
use super::Slice;

struct Slices {
    slices: Vec<Slice>,
}

impl Slices {
    fn set_value(&mut self, index: usize, value: u64) {
        self.slices[index].value = value;
        update_proportions(&mut self.slices);
    }
    fn add_value(&mut self, value: u64) {
        self.slices.push(Slice::new(value, "", None));
        update_proportions(&mut self.slices);
    }
    fn clear(&mut self) {
        self.slices.clear();
        update_proportions(&mut self.slices);
    }
}

fn chart_with(values: &[u64]) -> Slices {
    let mut slices: Vec<Slice> = values.iter().map(|v| Slice::new(*v, "", None)).collect();
    update_proportions(&mut slices);
    Slices { slices }
}

fn percents(c: &Slices) -> Vec<u16> {
    c.slices.iter().map(|s| s.percent).collect()
}

fn ranges(c: &Slices) -> Vec<(u64, u64)> {
    c.slices.iter().map(|s| (s.start, s.end)).collect()
}

fn check_invariants(c: &Slices) {
    let total: u128 = c.slices.iter().map(|s| s.value as u128).sum();
    let mut pos = 0;
    for s in &c.slices {
        assert_eq!(s.start, pos, "slices must be contiguous");
        assert!(s.end >= s.start);
        if s.value == 0 {
            assert_eq!(s.start, s.end, "zero-value slices take no space");
            assert_eq!(s.percent, 0);
        }
        pos = s.end;
    }
    let percent_sum: u32 = c.slices.iter().map(|s| s.percent as u32).sum();
    if total == 0 {
        assert_eq!(pos, 0);
        assert_eq!(percent_sum, 0);
    } else {
        assert_eq!(pos, FULL, "the last slice must end at FULL");
        assert_eq!(percent_sum, PERCENT_UNITS, "percents must add up to 100%");
    }
}

#[test]
fn check_proportions_empty() {
    let c = chart_with(&[]);
    check_invariants(&c);
}

#[test]
fn check_proportions_all_zero() {
    let c = chart_with(&[0, 0, 0]);
    assert_eq!(ranges(&c), vec![(0, 0), (0, 0), (0, 0)]);
    assert_eq!(percents(&c), vec![0, 0, 0]);
    check_invariants(&c);
}

#[test]
fn check_proportions_single_slice() {
    let c = chart_with(&[7]);
    assert_eq!(ranges(&c), vec![(0, FULL)]);
    assert_eq!(percents(&c), vec![10000]);
    check_invariants(&c);
}

#[test]
fn check_proportions_simple() {
    let c = chart_with(&[1, 1, 2]);
    assert_eq!(ranges(&c), vec![(0, FULL / 4), (FULL / 4, FULL / 2), (FULL / 2, FULL)]);
    assert_eq!(percents(&c), vec![2500, 2500, 5000]);
    check_invariants(&c);
}

#[test]
fn check_proportions_thirds() {
    // 33.333...% each: the leftover 0.01% goes to the first slice (tie -> lower index)
    let c = chart_with(&[1, 1, 1]);
    assert_eq!(percents(&c), vec![3334, 3333, 3333]);
    check_invariants(&c);
}

#[test]
fn check_proportions_with_zero_slices() {
    let c = chart_with(&[0, 3, 0, 1, 0]);
    assert_eq!(ranges(&c), vec![(0, 0), (0, FULL / 4 * 3), (FULL / 4 * 3, FULL / 4 * 3), (FULL / 4 * 3, FULL), (FULL, FULL)]);
    assert_eq!(percents(&c), vec![0, 7500, 0, 2500, 0]);
    check_invariants(&c);
}

#[test]
fn check_proportions_tiny_among_huge() {
    let c = chart_with(&[u64::MAX / 2, 1, u64::MAX / 2]);
    // the tiny slice still exists, but rounds to 0%
    assert_eq!(c.slices[1].percent, 0);
    assert!(c.slices[1].end >= c.slices[1].start);
    check_invariants(&c);
}

#[test]
fn check_proportions_overflow() {
    // the total does not fit in u64
    let c = chart_with(&[u64::MAX, u64::MAX, u64::MAX, u64::MAX]);
    assert_eq!(ranges(&c), vec![(0, FULL / 4), (FULL / 4, FULL / 2), (FULL / 2, FULL / 4 * 3), (FULL / 4 * 3, FULL)]);
    assert_eq!(percents(&c), vec![2500, 2500, 2500, 2500]);
    check_invariants(&c);
}

#[test]
fn check_proportions_many_slices() {
    // 7 equal slices: 14.2857...% each -> 1429 x 5 + 1428 x 2 (by index order on ties)
    let c = chart_with(&[5; 7]);
    assert_eq!(percents(&c), vec![1429, 1429, 1429, 1429, 1428, 1428, 1428]);
    check_invariants(&c);
    // many slices with different values
    let values: Vec<u64> = (1..=97).map(|i| i * i * 31 + 7).collect();
    check_invariants(&chart_with(&values));
}

#[test]
fn check_proportions_update_on_change() {
    let mut c = chart_with(&[1, 1]);
    assert_eq!(percents(&c), vec![5000, 5000]);
    c.set_value(1, 3);
    assert_eq!(percents(&c), vec![2500, 7500]);
    check_invariants(&c);
    c.set_value(0, 0);
    assert_eq!(percents(&c), vec![0, 10000]);
    check_invariants(&c);
    c.add_value(3);
    assert_eq!(percents(&c), vec![0, 5000, 5000]);
    check_invariants(&c);
    c.clear();
    check_invariants(&c);
}

#[test]
fn check_largest_remainder() {
    let mut out = [0u32; 3];
    largest_remainder([1u64, 1, 1].into_iter(), 3, 100, &mut out);
    assert_eq!(out, [34, 33, 33]);
    // the largest remainder wins, not the lower index
    largest_remainder([1u64, 2, 2].into_iter(), 5, 4, &mut out);
    // exact shares 0.8, 1.6, 1.6 -> floors 0,1,1 (sum 2) -> remainders .8,.6,.6 -> +1 to idx 0 and idx 1
    assert_eq!(out, [1, 2, 1]);
    // total 0
    largest_remainder([0u64, 0, 0].into_iter(), 0, 100, &mut out);
    assert_eq!(out, [0, 0, 0]);
    // a zero value never receives a unit
    let mut out2 = [0u32; 4];
    largest_remainder([0u64, 1, 0, 2].into_iter(), 3, 100, &mut out2);
    assert_eq!(out2, [0, 33, 0, 67]);
}

#[test]
fn check_surface_paint() {
    let script = "
        Paint.Enable(false)
        Paint('pie with 1/3 and 2/3 drawn on the chart background')
        CheckHash(0xD8E8A7FF19EBA5CE)
    ";
    App::new()
        .size(Size::new(40, 12))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            let mut c = PieChart::new(layout!("x:1,y:1,w:20,h:6"), piechart::Flags::None);
            c.add_value(1, "A", None);
            c.add_value(2, "B", None);
            w.add(c);
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_proportions_in_chart() {
    // the chart itself keeps the proportions up to date
    in_app(|| {
        let mut c = chart();
        c.add_value(1, "A", None);
        c.add_value(3, "B", None);
        assert_eq!(c.slices[0].percent, 2500);
        assert_eq!(c.slices[1].percent, 7500);
        assert_eq!(c.slices[1].end, FULL);
        c.set_value(0, 3);
        assert_eq!(c.slices[0].percent, 5000);
        assert_eq!(c.slices[0].end, FULL / 2);
        c.clear();
        assert_eq!(c.count(), 0);
    });
}


// ---------------------------------------------------------------------------
// step 3: rendering (solid circle, half blocks, automatic colors)
// ---------------------------------------------------------------------------
use super::piechart::slice_at;
use super::shapes::Ellipse;
use super::shapes::hole_size;

fn ellipse(cx: f64, cy: f64, rx: f64, ry: f64) -> Ellipse {
    ellipse_with_hole(cx, cy, rx, ry, super::HollowThickness::None)
}

fn ellipse_with_hole(cx: f64, cy: f64, rx: f64, ry: f64, hollow: super::HollowThickness) -> Ellipse {
    Ellipse::new(cx, cy, rx, ry, hollow)
}

fn check_render(name: &str, values: &'static [u64], w: u32, h: u32, hash: u64) {
    let script = format!(
        "
        Paint.Enable(false)
        Paint('{name}')
        CheckHash(0x{hash:X})
    "
    );
    App::new()
        .size(Size::new(44, 16))
        .debug_script(&script)
        .window(move || {
            let mut win = window!("Test,d:f");
            let mut c = PieChart::new(LayoutBuilder::new().x(1).y(0).width(w).height(h).build(), piechart::Flags::None);
            for v in values {
                c.add_value(*v, "", None);
            }
            win.add(c);
            win
        })
        .run()
        .unwrap();
}

#[test]
fn check_circle_fraction() {
    // circle centered at (10, 10), radius 5
    let c = ellipse(10.0, 10.0, 5.0, 5.0);
    let close = |a: Option<f64>, b: f64| (a.unwrap() - b).abs() < 1e-9;
    assert!(close(c.fraction(10.0, 6.0), 0.0)); // 12 o'clock
    assert!(close(c.fraction(14.0, 10.0), 0.25)); // 3 o'clock
    assert!(close(c.fraction(10.0, 14.0), 0.5)); // 6 o'clock
    assert!(close(c.fraction(6.0, 10.0), 0.75)); // 9 o'clock
    assert!(close(c.fraction(13.0, 7.0), 0.125)); // half way between 12 and 3
    assert_eq!(c.fraction(15.1, 10.0), None); // outside
    assert_eq!(c.fraction(14.0, 14.0), None); // outside (corner of the bounding box)
    // just left of 12 o'clock is almost 1.0, never 1.0
    let f = c.fraction(10.0 - 1e-12, 6.0).unwrap();
    assert!((0.0..1.0).contains(&f));
}

#[test]
fn check_slice_at() {
    let mut slices: Vec<Slice> = [0u64, 1, 0, 2, 1, 0].iter().map(|v| Slice::new(*v, "", None)).collect();
    update_proportions(&mut slices);
    // visible slices: 1 -> [0, .25), 3 -> [.25, .75), 4 -> [.75, 1)
    assert_eq!(slice_at(&slices, 0.0), 1);
    assert_eq!(slice_at(&slices, 0.2499), 1);
    assert_eq!(slice_at(&slices, 0.25), 3);
    assert_eq!(slice_at(&slices, 0.7499), 3);
    assert_eq!(slice_at(&slices, 0.75), 4);
    assert_eq!(slice_at(&slices, 0.999999), 4);
}

#[test]
fn check_slice_colors() {
    for (name, themes) in [("default", Themes::Default), ("dark gray", Themes::DarkGray), ("light", Themes::Light)] {
        let theme = Theme::new(themes);
        let mut slices: Vec<Slice> = (0..10).map(|_| Slice::new(1, "", None)).collect();
        // an explicit attribute: its foreground
        slices[2].attr = Some(CharAttribute::with_fore_color(Color::Pink));
        for (i, s) in slices.iter().enumerate() {
            let expected = if i == 2 { Color::Pink } else { theme.chart.bar[i % 4].foreground };
            assert_eq!(s.color(i, &theme), expected, "{name}: slice {i}");
        }
        // without an attribute, the colors repeat every 4 slices
        assert_eq!(slices[0].color(0, &theme), slices[4].color(4, &theme));
        assert_eq!(slices[1].color(1, &theme), slices[9].color(9, &theme));
    }
}

#[test]
fn check_explicit_color() {
    let script = "
        Paint.Enable(false)
        Paint('red and green halves')
        CheckHash(0x1FF0F62AA7C23C99)
    ";
    App::new()
        .size(Size::new(44, 16))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            let mut c = PieChart::new(layout!("x:1,y:0,w:20,h:10"), piechart::Flags::None);
            c.add_value(1, "", Some(CharAttribute::with_fore_color(Color::Red)));
            c.add_value(1, "", Some(CharAttribute::with_fore_color(Color::Green)));
            w.add(c);
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_render_even_size() {
    check_render("4 slices, 40x12", &[50, 25, 15, 10], 40, 12, 0x9F2DC2BEDBBDE643);
}

#[test]
fn check_render_odd_size() {
    check_render("4 slices, 21x7", &[50, 25, 15, 10], 21, 7, 0x50636918CC5D083);
}

#[test]
fn check_render_tall() {
    // width limits the circle; it is centered vertically
    check_render("3 slices, 10x12", &[1, 1, 1], 10, 12, 0xE628A9DBD77270E5);
}

#[test]
fn check_render_single_slice() {
    check_render("one slice fills the circle", &[5], 20, 8, 0x72D4D01D6E181B31);
}

#[test]
fn check_render_empty() {
    check_render("no slices: background only", &[], 20, 8, 0x77243921445A6CC9);
}

#[test]
fn check_render_all_zero() {
    check_render("all values 0: background only", &[0, 0], 20, 8, 0x77243921445A6CC9);
}

#[test]
fn check_render_tiny_slice() {
    // a tiny slice may cover no pixel at all; it must not break the drawing
    check_render("tiny slice", &[1_000_000, 1, 1_000_000], 20, 8, 0xF94C6C8548652881);
}

#[test]
fn check_render_many_slices() {
    check_render("13 slices", &[1; 13], 40, 14, 0x6386A76B510EA877);
}

#[test]
fn check_render_disabled() {
    let script = "
        Paint.Enable(false)
        Paint('disabled: gray silhouette')
        CheckHash(0xC2B786151B233A29)
    ";
    App::new()
        .size(Size::new(44, 16))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            let mut c = PieChart::new(layout!("x:1,y:0,w:20,h:10"), piechart::Flags::None);
            c.add_value(1, "", None);
            c.add_value(2, "", None);
            c.set_enabled(false);
            w.add(c);
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_render_after_set_value() {
    let script = "
        Paint.Enable(false)
        Paint('after set_value: 1/4 and 3/4')
        CheckHash(0x6E8CB6E15B0ECAAD)
    ";
    App::new()
        .size(Size::new(44, 16))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            let mut c = PieChart::new(layout!("x:1,y:0,w:20,h:10"), piechart::Flags::None);
            c.add_value(1, "", None);
            c.add_value(1, "", None);
            c.set_value(1, 3);
            w.add(c);
            w
        })
        .run()
        .unwrap();
}

// ---------------------------------------------------------------------------
// draw modes: ellipse and hollow shapes
// ---------------------------------------------------------------------------
use super::draw_mode::{plot_area, DrawMode};
use super::{DrawModeRenderer, DrawModeShape, HollowThickness};

fn check_render_with(name: &str, values: &'static [u64], w: u32, h: u32, setup: fn(&mut PieChart), hash: u64) {
    let script = format!(
        "
        Paint.Enable(false)
        Paint('{name}')
        CheckHash(0x{hash:X})
    "
    );
    App::new()
        .size(Size::new(44, 16))
        .debug_script(&script)
        .window(move || {
            let mut win = window!("Test,d:f");
            let mut c = PieChart::new(LayoutBuilder::new().x(1).y(0).width(w).height(h).build(), piechart::Flags::None);
            for v in values {
                c.add_value(*v, "", None);
            }
            setup(&mut c);
            win.add(c);
            win
        })
        .run()
        .unwrap();
}

#[test]
fn check_ellipse_fraction() {
    // ellipse centered at (10, 10), 16 units wide, 8 units tall
    let e = ellipse(10.0, 10.0, 8.0, 4.0);
    let close = |a: Option<f64>, b: f64| (a.unwrap() - b).abs() < 1e-9;
    assert!(close(e.fraction(10.0, 6.5), 0.0)); // 12 o'clock
    assert!(close(e.fraction(17.5, 10.0), 0.25)); // 3 o'clock
    assert!(close(e.fraction(10.0, 13.5), 0.5)); // 6 o'clock
    assert!(close(e.fraction(2.5, 10.0), 0.75)); // 9 o'clock
    // the diagonal of the bounding box (not 45 degrees) splits the first quadrant in half
    assert!(close(e.fraction(10.0 + 4.0, 10.0 - 2.0), 0.125));
    assert_eq!(e.fraction(18.5, 10.0), None);
    assert_eq!(e.fraction(10.0, 14.5), None);
}

#[test]
fn check_ellipse_hole() {
    let e = ellipse_with_hole(10.0, 10.0, 6.0, 6.0, HollowThickness::Percentage(50)); // hole radius 3
    assert_eq!(e.fraction(10.0, 10.0), None); // center: in the hole
    assert_eq!(e.fraction(10.0, 8.0), None); // 2 units from the center: in the hole
    assert!(e.fraction(10.0, 6.0).is_some()); // 4 units: in the band
    assert!(e.fraction(10.0, 4.5).is_some()); // 5.5 units: in the band
    assert_eq!(e.fraction(10.0, 3.5), None); // 6.5 units: outside
    // a hole larger than the shape is capped: nothing is left
    let e = ellipse_with_hole(10.0, 10.0, 6.0, 6.0, HollowThickness::Absolute(100));
    assert_eq!(e.fraction(10.0, 4.5), None);
}

#[test]
fn check_ellipse_areas_are_proportional() {
    // four equal slices on a very stretched ellipse must cover (almost) the same area
    let mut slices: Vec<Slice> = [1u64, 1, 1, 1].iter().map(|v| Slice::new(*v, "", None)).collect();
    update_proportions(&mut slices);
    // 80 x 10 cells = 80 x 20 physical units
    let shape = Ellipse::fill(Size::new(80, 10), HollowThickness::None);
    // sample a 1 x 1 physical unit grid (the half block resolution)
    let mut counts = [0usize; 4];
    for y in 0..20 {
        for x in 0..80 {
            if let Some(id) = shape.slice_at(x as f64 + 0.5, y as f64 + 0.5, &slices) {
                counts[id] += 1;
            }
        }
    }
    let total: usize = counts.iter().sum();
    for c in counts {
        let share = c as f64 / total as f64;
        assert!((share - 0.25).abs() < 0.01, "{counts:?}");
    }
}

#[test]
fn check_hole_radii() {
    // shape radii: 10 x 5 (physical units)
    assert_eq!(hole_size(HollowThickness::None, 10.0, 5.0), (0.0, 0.0));
    assert_eq!(hole_size(HollowThickness::Percentage(0), 10.0, 5.0), (0.0, 0.0));
    assert_eq!(hole_size(HollowThickness::Percentage(50), 10.0, 5.0), (5.0, 2.5));
    assert_eq!(hole_size(HollowThickness::Percentage(200), 10.0, 5.0), (10.0, 5.0));
    // 4 characters wide, same proportions as the shape
    assert_eq!(hole_size(HollowThickness::Absolute(4), 10.0, 5.0), (2.0, 1.0));
    // 6 x 2 characters = 6 x 4 physical units
    assert_eq!(hole_size(HollowThickness::Rectangle(6, 2), 10.0, 5.0), (3.0, 2.0));
    // default: solid circle, half blocks, no margins
    assert_eq!(DrawMode::default(), DrawMode::new(DrawModeShape::Circle, DrawModeRenderer::HalfBlocks, HollowThickness::None));
}

#[test]
fn check_render_ellipse() {
    check_render_with("ellipse, 4 slices, 40x10", &[50, 25, 15, 10], 40, 10, |c| c.set_shape(DrawModeShape::Ellipse), 0x814103108B058B2D);
}

#[test]
fn check_render_ellipse_hollow() {
    check_render_with(
        "ellipse with a 60% hole",
        &[50, 25, 15, 10],
        40,
        10,
        |c| {
            c.set_shape(DrawModeShape::Ellipse);
            c.set_hollow_thickness(HollowThickness::Percentage(60));
        },
        0xD03AFE9B7D00AD9,
    );
}

#[test]
fn check_render_ring_percentage() {
    check_render_with("ring, 50% hole", &[50, 25, 15, 10], 40, 12, |c| c.set_hollow_thickness(HollowThickness::Percentage(50)), 0x18AF6D29C9E69A6D);
}

#[test]
fn check_render_ring_absolute() {
    check_render_with("ring, hole 8 chars wide", &[50, 25, 15, 10], 40, 12, |c| c.set_hollow_thickness(HollowThickness::Absolute(8)), 0x4393BEBA9778CE2B);
}

#[test]
fn check_render_ring_rectangle() {
    check_render_with("circle, 12x3 hole", &[50, 25, 15, 10], 40, 12, |c| c.set_hollow_thickness(HollowThickness::Rectangle(12, 3)), 0xA0E4A3333B896A96);
}

#[test]
fn check_render_hole_fills_shape() {
    // a 100% hole leaves nothing: same screen as an empty chart of the same size
    check_render_with("100% hole: background only", &[5, 5], 20, 8, |c| c.set_hollow_thickness(HollowThickness::Percentage(100)), 0x77243921445A6CC9);
}

#[test]
fn check_render_hole_zero() {
    // a 0% hole is a solid circle: same screen as `check_render_single_slice`
    check_render_with("0% hole: solid", &[5], 20, 8, |c| c.set_hollow_thickness(HollowThickness::Percentage(0)), 0x72D4D01D6E181B31);
}

// ---------------------------------------------------------------------------
// padding: one character is always left empty on every side
// ---------------------------------------------------------------------------
use super::draw_mode::PADDING;

// the shapes that can be drawn
const SHAPES: [DrawModeShape; 8] = [
    DrawModeShape::Circle,
    DrawModeShape::Ellipse,
    DrawModeShape::Square,
    DrawModeShape::Rectangle,
    DrawModeShape::RoundSquare,
    DrawModeShape::RoundRectangle,
    DrawModeShape::Triangle,
    DrawModeShape::RoundTriangle,
];

fn paint_on_surface(mode: DrawMode, values: &[u64], w: u32, h: u32) -> Surface {
    let mut slices: Vec<Slice> = values.iter().map(|v| Slice::new(*v, "", None)).collect();
    update_proportions(&mut slices);
    let mut surface = Surface::new(w, h);
    surface.clear(Character::new('#', Color::White, Color::Black, CharFlags::None));
    let area = plot_area(surface.size());
    mode.paint(&mut surface, &Theme::new(Themes::Default), &slices, &mut PaletteTile::new(1, 1), &mut PaletteTile::new(w, h), area);
    surface
}

fn painted(surface: &Surface, x: i32, y: i32) -> bool {
    surface.char(x, y).unwrap().code != '#'
}

#[test]
fn check_padding() {
    assert_eq!(PADDING, 1);
    let holes = [HollowThickness::None, HollowThickness::Percentage(40), HollowThickness::Absolute(2), HollowThickness::Rectangle(3, 1)];
    let renderers = [DrawModeRenderer::HalfBlocks, DrawModeRenderer::Braille, DrawModeRenderer::Custom('*')];
    for shape in SHAPES {
        for (hole, renderer) in holes.iter().flat_map(|h| renderers.iter().map(|r| (*h, *r))) {
            for (w, h) in [(12, 6), (13, 7), (30, 5), (6, 14), (3, 3)] {
                let mode = DrawMode::new(shape, renderer, hole);
                let s = paint_on_surface(mode, &[3, 2, 1], w, h);
                let (w, h) = (w as i32, h as i32);
                for x in 0..w {
                    assert!(!painted(&s, x, 0), "{shape:?} {hole:?} {renderer:?} {w}x{h}: top row painted at x={x}");
                    assert!(!painted(&s, x, h - 1), "{shape:?} {hole:?} {w}x{h}: bottom row painted at x={x}");
                }
                for y in 0..h {
                    assert!(!painted(&s, 0, y), "{shape:?} {hole:?} {w}x{h}: left column painted at y={y}");
                    assert!(!painted(&s, w - 1, y), "{shape:?} {hole:?} {w}x{h}: right column painted at y={y}");
                }
                // without a hole, something is drawn inside (a hole may cover a very small shape,
                // a pixel 2 characters wide does not fit in a shape 1 character wide, and a triangle
                // in a single character is smaller than a pixel)
                let too_small = (w <= 3) && (matches!(renderer, DrawModeRenderer::Custom(_)) || matches!(shape, DrawModeShape::Triangle | DrawModeShape::RoundTriangle));
                if (hole == HollowThickness::None) && !too_small {
                    let inside = (1..w - 1).any(|x| (1..h - 1).any(|y| painted(&s, x, y)));
                    assert!(inside, "{shape:?} {w}x{h}: nothing drawn");
                }
            }
        }
    }
}

#[test]
fn check_padding_too_small() {
    // 2 characters or less in any direction: no room left, nothing is drawn
    for (w, h) in [(1, 1), (2, 10), (10, 2), (2, 2)] {
        let s = paint_on_surface(DrawMode::default(), &[1, 1], w, h);
        for x in 0..w as i32 {
            for y in 0..h as i32 {
                assert!(!painted(&s, x, y), "{w}x{h}: painted at ({x},{y})");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// renderers: braille and custom character
// ---------------------------------------------------------------------------

#[test]
fn check_render_braille() {
    check_render_with("braille, 4 slices, 24x8", &[50, 25, 15, 10], 24, 8, |c| c.set_renderer(DrawModeRenderer::Braille), 0xC3BEA35959A46531);
}

#[test]
fn check_render_braille_ring() {
    check_render_with(
        "braille ring, 50% hole",
        &[50, 25, 15, 10],
        24,
        8,
        |c| {
            c.set_renderer(DrawModeRenderer::Braille);
            c.set_hollow_thickness(HollowThickness::Percentage(50));
        },
        0x791737C2735DCA99,
    );
}

#[test]
fn check_render_custom_char() {
    check_render_with(
        "custom char, ellipse",
        &[50, 25, 15, 10],
        24,
        8,
        |c| {
            c.set_shape(DrawModeShape::Ellipse);
            c.set_renderer(DrawModeRenderer::Custom('o'));
        },
        0xA2DC847F45372289,
    );
}

#[test]
fn check_braille_majority_color() {
    // a cell split between two slices takes the color of the slice with most dots
    let mut slices: Vec<Slice> = [1u64, 1].iter().map(|v| Slice::new(*v, "", None)).collect();
    update_proportions(&mut slices);
    let mode = DrawMode::new(DrawModeShape::Circle, DrawModeRenderer::Braille, HollowThickness::None);
    let mut surface = Surface::new(12, 7);
    surface.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
    let area = plot_area(surface.size());
    mode.paint(&mut surface, &Theme::new(Themes::Default), &slices, &mut PaletteTile::new(1, 1), &mut PaletteTile::new(12, 7), area);
    for y in 1..6 {
        for x in 1..11 {
            let ch = surface.char(x, y).unwrap();
            if ch.code != ' ' {
                assert!(('\u{2801}'..='\u{28FF}').contains(&ch.code), "({x},{y}): {:?}", ch.code);
                let theme = Theme::new(Themes::Default);
                assert!(ch.foreground == slices[0].color(0, &theme) || ch.foreground == slices[1].color(1, &theme));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// paint map: which slice is drawn in each character
// ---------------------------------------------------------------------------
use super::selection;

const MAP_COLORS: [Color; 4] = [Color::Red, Color::Green, Color::Blue, Color::Yellow];

/// Paints 4 slices with distinct explicit colors and returns the surface and the paint map.
fn paint_with_map(mode: DrawMode, w: u32, h: u32) -> (Surface, PaletteTile) {
    let mut slices: Vec<Slice> = [40u64, 30, 20, 10]
        .iter()
        .zip(MAP_COLORS)
        .map(|(v, c)| Slice::new(*v, "", Some(CharAttribute::with_fore_color(c))))
        .collect();
    update_proportions(&mut slices);
    let mut surface = Surface::new(w, h);
    surface.clear(Character::new('#', Color::White, Color::Black, CharFlags::None));
    let mut map = PaletteTile::new(w, h);
    let area = plot_area(surface.size());
    mode.paint(&mut surface, &Theme::new(Themes::Default), &slices, &mut PaletteTile::new(1, 1), &mut map, area);
    (surface, map)
}

#[test]
fn check_paint_map() {
    let holes = [HollowThickness::None, HollowThickness::Percentage(50)];
    let renderers = [DrawModeRenderer::HalfBlocks, DrawModeRenderer::Braille, DrawModeRenderer::Custom('*')];
    for shape in SHAPES {
        for (hole, renderer) in holes.iter().flat_map(|h| renderers.iter().map(|r| (*h, *r))) {
            for (w, h) in [(20, 10), (13, 7), (40, 6)] {
                let mode = DrawMode::new(shape, renderer, hole);
                let (s, map) = paint_with_map(mode, w, h);
                let name = format!("{shape:?} {hole:?} {renderer:?} {w}x{h}");
                let mut mapped = 0;
                for y in 0..h as i32 {
                    for x in 0..w as i32 {
                        let ch = s.char(x, y).unwrap();
                        let index = map.get(x as u32, y as u32);
                        // a character is in the map if (and only if) it was drawn
                        assert_eq!(ch.code != '#', index.is_some(), "{name}: ({x},{y})");
                        if let Some(index) = index {
                            mapped += 1;
                            assert!((index as usize) < MAP_COLORS.len(), "{name}: ({x},{y}) -> {index}");
                            // the mapped slice is one of the colors drawn in that character
                            let color = MAP_COLORS[index as usize];
                            assert!(ch.foreground == color || ch.background == color, "{name}: ({x},{y}) -> {index}");
                            if renderer != DrawModeRenderer::HalfBlocks {
                                assert_eq!(ch.foreground, color, "{name}: ({x},{y})");
                            }
                        }
                    }
                }
                assert!(mapped > 0, "{name}: nothing mapped");
            }
        }
    }
}

#[test]
fn check_paint_map_padding() {
    // the padding is never mapped, and the map is reset on every paint
    let (_, mut map) = paint_with_map(DrawMode::default(), 12, 8);
    for x in 0..12 {
        assert_eq!(map.get(x, 0), None);
        assert_eq!(map.get(x, 7), None);
    }
    for y in 0..8 {
        assert_eq!(map.get(0, y), None);
        assert_eq!(map.get(11, y), None);
    }
    assert!(map.get(6, 4).is_some());
    map.resize(12, 8);
    assert_eq!(map.get(6, 4), None);
    // outside the map
    assert_eq!(map.get(12, 0), None);
    assert_eq!(map.get(0, 8), None);
}

#[test]
fn check_hover() {
    in_app(|| {
        let mut c = PieChart::new(layout!("x:0,y:0,w:22,h:12"), piechart::Flags::None);
        c.add_value(1, "A", None);
        c.add_value(1, "B", None);
        c.on_resize(Size::new(0, 0), Size::new(22, 12));
        assert_eq!(c.hovered_slice, None);
        // circle centered at (11, 6): right half is slice 0, left half is slice 1
        assert!(matches!(c.on_mouse_event(&MouseEvent::Over(Point::new(14, 6))), EventProcessStatus::Processed));
        assert_eq!(c.hovered_slice, Some(0));
        // same slice again: nothing changes
        assert!(matches!(c.on_mouse_event(&MouseEvent::Over(Point::new(15, 5))), EventProcessStatus::Ignored));
        assert_eq!(c.hovered_slice, Some(0));
        c.on_mouse_event(&MouseEvent::Over(Point::new(7, 6)));
        assert_eq!(c.hovered_slice, Some(1));
        // the padding and the corners are outside the chart
        c.on_mouse_event(&MouseEvent::Over(Point::new(0, 0)));
        assert_eq!(c.hovered_slice, None);
        c.on_mouse_event(&MouseEvent::Over(Point::new(2, 1)));
        assert_eq!(c.hovered_slice, None);
        // outside the control
        c.on_mouse_event(&MouseEvent::Over(Point::new(-1, 3)));
        assert_eq!(c.hovered_slice, None);
        c.on_mouse_event(&MouseEvent::Over(Point::new(30, 3)));
        assert_eq!(c.hovered_slice, None);
        // leaving the control clears the hovered slice
        c.on_mouse_event(&MouseEvent::Over(Point::new(14, 6)));
        assert_eq!(c.hovered_slice, Some(0));
        c.on_mouse_event(&MouseEvent::Leave);
        assert_eq!(c.hovered_slice, None);
        // a data change clears it too (indices may move)
        c.on_mouse_event(&MouseEvent::Over(Point::new(7, 6)));
        assert_eq!(c.hovered_slice, Some(1));
        c.add_value(2, "C", None);
        assert_eq!(c.hovered_slice, None);
        // after the change, the map follows the new layout: slice 2 is now the left half
        c.on_mouse_event(&MouseEvent::Over(Point::new(7, 6)));
        assert_eq!(c.hovered_slice, Some(2));
    });
}

// ---------------------------------------------------------------------------
// slice centers
// ---------------------------------------------------------------------------
use super::piechart::update_centers;

/// Builds a paint map from rows of text: a digit is the slice index, anything else is empty.
fn map_of(rows: &[&str]) -> PaletteTile {
    let mut m = PaletteTile::new(rows[0].len() as u32, rows.len() as u32);
    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if let Some(d) = ch.to_digit(10) {
                m.set(x as u32, y as u32, d as u16);
            }
        }
    }
    m
}

fn slices_n(n: usize) -> Vec<Slice> {
    (0..n).map(|_| Slice::new(1, "", None)).collect()
}

#[test]
fn check_center_is_the_average() {
    let map = map_of(&[
        "........", //
        ".000....", //
        ".000.11.", //
        ".000.11.", //
        "........",
    ]);
    let mut slices = slices_n(3);
    update_centers(&mut slices, &map);
    assert_eq!(slices[0].center, Some(Point::new(2, 2)));
    // 4 cells: average (5.5, 2.5) rounds to (6, 3)
    assert_eq!(slices[1].center, Some(Point::new(6, 3)));
    // no cells: no center
    assert_eq!(slices[2].center, None);
    assert_eq!(slices[0].cells, 9);
}

#[test]
fn check_center_falls_back_to_a_cell_of_the_slice() {
    // slice 0 wraps around an empty middle: its average (4, 2) is not one of its cells
    let map = map_of(&[
        "000000000", //
        "0.......0", //
        "0.......0", //
        "0.......0", //
        "000000000",
    ]);
    let mut slices = slices_n(1);
    update_centers(&mut slices, &map);
    let c = slices[0].center.unwrap();
    assert_eq!(map.get(c.x as u32, c.y as u32), Some(0));
    // nearest cell to (4, 2), a cell being twice as tall as wide: straight up or down (distance 4)
    // beats straight left or right (distance 4 too, but found later); the first one found wins
    assert_eq!(c, Point::new(4, 0));
}

#[test]
fn check_centers_are_recomputed() {
    let mut slices = slices_n(2);
    update_centers(&mut slices, &map_of(&["00.11"]));
    assert_eq!(slices[0].center, Some(Point::new(1, 0)));
    assert_eq!(slices[1].center, Some(Point::new(4, 0)));
    // a later paint where slice 1 is no longer drawn
    update_centers(&mut slices, &map_of(&["000.."]));
    assert_eq!(slices[0].center, Some(Point::new(1, 0)));
    assert_eq!(slices[1].center, None);
    assert_eq!(slices[1].cells, 0);
}

#[test]
fn check_centers_on_real_charts() {
    // on real drawings, every visible slice has a center, and it is one of its own cells
    let value_sets: [&[u64]; 5] = [&[50, 25, 15, 10], &[75, 25], &[90, 5, 5], &[1, 1, 1, 1, 1, 1, 1], &[1000, 1, 1000]];
    let holes = [HollowThickness::None, HollowThickness::Percentage(50), HollowThickness::Percentage(80)];
    let renderers = [DrawModeRenderer::HalfBlocks, DrawModeRenderer::Braille, DrawModeRenderer::Custom('*')];
    for values in value_sets {
        for shape in SHAPES {
            for (hole, renderer) in holes.iter().flat_map(|h| renderers.iter().map(|r| (*h, *r))) {
                for (w, h) in [(30, 13), (21, 8), (60, 20)] {
                    let mut slices: Vec<Slice> = values.iter().map(|v| Slice::new(*v, "", None)).collect();
                    update_proportions(&mut slices);
                                    let mut surface = Surface::new(w, h);
                    let mut map = PaletteTile::new(w, h);
                    let area = plot_area(surface.size());
                    DrawMode::new(shape, renderer, hole).paint(&mut surface, &Theme::new(Themes::Default), &slices, &mut PaletteTile::new(1, 1), &mut map, area);
                    update_centers(&mut slices, &map);
                    let name = format!("{values:?} {shape:?} {hole:?} {renderer:?} {w}x{h}");
                    for (i, s) in slices.iter().enumerate() {
                        match s.center {
                            Some(c) => assert_eq!(map.get(c.x as u32, c.y as u32), Some(i as u16), "{name}: slice {i}"),
                            None => assert_eq!(s.cells, 0, "{name}: slice {i} has cells but no center"),
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn check_center_of_a_big_ring_slice() {
    // a 75% slice of a ring: its average is in the hole, so the center moves onto the ring
    in_app(|| {
        let mut c = PieChart::new(layout!("x:0,y:0,w:32,h:15"), piechart::Flags::None);
        c.add_value(75, "A", None);
        c.add_value(25, "B", None);
        c.set_hollow_thickness(HollowThickness::Percentage(50));
        c.on_resize(Size::new(0, 0), Size::new(32, 15));
        for (i, s) in c.slices.iter().enumerate() {
            let center = s.center.unwrap();
            assert_eq!(c.slice_index_at(center.x, center.y), Some(i));
        }
        // the hole is around the middle of the control; the center of A is not there
        let a = c.slices[0].center.unwrap();
        assert!((a.x - 16).abs() > 3 || (a.y - 7).abs() > 2, "{a:?}");
    });
}

#[test]
fn check_tooltip_stays_on_the_slice_center() {
    // hovering two different cells of the same slice shows the tooltip in the same place
    let script = "
        Paint.Enable(false)
        Mouse.Move(23,5)
        Paint('tooltip from one cell of slice A')
        CheckHash(0x583A561B6F4B4CFC)
        Mouse.Move(26,9)
        Paint('tooltip from another cell of slice A')
        CheckHash(0x583A561B6F4B4CFC)
    ";
    App::new()
        .size(Size::new(44, 18))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            let mut c = PieChart::new(layout!("x:1,y:0,w:40,h:15"), piechart::Flags::None);
            c.add_value(50, "Slice A", None);
            c.add_value(50, "Slice B", None);
            w.add(c);
            w
        })
        .run()
        .unwrap();
}

// ---------------------------------------------------------------------------
// selection border
// ---------------------------------------------------------------------------
const SELECTION_CHARS: &str = "─│┌┐└┘├┤┬┴┼";

/// Line ends of a selection character: (up, right, down, left).
fn line_ends(c: char) -> (bool, bool, bool, bool) {
    match c {
        '─' => (false, true, false, true),
        '│' => (true, false, true, false),
        '┌' => (false, true, true, false),
        '┐' => (false, false, true, true),
        '└' => (true, true, false, false),
        '┘' => (true, false, false, true),
        '├' => (true, true, true, false),
        '┤' => (true, false, true, true),
        '┬' => (false, true, true, true),
        '┴' => (true, true, false, true),
        _ => (true, true, true, true),
    }
}

#[test]
fn check_selection_border() {
    let value_sets: [&[u64]; 5] = [&[50, 25, 15, 10], &[75, 25], &[90, 5, 5], &[1, 1, 1, 1, 1, 1, 1, 1], &[1000, 1, 1000]];
    let holes = [HollowThickness::None, HollowThickness::Percentage(50)];
    let renderers = [DrawModeRenderer::HalfBlocks, DrawModeRenderer::Braille];
    let mut borders = 0;
    for values in value_sets {
        for shape in SHAPES {
            for (hole, renderer) in holes.iter().flat_map(|h| renderers.iter().map(|r| (*h, *r))) {
                for (w, h) in [(34u32, 15u32), (21, 8), (60, 20)] {
                    let mut slices: Vec<Slice> = values.iter().map(|v| Slice::new(*v, "", None)).collect();
                    update_proportions(&mut slices);
                                    let mut surface = Surface::new(w, h);
                    let mut map = PaletteTile::new(w, h);
                    let area = plot_area(surface.size());
                    DrawMode::new(shape, renderer, hole).paint(&mut surface, &Theme::new(Themes::Default), &slices, &mut PaletteTile::new(1, 1), &mut map, area);
                    for sel in 0..values.len() {
                        let name = format!("{values:?} {shape:?} {hole:?} {renderer:?} {w}x{h} selected {sel}");
                        let mut s = Surface::new(w, h);
                        s.clear(Character::new('#', Color::White, Color::Black, CharFlags::None));
                        let before: Vec<Option<u16>> = (0..h).flat_map(|y| (0..w).map(move |x| (x, y))).map(|(x, y)| map.get(x, y)).collect();
                        selection::paint_selection(&map, sel, &mut s, CharAttribute::with_fore_color(Color::Yellow));
                        let after: Vec<Option<u16>> = (0..h).flat_map(|y| (0..w).map(move |x| (x, y))).map(|(x, y)| map.get(x, y)).collect();
                        assert_eq!(before, after, "{name}: the paint map must not change");
                        // the slice and its notches: cells with the slice (or a notch) on 3 or 4 sides, repeated until nothing changes
                        let mask = selected_mask(&map, sel as u16, w, h);
                        let selected = |x: i32, y: i32| x >= 0 && y >= 0 && x < w as i32 && y < h as i32 && mask[(y as u32 * w + x as u32) as usize];
                        let border = |x: i32, y: i32| s.char(x, y).is_some_and(|c| c.code != '#');
                        for y in 0..h as i32 {
                            for x in 0..w as i32 {
                                let touches = (-1..=1).any(|dy| (-1..=1).any(|dx| (dx, dy) != (0, 0) && selected(x + dx, y + dy)));
                                let ch = s.char(x, y).unwrap().code;
                                if selected(x, y) {
                                    assert_eq!(ch, '#', "{name}: border drawn over the selected slice at ({x},{y})");
                                } else if touches {
                                    // every cell touching the slice is part of the border...
                                    assert!(SELECTION_CHARS.contains(ch), "{name}: ({x},{y}) = {ch:?}");
                                    borders += 1;
                                    // ...and the line is continuous: each line end meets a cell of the border
                                    let (up, right, down, left) = line_ends(ch);
                                    assert!(!up || border(x, y - 1), "{name}: ({x},{y}) {ch} up");
                                    assert!(!right || border(x + 1, y), "{name}: ({x},{y}) {ch} right");
                                    assert!(!down || border(x, y + 1), "{name}: ({x},{y}) {ch} down");
                                    assert!(!left || border(x - 1, y), "{name}: ({x},{y}) {ch} left");
                                } else {
                                    assert_eq!(ch, '#', "{name}: ({x},{y}) is not next to the slice");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(borders > 10_000);
}

// the cells of slice `index`, plus every cell with 3 or 4 of these on its sides, repeated until nothing changes
fn selected_mask(map: &PaletteTile, index: u16, w: u32, h: u32) -> Vec<bool> {
    let mut mask: Vec<bool> = (0..h).flat_map(|y| (0..w).map(move |x| (x, y))).map(|(x, y)| map.get(x, y) == Some(index)).collect();
    loop {
        let mut changed = false;
        for y in 0..h as i32 {
            for x in 0..w as i32 {
                let at = |x: i32, y: i32| x >= 0 && y >= 0 && x < w as i32 && y < h as i32 && mask[(y as u32 * w + x as u32) as usize];
                if at(x, y) {
                    continue;
                }
                if [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().filter(|(dx, dy)| at(x + dx, y + dy)).count() >= 3 {
                    mask[(y as u32 * w + x as u32) as usize] = true;
                    changed = true;
                }
            }
        }
        if !changed {
            return mask;
        }
    }
}

#[test]
fn check_selection_border_edge_cases() {
    // a slice touching the edge of the map: the border stops at the edge, no panic
    let mut map = PaletteTile::new(4, 3);
    map.set(0, 0, 0);
    map.set(1, 0, 0);
    let mut s = Surface::new(4, 3);
    s.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
    selection::paint_selection(&map, 0, &mut s, CharAttribute::default());
    let row = |y: i32| (0..4).map(|x| s.char(x, y).unwrap().code).collect::<String>();
    assert_eq!(row(0), "  │ ");
    assert_eq!(row(1), "──┘ ");
    assert_eq!(row(2), "    ");
    // a slice that is not drawn, and an invalid index: nothing is painted
    for index in [3usize, 0xFFFF, 100_000] {
        let mut s = Surface::new(4, 3);
        s.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
        selection::paint_selection(&map, index, &mut s, CharAttribute::default());
        assert!((0..3).all(|y| row_is_blank(&s, y, 4)), "index {index}");
    }
}

fn row_is_blank(s: &Surface, y: i32, w: i32) -> bool {
    (0..w).all(|x| s.char(x, y).unwrap().code == ' ')
}

#[test]
fn check_selection_on_click() {
    let script = "
        Paint.Enable(false)
        Paint('nothing selected')
        CheckHash(0xDCE4A95EE1BA5A62)
        Mouse.Click(23,10,left)
        Paint('slice A (right half) selected: the other slices are dimmed')
        CheckHash(0x9E0559F0415963FC)
    ";
    App::new()
        .size(Size::new(44, 18))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            let mut c = PieChart::new(layout!("x:1,y:0,w:40,h:15"), piechart::Flags::DimSlicesOnSelection);
            c.add_value(50, "A", None);
            c.add_value(25, "B", None);
            c.add_value(25, "C", None);
            w.add(c);
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_selected_slice_api() {
    in_app(|| {
        let mut c = PieChart::new(layout!("x:0,y:0,w:30,h:10"), piechart::Flags::None);
        c.add_value(50, "A", None);
        c.add_value(25, "B", None);
        c.add_value(0, "empty", None);
        c.add_value(25, "C", None);
        assert_eq!(c.selected_slice(), None);
        c.set_selected_slice(1);
        assert_eq!(c.selected_slice(), Some(1));
        c.set_selected_slice(3);
        assert_eq!(c.selected_slice(), Some(3));
        // out of range, or a slice that is not drawn: nothing changes
        c.set_selected_slice(4);
        c.set_selected_slice(100);
        c.set_selected_slice(2);
        assert_eq!(c.selected_slice(), Some(3));
        c.clear_selected_slice();
        assert_eq!(c.selected_slice(), None);
        c.clear_selected_slice();
        assert_eq!(c.selected_slice(), None);
        // a slice whose value becomes 0 is not drawn any more: the selection is cleared
        c.set_selected_slice(1);
        c.set_value(0, 70);
        assert_eq!(c.selected_slice(), Some(1));
        c.set_value(1, 0);
        assert_eq!(c.selected_slice(), None);
        // the empty slice gets a value: it can be selected
        c.set_value(2, 10);
        c.set_selected_slice(2);
        assert_eq!(c.selected_slice(), Some(2));
        // removing all slices clears the selection; new slices are not selected
        c.clear();
        assert_eq!(c.selected_slice(), None);
        c.add_value(10, "X", None);
        c.add_value(10, "Y", None);
        c.add_value(10, "Z", None);
        assert_eq!(c.selected_slice(), None);
    });
}

#[test]
fn check_set_selected_slice_paints_like_a_click() {
    // the same chart as in `check_selection_on_click`, with slice A selected from code
    let script = "
        Paint.Enable(false)
        Paint('slice A (right half) selected: the other slices are dimmed')
        CheckHash(0x9E0559F0415963FC)
    ";
    App::new()
        .size(Size::new(44, 18))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            let mut c = PieChart::new(layout!("x:1,y:0,w:40,h:15"), piechart::Flags::DimSlicesOnSelection);
            c.add_value(50, "A", None);
            c.add_value(25, "B", None);
            c.add_value(25, "C", None);
            c.set_selected_slice(0);
            w.add(c);
            w
        })
        .run()
        .unwrap();
}

// ---------------------------------------------------------------------------
// labels
// ---------------------------------------------------------------------------
use super::labels::{self, fit, side_of, Place, Side};
use super::{LabelContent, LabelLayout};

fn labelled(values: &[(u64, &str)], content: LabelContent) -> Vec<Slice> {
    let mut slices: Vec<Slice> = values.iter().map(|(v, l)| Slice::new(*v, l, None)).collect();
    update_proportions(&mut slices);
    labels::update_texts(&mut slices, content, &FormatNumber::new(10).group(3, b','));
    slices
}

#[test]
fn check_label_texts() {
    let texts = |content| labelled(&[(45, "Rust"), (7, ""), (1234567, "Big")], content).iter().map(|s| s.text.clone()).collect::<Vec<_>>();
    assert_eq!(texts(LabelContent::Label), vec!["Rust", "", "Big"]);
    assert_eq!(texts(LabelContent::Value), vec!["45", "7", "1,234,567"]);
    assert_eq!(texts(LabelContent::LabelAndValue), vec!["Rust: 45", "7", "Big: 1,234,567"]);
}

#[test]
fn check_percentage_texts() {
    let texts = |values: &[(u64, &str)], content| labelled(values, content).iter().map(|s| s.text.clone()).collect::<Vec<_>>();
    let values = [(50, "Rust"), (25, ""), (25, "Go"), (0, "Zig")];
    assert_eq!(texts(&values, LabelContent::Percentage), vec!["50.00%", "25.00%", "25.00%", "0.00%"]);
    assert_eq!(texts(&values, LabelContent::LabelAndPercentage), vec!["Rust: 50.00%", "25.00%", "Go: 25.00%", "Zig: 0.00%"]);
    // always two decimals, and the percentages add up to exactly 100.00%
    assert_eq!(texts(&[(1, "a"), (1, "b"), (1, "c")], LabelContent::Percentage), vec!["33.34%", "33.33%", "33.33%"]);
    assert_eq!(texts(&[(1, ""), (2, ""), (5, "")], LabelContent::Percentage), vec!["12.50%", "25.00%", "62.50%"]);
    assert_eq!(texts(&[(1, ""), (99_999, "")], LabelContent::Percentage), vec!["0.00%", "100.00%"]);
    assert_eq!(texts(&[(3, ""), (99_997, "")], LabelContent::Percentage), vec!["0.00%", "100.00%"]);
    assert_eq!(texts(&[(6, ""), (99_994, "")], LabelContent::Percentage), vec!["0.01%", "99.99%"]);
    // the number format of the values is not used for percentages
    let mut slices: Vec<Slice> = [(1500u64, "Used"), (500, "Free")].iter().map(|(v, l)| Slice::new(*v, l, None)).collect();
    update_proportions(&mut slices);
    labels::update_texts(&mut slices, LabelContent::LabelAndPercentage, &FormatNumber::new(10).suffix(" MB"));
    assert_eq!(slices.iter().map(|s| s.text.as_str()).collect::<Vec<_>>(), vec!["Used: 75.00%", "Free: 25.00%"]);
    // the tooltip still shows the value
    assert_eq!(slices[0].tooltip, "Used: 1500 MB");
}

#[test]
fn check_labels_outside_percentage() {
    let script = "
        Paint.Enable(false)
        Paint('outside labels with percentages')
        CheckHash(0xD4540A40CAA2C4D0)
    ";
    App::new()
        .size(Size::new(64, 22))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            let mut c = PieChart::new(layout!("x:1,y:0,w:60,h:19"), piechart::Flags::None);
            for (v, l) in [(45, "Rust"), (25, "C++"), (15, "Go"), (10, "Zig"), (5, "")] {
                c.add_value(v, l, None);
            }
            c.set_label_layout(LabelLayout::Outside);
            c.set_label_content(LabelContent::LabelAndPercentage);
            w.add(c);
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_tooltip_texts() {
    // the tooltip always shows the value, whatever the label content is
    for content in [LabelContent::Label, LabelContent::Value, LabelContent::LabelAndValue] {
        let tooltips = labelled(&[(45, "Rust"), (7, ""), (1234567, "Big")], content).iter().map(|s| s.tooltip.clone()).collect::<Vec<_>>();
        assert_eq!(tooltips, vec!["Rust: 45", "7", "Big: 1,234,567"]);
    }
}

#[test]
fn check_label_sides() {
    // the middle of each slice decides the side: 12 o'clock is top, 3 is right, 6 is bottom, 9 is left
    let slices = labelled(&[(1, "a"); 8], LabelContent::Label);
    let sides: Vec<Side> = slices.iter().map(|s| side_of(s, &DrawMode::default())).collect();
    // 8 equal slices: middles at 22.5, 67.5, 112.5, 157.5, 202.5, 247.5, 292.5, 337.5 degrees
    use Side::*;
    assert_eq!(sides, vec![Top, Right, Right, Bottom, Bottom, Left, Left, Top]);
    let slices = labelled(&[(1, "a"), (3, "b")], LabelContent::Label);
    // 0..90 -> middle 45 -> right; 90..360 -> middle 225 -> left
    assert_eq!(slices.iter().map(|s| side_of(s, &DrawMode::default())).collect::<Vec<_>>(), vec![Right, Left]);
}

#[test]
fn check_outside_area() {
    let size = Size::new(50, 18);
    let inner = plot_area(size).unwrap();
    // labels on all four sides (4 equal slices: middles at 45, 135, 225 and 315 degrees ->
    // right, bottom, left, top): the same room left and right, and top and bottom
    let slices = labelled(&[(1, "Right side"), (1, "Bot"), (1, "L"), (1, "Top")], LabelContent::Label);
    assert_eq!(slices.iter().map(|s| side_of(s, &DrawMode::default())).collect::<Vec<_>>(), vec![Side::Right, Side::Bottom, Side::Left, Side::Top]);
    let (area, ok) = labels::outside_area(size, &slices, &DrawMode::default());
    let area = area.unwrap();
    assert!(ok);
    assert_eq!(area.left() - inner.left(), inner.right() - area.right());
    assert_eq!(area.top() - inner.top(), inner.bottom() - area.bottom());
    assert_eq!(area.left() - inner.left(), "Right side".len() as i32 + 2);
    assert_eq!(area.top() - inner.top(), 2);
    // no labels at all: the whole plot area
    let slices = labelled(&[(1, ""), (1, "")], LabelContent::Label);
    assert_eq!(labels::outside_area(size, &slices, &DrawMode::default()), (Some(inner), true));
    // no room: the plot area, and no outside labels
    let slices = labelled(&[(1, "Top"), (1, "Right side"), (1, "Bot"), (1, "Left side")], LabelContent::Label);
    let (area, ok) = labels::outside_area(Size::new(14, 6), &slices, &DrawMode::default());
    assert_eq!(area, plot_area(Size::new(14, 6)));
    assert!(!ok);
}

#[test]
fn check_fit() {
    let slices = labelled(&[(10, "a"), (20, "b"), (30, "c"), (1, "d")], LabelContent::Label);
    let place = |index, pos, len| Place { index, pos, len, anchor: Point::new(0, 0) };
    // overlapping items are pushed apart, keeping their order and a gap
    let mut items = vec![place(0, 5, 3), place(1, 6, 3), place(2, 6, 2)];
    fit(&mut items, &slices, 0, 20, 1);
    assert_eq!(items.iter().map(|p| (p.index, p.pos)).collect::<Vec<_>>(), vec![(0, 5), (1, 9), (2, 13)]);
    // pushed back inside the range when they would end past it
    let mut items = vec![place(0, 18, 3), place(1, 19, 3)];
    fit(&mut items, &slices, 0, 20, 1);
    assert_eq!(items.iter().map(|p| (p.index, p.pos)).collect::<Vec<_>>(), vec![(0, 14), (1, 18)]);
    // rows (length 1, no gap): 4 items, 3 rows -> the smallest slice (d) is dropped
    let mut items = vec![place(0, 1, 1), place(1, 1, 1), place(2, 1, 1), place(3, 1, 1)];
    fit(&mut items, &slices, 0, 2, 0);
    assert_eq!(items.iter().map(|p| (p.index, p.pos)).collect::<Vec<_>>(), vec![(0, 0), (1, 1), (2, 2)]);
}

#[test]
fn check_labels_keep_the_chart_and_padding() {
    // labels never draw over the chart or the padding
    let value_sets: [&[(u64, &str)]; 4] = [
        &[(45, "Rust"), (25, "C++"), (15, "Go"), (10, "Zig"), (5, "Other")],
        &[(1, "Jan"), (1, "Feb"), (1, "Mar"), (1, "Apr"), (1, "May"), (1, "Jun"), (1, "Jul"), (1, "Aug"), (1, "Sep"), (1, "Oct"), (1, "Nov"), (1, "Dec")],
        &[(97, "A very long label for a big slice"), (1, "a"), (1, "b"), (1, "c")],
        &[(1, "only")],
    ];
    let mut checked = 0;
    for values in value_sets {
        for content in [LabelContent::Label, LabelContent::Value, LabelContent::LabelAndValue] {
            for shape in SHAPES {
                for hole in [HollowThickness::None, HollowThickness::Percentage(50)] {
                    for (w, h) in [(50u32, 18u32), (30, 10), (80, 24), (14, 6)] {
                        let mut slices = labelled(values, content);
                        let size = Size::new(w, h);
                        let mut surface = Surface::new(w, h);
                        surface.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
                        let mut map = PaletteTile::new(size.width, size.height);
                        let (area, ok) = labels::outside_area(size, &slices, &DrawMode::default());
                        DrawMode::new(shape, DrawModeRenderer::HalfBlocks, hole).paint(&mut surface, &Theme::new(Themes::Default), &slices, &mut PaletteTile::new(1, 1), &mut map, area);
                        update_centers(&mut slices, &map);
                        let before: Vec<Character> = (0..h as i32).flat_map(|y| (0..w as i32).map(move |x| (x, y))).map(|(x, y)| *surface.char(x, y).unwrap()).collect();
                        if let (Some(area), true) = (area, ok) {
                            labels::paint_outside(&slices, &mut surface, &map, area, &DrawMode::default(), &Theme::new(Themes::Default), &mut Vec::new());
                        }
                        let name = format!("{values:?} {content:?} {shape:?} {hole:?} {w}x{h}");
                        for y in 0..h as i32 {
                            for x in 0..w as i32 {
                                let ch = *surface.char(x, y).unwrap();
                                let old = before[(y as u32 * w + x as u32) as usize];
                                if map.get(x as u32, y as u32).is_some() || x == 0 || y == 0 || x == w as i32 - 1 || y == h as i32 - 1 {
                                    assert_eq!(ch, old, "{name}: ({x},{y}) changed");
                                }
                            }
                        }
                        checked += 1;
                    }
                }
            }
        }
    }
    assert_eq!(checked, 4 * 3 * SHAPES.len() * 2 * 4);
}

#[test]
fn check_inside_label_is_centered_on_the_slice() {
    in_app(|| {
        let mut c = PieChart::new(layout!("x:0,y:0,w:40,h:16"), piechart::Flags::None);
        c.add_value(50, "Half", None);
        c.add_value(50, "Other", None);
        c.set_label_layout(LabelLayout::Inside);
        c.on_resize(Size::new(0, 0), Size::new(40, 16));
        for s in c.slices.iter() {
            let center = s.center.unwrap();
            let len = s.text.chars().count() as i32;
            let x = center.x - len / 2;
            let row: String = (0..len).map(|i| c.surface.char(x + i, center.y).unwrap().code).collect();
            assert_eq!(row, s.text);
        }
    });
}

fn check_label_screen(name: &str, values: &'static [(u64, &'static str)], w: u32, h: u32, setup: fn(&mut PieChart), hash: u64) {
    let script = format!(
        "
        Paint.Enable(false)
        Paint('{name}')
        CheckHash(0x{hash:X})
    "
    );
    App::new()
        .size(Size::new(w + 4, h + 2))
        .debug_script(&script)
        .window(move || {
            let mut win = window!("Test,d:f");
            let mut c = PieChart::new(LayoutBuilder::new().x(1).y(0).width(w).height(h).build(), piechart::Flags::None);
            for (v, l) in values {
                c.add_value(*v, l, None);
            }
            setup(&mut c);
            win.add(c);
            win
        })
        .run()
        .unwrap();
}

const LANGS: &[(u64, &str)] = &[(45, "Rust"), (25, "C++"), (15, "Go"), (10, "Zig"), (5, "Other")];

#[test]
fn check_labels_none() {
    check_label_screen("no labels (default)", LANGS, 44, 16, |_| {}, 0xC59F6401E4610271);
}

#[test]
fn check_labels_inside() {
    check_label_screen("inside, label", LANGS, 44, 16, |c| c.set_label_layout(LabelLayout::Inside), 0x374C569C9FBC5D46);
}

#[test]
fn check_labels_inside_label_and_value() {
    check_label_screen(
        "inside, label and value",
        LANGS,
        44,
        16,
        |c| {
            c.set_label_layout(LabelLayout::Inside);
            c.set_label_content(LabelContent::LabelAndValue);
        },
        0x3B79A6AC6BE64240,
    );
}

#[test]
fn check_labels_outside() {
    check_label_screen("outside, label", LANGS, 50, 18, |c| c.set_label_layout(LabelLayout::Outside), 0xEC2F6073F0451F96);
}

#[test]
fn check_labels_outside_many() {
    const MONTHS: &[(u64, &str)] = &[(1, "Jan"), (1, "Feb"), (1, "Mar"), (1, "Apr"), (1, "May"), (1, "Jun"), (1, "Jul"), (1, "Aug"), (1, "Sep"), (1, "Oct"), (1, "Nov"), (1, "Dec")];
    check_label_screen("outside, 12 slices", MONTHS, 50, 18, |c| c.set_label_layout(LabelLayout::Outside), 0x83D17A4DAC29CA0F);
}

#[test]
fn check_labels_outside_ring_values() {
    check_label_screen(
        "outside ring, value",
        LANGS,
        50,
        18,
        |c| {
            c.set_label_layout(LabelLayout::Outside);
            c.set_label_content(LabelContent::Value);
            c.set_hollow_thickness(HollowThickness::Percentage(50));
        },
        0xD4DE3DF7949730DD,
    );
}

#[test]
fn check_labels_outside_ellipse() {
    check_label_screen(
        "outside, ellipse",
        LANGS,
        56,
        14,
        |c| {
            c.set_label_layout(LabelLayout::Outside);
            c.set_shape(DrawModeShape::Ellipse);
        },
        0xAD51D27F536C2FB2,
    );
}

#[test]
fn check_labels_outside_no_room() {
    // not enough room: the chart is drawn without labels (same as no labels at all)
    check_label_screen("outside, too small", LANGS, 14, 6, |c| c.set_label_layout(LabelLayout::Outside), 0x26488A12E197FB66);
}

#[test]
fn check_number_format() {
    let texts = |values: &[(u64, &str)], format: FormatNumber| {
        let mut slices: Vec<Slice> = values.iter().map(|(v, l)| Slice::new(*v, l, None)).collect();
        labels::update_texts(&mut slices, LabelContent::LabelAndValue, &format);
        slices.iter().map(|s| s.text.clone()).collect::<Vec<_>>()
    };
    let values = [(1500u64, "Disk"), (0, "Empty"), (7, "")];
    assert_eq!(texts(&values, FormatNumber::new(10)), vec!["Disk: 1500", "Empty: 0", "7"]);
    assert_eq!(texts(&values, FormatNumber::new(10).group(3, b',').suffix(" MB")), vec!["Disk: 1,500 MB", "Empty: 0 MB", "7 MB"]);
    assert_eq!(texts(&values, FormatNumber::new(10).decimals(2)), vec!["Disk: 1500.00", "Empty: 0.00", "7.00"]);
    assert_eq!(texts(&values, FormatNumber::new(16).prefix("0x")), vec!["Disk: 0x5DC", "Empty: 0x0", "0x7"]);
    // exact for the largest values (no rounding through f64)
    assert_eq!(texts(&[(u64::MAX, "max")], FormatNumber::new(10).group(3, b',')), vec!["max: 18,446,744,073,709,551,615"]);
}

#[test]
fn check_set_number_format() {
    in_app(|| {
        let mut c = PieChart::new(layout!("x:0,y:0,w:40,h:16"), piechart::Flags::None);
        c.add_value(1500, "Disk", None);
        c.add_value(2500, "Free", None);
        c.set_label_content(LabelContent::Value);
        // default: digits grouped by thousands
        assert_eq!(c.slices[0].text, "1,500");
        c.set_number_format(FormatNumber::new(10).suffix(" MB"));
        assert_eq!(c.slices[0].text, "1500 MB");
        assert_eq!(c.slices[1].text, "2500 MB");
        // the format is kept when data or content changes
        c.set_value(0, 42);
        assert_eq!(c.slices[0].text, "42 MB");
        c.set_label_content(LabelContent::LabelAndValue);
        assert_eq!(c.slices[1].text, "Free: 2500 MB");
    });
}

// ---------------------------------------------------------------------------
// rectangle and square
// ---------------------------------------------------------------------------
use super::shapes::Rectangle;

fn rectangle(cx: f64, cy: f64, rx: f64, ry: f64, hollow: HollowThickness) -> Rectangle {
    Rectangle::new(cx, cy, rx, ry, hollow, false)
}

#[test]
fn check_rectangle_fraction() {
    // 20 x 10 physical units, centered at (10, 5)
    let r = rectangle(10.0, 5.0, 10.0, 5.0, HollowThickness::None);
    let close = |a: Option<f64>, b: f64| (a.unwrap() - b).abs() < 1e-9;
    // the fraction only depends on the direction from the center
    assert!(close(r.fraction(10.0, 0.0), 0.0)); // 12 o'clock
    assert!(close(r.fraction(15.0, 2.5), 0.125)); // towards the top-right corner
    assert!(close(r.fraction(19.0, 5.0), 0.25)); // 3 o'clock
    assert!(close(r.fraction(15.0, 7.5), 0.375)); // towards the bottom-right corner
    assert!(close(r.fraction(10.0, 9.0), 0.5)); // 6 o'clock
    assert!(close(r.fraction(5.0, 7.5), 0.625)); // towards the bottom-left corner
    assert!(close(r.fraction(0.0, 5.0), 0.75)); // 9 o'clock
    assert!(close(r.fraction(0.0, 0.0), 0.875)); // top-left corner
    // the area grows linearly along an edge: half way between 12 o'clock and the corner
    assert!(close(r.fraction(15.0, 0.0), 0.0625));
    assert!(close(r.fraction(7.5, 2.5), 0.9375));
    // half open: the left and top edges are inside, the right and bottom ones are not
    assert!(r.fraction(0.0, 9.9).is_some());
    assert_eq!(r.fraction(20.0, 5.0), None);
    assert_eq!(r.fraction(10.0, 10.0), None);
    assert_eq!(r.fraction(10.0, -0.1), None);
    // just left of 12 o'clock is almost 1.0, never 1.0
    let f = r.fraction(10.0 - 1e-12, 0.0).unwrap();
    assert!((0.0..1.0).contains(&f));
    // the hole
    let r = rectangle(10.0, 5.0, 10.0, 5.0, HollowThickness::Percentage(50));
    assert_eq!(r.fraction(10.0, 5.0), None);
    assert_eq!(r.fraction(14.0, 7.0), None);
    assert!(close(r.fraction(19.0, 5.0), 0.25));
    assert!(close(r.fraction(15.0, 5.0), 0.25));
    // a hole as large as the shape leaves nothing
    let r = rectangle(10.0, 5.0, 10.0, 5.0, HollowThickness::Percentage(100));
    assert_eq!(r.fraction(10.0, 0.0), None);
    assert_eq!(r.fraction(19.0, 5.0), None);
}

#[test]
fn check_rectangle_areas_are_proportional() {
    // four equal slices must cover the same area on a very stretched rectangle,
    // with or without a hole, even if the hole does not have the shape's proportions
    let mut slices: Vec<Slice> = [1u64, 1, 1, 1].iter().map(|v| Slice::new(*v, "", None)).collect();
    update_proportions(&mut slices);
    for hole in [HollowThickness::None, HollowThickness::Percentage(60), HollowThickness::Absolute(30), HollowThickness::Rectangle(70, 2)] {
        // 80 x 10 cells = 80 x 20 physical units
        let shape = Rectangle::fill(Size::new(80, 10), hole, false);
        let mut counts = [0usize; 4];
        for y in 0..80 {
            for x in 0..320 {
                if let Some(id) = shape.slice_at((x as f64 + 0.5) / 4.0, (y as f64 + 0.5) / 4.0, &slices) {
                    counts[id] += 1;
                }
            }
        }
        let total: usize = counts.iter().sum();
        for c in counts {
            let share = c as f64 / total as f64;
            assert!((share - 0.25).abs() < 0.01, "{hole:?}: {counts:?}");
        }
    }
}

#[test]
fn check_square_is_square() {
    // a square looks square: twice as many columns as rows
    // (13 x 9: the square is 11 characters wide, the closest with pixels 2 characters wide is 10)
    for (w, h, side_w, side_h) in [(40u32, 12u32, 20, 10), (12, 20, 10, 5), (13, 9, 10, 5)] {
        let (s, map) = paint_with_map(DrawMode::new(DrawModeShape::Square, DrawModeRenderer::Custom('*'), HollowThickness::None), w, h);
        let cols: Vec<i32> = (0..w as i32).filter(|x| (0..h as i32).any(|y| map.get(*x as u32, y as u32).is_some())).collect();
        let rows: Vec<i32> = (0..h as i32).filter(|y| (0..w as i32).any(|x| map.get(x as u32, *y as u32).is_some())).collect();
        assert_eq!((cols.len(), rows.len()), (side_w, side_h), "{w}x{h}");
        // centered in the plot area
        let (left, right) = (cols[0] - 1, w as i32 - 2 - cols[cols.len() - 1]);
        let (top, bottom) = (rows[0] - 1, h as i32 - 2 - rows[rows.len() - 1]);
        assert!((left - right).abs() <= 1 && (top - bottom).abs() <= 1, "{w}x{h}: {left} {right} {top} {bottom}");
        // and filled completely
        for y in rows.iter() {
            for x in cols.iter() {
                assert!(s.char(*x, *y).unwrap().code == '*', "{w}x{h}: ({x},{y})");
            }
        }
    }
}

#[test]
fn check_render_square() {
    check_render_with("square, 4 slices, 40x12", &[50, 25, 15, 10], 40, 12, |c| c.set_shape(DrawModeShape::Square), 0x677A69C539DCD3F9);
}

#[test]
fn check_render_square_hollow() {
    check_render_with(
        "square with a 50% hole",
        &[50, 25, 15, 10],
        40,
        12,
        |c| {
            c.set_shape(DrawModeShape::Square);
            c.set_hollow_thickness(HollowThickness::Percentage(50));
        },
        0xDB0056F9D9D6563A,
    );
}

#[test]
fn check_render_rectangle() {
    check_render_with("rectangle, 4 slices, 40x10", &[50, 25, 15, 10], 40, 10, |c| c.set_shape(DrawModeShape::Rectangle), 0x9C870269CFE08CF0);
}

#[test]
fn check_render_rectangle_hollow() {
    check_render_with(
        "rectangle with a 30x4 hole",
        &[50, 25, 15, 10],
        40,
        10,
        |c| {
            c.set_shape(DrawModeShape::Rectangle);
            c.set_hollow_thickness(HollowThickness::Rectangle(30, 4));
        },
        0x5B2019D5BA78F3C8,
    );
}

#[test]
fn check_render_rectangle_braille() {
    check_render_with(
        "rectangle, braille, 60% hole",
        &[50, 25, 15, 10],
        40,
        10,
        |c| {
            c.set_shape(DrawModeShape::Rectangle);
            c.set_renderer(DrawModeRenderer::Braille);
            c.set_hollow_thickness(HollowThickness::Percentage(60));
        },
        0x2D57F93E3F47B551,
    );
}

// ---------------------------------------------------------------------------
// rounded rectangle and square
// ---------------------------------------------------------------------------
fn round_rectangle(cx: f64, cy: f64, rx: f64, ry: f64, hollow: HollowThickness) -> Rectangle {
    Rectangle::new(cx, cy, rx, ry, hollow, true)
}

#[test]
fn check_round_rectangle_fraction() {
    // 20 x 10 physical units, centered at (10, 5); corner radius 2.5
    let r = round_rectangle(10.0, 5.0, 10.0, 5.0, HollowThickness::None);
    let close = |a: Option<f64>, b: f64| (a.unwrap() - b).abs() < 1e-9;
    assert!(close(r.fraction(10.0, 0.0), 0.0)); // 12 o'clock
    assert!(close(r.fraction(19.0, 5.0), 0.25)); // 3 o'clock
    assert!(close(r.fraction(10.0, 9.0), 0.5)); // 6 o'clock
    assert!(close(r.fraction(0.0, 5.0), 0.75)); // 9 o'clock
    // the corners are cut
    let plain = rectangle(10.0, 5.0, 10.0, 5.0, HollowThickness::None);
    for (x, y) in [(0.2, 0.2), (19.7, 0.2), (19.7, 9.7), (0.2, 9.7)] {
        assert!(plain.fraction(x, y).is_some(), "({x},{y})");
        assert_eq!(r.fraction(x, y), None, "({x},{y})");
    }
    // ... but not the edges
    assert!(r.fraction(10.0, 0.1).is_some());
    assert!(r.fraction(0.1, 5.0).is_some());
    // a square is symmetric: the diagonal is 1/8 of the area
    let s = round_rectangle(10.0, 10.0, 10.0, 10.0, HollowThickness::None);
    assert!(close(s.fraction(14.0, 6.0), 0.125));
    assert!(close(s.fraction(6.0, 14.0), 0.625));
    // just left of 12 o'clock is almost 1.0, never 1.0
    let f = r.fraction(10.0 - 1e-12, 0.0).unwrap();
    assert!((0.0..1.0).contains(&f));
}

#[test]
fn check_round_rectangle_fraction_grows_continuously() {
    // going around the shape, the fraction always grows, without jumps (also where an edge meets an arc)
    for hole in [HollowThickness::None, HollowThickness::Percentage(50), HollowThickness::Rectangle(12, 2)] {
        let r = round_rectangle(20.0, 8.0, 20.0, 8.0, hole);
        let steps = 5000;
        let mut last = 0.0;
        for i in 1..steps {
            let angle = std::f64::consts::TAU * i as f64 / steps as f64;
            // the fraction only depends on the direction: take the first point of the ray
            // (at this angle, y grows downwards) that is inside the shape
            let (ux, uy) = (angle.sin(), -angle.cos());
            let f = (1..400).find_map(|k| r.fraction(20.0 + ux * k as f64 * 0.05, 8.0 + uy * k as f64 * 0.05)).unwrap();
            assert!(f >= last - 1e-12, "{hole:?}: {f} after {last} at step {i}");
            assert!(f - last < 0.003, "{hole:?}: jump from {last} to {f} at step {i}");
            last = f;
        }
        assert!(last > 0.999, "{hole:?}: {last}");
    }
}

#[test]
fn check_round_rectangle_areas_are_proportional() {
    // slices cover areas proportional to their values, with or without a hole
    for values in [&[1u64, 1, 1, 1][..], &[50, 25, 15, 10]] {
        let mut slices: Vec<Slice> = values.iter().map(|v| Slice::new(*v, "", None)).collect();
        update_proportions(&mut slices);
        let total: u64 = values.iter().sum();
        for hole in [HollowThickness::None, HollowThickness::Percentage(60), HollowThickness::Absolute(30), HollowThickness::Rectangle(70, 2)] {
            // 80 x 10 cells = 80 x 20 physical units
            let shape = Rectangle::fill(Size::new(80, 10), hole, true);
            let mut counts = vec![0usize; values.len()];
            for y in 0..80 {
                for x in 0..320 {
                    if let Some(id) = shape.slice_at((x as f64 + 0.5) / 4.0, (y as f64 + 0.5) / 4.0, &slices) {
                        counts[id] += 1;
                    }
                }
            }
            let covered: usize = counts.iter().sum();
            for (c, v) in counts.iter().zip(values) {
                let share = *c as f64 / covered as f64;
                let expected = *v as f64 / total as f64;
                assert!((share - expected).abs() < 0.01, "{values:?} {hole:?}: {counts:?}");
            }
        }
    }
}

#[test]
fn check_render_round_square() {
    check_render_with("round square, 4 slices, 40x12", &[50, 25, 15, 10], 40, 12, |c| c.set_shape(DrawModeShape::RoundSquare), 0x51639EC34B0A5CBE);
}

#[test]
fn check_render_round_square_hollow() {
    check_render_with(
        "round square with a 50% hole",
        &[50, 25, 15, 10],
        40,
        12,
        |c| {
            c.set_shape(DrawModeShape::RoundSquare);
            c.set_hollow_thickness(HollowThickness::Percentage(50));
        },
        0x9219681F5C2ECE1F,
    );
}

#[test]
fn check_render_round_rectangle() {
    check_render_with("round rectangle, 4 slices, 40x10", &[50, 25, 15, 10], 40, 10, |c| c.set_shape(DrawModeShape::RoundRectangle), 0x9D3842B92D80570B);
}

#[test]
fn check_render_round_rectangle_braille() {
    check_render_with(
        "round rectangle, braille, 60% hole",
        &[50, 25, 15, 10],
        40,
        10,
        |c| {
            c.set_shape(DrawModeShape::RoundRectangle);
            c.set_renderer(DrawModeRenderer::Braille);
            c.set_hollow_thickness(HollowThickness::Percentage(60));
        },
        0x9D0A407CF5979639,
    );
}

// ---------------------------------------------------------------------------
// triangle and rounded triangle
// ---------------------------------------------------------------------------
use super::shapes::Triangle;

// an equilateral triangle with side 20, its centroid at (20, 20)
fn triangle(hollow: HollowThickness, rounded: bool) -> Triangle {
    Triangle::new(20.0, 20.0, 10.0, 10.0 * 3f64.sqrt(), hollow, rounded)
}

#[test]
fn check_triangle_fraction() {
    let close = |a: Option<f64>, b: f64| (a.unwrap() - b).abs() < 1e-9;
    let h = 10.0 * 3f64.sqrt();
    for rounded in [false, true] {
        let t = triangle(HollowThickness::None, rounded);
        // 12 o'clock, straight down, and the directions of the corners (an equilateral
        // triangle is symmetric: every corner is a third of the area)
        assert!(close(t.fraction(20.0, 15.0), 0.0), "rounded: {rounded}");
        assert!(close(t.fraction(20.0, 24.0), 0.5), "rounded: {rounded}");
        assert!(close(t.fraction(20.0 + 5.0, 20.0 + h / 6.0), 1.0 / 3.0), "rounded: {rounded}");
        assert!(close(t.fraction(20.0 - 5.0, 20.0 + h / 6.0), 2.0 / 3.0), "rounded: {rounded}");
        // just left of 12 o'clock is almost 1.0, never 1.0
        let f = t.fraction(20.0 - 1e-12, 15.0).unwrap();
        assert!((0.0..1.0).contains(&f), "rounded: {rounded}");
        // outside: above the apex, below the base, beside the slanted edges
        assert_eq!(t.fraction(20.0, 20.0 - 2.0 * h / 3.0 - 0.1), None);
        assert_eq!(t.fraction(20.0, 20.0 + h / 3.0 + 0.1), None);
        assert_eq!(t.fraction(12.0, 12.0), None);
        assert_eq!(t.fraction(28.0, 12.0), None);
    }
    // the corners of a rounded triangle are cut, its edges are not
    let plain = triangle(HollowThickness::None, false);
    let round = triangle(HollowThickness::None, true);
    let corners = [(20.0, 20.0 - 2.0 * h / 3.0 + 0.3), (29.7, 20.0 + h / 3.0 - 0.1), (10.3, 20.0 + h / 3.0 - 0.1)];
    for (x, y) in corners {
        assert!(plain.fraction(x, y).is_some(), "({x},{y})");
        assert_eq!(round.fraction(x, y), None, "({x},{y})");
    }
    assert!(round.fraction(20.0, 20.0 + h / 3.0 - 0.1).is_some());
    // the hole
    let t = triangle(HollowThickness::Percentage(50), true);
    assert_eq!(t.fraction(20.0, 20.0), None);
    assert!(close(t.fraction(20.0, 20.0 + h / 3.0 - 0.5), 0.5));
}

#[test]
fn check_triangle_fraction_grows_continuously() {
    // going around the shape, the fraction always grows, without jumps (also where an edge meets an arc)
    for rounded in [false, true] {
        for hole in [HollowThickness::None, HollowThickness::Percentage(50), HollowThickness::Rectangle(12, 3)] {
            let t = triangle(hole, rounded);
            let steps = 5000;
            let mut last = 0.0;
            for i in 1..steps {
                let angle = std::f64::consts::TAU * i as f64 / steps as f64;
                // the fraction only depends on the direction: take the first point of the ray
                // (at this angle, y grows downwards) that is inside the shape
                let (ux, uy) = (angle.sin(), -angle.cos());
                let f = (1..400).find_map(|k| t.fraction(20.0 + ux * k as f64 * 0.05, 20.0 + uy * k as f64 * 0.05)).unwrap();
                assert!(f >= last - 1e-12, "{hole:?} rounded: {rounded}: {f} after {last} at step {i}");
                assert!(f - last < 0.003, "{hole:?} rounded: {rounded}: jump from {last} to {f} at step {i}");
                last = f;
            }
            assert!(last > 0.999, "{hole:?} rounded: {rounded}: {last}");
        }
    }
}

#[test]
fn check_triangle_areas_are_proportional() {
    // slices cover areas proportional to their values, with or without a hole
    for rounded in [false, true] {
        for values in [&[1u64, 1, 1, 1][..], &[50, 25, 15, 10]] {
            let mut slices: Vec<Slice> = values.iter().map(|v| Slice::new(*v, "", None)).collect();
            update_proportions(&mut slices);
            let total: u64 = values.iter().sum();
            for hole in [HollowThickness::None, HollowThickness::Percentage(60), HollowThickness::Absolute(20), HollowThickness::Rectangle(30, 4)] {
                // 60 x 30 cells = 60 x 60 physical units
                let shape = Triangle::fit(Size::new(60, 30), hole, rounded);
                let mut counts = vec![0usize; values.len()];
                for y in 0..240 {
                    for x in 0..240 {
                        if let Some(id) = shape.slice_at((x as f64 + 0.5) / 4.0, (y as f64 + 0.5) / 4.0, &slices) {
                            counts[id] += 1;
                        }
                    }
                }
                let covered: usize = counts.iter().sum();
                for (c, v) in counts.iter().zip(values) {
                    let share = *c as f64 / covered as f64;
                    let expected = *v as f64 / total as f64;
                    assert!((share - expected).abs() < 0.01, "{values:?} {hole:?} rounded: {rounded}: {counts:?}");
                }
            }
        }
    }
}

#[test]
fn check_triangle_is_equilateral() {
    // the base is as wide as the sides are long (a character is twice as tall as it is wide),
    // and the triangle is centered in the plot area
    for (w, h) in [(42u32, 14u32), (22, 30), (60, 12)] {
        let (_, map) = paint_with_map(DrawMode::new(DrawModeShape::Triangle, DrawModeRenderer::HalfBlocks, HollowThickness::None), w, h);
        let cols: Vec<i32> = (0..w as i32).filter(|x| (0..h as i32).any(|y| map.get(*x as u32, y as u32).is_some())).collect();
        let rows: Vec<i32> = (0..h as i32).filter(|y| (0..w as i32).any(|x| map.get(x as u32, *y as u32).is_some())).collect();
        let base = cols.len() as f64;
        let height = rows.len() as f64 * 2.0;
        assert!((height / base - 3f64.sqrt() / 2.0).abs() < 0.12, "{w}x{h}: {base} x {height}");
        let (left, right) = (cols[0] - 1, w as i32 - 2 - cols[cols.len() - 1]);
        let (top, bottom) = (rows[0] - 1, h as i32 - 2 - rows[rows.len() - 1]);
        assert!((left - right).abs() <= 1 && (top - bottom).abs() <= 1, "{w}x{h}: {left} {right} {top} {bottom}");
    }
}

#[test]
fn check_render_triangle() {
    check_render_with("triangle, 4 slices, 40x14", &[50, 25, 15, 10], 40, 14, |c| c.set_shape(DrawModeShape::Triangle), 0x61BEE3DCBA0FD1E1);
}

#[test]
fn check_render_round_triangle_hollow() {
    check_render_with(
        "round triangle with a 40% hole",
        &[50, 25, 15, 10],
        40,
        14,
        |c| {
            c.set_shape(DrawModeShape::RoundTriangle);
            c.set_hollow_thickness(HollowThickness::Percentage(40));
        },
        0xADAA77AAC8AAF46E,
    );
}

#[test]
fn check_render_triangle_braille() {
    check_render_with(
        "triangle, braille, 50% hole",
        &[50, 25, 15, 10],
        40,
        14,
        |c| {
            c.set_shape(DrawModeShape::Triangle);
            c.set_renderer(DrawModeRenderer::Braille);
            c.set_hollow_thickness(HollowThickness::Percentage(50));
        },
        0xFE86104D5E73045C,
    );
}

#[test]
fn check_render_round_triangle_custom_char() {
    check_render_with(
        "round triangle, custom char",
        &[50, 25, 15, 10],
        40,
        14,
        |c| {
            c.set_shape(DrawModeShape::RoundTriangle);
            c.set_renderer(DrawModeRenderer::Custom('o'));
        },
        0x42D836B1F74FB209,
    );
}

// ---------------------------------------------------------------------------
// start angle and direction
// ---------------------------------------------------------------------------
use super::shapes::Rotation;
use super::Direction;

fn rotated_map(shape: DrawModeShape, values: &[u64], start_angle: u32, direction: Direction, w: u32, h: u32) -> PaletteTile {
    let mut slices: Vec<Slice> = values.iter().map(|v| Slice::new(*v, "", None)).collect();
    update_proportions(&mut slices);
    let mut surface = Surface::new(w, h);
    let mut map = PaletteTile::new(w, h);
    let mut mode = DrawMode::new(shape, DrawModeRenderer::HalfBlocks, HollowThickness::None);
    mode.start_angle = start_angle;
    mode.direction = direction;
    let area = plot_area(surface.size());
    mode.paint(&mut surface, &Theme::new(Themes::Default), &slices, &mut PaletteTile::new(1, 1), &mut map, area);
    map
}

#[test]
fn check_rotation_apply() {
    let close = |a: f64, b: f64| (a - b).abs() < 1e-12;
    let clockwise = Rotation::new(0, Direction::Clockwise);
    let counter_clockwise = Rotation::new(0, Direction::CounterClockwise);
    assert!(close(clockwise.apply(0.3, 0.0), 0.3));
    assert!(close(counter_clockwise.apply(0.25, 0.0), 0.75));
    assert!(close(counter_clockwise.apply(0.0, 0.0), 0.0));
    // measured from a start at 0.25
    assert!(close(clockwise.apply(0.25, 0.25), 0.0));
    assert!(close(clockwise.apply(0.1, 0.25), 0.85));
    assert!(close(counter_clockwise.apply(0.1, 0.25), 0.15));
    // never 1.0
    assert!(clockwise.apply(0.25 - 1e-17, 0.25) < 1.0);
    assert!(counter_clockwise.apply(0.25 + 1e-17, 0.25) < 1.0);
}

#[test]
fn check_start_angle_and_direction() {
    // 4 equal slices on shapes that are symmetric left-right and top-bottom: the slice in each quarter
    // (top-right, bottom-right, bottom-left, top-left)
    let shapes = [DrawModeShape::Circle, DrawModeShape::Ellipse, DrawModeShape::Square, DrawModeShape::Rectangle, DrawModeShape::RoundSquare, DrawModeShape::RoundRectangle];
    let cases: [(u32, Direction, [u16; 4]); 7] = [
        (0, Direction::Clockwise, [0, 1, 2, 3]),
        (90, Direction::Clockwise, [3, 0, 1, 2]),
        (180, Direction::Clockwise, [2, 3, 0, 1]),
        (0, Direction::CounterClockwise, [3, 2, 1, 0]),
        (180, Direction::CounterClockwise, [1, 0, 3, 2]),
        // 360 is the same as 0, larger values are treated as 360
        (360, Direction::Clockwise, [0, 1, 2, 3]),
        (1000, Direction::CounterClockwise, [3, 2, 1, 0]),
    ];
    for shape in shapes {
        for (angle, direction, expected) in cases {
            let map = rotated_map(shape, &[1, 1, 1, 1], angle, direction, 42, 22);
            // the area is 40 x 20 cells, its center is (21, 11); a point in each quarter
            let quarters = [(25, 8), (25, 14), (16, 14), (16, 8)].map(|(x, y)| map.get(x, y).unwrap());
            assert_eq!(quarters, expected, "{shape:?} {angle} {direction:?}");
        }
    }
}

#[test]
fn check_start_angle_is_where_the_first_slice_starts() {
    // a circle, 2 slices (10% and 90%), starting at 3 o'clock: the small slice is just below the start, the big one just above
    for (direction, below, above) in [(Direction::Clockwise, 0, 1), (Direction::CounterClockwise, 1, 0)] {
        let map = rotated_map(DrawModeShape::Circle, &[10, 90], 90, direction, 42, 22);
        assert_eq!(map.get(30, 12), Some(below), "{direction:?}");
        assert_eq!(map.get(30, 9), Some(above), "{direction:?}");
    }
}

#[test]
fn check_rotation_keeps_areas_proportional() {
    // whatever the start and the direction, every slice covers an area proportional to its value
    let values = [50u64, 25, 15, 10];
    for shape in SHAPES {
        for (angle, direction) in [(37, Direction::CounterClockwise), (200, Direction::Clockwise), (300, Direction::CounterClockwise)] {
            let map = rotated_map(shape, &values, angle, direction, 162, 62);
            let mut counts = [0usize; 4];
            for y in 0..62 {
                for x in 0..162 {
                    if let Some(id) = map.get(x, y) {
                        counts[id as usize] += 1;
                    }
                }
            }
            let covered: usize = counts.iter().sum();
            for (c, v) in counts.iter().zip(values) {
                let share = *c as f64 / covered as f64;
                assert!((share - v as f64 / 100.0).abs() < 0.02, "{shape:?} {angle} {direction:?}: {counts:?}");
            }
        }
    }
}

#[test]
fn check_label_sides_with_rotation() {
    // 4 equal slices: the middle of each slice decides the side of its label
    let slices = labelled(&[(1, "a"); 4], LabelContent::Label);
    let mut mode = DrawMode { start_angle: 90, ..Default::default() };
    // middles at 135, 225, 315 and 45 degrees
    assert_eq!(slices.iter().map(|s| side_of(s, &mode)).collect::<Vec<_>>(), vec![Side::Bottom, Side::Left, Side::Top, Side::Right]);
    mode.start_angle = 0;
    mode.direction = Direction::CounterClockwise;
    // middles at 315, 225, 135 and 45 degrees
    assert_eq!(slices.iter().map(|s| side_of(s, &mode)).collect::<Vec<_>>(), vec![Side::Top, Side::Left, Side::Bottom, Side::Right]);
}

#[test]
fn check_render_start_angle() {
    check_render_with("4 slices, starting at 90 degrees", &[50, 25, 15, 10], 40, 12, |c| c.set_start_angle(90), 0x2B8447199D1024A7);
}

#[test]
fn check_render_counter_clockwise() {
    check_render_with(
        "rectangle, 60% hole, from 45 degrees, counter-clockwise",
        &[50, 25, 15, 10],
        40,
        10,
        |c| {
            c.set_shape(DrawModeShape::Rectangle);
            c.set_hollow_thickness(HollowThickness::Percentage(60));
            c.set_start_angle(45);
            c.set_direction(Direction::CounterClockwise);
        },
        0x347E4D9339915594,
    );
}

#[test]
fn check_labels_outside_counter_clockwise() {
    let script = "
        Paint.Enable(false)
        Paint('outside labels, from 270 degrees, counter-clockwise')
        CheckHash(0x1A86168CF262E606)
    ";
    App::new()
        .size(Size::new(64, 22))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Test,d:f");
            let mut c = PieChart::new(layout!("x:1,y:0,w:60,h:19"), piechart::Flags::None);
            for (v, l) in [(45, "Rust"), (25, "C++"), (15, "Go"), (10, "Zig"), (5, "Other")] {
                c.add_value(v, l, None);
            }
            c.set_label_layout(LabelLayout::Outside);
            c.set_start_angle(270);
            c.set_direction(Direction::CounterClockwise);
            w.add(c);
            w
        })
        .run()
        .unwrap();
}

// ---------------------------------------------------------------------------
// events
// ---------------------------------------------------------------------------
use std::sync::Mutex;

static EVENTS: Mutex<Vec<String>> = Mutex::new(Vec::new());

#[test]
fn check_events() {
    #[Window(events = PieChartEvents, internal: true)]
    struct MyWin {}
    impl MyWin {
        fn new() -> Self {
            let mut win = Self { base: window!("Test,d:f") };
            let mut c = PieChart::new(layout!("x:1,y:0,w:40,h:15"), piechart::Flags::None);
            c.add_value(50, "A", None);
            c.add_value(25, "B", None);
            c.add_value(25, "C", None);
            // selecting from code does not raise an event
            c.set_selected_slice(2);
            c.clear_selected_slice();
            win.add(c);
            win.add(button!("Other,x:1,y:15,w:10"));
            win
        }
    }
    impl PieChartEvents for MyWin {
        fn on_slice_selected(&mut self, handle: Handle<PieChart>, index: usize) -> EventProcessStatus {
            let label = self.control(handle).and_then(|c| c.label(index).map(String::from)).unwrap_or_default();
            EVENTS.lock().unwrap().push(format!("selected {index} ({label})"));
            EventProcessStatus::Processed
        }
        fn on_clear_selection(&mut self, handle: Handle<PieChart>) -> EventProcessStatus {
            let selected = self.control(handle).map(|c| c.selected_slice());
            EVENTS.lock().unwrap().push(format!("cleared ({selected:?})"));
            EventProcessStatus::Processed
        }
    }
    EVENTS.lock().unwrap().clear();
    // slice A is the right half, B the bottom-left quarter, C the top-left quarter
    let script = "
        Paint.Enable(false)
        Mouse.Click(23,10,left)
        Mouse.Click(14,12,left)
        Mouse.Click(14,12,left)
        Mouse.Click(14,5,left)
        Mouse.Click(3,1,left)
        Mouse.Click(23,10,left)
        Mouse.Click(4,16,left)
    ";
    App::new().size(Size::new(44, 19)).debug_script(script).window(MyWin::new).run().unwrap();
    let events = EVENTS.lock().unwrap().clone();
    assert_eq!(
        events,
        vec![
            "selected 0 (A)",
            "selected 1 (B)",
            "selected 2 (C)",
            "cleared (Some(None))",
            "selected 0 (A)",
            "cleared (Some(None))",
        ]
    );
}
