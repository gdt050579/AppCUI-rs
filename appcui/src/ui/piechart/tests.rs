use crate::prelude::*;

fn chart() -> PieChart {
    PieChart::new(layout!("x:1,y:1,w:20,h:10"), piechart::Flags::None)
}

/// Runs `test` inside a test application. `PieChart` repaints its internal surface
/// whenever its data changes, which needs a running application (same as `VBarChart`).
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

/// A list of slices with their proportions computed (no `PieChart` needed).
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

/// Checks the invariants every set of slices must satisfy.
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
fn check_surface_background() {
    let script = "
        Paint.Enable(false)
        Paint('chart area is filled with the chart background')
        CheckHash(0xD2DBF921946B6BC1)
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
