use crate::prelude::*;
use std::ops::DerefMut;
use crate::ui::vstackedbarchart::{BarsStack, BarsStackBuilder, StackBar, StackBarBuilder, StackBarDrawMode, StackBarScale, XAxisLabelMode};
use super::XAxisLabelFormat;
use super::mapping::ValueMapping;
use super::stackbar::{paint_segment, resolve_attr};
use crate::ui::components::BarFillType;

fn in_app(f: impl FnOnce() + 'static) {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
    ";
    App::new()
        .size(Size::new(30, 10))
        .debug_script(script)
        .window(move || {
            f();
            window!("Test,d:f")
        })
        .run()
        .unwrap();
}

fn new_chart() -> VStackedBarChart<i32> {
    VStackedBarChart::<i32>::new(layout!("x:1,y:1,w:10,h:5"), vstackedbarchart::Flags::None)
}

fn values_of(stack: &BarsStack<i32>) -> Vec<i32> {
    (0..stack.count()).map(|i| stack.get(i).unwrap().value()).collect()
}

fn rect() -> StackBarDrawMode {
    StackBarDrawMode::Rectangle(LineType::Single)
}

fn filled_rect() -> StackBarDrawMode {
    StackBarDrawMode::FilledRectangle(LineType::Single)
}

fn fill() -> StackBarDrawMode {
    StackBarDrawMode::Fill(BarFillType::Solid)
}

#[test]
fn check_creation() {
    in_app(|| {
        let chart = new_chart();
        assert_eq!(chart.stacks_count(), 0);
        assert!(chart.get_stack(0).is_none());
    });
}

#[test]
fn check_creation_with_scrollbars() {
    in_app(|| {
        let chart = VStackedBarChart::<f64>::new(
            layout!("x:1,y:1,w:10,h:5"),
            vstackedbarchart::Flags::ScrollBars | vstackedbarchart::Flags::ShowZeroLineOnYAxis,
        );
        assert_eq!(chart.stacks_count(), 0);
    });
}

#[test]
fn check_draw_mode_min_thickness() {
    assert_eq!(fill().min_thickness(), 1);
    assert_eq!(rect().min_thickness(), 2);
    assert_eq!(filled_rect().min_thickness(), 3);
}

#[test]
fn check_draw_mode_default() {
    assert_eq!(StackBarDrawMode::default(), fill());
}

#[test]
fn check_stack_bar_from_value() {
    let bar: StackBar<i32> = 7.into();
    assert_eq!(bar.value(), 7);
    assert_eq!(bar.label(), "");
    assert!(bar.attr().is_none());
    assert!(bar.draw_mode().is_none());
}

#[test]
fn check_stack_bar_from_reference() {
    let value = -4;
    let bar: StackBar<i32> = (&value).into();
    assert_eq!(bar.value(), -4);
}

#[test]
fn check_stack_bar_builder() {
    let bar = StackBarBuilder::new(5)
        .label("Jun")
        .attr(CharAttribute::with_fore_color(Color::Yellow))
        .draw_mode(rect())
        .build();
    assert_eq!(bar.value(), 5);
    assert_eq!(bar.label(), "Jun");
    assert!(bar.attr().is_some());
    assert_eq!(bar.draw_mode(), Some(rect()));
}

#[test]
fn check_stack_bar_setters_and_clear() {
    let mut bar: StackBar<i32> = 1.into();
    bar.set_value(9)
        .set_label("A")
        .set_attr(CharAttribute::with_fore_color(Color::Red))
        .set_draw_mode(filled_rect());
    assert_eq!(bar.value(), 9);
    assert_eq!(bar.label(), "A");
    assert!(bar.attr().is_some());
    assert_eq!(bar.draw_mode(), Some(filled_rect()));
    bar.set_label("BC");
    assert_eq!(bar.label(), "BC");
    bar.clear_attr().clear_draw_mode();
    assert!(bar.attr().is_none());
    assert!(bar.draw_mode().is_none());
}

#[test]
fn check_stack_bar_accepts_negative_and_float_values() {
    let negative: StackBar<i32> = (-12).into();
    assert_eq!(negative.value(), -12);
    let float: StackBar<f64> = (-1.5).into();
    assert_eq!(float.value(), -1.5);
}

#[test]
fn check_bars_stack_new_is_empty() {
    let stack = BarsStack::<i32>::new();
    assert_eq!(stack.count(), 0);
    assert!(stack.get(0).is_none());
    assert_eq!(stack.thickness(), None);
    assert_eq!(stack.spacing(), None);
    let default_stack = BarsStack::<i32>::default();
    assert_eq!(default_stack.count(), 0);
}

#[test]
fn check_bars_stack_add_bar_and_bars() {
    let mut stack = BarsStack::<i32>::new();
    stack.add_bar(1).add_bar(StackBarBuilder::new(-2).label("neg").build());
    stack.add_bars([3, -4]);
    stack.add_bars([5, 6]);
    assert_eq!(values_of(&stack), vec![1, -2, 3, -4, 5, 6]);
    assert_eq!(stack.get(1).unwrap().label(), "neg");
    assert!(stack.get(6).is_none());
}

#[test]
fn check_bars_stack_get_mut() {
    let mut stack = BarsStack::<i32>::new();
    stack.add_bars([1, 2, 3]);
    stack.get_mut(1).unwrap().set_value(20).set_label("mid");
    assert_eq!(values_of(&stack), vec![1, 20, 3]);
    assert_eq!(stack.get(1).unwrap().label(), "mid");
    assert!(stack.get_mut(3).is_none());
}

#[test]
fn check_bars_stack_thickness_and_spacing_options() {
    let mut stack = BarsStack::<i32>::new();
    stack.set_thickness(4).set_spacing(2);
    assert_eq!(stack.thickness(), Some(4));
    assert_eq!(stack.spacing(), Some(2));
    stack.clear_thickness().clear_spacing();
    assert_eq!(stack.thickness(), None);
    assert_eq!(stack.spacing(), None);
}

#[test]
fn check_bars_stack_builder() {
    let stack = BarsStackBuilder::<i32>::new()
        .bar(1)
        .bar(StackBarBuilder::new(2).draw_mode(rect()).build())
        .bars([3, 4])
        .thickness(5)
        .spacing(2)
        .build();
    assert_eq!(values_of(&stack), vec![1, 2, 3, 4]);
    assert_eq!(stack.thickness(), Some(5));
    assert_eq!(stack.spacing(), Some(2));
    assert_eq!(stack.get(1).unwrap().draw_mode(), Some(rect()));
    let default_builder = BarsStackBuilder::<i32>::default().build();
    assert_eq!(default_builder.count(), 0);
}

#[test]
fn check_bars_stack_actual_spacing() {
    let mut stack = BarsStack::<i32>::new();
    assert_eq!(stack.actual_spacing(1), 1);
    assert_eq!(stack.actual_spacing(0), 0);
    stack.set_spacing(3);
    assert_eq!(stack.actual_spacing(1), 3);
    stack.set_spacing(0);
    assert_eq!(stack.actual_spacing(1), 0);
}

#[test]
fn check_actual_thickness_empty_stack() {
    let mut stack = BarsStack::<i32>::new();
    stack.update_actual_thickness(1, fill());
    assert_eq!(stack.actual_thickness(), 1);
    stack.update_actual_thickness(6, fill());
    assert_eq!(stack.actual_thickness(), 6);
}

#[test]
fn check_actual_thickness_uses_requested_value() {
    let mut stack = BarsStack::<i32>::from([1, 2, 3]);
    stack.set_thickness(4);
    stack.update_actual_thickness(1, fill());
    assert_eq!(stack.actual_thickness(), 4);
}

#[test]
fn check_actual_thickness_falls_back_to_default() {
    let mut stack = BarsStack::<i32>::from([1, 2, 3]);
    stack.update_actual_thickness(3, fill());
    assert_eq!(stack.actual_thickness(), 3);
}

#[test]
fn check_actual_thickness_never_below_one() {
    let mut stack = BarsStack::<i32>::from([1, 2, 3]);
    stack.set_thickness(0);
    stack.update_actual_thickness(0, fill());
    assert_eq!(stack.actual_thickness(), 1);
}

#[test]
fn check_actual_thickness_rectangle_raises_to_two() {
    let mut stack = BarsStack::<i32>::new();
    stack.add_bar(StackBarBuilder::new(1).draw_mode(rect()).build());
    stack.set_thickness(1);
    stack.update_actual_thickness(1, fill());
    assert_eq!(stack.actual_thickness(), 2);
}

#[test]
fn check_actual_thickness_filled_rectangle_raises_to_three() {
    let mut stack = BarsStack::<i32>::new();
    stack.add_bar(StackBarBuilder::new(1).draw_mode(filled_rect()).build());
    stack.update_actual_thickness(1, fill());
    assert_eq!(stack.actual_thickness(), 3);
}

#[test]
fn check_actual_thickness_mixed_fill_and_rectangle() {
    let mut stack = BarsStack::<i32>::new();
    stack.add_bar(StackBarBuilder::new(1).draw_mode(fill()).build());
    stack.add_bar(StackBarBuilder::new(2).draw_mode(rect()).build());
    stack.update_actual_thickness(1, fill());
    assert_eq!(stack.actual_thickness(), 2);
}

#[test]
fn check_actual_thickness_uses_largest_minimum() {
    let mut stack = BarsStack::<i32>::new();
    stack.add_bar(StackBarBuilder::new(1).draw_mode(rect()).build());
    stack.add_bar(StackBarBuilder::new(2).draw_mode(filled_rect()).build());
    stack.add_bar(StackBarBuilder::new(3).draw_mode(fill()).build());
    stack.update_actual_thickness(1, fill());
    assert_eq!(stack.actual_thickness(), 3);
}

#[test]
fn check_actual_thickness_request_larger_than_minimum() {
    let mut stack = BarsStack::<i32>::new();
    stack.add_bar(StackBarBuilder::new(1).draw_mode(filled_rect()).build());
    stack.set_thickness(5);
    stack.update_actual_thickness(1, fill());
    assert_eq!(stack.actual_thickness(), 5);
}

#[test]
fn check_actual_thickness_default_draw_mode_applies_to_bars_without_override() {
    let mut stack = BarsStack::<i32>::from([1, 2]);
    stack.update_actual_thickness(1, rect());
    assert_eq!(stack.actual_thickness(), 2);
    stack.update_actual_thickness(1, filled_rect());
    assert_eq!(stack.actual_thickness(), 3);
}

#[test]
fn check_actual_thickness_bar_override_beats_default_draw_mode() {
    let mut stack = BarsStack::<i32>::new();
    stack.add_bar(StackBarBuilder::new(1).draw_mode(fill()).build());
    stack.update_actual_thickness(1, filled_rect());
    assert_eq!(stack.actual_thickness(), 1);
}

#[test]
fn check_actual_thickness_recomputed_after_bar_change() {
    let mut stack = BarsStack::<i32>::from([1, 2, 3]);
    stack.update_actual_thickness(1, fill());
    assert_eq!(stack.actual_thickness(), 1);
    stack.get_mut(2).unwrap().set_draw_mode(filled_rect());
    stack.update_actual_thickness(1, fill());
    assert_eq!(stack.actual_thickness(), 3);
    stack.get_mut(2).unwrap().clear_draw_mode();
    stack.update_actual_thickness(1, fill());
    assert_eq!(stack.actual_thickness(), 1);
}

#[test]
fn check_bars_stack_from_slice() {
    let values = [1, -2, 3];
    let stack: BarsStack<i32> = (&values[..]).into();
    assert_eq!(values_of(&stack), vec![1, -2, 3]);
}

#[test]
fn check_bars_stack_from_vec() {
    let stack: BarsStack<i32> = vec![4, 5, 6].into();
    assert_eq!(values_of(&stack), vec![4, 5, 6]);
}

#[test]
fn check_bars_stack_from_vec_reference() {
    let values = vec![7, 8];
    let stack: BarsStack<i32> = (&values).into();
    assert_eq!(values_of(&stack), vec![7, 8]);
    assert_eq!(values.len(), 2);
}

#[test]
fn check_bars_stack_from_array() {
    let stack: BarsStack<i32> = [9, 10, 11, 12].into();
    assert_eq!(values_of(&stack), vec![9, 10, 11, 12]);
}

#[test]
fn check_bars_stack_from_array_reference() {
    let values = [1, 2, 3];
    let stack: BarsStack<i32> = (&values).into();
    assert_eq!(values_of(&stack), vec![1, 2, 3]);
}

#[test]
fn check_bars_stack_from_empty_slice() {
    let values: [i32; 0] = [];
    let stack: BarsStack<i32> = (&values[..]).into();
    assert_eq!(stack.count(), 0);
}

#[test]
fn check_bars_stack_from_stack_reference_is_a_copy() {
    let mut original = BarsStackBuilder::<i32>::new().bars([1, 2]).thickness(3).spacing(1).build();
    let copy: BarsStack<i32> = (&original).into();
    original.get_mut(0).unwrap().set_value(100);
    assert_eq!(values_of(&copy), vec![1, 2]);
    assert_eq!(copy.thickness(), Some(3));
    assert_eq!(copy.spacing(), Some(1));
}

#[test]
fn check_add_stack_from_stack() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack(BarsStackBuilder::<i32>::new().bars([1, 2, 3]).build());
        assert_eq!(chart.stacks_count(), 1);
        assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![1, 2, 3]);
    });
}

#[test]
fn check_add_stack_from_values() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1, 2, 3]);
        chart.add_stack([4, 5]);
        chart.add_stack(vec![6]);
        chart.add_stack(vec![7, 8]);
        chart.add_stack(&[9, 10, 11][..]);
        assert_eq!(chart.stacks_count(), 5);
        assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![1, 2, 3]);
        assert_eq!(values_of(chart.get_stack(1).unwrap()), vec![4, 5]);
        assert_eq!(values_of(chart.get_stack(2).unwrap()), vec![6]);
        assert_eq!(values_of(chart.get_stack(3).unwrap()), vec![7, 8]);
        assert_eq!(values_of(chart.get_stack(4).unwrap()), vec![9, 10, 11]);
    });
}

#[test]
fn check_add_stack_keeps_negative_and_positive_values_in_order() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([5, -3, 2, -1]);
        assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![5, -3, 2, -1]);
    });
}

#[test]
fn check_add_stack_flat_list_is_one_stack() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1, 2, 3]);
        assert_eq!(chart.stacks_count(), 1);
        assert_eq!(chart.get_stack(0).unwrap().count(), 3);
    });
}

#[test]
fn check_add_stack_computes_thickness() {
    in_app(|| {
        let mut chart = new_chart();
        let mut stack = BarsStack::<i32>::new();
        stack.add_bar(StackBarBuilder::new(1).draw_mode(filled_rect()).build());
        chart.add_stack(stack);
        chart.add_stack([1, 2]);
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 3);
        assert_eq!(chart.get_stack(1).unwrap().actual_thickness(), 1);
    });
}

#[test]
fn check_add_stack_uses_requested_thickness() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack(BarsStackBuilder::<i32>::new().bars([1, 2]).thickness(4).build());
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 4);
    });
}

#[test]
fn check_add_stacks_from_stacks() {
    in_app(|| {
        let mut chart = new_chart();
        let stacks = vec![BarsStack::from([1, 2]), BarsStack::from([3]), BarsStack::from([4, 5, 6])];
        chart.add_stacks(stacks);
        assert_eq!(chart.stacks_count(), 3);
        assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![1, 2]);
        assert_eq!(values_of(chart.get_stack(1).unwrap()), vec![3]);
        assert_eq!(values_of(chart.get_stack(2).unwrap()), vec![4, 5, 6]);
    });
}

#[test]
fn check_add_stacks_from_stack_references() {
    in_app(|| {
        let mut chart = new_chart();
        let stacks = vec![BarsStack::from([1, 2]), BarsStack::from([3])];
        chart.add_stacks(&stacks);
        assert_eq!(chart.stacks_count(), 2);
        assert_eq!(stacks.len(), 2);
        assert_eq!(values_of(chart.get_stack(1).unwrap()), vec![3]);
    });
}

#[test]
fn check_add_stacks_from_array_matrix() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(chart.stacks_count(), 2);
        assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![1, 2, 3]);
        assert_eq!(values_of(chart.get_stack(1).unwrap()), vec![4, 5, 6]);
    });
}

#[test]
fn check_add_stacks_from_ragged_vec_matrix() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks(vec![vec![1, 2, 3], vec![4], vec![]]);
        assert_eq!(chart.stacks_count(), 3);
        assert_eq!(chart.get_stack(0).unwrap().count(), 3);
        assert_eq!(chart.get_stack(1).unwrap().count(), 1);
        assert_eq!(chart.get_stack(2).unwrap().count(), 0);
    });
}

#[test]
fn check_add_stacks_from_slice_of_slices() {
    in_app(|| {
        let mut chart = new_chart();
        let rows: Vec<&[i32]> = vec![&[1, 2][..], &[3, 4, 5][..]];
        chart.add_stacks(rows.iter().copied());
        assert_eq!(chart.stacks_count(), 2);
        assert_eq!(values_of(chart.get_stack(1).unwrap()), vec![3, 4, 5]);
    });
}

#[test]
fn check_add_stacks_empty_iterator() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks(Vec::<BarsStack<i32>>::new());
        assert_eq!(chart.stacks_count(), 0);
    });
}

#[test]
fn check_add_stacks_appends_after_existing() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1]);
        chart.add_stacks([[2], [3]]);
        chart.add_stack([4]);
        assert_eq!(chart.stacks_count(), 4);
        let first: Vec<i32> = (0..4).map(|i| chart.get_stack(i).unwrap().get(0).unwrap().value()).collect();
        assert_eq!(first, vec![1, 2, 3, 4]);
    });
}

#[test]
fn check_add_stacks_computes_thickness_for_every_stack() {
    in_app(|| {
        let mut chart = new_chart();
        let mut rect_stack = BarsStack::<i32>::new();
        rect_stack.add_bar(StackBarBuilder::new(1).draw_mode(rect()).build());
        chart.add_stacks(vec![rect_stack, BarsStack::from([1, 2])]);
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 2);
        assert_eq!(chart.get_stack(1).unwrap().actual_thickness(), 1);
    });
}

#[test]
fn check_get_stack_out_of_range() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1]);
        assert!(chart.get_stack(0).is_some());
        assert!(chart.get_stack(1).is_none());
        assert!(chart.get_stack(usize::MAX).is_none());
    });
}

#[test]
fn check_update_stack_modifies_values() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1, 2, 3]);
        chart.update_stack(0, |stack| {
            stack.get_mut(1).unwrap().set_value(20);
            stack.add_bar(-4);
        });
        assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![1, 20, 3, -4]);
    });
}

#[test]
fn check_update_stack_recomputes_thickness() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1, 2, 3]);
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 1);
        chart.update_stack(0, |stack| {
            stack.get_mut(0).unwrap().set_draw_mode(rect());
        });
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 2);
        chart.update_stack(0, |stack| {
            stack.get_mut(1).unwrap().set_draw_mode(filled_rect());
        });
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 3);
        chart.update_stack(0, |stack| {
            stack.get_mut(1).unwrap().clear_draw_mode();
            stack.get_mut(0).unwrap().clear_draw_mode();
        });
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 1);
    });
}

#[test]
fn check_update_stack_thickness_request() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1, 2]);
        chart.update_stack(0, |stack| {
            stack.set_thickness(6);
        });
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 6);
        assert_eq!(chart.get_stack(0).unwrap().thickness(), Some(6));
    });
}

#[test]
fn check_update_stack_touches_only_the_selected_stack() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks([[1, 2], [3, 4]]);
        chart.update_stack(1, |stack| {
            stack.get_mut(0).unwrap().set_value(30).set_draw_mode(filled_rect());
        });
        assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![1, 2]);
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 1);
        assert_eq!(values_of(chart.get_stack(1).unwrap()), vec![30, 4]);
        assert_eq!(chart.get_stack(1).unwrap().actual_thickness(), 3);
    });
}

#[test]
fn check_update_stack_out_of_range_does_nothing() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1, 2]);
        let mut called = false;
        chart.update_stack(5, |_| called = true);
        assert!(!called);
        assert_eq!(chart.stacks_count(), 1);
        assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![1, 2]);
    });
}

#[test]
fn check_update_stack_on_empty_chart() {
    in_app(|| {
        let mut chart = new_chart();
        let mut called = false;
        chart.update_stack(0, |_| called = true);
        assert!(!called);
        assert_eq!(chart.stacks_count(), 0);
    });
}

#[test]
fn check_clear() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks([[1, 2], [3, 4]]);
        assert_eq!(chart.stacks_count(), 2);
        chart.clear();
        assert_eq!(chart.stacks_count(), 0);
        assert!(chart.get_stack(0).is_none());
        chart.add_stack([9]);
        assert_eq!(chart.stacks_count(), 1);
        assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![9]);
    });
}

#[test]
fn check_clear_on_empty_chart() {
    in_app(|| {
        let mut chart = new_chart();
        chart.clear();
        assert_eq!(chart.stacks_count(), 0);
    });
}

#[test]
fn check_float_chart_with_mixed_signs() {
    in_app(|| {
        let mut chart = VStackedBarChart::<f64>::new(layout!("x:1,y:1,w:10,h:5"), vstackedbarchart::Flags::None);
        chart.add_stack([1.5, -2.5, 0.0]);
        chart.add_stacks(vec![vec![-1.0, 2.0], vec![3.25]]);
        assert_eq!(chart.stacks_count(), 3);
        let first = chart.get_stack(0).unwrap();
        assert_eq!(first.get(0).unwrap().value(), 1.5);
        assert_eq!(first.get(1).unwrap().value(), -2.5);
        assert_eq!(first.get(2).unwrap().value(), 0.0);
        assert_eq!(chart.get_stack(2).unwrap().get(0).unwrap().value(), 3.25);
    });
}

#[test]
fn check_zero_values_are_kept() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([0, 0, 0]);
        assert_eq!(chart.get_stack(0).unwrap().count(), 3);
    });
}

#[test]
fn check_scale_variants_are_comparable() {
    assert_eq!(StackBarScale::<i32>::FromZero, StackBarScale::FromZero);
    assert_eq!(StackBarScale::<i32>::FitData, StackBarScale::FitData);
    assert_eq!(StackBarScale::<i32>::Percentage, StackBarScale::Percentage);
    assert_eq!(StackBarScale::Fixed { min: -5, max: 10 }, StackBarScale::Fixed { min: -5, max: 10 });
    assert_eq!(
        StackBarScale::FromZeroMinRange { min: -1, max: 1 },
        StackBarScale::FromZeroMinRange { min: -1, max: 1 }
    );
    assert_ne!(StackBarScale::<i32>::FromZero, StackBarScale::FitData);
    assert_ne!(StackBarScale::<i32>::FitData, StackBarScale::Percentage);
    assert_ne!(StackBarScale::Fixed { min: 0, max: 10 }, StackBarScale::Fixed { min: 0, max: 11 });
    assert_ne!(StackBarScale::Fixed { min: 0, max: 10 }, StackBarScale::FromZeroMinRange { min: 0, max: 10 });
}

#[test]
fn check_scale_is_copy() {
    let scale = StackBarScale::Fixed { min: -2.5f64, max: 7.5 };
    let copy = scale;
    assert_eq!(scale, copy);
    let float_scale = StackBarScale::<f64>::Percentage;
    let float_copy = float_scale;
    assert_eq!(float_scale, float_copy);
}

#[test]
fn check_scale_keeps_range_values() {
    match (StackBarScale::Fixed { min: -3, max: 9 }) {
        StackBarScale::Fixed { min, max } => {
            assert_eq!(min, -3);
            assert_eq!(max, 9);
        }
        _ => panic!("expected a fixed scale"),
    }
    match (StackBarScale::FromZeroMinRange { min: -10, max: 20 }) {
        StackBarScale::FromZeroMinRange { min, max } => {
            assert_eq!(min, -10);
            assert_eq!(max, 20);
        }
        _ => panic!("expected a from-zero scale with a minimum range"),
    }
}

#[test]
fn check_stack_label_default_is_empty() {
    let stack = BarsStack::<i32>::new();
    assert_eq!(stack.label(), "");
    let from_values: BarsStack<i32> = [1, 2].into();
    assert_eq!(from_values.label(), "");
}

#[test]
fn check_stack_label_set_and_replace() {
    let mut stack = BarsStack::<i32>::new();
    stack.set_label("Q1").add_bar(1);
    assert_eq!(stack.label(), "Q1");
    stack.set_label("Second");
    assert_eq!(stack.label(), "Second");
    stack.set_label("");
    assert_eq!(stack.label(), "");
}

#[test]
fn check_stack_label_builder_and_clone() {
    let stack = BarsStackBuilder::<i32>::new().label("North").bars([1, 2]).build();
    assert_eq!(stack.label(), "North");
    let copy: BarsStack<i32> = (&stack).into();
    assert_eq!(copy.label(), "North");
}

#[test]
fn check_chart_keeps_stack_label() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack(BarsStackBuilder::<i32>::new().label("A").bars([1]).build());
        chart.update_stack(0, |stack| {
            stack.set_label("B");
        });
        assert_eq!(chart.get_stack(0).unwrap().label(), "B");
    });
}

#[test]
fn check_xaxis_label_format_height() {
    use super::XAxisLabelFormat;
    assert!(XAxisLabelFormat::None.is_none());
    assert!(!XAxisLabelFormat::Index(0).is_none());
    assert!(!XAxisLabelFormat::StackLabels.is_none());
    assert!(!XAxisLabelFormat::Custom.is_none());
    assert_eq!(XAxisLabelFormat::None.height(), 0);
    assert_eq!(XAxisLabelFormat::Index(5).height(), 2);
    assert_eq!(XAxisLabelFormat::StackLabels.height(), 2);
    assert_eq!(XAxisLabelFormat::Custom.height(), 2);
    assert!(XAxisLabelFormat::Index(1) == XAxisLabelFormat::Index(1));
    assert!(XAxisLabelFormat::Index(1) != XAxisLabelFormat::Index(2));
}

fn format_float(format: &FormatNumber, value: f64) -> Option<String> {
    let mut buffer = [0u8; 32];
    format.write_float(value, &mut buffer).map(|text| text.to_string())
}

#[test]
fn check_new_chart_defaults() {
    in_app(|| {
        let chart = new_chart();
        assert!(chart.scale == StackBarScale::FromZero);
        assert_eq!(chart.defaults.thickness, 1);
        assert_eq!(chart.defaults.spacing, 1);
        assert_eq!(chart.defaults.draw_mode, fill());
        assert!(chart.defaults.attr.is_none());
        assert_eq!(chart.yaxis.width, 6);
        assert_eq!(chart.yaxis.step, 3);
        assert!(chart.yaxis.visible);
        assert!(chart.yaxis.show_grid);
        assert!(chart.xaxis.label_format == XAxisLabelFormat::None);
        assert!(chart.xaxis.spans.is_empty());
    });
}

#[test]
fn check_set_stacks_scale() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::Fixed { min: -5, max: 20 });
        assert!(chart.scale == StackBarScale::Fixed { min: -5, max: 20 });
        chart.set_stacks_scale(StackBarScale::Percentage);
        assert!(chart.scale == StackBarScale::Percentage);
        chart.set_stacks_scale(StackBarScale::FitData);
        assert!(chart.scale == StackBarScale::FitData);
        chart.set_stacks_scale(StackBarScale::FromZeroMinRange { min: -1, max: 1 });
        assert!(chart.scale == StackBarScale::FromZeroMinRange { min: -1, max: 1 });
        chart.set_stacks_scale(StackBarScale::FromZero);
        assert!(chart.scale == StackBarScale::FromZero);
    });
}

#[test]
fn check_set_number_format() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_number_format(FormatNumber::new(10).decimals(3));
        let custom = FormatNumber::new(10).decimals(3);
        assert_eq!(format_float(&chart.number_format, 1.0), format_float(&custom, 1.0));
        assert_eq!(format_float(&chart.number_format, 2.5), format_float(&custom, 2.5));
        let default_chart = new_chart();
        assert_ne!(format_float(&default_chart.number_format, 1.0), format_float(&custom, 1.0));
    });
}

#[test]
fn check_default_number_format_depends_on_value_type() {
    in_app(|| {
        let int_chart = new_chart();
        let float_chart = VStackedBarChart::<f64>::new(layout!("x:1,y:1,w:10,h:5"), vstackedbarchart::Flags::None);
        assert_ne!(format_float(&int_chart.number_format, 1.0), format_float(&float_chart.number_format, 1.0));
    });
}

#[test]
fn check_set_default_bar_width() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_default_bar_width(4);
        assert_eq!(chart.defaults.thickness, 4);
        chart.set_default_bar_width(0);
        assert_eq!(chart.defaults.thickness, 1);
    });
}

#[test]
fn check_set_default_bar_width_refreshes_every_stack() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks([[1, 2], [3, 4]]);
        chart.add_stack(BarsStackBuilder::<i32>::new().bars([5]).thickness(2).build());
        chart.set_default_bar_width(5);
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 5);
        assert_eq!(chart.get_stack(1).unwrap().actual_thickness(), 5);
        assert_eq!(chart.get_stack(2).unwrap().actual_thickness(), 2);
        chart.set_default_bar_width(1);
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 1);
        assert_eq!(chart.get_stack(2).unwrap().actual_thickness(), 2);
    });
}

#[test]
fn check_set_default_bar_spacing() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_default_bar_spacing(3);
        assert_eq!(chart.defaults.spacing, 3);
        chart.set_default_bar_spacing(0);
        assert_eq!(chart.defaults.spacing, 0);
    });
}

#[test]
fn check_set_default_bar_drawmode() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_default_bar_drawmode(rect());
        assert_eq!(chart.defaults.draw_mode, rect());
        chart.set_default_bar_drawmode(filled_rect());
        assert_eq!(chart.defaults.draw_mode, filled_rect());
    });
}

#[test]
fn check_set_default_bar_drawmode_refreshes_every_stack() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks([[1, 2], [3, 4]]);
        let mut override_stack = BarsStack::<i32>::new();
        override_stack.add_bar(StackBarBuilder::new(1).draw_mode(fill()).build());
        chart.add_stack(override_stack);
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 1);
        chart.set_default_bar_drawmode(rect());
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 2);
        assert_eq!(chart.get_stack(1).unwrap().actual_thickness(), 2);
        assert_eq!(chart.get_stack(2).unwrap().actual_thickness(), 1);
        chart.set_default_bar_drawmode(filled_rect());
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 3);
        assert_eq!(chart.get_stack(1).unwrap().actual_thickness(), 3);
        assert_eq!(chart.get_stack(2).unwrap().actual_thickness(), 1);
        chart.set_default_bar_drawmode(fill());
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 1);
    });
}

#[test]
fn check_stacks_added_after_defaults_change_use_new_defaults() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_default_bar_width(3);
        chart.set_default_bar_drawmode(rect());
        chart.add_stack([1, 2]);
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 3);
        chart.set_default_bar_width(1);
        assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 2);
    });
}

#[test]
fn check_set_default_bar_attr() {
    in_app(|| {
        let mut chart = new_chart();
        assert!(chart.defaults.attr.is_none());
        chart.set_default_bar_attr(CharAttribute::with_fore_color(Color::Yellow));
        assert!(chart.defaults.attr.is_some());
    });
}

#[test]
fn check_set_yaxis_visible_and_grid() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_yaxis_visible(false);
        assert!(!chart.yaxis.visible);
        assert!(chart.yaxis.show_grid);
        chart.set_yaxis_show_grid(false);
        assert!(!chart.yaxis.show_grid);
        chart.set_yaxis_visible(true);
        chart.set_yaxis_show_grid(true);
        assert!(chart.yaxis.visible);
        assert!(chart.yaxis.show_grid);
    });
}

#[test]
fn check_set_yaxis_width() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_yaxis_width(9);
        assert_eq!(chart.yaxis.width, 9);
        chart.set_yaxis_width(0);
        assert_eq!(chart.yaxis.width, 1);
    });
}

#[test]
fn check_set_yaxis_step() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_yaxis_step(5);
        assert_eq!(chart.yaxis.step, 5);
        chart.set_yaxis_step(0);
        assert_eq!(chart.yaxis.step, 1);
    });
}

#[test]
fn check_set_xaxis_label_mode_simple_modes() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_xaxis_label_mode(XAxisLabelMode::Index(5));
        assert!(chart.xaxis.label_format == XAxisLabelFormat::Index(5));
        chart.set_xaxis_label_mode(XAxisLabelMode::StackLabels);
        assert!(chart.xaxis.label_format == XAxisLabelFormat::StackLabels);
        chart.set_xaxis_label_mode(XAxisLabelMode::None);
        assert!(chart.xaxis.label_format == XAxisLabelFormat::None);
    });
}

#[test]
fn check_set_xaxis_label_mode_custom_sorts_spans() {
    in_app(|| {
        let mut chart = new_chart();
        let spans = [BarSpan::new(6, 2, "C"), BarSpan::new(0, 3, "A"), BarSpan::new(3, 2, "B")];
        chart.set_xaxis_label_mode(XAxisLabelMode::Custom(&spans));
        assert!(chart.xaxis.label_format == XAxisLabelFormat::Custom);
        let starts: Vec<u32> = chart.xaxis.spans.iter().map(|span| span.start).collect();
        assert_eq!(starts, vec![0, 3, 6]);
    });
}

#[test]
fn check_set_xaxis_label_mode_custom_drops_overlapping_spans() {
    in_app(|| {
        let mut chart = new_chart();
        let spans = [
            BarSpan::new(6, 2, "C"),
            BarSpan::new(0, 3, "A"),
            BarSpan::new(2, 2, "B"),
            BarSpan::new(3, 1, "D"),
        ];
        chart.set_xaxis_label_mode(XAxisLabelMode::Custom(&spans));
        let ranges: Vec<(u32, u32)> = chart.xaxis.spans.iter().map(|span| (span.start, span.end)).collect();
        assert_eq!(ranges, vec![(0, 2), (3, 3), (6, 7)]);
    });
}

#[test]
fn check_set_xaxis_label_mode_custom_replaces_previous_spans() {
    in_app(|| {
        let mut chart = new_chart();
        let first = [BarSpan::new(0, 2, "A"), BarSpan::new(2, 2, "B")];
        chart.set_xaxis_label_mode(XAxisLabelMode::Custom(&first));
        assert_eq!(chart.xaxis.spans.len(), 2);
        let second = [BarSpan::new(1, 1, "X")];
        chart.set_xaxis_label_mode(XAxisLabelMode::Custom(&second));
        assert_eq!(chart.xaxis.spans.len(), 1);
        assert_eq!(chart.xaxis.spans[0].start, 1);
        chart.set_xaxis_label_mode(XAxisLabelMode::Custom(&[]));
        assert!(chart.xaxis.spans.is_empty());
        assert!(chart.xaxis.label_format == XAxisLabelFormat::Custom);
    });
}

#[test]
fn check_setters_do_not_change_stacks_data() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks([[1, -2], [3, 4]]);
        chart.set_stacks_scale(StackBarScale::Percentage);
        chart.set_default_bar_width(2);
        chart.set_default_bar_spacing(2);
        chart.set_yaxis_width(4);
        chart.set_xaxis_label_mode(XAxisLabelMode::Index(1));
        assert_eq!(chart.stacks_count(), 2);
        assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![1, -2]);
        assert_eq!(values_of(chart.get_stack(1).unwrap()), vec![3, 4]);
    });
}

#[test]
fn check_percentage_format_default() {
    in_app(|| {
        let chart = new_chart();
        let expected = FormatNumber::new(10).suffix("%");
        assert_eq!(format_float(&chart.percentage_format, 50.0), format_float(&expected, 50.0));
        let text = format_float(&chart.percentage_format, 50.0).unwrap();
        assert!(text.ends_with('%'));
        assert!(text.starts_with("50"));
    });
}

#[test]
fn check_percentage_format_has_no_decimals() {
    in_app(|| {
        let chart = new_chart();
        let text = format_float(&chart.percentage_format, 33.333).unwrap();
        assert!(!text.contains('.'));
        assert!(text.ends_with('%'));
        let float_chart = VStackedBarChart::<f64>::new(layout!("x:1,y:1,w:10,h:5"), vstackedbarchart::Flags::None);
        let float_text = format_float(&float_chart.percentage_format, 33.333).unwrap();
        assert!(!float_text.contains('.'));
        assert!(float_text.ends_with('%'));
    });
}

#[test]
fn check_set_percentage_format() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_percentage_format(FormatNumber::new(10).decimals(1).suffix(" pct"));
        let custom = FormatNumber::new(10).decimals(1).suffix(" pct");
        assert_eq!(format_float(&chart.percentage_format, 12.5), format_float(&custom, 12.5));
        let default_chart = new_chart();
        assert_ne!(format_float(&default_chart.percentage_format, 12.5), format_float(&chart.percentage_format, 12.5));
    });
}

#[test]
fn check_set_percentage_format_keeps_number_format() {
    in_app(|| {
        let mut chart = new_chart();
        let before = format_float(&chart.number_format, 1234.0);
        chart.set_percentage_format(FormatNumber::new(10).decimals(2).suffix("%"));
        assert_eq!(format_float(&chart.number_format, 1234.0), before);
    });
}

#[test]
fn check_yaxis_format_follows_scale() {
    in_app(|| {
        let mut chart = new_chart();
        let plain = format_float(chart.yaxis_format(), 50.0);
        assert_eq!(plain, format_float(&chart.number_format, 50.0));
        assert!(!plain.unwrap().ends_with('%'));

        chart.set_stacks_scale(StackBarScale::Percentage);
        let percent = format_float(chart.yaxis_format(), 50.0);
        assert_eq!(percent, format_float(&chart.percentage_format, 50.0));
        assert!(percent.unwrap().ends_with('%'));

        chart.set_stacks_scale(StackBarScale::FitData);
        assert_eq!(format_float(chart.yaxis_format(), 50.0), format_float(&chart.number_format, 50.0));
    });
}

#[test]
fn check_yaxis_format_uses_custom_formats() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_number_format(FormatNumber::new(10).decimals(3));
        chart.set_percentage_format(FormatNumber::new(10).decimals(1).suffix("%"));
        let number = format_float(chart.yaxis_format(), 2.0);
        assert_eq!(number, format_float(&FormatNumber::new(10).decimals(3), 2.0));
        chart.set_stacks_scale(StackBarScale::Percentage);
        let percent = format_float(chart.yaxis_format(), 2.0);
        assert_eq!(percent, format_float(&FormatNumber::new(10).decimals(1).suffix("%"), 2.0));
        assert_ne!(number, percent);
    });
}

#[test]
fn check_other_scales_use_number_format() {
    in_app(|| {
        let mut chart = new_chart();
        let expected = format_float(&chart.number_format, 7.0);
        chart.set_stacks_scale(StackBarScale::FromZero);
        assert_eq!(format_float(chart.yaxis_format(), 7.0), expected);
        chart.set_stacks_scale(StackBarScale::FromZeroMinRange { min: -1, max: 1 });
        assert_eq!(format_float(chart.yaxis_format(), 7.0), expected);
        chart.set_stacks_scale(StackBarScale::Fixed { min: 0, max: 10 });
        assert_eq!(format_float(chart.yaxis_format(), 7.0), expected);
    });
}

fn make_stack(thickness: Option<u8>, spacing: Option<u8>) -> BarsStack<i32> {
    let mut stack = BarsStack::<i32>::from([1, 2]);
    if let Some(value) = thickness {
        stack.set_thickness(value);
    }
    if let Some(value) = spacing {
        stack.set_spacing(value);
    }
    stack
}

fn positions(chart: &VStackedBarChart<i32>) -> Vec<i32> {
    chart.stacks.iter().map(|entry| entry.pos).collect()
}

#[test]
fn check_columns_layout_empty_chart() {
    in_app(|| {
        let chart = new_chart();
        assert_eq!(chart.stacks_width, 0);
        assert!(positions(&chart).is_empty());
    });
}

#[test]
fn check_columns_layout_single_stack() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1, 2]);
        assert_eq!(positions(&chart), vec![1]);
        assert_eq!(chart.stacks_width, 3);
    });
}

#[test]
fn check_columns_layout_default_spacing_and_thickness() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks([[1, 2], [3, 4], [5, 6]]);
        assert_eq!(positions(&chart), vec![1, 3, 5]);
        assert_eq!(chart.stacks_width, 7);
    });
}

#[test]
fn check_columns_layout_per_stack_thickness() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack(make_stack(Some(3), None));
        chart.add_stack(make_stack(Some(1), None));
        chart.add_stack(make_stack(Some(2), None));
        assert_eq!(positions(&chart), vec![1, 5, 7]);
        assert_eq!(chart.stacks_width, 10);
    });
}

#[test]
fn check_columns_layout_per_stack_spacing() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack(make_stack(None, Some(0)));
        chart.add_stack(make_stack(None, Some(2)));
        chart.add_stack(make_stack(None, Some(3)));
        assert_eq!(positions(&chart), vec![0, 3, 7]);
        assert_eq!(chart.stacks_width, 8);
    });
}

#[test]
fn check_columns_layout_trailing_margin_is_first_stack_spacing() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack(make_stack(None, Some(5)));
        chart.add_stack(make_stack(None, None));
        assert_eq!(positions(&chart), vec![5, 7]);
        assert_eq!(chart.stacks_width, 13);
    });
}

#[test]
fn check_columns_layout_default_spacing_change() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks([[1, 2], [3, 4], [5, 6]]);
        chart.set_default_bar_spacing(2);
        assert_eq!(positions(&chart), vec![2, 5, 8]);
        assert_eq!(chart.stacks_width, 11);
        chart.set_default_bar_spacing(0);
        assert_eq!(positions(&chart), vec![0, 1, 2]);
        assert_eq!(chart.stacks_width, 3);
    });
}

#[test]
fn check_columns_layout_default_width_change() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks([[1, 2], [3, 4], [5, 6]]);
        chart.set_default_bar_width(3);
        assert_eq!(positions(&chart), vec![1, 5, 9]);
        assert_eq!(chart.stacks_width, 13);
        chart.set_default_bar_width(1);
        assert_eq!(positions(&chart), vec![1, 3, 5]);
        assert_eq!(chart.stacks_width, 7);
    });
}

#[test]
fn check_columns_layout_default_drawmode_raises_thickness() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks([[1, 2], [3, 4], [5, 6]]);
        chart.set_default_bar_drawmode(rect());
        assert_eq!(positions(&chart), vec![1, 4, 7]);
        assert_eq!(chart.stacks_width, 10);
        chart.set_default_bar_drawmode(fill());
        assert_eq!(positions(&chart), vec![1, 3, 5]);
    });
}

#[test]
fn check_columns_layout_stack_with_wider_draw_mode() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1, 2]);
        let mut wide = BarsStack::<i32>::new();
        wide.add_bar(StackBarBuilder::new(1).draw_mode(filled_rect()).build());
        chart.add_stack(wide);
        chart.add_stack([3]);
        assert_eq!(positions(&chart), vec![1, 3, 7]);
        assert_eq!(chart.stacks_width, 9);
    });
}

#[test]
fn check_columns_layout_update_stack_moves_following_columns() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks([[1, 2], [3, 4], [5, 6]]);
        chart.update_stack(0, |stack| {
            stack.set_thickness(4);
        });
        assert_eq!(positions(&chart), vec![1, 6, 8]);
        assert_eq!(chart.stacks_width, 10);
        chart.update_stack(1, |stack| {
            stack.set_spacing(0);
        });
        assert_eq!(positions(&chart), vec![1, 5, 7]);
        assert_eq!(chart.stacks_width, 9);
    });
}

#[test]
fn check_columns_layout_update_stack_draw_mode_moves_following_columns() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks([[1, 2], [3, 4]]);
        chart.update_stack(0, |stack| {
            stack.get_mut(0).unwrap().set_draw_mode(filled_rect());
        });
        assert_eq!(positions(&chart), vec![1, 5]);
        assert_eq!(chart.stacks_width, 7);
    });
}

#[test]
fn check_columns_layout_add_after_add() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1, 2]);
        chart.add_stack([3, 4]);
        assert_eq!(positions(&chart), vec![1, 3]);
        chart.add_stack([5, 6]);
        assert_eq!(positions(&chart), vec![1, 3, 5]);
        assert_eq!(chart.stacks_width, 7);
    });
}

#[test]
fn check_columns_layout_clear_resets_width() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks([[1, 2], [3, 4]]);
        assert!(chart.stacks_width > 0);
        chart.clear();
        assert_eq!(chart.stacks_width, 0);
        assert!(positions(&chart).is_empty());
        chart.add_stack([1]);
        assert_eq!(positions(&chart), vec![1]);
        assert_eq!(chart.stacks_width, 3);
    });
}

#[test]
fn check_columns_layout_empty_stack_still_takes_space() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack(BarsStack::<i32>::new());
        chart.add_stack(BarsStack::<i32>::new());
        assert_eq!(positions(&chart), vec![1, 3]);
        assert_eq!(chart.stacks_width, 5);
    });
}

#[test]
fn check_columns_layout_columns_never_overlap() {
    in_app(|| {
        let mut chart = new_chart();
        for index in 0..40u8 {
            chart.add_stack(make_stack(Some(1 + index % 5), Some(index % 4)));
        }
        let mut previous_end = i32::MIN;
        for (index, entry) in chart.stacks.iter().enumerate() {
            let spacing = entry.stack.actual_spacing(chart.defaults.spacing) as i32;
            assert!(entry.pos >= previous_end + spacing, "stack {} overlaps the previous one", index);
            previous_end = entry.pos + entry.stack.actual_thickness() as i32;
        }
        let first_spacing = chart.stacks[0].stack.actual_spacing(chart.defaults.spacing) as u32;
        assert_eq!(chart.stacks_width, previous_end as u32 + first_spacing);
    });
}

#[test]
fn check_columns_layout_many_stacks() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_default_bar_width(10);
        chart.set_default_bar_spacing(2);
        chart.add_stacks((0..1000).map(|_| [1, 2]));
        assert_eq!(chart.stacks_count(), 1000);
        assert_eq!(chart.stacks[0].pos, 2);
        assert_eq!(chart.stacks[999].pos, 999 * 12 + 2);
        assert_eq!(chart.stacks_width, 1000 * 12 + 2);
    });
}

fn sums_of(chart: &VStackedBarChart<i32>, index: usize) -> (f64, f64, f64) {
    let entry = &chart.stacks[index];
    (entry.positive_sum, entry.negative_sum, entry.abs_total())
}

fn float_chart() -> VStackedBarChart<f64> {
    VStackedBarChart::<f64>::new(layout!("x:1,y:1,w:10,h:5"), vstackedbarchart::Flags::None)
}

#[test]
fn check_sums_only_positive_values() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1, 2, 3]);
        assert_eq!(sums_of(&chart, 0), (6.0, 0.0, 6.0));
        assert!(chart.stacks[0].has_positive());
        assert!(!chart.stacks[0].has_negative());
    });
}

#[test]
fn check_sums_only_negative_values() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([-1, -2]);
        assert_eq!(sums_of(&chart, 0), (0.0, -3.0, 3.0));
        assert!(!chart.stacks[0].has_positive());
        assert!(chart.stacks[0].has_negative());
    });
}

#[test]
fn check_sums_mixed_values() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([5, -3, 2, -1]);
        assert_eq!(sums_of(&chart, 0), (7.0, -4.0, 11.0));
        assert!(chart.stacks[0].has_positive());
        assert!(chart.stacks[0].has_negative());
    });
}

#[test]
fn check_sums_zero_values() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([0, 0, 0]);
        assert_eq!(sums_of(&chart, 0), (0.0, 0.0, 0.0));
        assert!(!chart.stacks[0].has_positive());
        assert!(!chart.stacks[0].has_negative());
    });
}

#[test]
fn check_sums_empty_stack() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack(BarsStack::<i32>::new());
        assert_eq!(sums_of(&chart, 0), (0.0, 0.0, 0.0));
        assert!(!chart.stacks[0].has_positive());
        assert!(!chart.stacks[0].has_negative());
    });
}

#[test]
fn check_sums_do_not_depend_on_value_order() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1, -1, 1, -1]);
        chart.add_stack([1, 1, -1, -1]);
        chart.add_stack([-1, -1, 1, 1]);
        assert_eq!(sums_of(&chart, 0), (2.0, -2.0, 4.0));
        assert_eq!(sums_of(&chart, 1), (2.0, -2.0, 4.0));
        assert_eq!(sums_of(&chart, 2), (2.0, -2.0, 4.0));
    });
}

#[test]
fn check_sums_are_per_stack() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks(vec![vec![1, 2], vec![-3], vec![4, -5, 6], vec![]]);
        assert_eq!(sums_of(&chart, 0), (3.0, 0.0, 3.0));
        assert_eq!(sums_of(&chart, 1), (0.0, -3.0, 3.0));
        assert_eq!(sums_of(&chart, 2), (10.0, -5.0, 15.0));
        assert_eq!(sums_of(&chart, 3), (0.0, 0.0, 0.0));
    });
}

#[test]
fn check_sums_abs_total_is_positive_minus_negative() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([7, -2, 4, -9]);
        let entry = &chart.stacks[0];
        assert_eq!(entry.abs_total(), entry.positive_sum - entry.negative_sum);
        assert_eq!(entry.abs_total(), 22.0);
    });
}

#[test]
fn check_sums_large_values_do_not_overflow() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([i32::MAX, i32::MAX]);
        chart.add_stack([i32::MIN, i32::MIN]);
        assert_eq!(chart.stacks[0].positive_sum, 2.0 * i32::MAX as f64);
        assert_eq!(chart.stacks[1].negative_sum, 2.0 * i32::MIN as f64);
    });
}

#[test]
fn check_sums_float_values() {
    in_app(|| {
        let mut chart = float_chart();
        chart.add_stack(vec![1.5, -2.5, 0.25]);
        let entry = &chart.stacks[0];
        assert_eq!(entry.positive_sum, 1.75);
        assert_eq!(entry.negative_sum, -2.5);
        assert_eq!(entry.abs_total(), 4.25);
    });
}

#[test]
fn check_sums_ignore_non_finite_values() {
    in_app(|| {
        let mut chart = float_chart();
        chart.add_stack(vec![f64::NAN, 1.0, f64::INFINITY, f64::NEG_INFINITY, -2.0]);
        let entry = &chart.stacks[0];
        assert_eq!(entry.positive_sum, 1.0);
        assert_eq!(entry.negative_sum, -2.0);
        assert_eq!(entry.abs_total(), 3.0);
    });
}

#[test]
fn check_sums_only_non_finite_values() {
    in_app(|| {
        let mut chart = float_chart();
        chart.add_stack(vec![f64::NAN, f64::INFINITY]);
        let entry = &chart.stacks[0];
        assert_eq!(entry.positive_sum, 0.0);
        assert_eq!(entry.negative_sum, 0.0);
        assert!(!entry.has_positive());
        assert!(!entry.has_negative());
    });
}

#[test]
fn check_sums_after_update_stack() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1, 2, 3]);
        assert_eq!(sums_of(&chart, 0), (6.0, 0.0, 6.0));
        chart.update_stack(0, |stack| {
            stack.get_mut(0).unwrap().set_value(-10);
        });
        assert_eq!(sums_of(&chart, 0), (5.0, -10.0, 15.0));
        chart.update_stack(0, |stack| {
            stack.add_bar(4);
            stack.add_bar(-1);
        });
        assert_eq!(sums_of(&chart, 0), (9.0, -11.0, 20.0));
    });
}

#[test]
fn check_sums_after_update_only_touch_that_stack_values() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks([[1, 2], [3, 4]]);
        chart.update_stack(1, |stack| {
            stack.get_mut(0).unwrap().set_value(-3);
        });
        assert_eq!(sums_of(&chart, 0), (3.0, 0.0, 3.0));
        assert_eq!(sums_of(&chart, 1), (4.0, -3.0, 7.0));
    });
}

#[test]
fn check_sums_survive_setters() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([2, -3]);
        chart.set_default_bar_width(4);
        chart.set_default_bar_spacing(0);
        chart.set_default_bar_drawmode(rect());
        chart.set_stacks_scale(StackBarScale::Percentage);
        assert_eq!(sums_of(&chart, 0), (2.0, -3.0, 5.0));
    });
}

#[test]
fn check_sums_after_clear_and_add() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([5, -5]);
        chart.clear();
        assert!(chart.stacks.is_empty());
        chart.add_stack([1]);
        assert_eq!(sums_of(&chart, 0), (1.0, 0.0, 1.0));
    });
}

fn range_of(chart: &VStackedBarChart<i32>) -> (f64, f64) {
    chart.compute_range()
}

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-9, "expected {} but got {}", expected, actual);
}

#[test]
fn check_range_from_zero_empty_chart() {
    in_app(|| {
        let chart = new_chart();
        assert_eq!(range_of(&chart), (0.0, 0.0));
    });
}

#[test]
fn check_range_from_zero_only_positive() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks(vec![vec![1, 2, 3], vec![4]]);
        assert_eq!(range_of(&chart), (0.0, 6.0));
    });
}

#[test]
fn check_range_from_zero_only_negative() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks(vec![vec![-1, -2], vec![-5]]);
        assert_eq!(range_of(&chart), (-5.0, 0.0));
    });
}

#[test]
fn check_range_from_zero_mixed_signs() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks(vec![vec![5, -3], vec![2, -8]]);
        assert_eq!(range_of(&chart), (-8.0, 5.0));
    });
}

#[test]
fn check_range_from_zero_uses_stack_sums_not_single_values() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([4, 4, 4, -1, -1]);
        assert_eq!(range_of(&chart), (-2.0, 12.0));
    });
}

#[test]
fn check_range_from_zero_all_zero_values() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([0, 0]);
        assert_eq!(range_of(&chart), (0.0, 0.0));
    });
}

#[test]
fn check_range_from_zero_min_range_expands() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::FromZeroMinRange { min: -10, max: 20 });
        chart.add_stack([1, 2]);
        assert_eq!(range_of(&chart), (-10.0, 20.0));
    });
}

#[test]
fn check_range_from_zero_min_range_data_wins_when_larger() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::FromZeroMinRange { min: -1, max: 2 });
        chart.add_stacks(vec![vec![30, -4], vec![5, -6]]);
        assert_eq!(range_of(&chart), (-6.0, 30.0));
    });
}

#[test]
fn check_range_from_zero_min_range_always_includes_zero() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::FromZeroMinRange { min: 5, max: 8 });
        chart.add_stack([1]);
        assert_eq!(range_of(&chart), (0.0, 8.0));
        chart.set_stacks_scale(StackBarScale::FromZeroMinRange { min: -8, max: -5 });
        assert_eq!(range_of(&chart), (-8.0, 1.0));
    });
}

#[test]
fn check_range_from_zero_min_range_empty_chart() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::FromZeroMinRange { min: -3, max: 4 });
        assert_eq!(range_of(&chart), (-3.0, 4.0));
    });
}

#[test]
fn check_range_from_zero_min_range_ignores_non_finite_limits() {
    in_app(|| {
        let mut chart = float_chart();
        chart.set_stacks_scale(StackBarScale::FromZeroMinRange { min: f64::NAN, max: f64::INFINITY });
        chart.add_stack(vec![2.0, -1.0]);
        assert_eq!(chart.compute_range(), (-1.0, 2.0));
    });
}

#[test]
fn check_range_fit_data_only_positive() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::FitData);
        chart.add_stacks(vec![vec![1, 2], vec![4], vec![10]]);
        assert_eq!(range_of(&chart), (3.0, 10.0));
    });
}

#[test]
fn check_range_fit_data_only_negative() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::FitData);
        chart.add_stacks(vec![vec![-1, -2], vec![-5]]);
        assert_eq!(range_of(&chart), (-5.0, -3.0));
    });
}

#[test]
fn check_range_fit_data_mixed_signs_includes_zero() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::FitData);
        chart.add_stacks(vec![vec![10, -4], vec![3]]);
        assert_eq!(range_of(&chart), (-4.0, 10.0));
    });
}

#[test]
fn check_range_fit_data_zero_is_not_forced_into_the_range() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::FitData);
        chart.add_stacks(vec![vec![50], vec![80]]);
        let (lo, hi) = range_of(&chart);
        assert!(lo > 0.0);
        assert_eq!((lo, hi), (50.0, 80.0));
    });
}

#[test]
fn check_range_fit_data_single_stack_is_degenerate() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::FitData);
        chart.add_stack([2, 3]);
        assert_eq!(range_of(&chart), (5.0, 5.0));
    });
}

#[test]
fn check_range_fit_data_without_tips() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::FitData);
        assert_eq!(range_of(&chart), (0.0, 0.0));
        chart.add_stacks(vec![vec![0, 0], vec![]]);
        assert_eq!(range_of(&chart), (0.0, 0.0));
    });
}

#[test]
fn check_range_fit_data_zero_values_do_not_make_a_tip() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::FitData);
        chart.add_stacks(vec![vec![0, 5], vec![0, 0], vec![7]]);
        assert_eq!(range_of(&chart), (5.0, 7.0));
    });
}

#[test]
fn check_range_fixed_passthrough() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::Fixed { min: -5, max: 20 });
        assert_eq!(range_of(&chart), (-5.0, 20.0));
        chart.add_stack([100, -100]);
        assert_eq!(range_of(&chart), (-5.0, 20.0));
    });
}

#[test]
fn check_range_fixed_reversed_or_equal_is_returned_unchanged() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::Fixed { min: 10, max: 2 });
        assert_eq!(range_of(&chart), (10.0, 2.0));
        chart.set_stacks_scale(StackBarScale::Fixed { min: 4, max: 4 });
        assert_eq!(range_of(&chart), (4.0, 4.0));
    });
}

#[test]
fn check_range_fixed_non_finite_limits_are_degenerate() {
    in_app(|| {
        let mut chart = float_chart();
        chart.set_stacks_scale(StackBarScale::Fixed { min: f64::NAN, max: 5.0 });
        assert_eq!(chart.compute_range(), (0.0, 0.0));
        chart.set_stacks_scale(StackBarScale::Fixed { min: 0.0, max: f64::INFINITY });
        assert_eq!(chart.compute_range(), (0.0, 0.0));
    });
}

#[test]
fn check_range_percentage_only_positive() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::Percentage);
        chart.add_stacks(vec![vec![1, 3], vec![5]]);
        assert_eq!(range_of(&chart), (0.0, 100.0));
    });
}

#[test]
fn check_range_percentage_only_negative() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::Percentage);
        chart.add_stack([-2, -2]);
        assert_eq!(range_of(&chart), (-100.0, 0.0));
    });
}

#[test]
fn check_range_percentage_mixed_signs() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::Percentage);
        chart.add_stacks(vec![vec![3, -1], vec![1, -1]]);
        assert_eq!(range_of(&chart), (-50.0, 75.0));
    });
}

#[test]
fn check_range_percentage_each_stack_is_normalized_separately() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::Percentage);
        chart.add_stacks(vec![vec![1000, -1000], vec![1, -3]]);
        assert_eq!(range_of(&chart), (-75.0, 50.0));
    });
}

#[test]
fn check_range_percentage_ignores_empty_and_zero_stacks() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::Percentage);
        chart.add_stacks(vec![vec![], vec![0, 0], vec![2, -2]]);
        assert_eq!(range_of(&chart), (-50.0, 50.0));
    });
}

#[test]
fn check_range_percentage_without_values() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::Percentage);
        assert_eq!(range_of(&chart), (0.0, 0.0));
        chart.add_stack(BarsStack::<i32>::new());
        assert_eq!(range_of(&chart), (0.0, 0.0));
    });
}

#[test]
fn check_range_percentage_with_fractions() {
    in_app(|| {
        let mut chart = new_chart();
        chart.set_stacks_scale(StackBarScale::Percentage);
        chart.add_stack([1, 1, 1, -1]);
        let (lo, hi) = range_of(&chart);
        assert_close(hi, 75.0);
        assert_close(lo, -25.0);
    });
}

#[test]
fn check_range_percentage_parts_of_one_stack_add_up_to_100() {
    in_app(|| {
        let mut chart = float_chart();
        chart.set_stacks_scale(StackBarScale::Percentage);
        chart.add_stack(vec![1.0, 2.0, -4.0, 0.5]);
        let (lo, hi) = chart.compute_range();
        assert_close(hi - lo, 100.0);
    });
}

#[test]
fn check_range_follows_scale_changes() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stacks(vec![vec![10, -4], vec![3]]);
        assert_eq!(range_of(&chart), (-4.0, 10.0));
        chart.set_stacks_scale(StackBarScale::Fixed { min: 0, max: 50 });
        assert_eq!(range_of(&chart), (0.0, 50.0));
        chart.set_stacks_scale(StackBarScale::Percentage);
        let (lo, hi) = range_of(&chart);
        assert_close(lo, -4.0 / 14.0 * 100.0);
        assert_close(hi, 100.0);
    });
}

#[test]
fn check_mapping_symmetric_range() {
    let m = ValueMapping::new(-10.0, 10.0, 20);
    assert_eq!(m.height(), 20);
    assert_eq!(m.zero(), 10);
    assert_eq!(m.above(), 10);
    assert_eq!(m.below(), 10);
    assert_eq!(m.offset(0.0), 0);
    assert_eq!(m.offset(10.0), 10);
    assert_eq!(m.offset(-10.0), -10);
    assert_eq!(m.offset(5.0), 5);
    assert_eq!(m.offset(-5.0), -5);
}

#[test]
fn check_mapping_only_positive_range() {
    let m = ValueMapping::new(0.0, 10.0, 10);
    assert_eq!(m.zero(), 0);
    assert_eq!(m.below(), 0);
    assert_eq!(m.above(), 10);
    assert_eq!(m.offset(3.0), 3);
    assert_eq!(m.offset(10.0), 10);
}

#[test]
fn check_mapping_only_negative_range() {
    let m = ValueMapping::new(-10.0, 0.0, 10);
    assert_eq!(m.zero(), 10);
    assert_eq!(m.above(), 0);
    assert_eq!(m.below(), 10);
    assert_eq!(m.offset(-4.0), -4);
    assert_eq!(m.offset(-10.0), -10);
    assert_eq!(m.offset(0.0), 0);
}

#[test]
fn check_mapping_scales_to_height() {
    let m = ValueMapping::new(0.0, 100.0, 5);
    assert_eq!(m.offset(20.0), 1);
    assert_eq!(m.offset(50.0), 3);
    assert_eq!(m.offset(100.0), 5);
    let m = ValueMapping::new(0.0, 1.0, 10);
    assert_eq!(m.offset(0.04), 0);
    assert_eq!(m.offset(0.06), 1);
}

#[test]
fn check_mapping_clamps_values_outside_range() {
    let m = ValueMapping::new(-10.0, 10.0, 20);
    assert_eq!(m.offset(1000.0), 10);
    assert_eq!(m.offset(-1000.0), -10);
    assert_eq!(m.offset(f64::INFINITY), 10);
    assert_eq!(m.offset(f64::NEG_INFINITY), -10);
}

#[test]
fn check_mapping_nan_maps_to_zero() {
    let m = ValueMapping::new(-10.0, 10.0, 20);
    assert_eq!(m.offset(f64::NAN), 0);
    assert_eq!(m.position(f64::NAN), m.zero());
}

#[test]
fn check_mapping_zero_below_visible_range() {
    let m = ValueMapping::new(2.0, 10.0, 8);
    assert_eq!(m.zero(), 0);
    assert_eq!(m.offset(2.0), 0);
    assert_eq!(m.offset(1.0), 0);
    assert_eq!(m.offset(0.0), 0);
    assert_eq!(m.offset(6.0), 4);
    assert_eq!(m.offset(10.0), 8);
}

#[test]
fn check_mapping_zero_above_visible_range() {
    let m = ValueMapping::new(-10.0, -2.0, 8);
    assert_eq!(m.zero(), 8);
    assert_eq!(m.above(), 0);
    assert_eq!(m.offset(0.0), 0);
    assert_eq!(m.offset(-2.0), 0);
    assert_eq!(m.offset(-6.0), -4);
    assert_eq!(m.offset(-10.0), -8);
}

#[test]
fn check_mapping_zero_height() {
    let m = ValueMapping::new(-10.0, 10.0, 0);
    assert_eq!(m.height(), 0);
    assert_eq!(m.zero(), 0);
    assert_eq!(m.above(), 0);
    assert_eq!(m.below(), 0);
    assert_eq!(m.offset(5.0), 0);
    assert_eq!(m.offset(-5.0), 0);
}

#[test]
fn check_mapping_degenerate_range() {
    for (lo, hi) in [(0.0, 0.0), (5.0, 5.0), (10.0, -10.0), (f64::NAN, f64::NAN), (0.0, f64::INFINITY), (f64::NEG_INFINITY, f64::INFINITY)] {
        let m = ValueMapping::new(lo, hi, 10);
        assert_eq!(m.zero(), 0);
        assert_eq!(m.above(), 10);
        assert_eq!(m.offset(7.0), 0);
        assert_eq!(m.offset(-7.0), 0);
        assert_eq!(m.offset(f64::INFINITY), 0);
    }
}

#[test]
fn check_mapping_offset_is_monotonic() {
    let m = ValueMapping::new(-7.5, 13.25, 17);
    let mut previous = m.offset(-100.0);
    let mut v = -100.0;
    while v <= 100.0 {
        let current = m.offset(v);
        assert!(current >= previous);
        assert!(current >= -(m.below() as i32) && current <= m.above() as i32);
        previous = current;
        v += 0.37;
    }
}

#[test]
fn check_mapping_zero_row_is_clamped_to_plot() {
    for (lo, hi) in [(-3.0, 9.0), (0.0, 4.0), (-4.0, 0.0), (1.0, 5.0), (-5.0, -1.0), (-1e9, 1.0)] {
        for height in 0..30u32 {
            let m = ValueMapping::new(lo, hi, height);
            assert!(m.zero() <= m.height());
            assert_eq!(m.below() + m.above(), m.height());
        }
    }
}

#[test]
fn check_chart_value_mapping_from_zero() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack(BarsStack::from([4, -2]));
        chart.add_stack(BarsStack::from([6]));
        let m = chart.value_mapping(8);
        assert_eq!(m.zero(), 2);
        assert_eq!(m.offset(6.0), 6);
        assert_eq!(m.offset(-2.0), -2);
    });
}

#[test]
fn check_chart_value_mapping_fixed_clamps() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack(BarsStack::from([100]));
        chart.set_stacks_scale(StackBarScale::Fixed { min: -5, max: 5 });
        let m = chart.value_mapping(10);
        assert_eq!(m.zero(), 5);
        assert_eq!(m.offset(100.0), 5);
    });
}

#[test]
fn check_chart_value_mapping_empty_chart() {
    in_app(|| {
        let chart = new_chart();
        let m = chart.value_mapping(10);
        assert_eq!(m.zero(), 0);
        assert_eq!(m.offset(3.0), 0);
    });
}

fn segments_of<T: Number + 'static>(chart: &mut VStackedBarChart<T>, height: u32, index: usize) -> Vec<(i16, i16)> {
    chart.update_segments_layout(height);
    chart.stacks[index].layout.iter().map(|s| (s.offset, s.len)).collect()
}

#[test]
fn check_segments_single_positive_bar() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([3]);
        assert_eq!(segments_of(&mut chart, 3, 0), vec![(0, 3)]);
    });
}

#[test]
fn check_segments_positive_bars_stack_upward() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([2, 3]);
        assert_eq!(segments_of(&mut chart, 5, 0), vec![(0, 2), (2, 3)]);
    });
}

#[test]
fn check_segments_negative_bars_stack_downward() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([-2, -3]);
        assert_eq!(segments_of(&mut chart, 5, 0), vec![(0, -2), (-2, -3)]);
    });
}

#[test]
fn check_segments_mixed_signs_keep_insertion_order() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([4, -2, 3, -1]);
        assert_eq!(segments_of(&mut chart, 10, 0), vec![(0, 4), (0, -2), (4, 3), (-2, -1)]);
    });
}

#[test]
fn check_segments_zero_value_has_empty_segment() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([0, 2]);
        assert_eq!(segments_of(&mut chart, 2, 0), vec![(0, 0), (0, 2)]);
    });
}

#[test]
fn check_segments_tiny_bars_vanish() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1000]);
        chart.add_stack([1, 1, 1, 1000]);
        assert_eq!(segments_of(&mut chart, 10, 1), vec![(0, 0), (0, 0), (0, 0), (0, 10)]);
    });
}

#[test]
fn check_segments_rounding_sums_exactly() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1, 1, 1]);
        let segs = segments_of(&mut chart, 2, 0);
        assert_eq!(segs, vec![(0, 1), (1, 0), (1, 1)]);
        assert_eq!(segs.iter().map(|s| s.1 as i32).sum::<i32>(), 2);
    });
}

#[test]
fn check_segments_are_contiguous() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([7, 3, 11, 2, 5, 13, 1]);
        chart.add_stack([-7, -3, -11, -2, -5]);
        for height in 1..40u32 {
            for index in 0..2 {
                let segs = segments_of(&mut chart, height, index);
                let mut expected = 0i32;
                for (offset, len) in segs {
                    if len == 0 {
                        continue;
                    }
                    let start = offset as i32;
                    assert_eq!(start, expected);
                    expected = start + len as i32;
                }
            }
        }
    });
}

#[test]
fn check_segments_total_matches_tip_position() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([3, 4, 5]);
        chart.add_stack([9]);
        for height in 1..30u32 {
            let mapping = chart.value_mapping(height);
            let segs = segments_of(&mut chart, height, 0);
            let sum: i32 = segs.iter().map(|s| s.1 as i32).sum();
            assert_eq!(sum, mapping.offset(12.0));
        }
    });
}

#[test]
fn check_segments_fixed_scale_clamps() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([100]);
        chart.add_stack([-100]);
        chart.add_stack([3, 100]);
        chart.set_stacks_scale(StackBarScale::Fixed { min: -5, max: 5 });
        assert_eq!(segments_of(&mut chart, 10, 0), vec![(0, 5)]);
        assert_eq!(segments_of(&mut chart, 10, 1), vec![(0, -5)]);
        assert_eq!(segments_of(&mut chart, 10, 2), vec![(0, 3), (3, 2)]);
    });
}

#[test]
fn check_segments_fixed_scale_zero_not_visible() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([2, 3, 20]);
        chart.set_stacks_scale(StackBarScale::Fixed { min: 2, max: 10 });
        assert_eq!(segments_of(&mut chart, 8, 0), vec![(0, 0), (0, 3), (3, 5)]);
    });
}

#[test]
fn check_segments_fit_data_smallest_stack_is_empty() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([5]);
        chart.add_stack([10]);
        chart.set_stacks_scale(StackBarScale::FitData);
        assert_eq!(segments_of(&mut chart, 5, 0), vec![(0, 0)]);
        assert_eq!(segments_of(&mut chart, 5, 1), vec![(0, 5)]);
    });
}

#[test]
fn check_segments_percentage_positive_only() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1, 1]);
        chart.add_stack([3]);
        chart.set_stacks_scale(StackBarScale::Percentage);
        assert_eq!(segments_of(&mut chart, 10, 0), vec![(0, 5), (5, 5)]);
        assert_eq!(segments_of(&mut chart, 10, 1), vec![(0, 10)]);
    });
}

#[test]
fn check_segments_percentage_with_negatives() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([5, -5]);
        chart.set_stacks_scale(StackBarScale::Percentage);
        assert_eq!(segments_of(&mut chart, 10, 0), vec![(0, 5), (0, -5)]);
    });
}

#[test]
fn check_segments_percentage_all_zero_stack_is_empty() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([0, 0]);
        chart.add_stack([2]);
        chart.set_stacks_scale(StackBarScale::Percentage);
        assert_eq!(segments_of(&mut chart, 10, 0), vec![(0, 0), (0, 0)]);
        assert_eq!(segments_of(&mut chart, 10, 1), vec![(0, 10)]);
    });
}

#[test]
fn check_segments_ignore_non_finite_values() {
    in_app(|| {
        let mut chart = float_chart();
        chart.add_stack([f64::NAN, 2.0, f64::INFINITY]);
        assert_eq!(segments_of(&mut chart, 2, 0), vec![(0, 0), (0, 2), (0, 0)]);
    });
}

#[test]
fn check_segments_zero_height() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([3, -2]);
        assert_eq!(segments_of(&mut chart, 0, 0), vec![(0, 0), (0, 0)]);
    });
}

#[test]
fn check_segments_empty_stack_and_empty_chart() {
    in_app(|| {
        let mut chart = new_chart();
        chart.update_segments_layout(10);
        chart.add_stack(BarsStack::<i32>::new());
        assert!(segments_of(&mut chart, 10, 0).is_empty());
    });
}

#[test]
fn check_segments_recomputed_on_each_call() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([4]);
        assert_eq!(segments_of(&mut chart, 4, 0), vec![(0, 4)]);
        assert_eq!(segments_of(&mut chart, 8, 0), vec![(0, 8)]);
        chart.update_stack(0, |s| {
            s.add_bar(4);
        });
        assert_eq!(segments_of(&mut chart, 8, 0), vec![(0, 4), (4, 4)]);
    });
}

#[test]
fn check_segments_huge_height_does_not_overflow() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1]);
        assert_eq!(segments_of(&mut chart, 100000, 0), vec![(0, i16::MAX)]);
    });
}

fn yaxis_of<T: Number + 'static>(chart: &mut VStackedBarChart<T>, height: u32) -> (i32, f64, f64) {
    chart.update_yaxis_data(height);
    (chart.yaxis.zero, chart.yaxis.bottom_value, chart.yaxis.bottom_step)
}

#[test]
fn check_yaxis_defaults() {
    in_app(|| {
        let chart = new_chart();
        assert_eq!(chart.yaxis.zero, 0);
        assert_eq!(chart.yaxis.bottom_value, 0.0);
        assert_eq!(chart.yaxis.bottom_step, 0.0);
    });
}

#[test]
fn check_yaxis_from_zero_positive_only() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([10]);
        let (zero, bottom, step) = yaxis_of(&mut chart, 10);
        assert_eq!(zero, 0);
        assert_close(bottom, 0.0);
        assert_close(step, -3.0);
    });
}

#[test]
fn check_yaxis_from_zero_mixed_values() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([10, -10]);
        let (zero, bottom, step) = yaxis_of(&mut chart, 20);
        assert_eq!(zero, 10);
        assert_close(bottom, -10.0);
        assert_close(step, -3.0);
    });
}

#[test]
fn check_yaxis_from_zero_negative_only() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([-8]);
        let (zero, bottom, step) = yaxis_of(&mut chart, 8);
        assert_eq!(zero, 8);
        assert_close(bottom, -8.0);
        assert_close(step, -3.0);
    });
}

#[test]
fn check_yaxis_step_follows_setting() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([10]);
        chart.set_yaxis_step(5);
        let (_, _, step) = yaxis_of(&mut chart, 10);
        assert_close(step, -5.0);
        chart.set_yaxis_step(1);
        let (_, _, step) = yaxis_of(&mut chart, 20);
        assert_close(step, -0.5);
    });
}

#[test]
fn check_yaxis_fit_data() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([5]);
        chart.add_stack([10]);
        chart.set_stacks_scale(StackBarScale::FitData);
        let (zero, bottom, step) = yaxis_of(&mut chart, 5);
        assert_eq!(zero, 0);
        assert_close(bottom, 5.0);
        assert_close(step, -3.0);
    });
}

#[test]
fn check_yaxis_fixed() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1]);
        chart.set_stacks_scale(StackBarScale::Fixed { min: -20, max: 40 });
        let (zero, bottom, step) = yaxis_of(&mut chart, 30);
        assert_eq!(zero, 10);
        assert_close(bottom, -20.0);
        assert_close(step, -6.0);
    });
}

#[test]
fn check_yaxis_percentage() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([1, 1]);
        chart.set_stacks_scale(StackBarScale::Percentage);
        let (zero, bottom, step) = yaxis_of(&mut chart, 10);
        assert_eq!(zero, 0);
        assert_close(bottom, 0.0);
        assert_close(step, -30.0);
        chart.add_stack([5, -5]);
        let (zero, bottom, step) = yaxis_of(&mut chart, 15);
        assert_eq!(zero, 5);
        assert_close(bottom, -50.0);
        assert_close(step, -30.0);
    });
}

#[test]
fn check_yaxis_zero_matches_mapping_zero() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([7, -3]);
        chart.add_stack([-9, 2]);
        for height in 0..40u32 {
            let mapping = chart.value_mapping(height);
            let (zero, _, _) = yaxis_of(&mut chart, height);
            assert_eq!(zero, mapping.zero() as i32);
        }
    });
}

#[test]
fn check_yaxis_zero_line_row_from_bottom_value() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([12, -6]);
        let height = 18u32;
        let (zero, bottom, step) = yaxis_of(&mut chart, height);
        let rows_to_zero = (bottom * chart.yaxis.step as f64 / step).round() as i32;
        assert_eq!(rows_to_zero, zero);
    });
}

#[test]
fn check_yaxis_empty_chart_and_zero_height() {
    in_app(|| {
        let mut chart = new_chart();
        assert_eq!(yaxis_of(&mut chart, 10), (0, 0.0, 0.0));
        chart.add_stack([3, -3]);
        assert_eq!(yaxis_of(&mut chart, 0), (0, -3.0, 0.0));
    });
}

#[test]
fn check_yaxis_degenerate_fixed_range() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([3]);
        chart.set_stacks_scale(StackBarScale::Fixed { min: 5, max: 5 });
        let (zero, bottom, step) = yaxis_of(&mut chart, 10);
        assert_eq!(zero, 0);
        assert_close(bottom, 5.0);
        assert_close(step, 0.0);
    });
}

#[test]
fn check_plot_layout_updates_axis_and_segments() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([4, -2, 3]);
        chart.update_plot_layout(9);
        assert_eq!(chart.yaxis.zero, 2);
        assert_close(chart.yaxis.bottom_value, -2.0);
        let segs: Vec<(i16, i16)> = chart.stacks[0].layout.iter().map(|s| (s.offset, s.len)).collect();
        assert_eq!(segs, vec![(0, 4), (0, -2), (4, 3)]);
    });
}

#[test]
fn check_plot_layout_empty_chart() {
    in_app(|| {
        let mut chart = new_chart();
        chart.update_plot_layout(10);
        chart.update_plot_layout(0);
        assert_eq!(chart.stacks_count(), 0);
        assert_eq!(chart.yaxis.zero, 0);
    });
}

#[test]
fn check_plot_layout_after_clear() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([4, -2]);
        chart.update_plot_layout(10);
        chart.clear();
        chart.update_plot_layout(10);
        assert_eq!(chart.yaxis.zero, 0);
        assert_close(chart.yaxis.bottom_step, 0.0);
    });
}

#[test]
fn check_plot_layout_single_stack_all_scales() {
    in_app(|| {
        let scales = [
            StackBarScale::FromZero,
            StackBarScale::FromZeroMinRange { min: -5, max: 5 },
            StackBarScale::FitData,
            StackBarScale::Fixed { min: -3, max: 3 },
            StackBarScale::Percentage,
        ];
        for scale in scales {
            for height in 0..12u32 {
                let mut chart = new_chart();
                chart.add_stack([2, -1, 3]);
                chart.set_stacks_scale(scale);
                chart.update_plot_layout(height);
                assert_eq!(chart.stacks[0].layout.len(), 3);
                assert!(chart.yaxis.zero >= 0 && chart.yaxis.zero <= height as i32);
            }
        }
    });
}

#[test]
fn check_plot_layout_all_zero_stack_all_scales() {
    in_app(|| {
        let scales = [
            StackBarScale::FromZero,
            StackBarScale::FromZeroMinRange { min: 0, max: 0 },
            StackBarScale::FitData,
            StackBarScale::Fixed { min: 0, max: 0 },
            StackBarScale::Percentage,
        ];
        for scale in scales {
            for height in 0..6u32 {
                let mut chart = new_chart();
                chart.add_stack([0, 0, 0]);
                chart.set_stacks_scale(scale);
                chart.update_plot_layout(height);
                assert!(chart.stacks[0].layout.iter().all(|s| s.len == 0));
            }
        }
    });
}

#[test]
fn check_plot_layout_stack_without_bars_all_scales() {
    in_app(|| {
        let scales = [
            StackBarScale::FromZero,
            StackBarScale::FitData,
            StackBarScale::Fixed { min: -2, max: 2 },
            StackBarScale::Percentage,
        ];
        for scale in scales {
            let mut chart = new_chart();
            chart.add_stack(BarsStack::<i32>::new());
            chart.add_stack([3]);
            chart.set_stacks_scale(scale);
            chart.update_plot_layout(6);
            assert!(chart.stacks[0].layout.is_empty());
            assert_eq!(chart.stacks[1].layout.len(), 1);
        }
    });
}

#[test]
fn check_plot_layout_extreme_values() {
    in_app(|| {
        let mut chart = float_chart();
        chart.add_stack([f64::MAX, -f64::MAX, f64::MAX]);
        chart.add_stack([f64::NAN, f64::INFINITY, f64::NEG_INFINITY]);
        for height in 0..8u32 {
            chart.update_plot_layout(height);
        }
        chart.set_stacks_scale(StackBarScale::Percentage);
        chart.update_plot_layout(8);
        chart.set_stacks_scale(StackBarScale::FitData);
        chart.update_plot_layout(8);
        assert_eq!(chart.stacks[1].layout.len(), 3);
    });
}

#[test]
fn check_plot_layout_follows_data_updates() {
    in_app(|| {
        let mut chart = new_chart();
        chart.add_stack([2]);
        chart.update_plot_layout(4);
        assert_eq!(chart.stacks[0].layout[0].len, 4);
        chart.add_stack([4]);
        chart.update_plot_layout(4);
        assert_eq!(chart.stacks[0].layout[0].len, 2);
        assert_eq!(chart.stacks[1].layout[0].len, 4);
    });
}

#[test]
fn check_mapping_bottom_value_and_value_per_cell() {
    let m = ValueMapping::new(-10.0, 30.0, 20);
    assert_close(m.bottom_value(), -10.0);
    assert_close(m.value_per_cell(), 2.0);
    let m = ValueMapping::new(0.0, 0.0, 20);
    assert_close(m.value_per_cell(), 0.0);
    let m = ValueMapping::new(0.0, 10.0, 0);
    assert_close(m.value_per_cell(), 0.0);
}

fn paint_in_app(width: u32, height: u32, build: impl FnOnce() -> VStackedBarChart<i32> + 'static) {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
    ";
    App::new()
        .size(Size::new(width, height))
        .debug_script(script)
        .window(move || {
            let chart = build();
            let mut w = window!("Test,d:f");
            w.add(chart);
            w
        })
        .run()
        .unwrap();
}

fn painted_chart() -> VStackedBarChart<i32> {
    let mut chart = VStackedBarChart::<i32>::new(layout!("d:f"), vstackedbarchart::Flags::None);
    chart.add_stack(BarsStackBuilder::new().label("A").bar(5).bar(-3).bar(2).build());
    chart.add_stack(BarsStackBuilder::new().label("B").bar(8).bar(1).build());
    chart.add_stack(BarsStackBuilder::new().label("C").bar(-6).bar(-2).build());
    chart
}

#[test]
fn check_paint_empty_chart() {
    paint_in_app(30, 10, || VStackedBarChart::<i32>::new(layout!("d:f"), vstackedbarchart::Flags::None));
}

#[test]
fn check_paint_axes_with_data() {
    paint_in_app(30, 10, painted_chart);
}

#[test]
fn check_paint_scale_from_zero_min_range() {
    paint_in_app(30, 10, || {
        let mut chart = painted_chart();
        chart.set_stacks_scale(StackBarScale::FromZeroMinRange { min: -20, max: 20 });
        chart
    });
}

#[test]
fn check_paint_scale_fit_data() {
    paint_in_app(30, 10, || {
        let mut chart = painted_chart();
        chart.set_stacks_scale(StackBarScale::FitData);
        chart
    });
}

#[test]
fn check_paint_scale_fixed() {
    paint_in_app(30, 10, || {
        let mut chart = painted_chart();
        chart.set_stacks_scale(StackBarScale::Fixed { min: -4, max: 4 });
        chart
    });
}

#[test]
fn check_paint_scale_percentage() {
    paint_in_app(30, 10, || {
        let mut chart = painted_chart();
        chart.set_stacks_scale(StackBarScale::Percentage);
        chart
    });
}

#[test]
fn check_paint_xaxis_index() {
    paint_in_app(30, 10, || {
        let mut chart = painted_chart();
        chart.set_xaxis_label_mode(XAxisLabelMode::Index(1));
        chart
    });
}

#[test]
fn check_paint_xaxis_stack_labels() {
    paint_in_app(30, 10, || {
        let mut chart = painted_chart();
        chart.set_xaxis_label_mode(XAxisLabelMode::StackLabels);
        chart
    });
}

#[test]
fn check_paint_xaxis_custom() {
    paint_in_app(30, 10, || {
        let mut chart = painted_chart();
        let spans = [BarSpan::new(0, 2, "First"), BarSpan::new(2, 5, "Second")];
        chart.set_xaxis_label_mode(XAxisLabelMode::Custom(&spans));
        chart
    });
}

#[test]
fn check_paint_yaxis_hidden() {
    paint_in_app(30, 10, || {
        let mut chart = painted_chart();
        chart.set_yaxis_visible(false);
        chart
    });
}

#[test]
fn check_paint_yaxis_no_grid() {
    paint_in_app(30, 10, || {
        let mut chart = painted_chart();
        chart.set_yaxis_show_grid(false);
        chart
    });
}

#[test]
fn check_paint_yaxis_narrow_small_step() {
    paint_in_app(30, 10, || {
        let mut chart = painted_chart();
        chart.set_yaxis_width(3);
        chart.set_yaxis_step(1);
        chart
    });
}

#[test]
fn check_paint_tiny_surface() {
    paint_in_app(12, 3, painted_chart);
}

#[test]
fn check_paint_tiny_surface_with_xaxis() {
    paint_in_app(12, 3, || {
        let mut chart = painted_chart();
        chart.set_xaxis_label_mode(XAxisLabelMode::StackLabels);
        chart
    });
}

#[test]
fn check_paint_data_changes_after_attach() {
    paint_in_app(30, 10, || {
        let mut chart = painted_chart();
        chart.add_stack([1, 2, 3]);
        chart.update_stack(0, |stack| {
            stack.add_bar(4);
        });
        chart.clear();
        chart.add_stack([9]);
        chart
    });
}

fn paint_surface() -> Surface {
    let mut surface = Surface::new(20, 12);
    surface.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
    surface
}

fn code_at(surface: &Surface, x: i32, y: i32) -> char {
    surface.char(x, y).unwrap().code
}

fn row_of(surface: &Surface, y: i32, from: i32, to: i32) -> String {
    (from..=to).map(|x| code_at(surface, x, y)).collect()
}

fn seg_attr() -> CharAttribute {
    charattr!("red,blue")
}

#[test]
fn check_paint_segment_fill_positive() {
    let mut s = paint_surface();
    paint_segment(&mut s, 2, 8, 3, 4, fill(), seg_attr());
    for y in 5..=8 {
        assert_eq!(row_of(&s, y, 1, 5), " \u{2588}\u{2588}\u{2588} ");
    }
    assert_eq!(row_of(&s, 4, 1, 5), "     ");
    assert_eq!(row_of(&s, 9, 1, 5), "     ");
}

#[test]
fn check_paint_segment_fill_negative() {
    let mut s = paint_surface();
    paint_segment(&mut s, 2, 4, 2, -3, fill(), seg_attr());
    assert_eq!(row_of(&s, 4, 1, 4), "    ");
    for y in 5..=7 {
        assert_eq!(row_of(&s, y, 1, 4), " \u{2588}\u{2588} ");
    }
    assert_eq!(row_of(&s, 8, 1, 4), "    ");
}

#[test]
fn check_paint_segment_fill_applies_attribute() {
    let mut s = paint_surface();
    paint_segment(&mut s, 0, 3, 1, 1, fill(), seg_attr());
    let ch = s.char(0, 3).unwrap();
    assert_eq!(ch.foreground, Color::Red);
    assert_eq!(ch.background, Color::Blue);
}

#[test]
fn check_paint_segment_zero_length_draws_nothing() {
    let mut s = paint_surface();
    paint_segment(&mut s, 2, 5, 3, 0, fill(), seg_attr());
    paint_segment(&mut s, 2, 5, 3, 0, rect(), seg_attr());
    paint_segment(&mut s, 2, 5, 3, 0, filled_rect(), seg_attr());
    for y in 0..12 {
        assert_eq!(row_of(&s, y, 0, 19), " ".repeat(20));
    }
}

#[test]
fn check_paint_segment_zero_thickness_draws_nothing() {
    let mut s = paint_surface();
    paint_segment(&mut s, 2, 5, 0, 3, fill(), seg_attr());
    for y in 0..12 {
        assert_eq!(row_of(&s, y, 0, 19), " ".repeat(20));
    }
}

#[test]
fn check_paint_segment_fill_types() {
    let cases = [
        (BarFillType::Solid, '\u{2588}'),
        (BarFillType::Shade75, '\u{2593}'),
        (BarFillType::Shade50, '\u{2592}'),
        (BarFillType::Shade25, '\u{2591}'),
        (BarFillType::Grid, '\u{253C}'),
        (BarFillType::CrossHatch, '\u{2573}'),
        (BarFillType::Custom('#'), '#'),
    ];
    for (fill_type, expected) in cases {
        let mut s = paint_surface();
        paint_segment(&mut s, 1, 4, 2, 2, StackBarDrawMode::Fill(fill_type), seg_attr());
        assert_eq!(code_at(&s, 1, 3), expected);
        assert_eq!(code_at(&s, 2, 4), expected);
    }
}

#[test]
fn check_paint_segment_rectangle_positive() {
    let mut s = paint_surface();
    paint_segment(&mut s, 2, 8, 4, 4, rect(), seg_attr());
    assert_eq!(row_of(&s, 5, 2, 5), "\u{250C}\u{2500}\u{2500}\u{2510}");
    assert_eq!(row_of(&s, 6, 2, 5), "\u{2502}  \u{2502}");
    assert_eq!(row_of(&s, 7, 2, 5), "\u{2502}  \u{2502}");
    assert_eq!(row_of(&s, 8, 2, 5), "\u{2514}\u{2500}\u{2500}\u{2518}");
    assert_eq!(row_of(&s, 4, 2, 5), "    ");
    assert_eq!(row_of(&s, 9, 2, 5), "    ");
}

#[test]
fn check_paint_segment_rectangle_negative() {
    let mut s = paint_surface();
    paint_segment(&mut s, 2, 4, 3, -3, rect(), seg_attr());
    assert_eq!(row_of(&s, 4, 2, 4), "   ");
    assert_eq!(row_of(&s, 5, 2, 4), "\u{250C}\u{2500}\u{2510}");
    assert_eq!(row_of(&s, 6, 2, 4), "\u{2502} \u{2502}");
    assert_eq!(row_of(&s, 7, 2, 4), "\u{2514}\u{2500}\u{2518}");
}

#[test]
fn check_paint_segment_rectangle_two_rows() {
    let mut s = paint_surface();
    paint_segment(&mut s, 2, 6, 3, 2, rect(), seg_attr());
    assert_eq!(row_of(&s, 5, 2, 4), "\u{250C}\u{2500}\u{2510}");
    assert_eq!(row_of(&s, 6, 2, 4), "\u{2514}\u{2500}\u{2518}");
}

#[test]
fn check_paint_segment_rectangle_one_row_is_a_line() {
    let mut s = paint_surface();
    paint_segment(&mut s, 2, 6, 4, 1, rect(), seg_attr());
    assert_eq!(row_of(&s, 6, 1, 6), " \u{2500}\u{2500}\u{2500}\u{2500} ");
    assert_eq!(row_of(&s, 5, 1, 6), "      ");
    let mut s = paint_surface();
    paint_segment(&mut s, 2, 6, 4, -1, rect(), seg_attr());
    assert_eq!(row_of(&s, 7, 1, 6), " \u{2500}\u{2500}\u{2500}\u{2500} ");
    assert_eq!(row_of(&s, 6, 1, 6), "      ");
}

#[test]
fn check_paint_segment_rectangle_line_types() {
    let mut s = paint_surface();
    paint_segment(&mut s, 1, 6, 3, 3, StackBarDrawMode::Rectangle(LineType::Double), seg_attr());
    assert_eq!(code_at(&s, 1, 4), '\u{2554}');
    assert_eq!(code_at(&s, 2, 4), '\u{2550}');
    assert_eq!(code_at(&s, 3, 6), '\u{255D}');
}

#[test]
fn check_paint_segment_filled_rectangle() {
    let mut s = paint_surface();
    paint_segment(&mut s, 2, 8, 4, 4, filled_rect(), seg_attr());
    assert_eq!(row_of(&s, 5, 2, 5), "\u{250C}\u{2500}\u{2500}\u{2510}");
    assert_eq!(row_of(&s, 6, 2, 5), "\u{2502}\u{2588}\u{2588}\u{2502}");
    assert_eq!(row_of(&s, 7, 2, 5), "\u{2502}\u{2588}\u{2588}\u{2502}");
    assert_eq!(row_of(&s, 8, 2, 5), "\u{2514}\u{2500}\u{2500}\u{2518}");
}

#[test]
fn check_paint_segment_filled_rectangle_one_row_is_a_line() {
    let mut s = paint_surface();
    paint_segment(&mut s, 2, 6, 3, 1, filled_rect(), seg_attr());
    assert_eq!(row_of(&s, 6, 1, 5), " \u{2500}\u{2500}\u{2500} ");
}

#[test]
fn check_paint_segment_filled_rectangle_two_rows_has_no_interior() {
    let mut s = paint_surface();
    paint_segment(&mut s, 2, 6, 3, 2, filled_rect(), seg_attr());
    assert_eq!(row_of(&s, 5, 2, 4), "\u{250C}\u{2500}\u{2510}");
    assert_eq!(row_of(&s, 6, 2, 4), "\u{2514}\u{2500}\u{2518}");
}

#[test]
fn check_paint_segment_adjacent_positive_segments_touch() {
    let mut s = paint_surface();
    paint_segment(&mut s, 1, 9, 2, 3, fill(), seg_attr());
    paint_segment(&mut s, 1, 6, 2, 2, StackBarDrawMode::Fill(BarFillType::Shade50), seg_attr());
    for y in 7..=9 {
        assert_eq!(code_at(&s, 1, y), '\u{2588}');
    }
    for y in 5..=6 {
        assert_eq!(code_at(&s, 1, y), '\u{2592}');
    }
    assert_eq!(code_at(&s, 1, 4), ' ');
}

#[test]
fn check_paint_segment_adjacent_negative_segments_touch() {
    let mut s = paint_surface();
    paint_segment(&mut s, 1, 4, 2, -2, fill(), seg_attr());
    paint_segment(&mut s, 1, 6, 2, -3, StackBarDrawMode::Fill(BarFillType::Shade50), seg_attr());
    for y in 5..=6 {
        assert_eq!(code_at(&s, 1, y), '\u{2588}');
    }
    for y in 7..=9 {
        assert_eq!(code_at(&s, 1, y), '\u{2592}');
    }
}

#[test]
fn check_paint_segment_outside_surface_does_not_panic() {
    let mut s = paint_surface();
    for mode in [fill(), rect(), filled_rect()] {
        paint_segment(&mut s, -5, -3, 4, 6, mode, seg_attr());
        paint_segment(&mut s, 18, 14, 4, -6, mode, seg_attr());
        paint_segment(&mut s, 100, 100, 3, 3, mode, seg_attr());
        paint_segment(&mut s, -100, -100, 3, -3, mode, seg_attr());
        paint_segment(&mut s, 0, 0, 255, i16::MAX, mode, seg_attr());
        paint_segment(&mut s, 0, 0, 255, i16::MIN, mode, seg_attr());
    }
}

#[test]
fn check_paint_segment_clipped_at_surface_edge() {
    let mut s = paint_surface();
    paint_segment(&mut s, 2, 1, 2, 4, fill(), seg_attr());
    assert_eq!(code_at(&s, 2, 0), '\u{2588}');
    assert_eq!(code_at(&s, 2, 1), '\u{2588}');
}

fn plot_colors() -> [CharAttribute; 4] {
    [charattr!("red,black"), charattr!("green,black"), charattr!("blue,black"), charattr!("yellow,black")]
}

fn plot_chart(width: u32, height: u32) -> VStackedBarChart<i32> {
    let mut chart = new_chart();
    chart.surface = Surface::new(width, height);
    chart.surface.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
    chart
}

fn paint_plot(chart: &mut VStackedBarChart<i32>, width: u32, height: u32) {
    chart.surface.reset_clip();
    chart.surface.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
    let plot_bottom = height as i32 - 1;
    chart.update_plot_layout(plot_bottom as u32);
    chart.paint_stacks(&plot_colors(), width as i32, plot_bottom);
}

fn plot_cell(chart: &VStackedBarChart<i32>, x: i32, y: i32) -> (char, Color) {
    let ch = chart.surface.char(x, y).unwrap();
    (ch.code, ch.foreground)
}

#[test]
fn check_resolve_attr_prefers_bar_then_default_then_theme() {
    let bar = charattr!("red,black");
    let default = charattr!("green,black");
    let theme = charattr!("blue,black");
    assert_eq!(resolve_attr(Some(bar), Some(default), theme), bar);
    assert_eq!(resolve_attr(Some(bar), None, theme), bar);
    assert_eq!(resolve_attr(None, Some(default), theme), default);
    assert_eq!(resolve_attr(None, None, theme), theme);
}

#[test]
fn check_paint_stacks_positive_segments() {
    in_app(|| {
        let mut chart = plot_chart(20, 8);
        chart.add_stack([4, 3]);
        paint_plot(&mut chart, 20, 8);
        for y in 4..=7 {
            assert_eq!(plot_cell(&chart, 9, y), ('\u{2588}', Color::Red));
        }
        for y in 1..=3 {
            assert_eq!(plot_cell(&chart, 9, y), ('\u{2588}', Color::Green));
        }
        assert_eq!(plot_cell(&chart, 9, 0).0, ' ');
        assert_eq!(plot_cell(&chart, 8, 5).0, ' ');
        assert_eq!(plot_cell(&chart, 10, 5).0, ' ');
    });
}

#[test]
fn check_paint_stacks_negative_segments() {
    in_app(|| {
        let mut chart = plot_chart(20, 8);
        chart.add_stack([-4, -3]);
        paint_plot(&mut chart, 20, 8);
        assert_eq!(plot_cell(&chart, 9, 0).0, ' ');
        for y in 1..=4 {
            assert_eq!(plot_cell(&chart, 9, y), ('\u{2588}', Color::Red));
        }
        for y in 5..=7 {
            assert_eq!(plot_cell(&chart, 9, y), ('\u{2588}', Color::Green));
        }
    });
}

#[test]
fn check_paint_stacks_mixed_segments() {
    in_app(|| {
        let mut chart = plot_chart(20, 10);
        chart.add_stack([4, -2, 3]);
        paint_plot(&mut chart, 20, 10);
        for y in 4..=7 {
            assert_eq!(plot_cell(&chart, 9, y), ('\u{2588}', Color::Red));
        }
        for y in 8..=9 {
            assert_eq!(plot_cell(&chart, 9, y), ('\u{2588}', Color::Green));
        }
        for y in 1..=3 {
            assert_eq!(plot_cell(&chart, 9, y), ('\u{2588}', Color::Blue));
        }
        assert_eq!(plot_cell(&chart, 9, 0).0, ' ');
    });
}

#[test]
fn check_paint_stacks_skips_empty_segments() {
    in_app(|| {
        let mut chart = plot_chart(20, 11);
        chart.add_stack([1, 1, 1, 1000]);
        paint_plot(&mut chart, 20, 11);
        for y in 1..=10 {
            assert_eq!(plot_cell(&chart, 9, y), ('\u{2588}', Color::Yellow));
        }
        assert_eq!(plot_cell(&chart, 9, 0).0, ' ');
    });
}

#[test]
fn check_paint_stacks_colors_cycle_by_bar_index() {
    in_app(|| {
        let mut chart = plot_chart(20, 11);
        chart.add_stack([2, 2, 2, 2, 2]);
        paint_plot(&mut chart, 20, 11);
        assert_eq!(plot_cell(&chart, 9, 10).1, Color::Red);
        assert_eq!(plot_cell(&chart, 9, 8).1, Color::Green);
        assert_eq!(plot_cell(&chart, 9, 6).1, Color::Blue);
        assert_eq!(plot_cell(&chart, 9, 4).1, Color::Yellow);
        assert_eq!(plot_cell(&chart, 9, 2).1, Color::Red);
    });
}

#[test]
fn check_paint_stacks_bar_attr_and_default_attr() {
    in_app(|| {
        let mut chart = plot_chart(20, 7);
        chart.set_default_bar_attr(charattr!("white,black"));
        chart.add_stack(BarsStackBuilder::new().bar(2).bar(StackBarBuilder::new(2).attr(charattr!("magenta,black")).build()).bar(2).build());
        paint_plot(&mut chart, 20, 7);
        assert_eq!(plot_cell(&chart, 9, 6).1, Color::White);
        assert_eq!(plot_cell(&chart, 9, 4).1, Color::Magenta);
        assert_eq!(plot_cell(&chart, 9, 2).1, Color::White);
    });
}

#[test]
fn check_paint_stacks_per_bar_draw_mode() {
    in_app(|| {
        let mut chart = plot_chart(20, 9);
        chart.add_stack(BarsStackBuilder::new().bar(StackBarBuilder::new(4).draw_mode(rect()).build()).bar(4).build());
        paint_plot(&mut chart, 20, 9);
        assert_eq!(plot_cell(&chart, 9, 5).0, '\u{250C}');
        assert_eq!(plot_cell(&chart, 10, 5).0, '\u{2510}');
        assert_eq!(plot_cell(&chart, 9, 8).0, '\u{2514}');
        assert_eq!(plot_cell(&chart, 9, 7).0, '\u{2502}');
        assert_eq!(plot_cell(&chart, 9, 4).0, '\u{2588}');
        assert_eq!(plot_cell(&chart, 10, 1).0, '\u{2588}');
    });
}

#[test]
fn check_paint_stacks_uses_default_draw_mode() {
    in_app(|| {
        let mut chart = plot_chart(20, 6);
        chart.set_default_bar_drawmode(StackBarDrawMode::Fill(BarFillType::Shade50));
        chart.add_stack([5]);
        paint_plot(&mut chart, 20, 6);
        assert_eq!(plot_cell(&chart, 9, 5).0, '\u{2592}');
        assert_eq!(plot_cell(&chart, 9, 1).0, '\u{2592}');
    });
}

#[test]
fn check_paint_stacks_positions_follow_thickness_and_spacing() {
    in_app(|| {
        let mut chart = plot_chart(30, 6);
        chart.set_default_bar_width(2);
        chart.set_default_bar_spacing(3);
        chart.add_stack([5]);
        chart.add_stack([5]);
        paint_plot(&mut chart, 30, 6);
        assert_eq!(plot_cell(&chart, 10, 5).0, ' ');
        assert_eq!(plot_cell(&chart, 11, 5).0, '\u{2588}');
        assert_eq!(plot_cell(&chart, 12, 5).0, '\u{2588}');
        assert_eq!(plot_cell(&chart, 13, 5).0, ' ');
        assert_eq!(plot_cell(&chart, 15, 5).0, ' ');
        assert_eq!(plot_cell(&chart, 16, 5).0, '\u{2588}');
        assert_eq!(plot_cell(&chart, 17, 5).0, '\u{2588}');
        assert_eq!(plot_cell(&chart, 18, 5).0, ' ');
    });
}

#[test]
fn check_paint_stacks_left_scroll_shifts_and_clips() {
    in_app(|| {
        let mut chart = plot_chart(20, 6);
        chart.add_stack([5]);
        chart.add_stack([5]);
        chart.left_scroll = 1;
        paint_plot(&mut chart, 20, 6);
        assert_eq!(plot_cell(&chart, 8, 5).0, '\u{2588}');
        assert_eq!(plot_cell(&chart, 10, 5).0, '\u{2588}');
        chart.left_scroll = 3;
        paint_plot(&mut chart, 20, 6);
        assert_eq!(plot_cell(&chart, 6, 5).0, ' ');
        assert_eq!(plot_cell(&chart, 7, 5).0, ' ');
        assert_eq!(plot_cell(&chart, 8, 5).0, '\u{2588}');
    });
}

#[test]
fn check_paint_stacks_first_visible_stack() {
    in_app(|| {
        let mut chart = plot_chart(20, 6);
        chart.add_stack([5]);
        chart.add_stack([5]);
        chart.first_visible_stack = 1;
        paint_plot(&mut chart, 20, 6);
        assert_eq!(plot_cell(&chart, 9, 5).0, ' ');
        assert_eq!(plot_cell(&chart, 11, 5).0, '\u{2588}');
    });
}

#[test]
fn check_paint_stacks_stops_at_right_edge() {
    in_app(|| {
        let mut chart = plot_chart(14, 6);
        chart.add_stack([5]);
        chart.add_stack([5]);
        chart.add_stack([5]);
        chart.add_stack([5]);
        paint_plot(&mut chart, 14, 6);
        assert_eq!(plot_cell(&chart, 9, 5).0, '\u{2588}');
        assert_eq!(plot_cell(&chart, 11, 5).0, '\u{2588}');
        assert_eq!(plot_cell(&chart, 13, 5).0, '\u{2588}');
    });
}

#[test]
fn check_paint_stacks_does_not_touch_y_axis_column() {
    in_app(|| {
        let mut chart = plot_chart(20, 6);
        chart.add_stack([5]);
        chart.left_scroll = 2;
        paint_plot(&mut chart, 20, 6);
        for y in 0..6 {
            assert_eq!(plot_cell(&chart, 7, y).0, ' ');
        }
    });
}

#[test]
fn check_paint_stacks_empty_chart_and_odd_sizes() {
    in_app(|| {
        let mut chart = plot_chart(20, 6);
        paint_plot(&mut chart, 20, 6);
        chart.add_stack([5, -5]);
        chart.add_stack(BarsStack::<i32>::new());
        paint_plot(&mut chart, 20, 6);
        let mut tiny = plot_chart(3, 1);
        tiny.add_stack([5, -5]);
        paint_plot(&mut tiny, 3, 1);
        chart.paint_stacks(&[], 20, 5);
        chart.paint_stacks(&plot_colors(), 0, -3);
    });
}

#[test]
fn check_paint_stacks_percentage_scale() {
    in_app(|| {
        let mut chart = plot_chart(20, 11);
        chart.set_stacks_scale(StackBarScale::Percentage);
        chart.add_stack([1, 1]);
        chart.add_stack([3]);
        paint_plot(&mut chart, 20, 11);
        for y in 6..=10 {
            assert_eq!(plot_cell(&chart, 9, y).1, Color::Red);
        }
        for y in 1..=5 {
            assert_eq!(plot_cell(&chart, 9, y).1, Color::Green);
        }
        for y in 1..=10 {
            assert_eq!(plot_cell(&chart, 11, y).1, Color::Red);
        }
    });
}

#[test]
fn check_paint_stacks_uses_theme_colors_by_bar_index() {
    in_app(|| {
        let mut chart = plot_chart(20, 13);
        chart.add_stack([2, 2, 2, 2, 2, 2]);
        chart.update_plot_layout(12);
        let colors = chart.theme().chart.bar;
        chart.paint_stacks(&colors, 20, 12);
        for index in 0..6usize {
            let expected = colors[index % colors.len()];
            let ch = chart.surface.char(9, 12 - 2 * index as i32).unwrap();
            assert_eq!(ch.foreground, expected.foreground);
            assert_eq!(ch.background, expected.background);
        }
    });
}

#[test]
fn check_paint_stacks_default_attr_overrides_theme_colors() {
    in_app(|| {
        let mut chart = plot_chart(20, 13);
        chart.set_default_bar_attr(charattr!("white,black"));
        chart.add_stack([2, 2, 2, 2, 2, 2]);
        chart.update_plot_layout(12);
        let colors = chart.theme().chart.bar;
        chart.paint_stacks(&colors, 20, 12);
        for index in 0..6i32 {
            assert_eq!(chart.surface.char(9, 12 - 2 * index).unwrap().foreground, Color::White);
        }
    });
}

#[test]
fn check_paint_stacks_does_not_store_resolved_colors() {
    in_app(|| {
        let mut chart = plot_chart(20, 7);
        chart.add_stack([2, 2]);
        paint_plot(&mut chart, 20, 7);
        let stack = chart.get_stack(0).unwrap();
        assert!(stack.get(0).unwrap().attr().is_none());
        assert!(stack.get(1).unwrap().attr().is_none());
    });
}

#[test]
fn check_paint_stacks_bar_attr_wins_over_default_attr() {
    in_app(|| {
        let mut chart = plot_chart(20, 5);
        chart.set_default_bar_attr(charattr!("white,black"));
        chart.add_stack(BarsStackBuilder::new().bar(StackBarBuilder::new(4).attr(charattr!("red,black")).build()).build());
        paint_plot(&mut chart, 20, 5);
        assert_eq!(plot_cell(&chart, 9, 4).1, Color::Red);
        assert_eq!(plot_cell(&chart, 9, 1).1, Color::Red);
    });
}

fn zero_line_chart(flags: vstackedbarchart::Flags, width: u32, height: u32) -> VStackedBarChart<i32> {
    let mut chart = VStackedBarChart::<i32>::new(layout!("x:1,y:1,w:10,h:5"), flags);
    chart.surface = Surface::new(width, height);
    chart.surface.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
    chart
}

fn paint_axis(chart: &mut VStackedBarChart<i32>, width: u32, height: u32) {
    chart.surface.reset_clip();
    chart.surface.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
    let plot_bottom = height - if chart.xaxis.label_format.is_none() { 1 } else { 3 };
    chart.update_plot_layout(plot_bottom);
    chart.paint_yaxis(width as i32, height as i32, charattr!("gray,black"), charattr!("silver,black"), charattr!("white,black"));
}

#[test]
fn check_zero_line_is_drawn_when_flag_is_set() {
    in_app(|| {
        let mut chart = zero_line_chart(vstackedbarchart::Flags::ShowZeroLineOnYAxis, 20, 12);
        chart.add_stack([6, -5]);
        paint_axis(&mut chart, 20, 12);
        for x in 8..20 {
            assert_eq!(chart.surface.char(x, 6).unwrap().code, '\u{2500}');
        }
        assert_eq!(chart.surface.char(6, 6).unwrap().code, '0');
    });
}

#[test]
fn check_zero_line_is_not_drawn_without_flag() {
    in_app(|| {
        let mut chart = zero_line_chart(vstackedbarchart::Flags::None, 20, 12);
        chart.add_stack([6, -5]);
        paint_axis(&mut chart, 20, 12);
        for x in 8..20 {
            assert_eq!(chart.surface.char(x, 6).unwrap().code, ' ');
        }
        assert_eq!(chart.surface.char(6, 6).unwrap().code, ' ');
    });
}

#[test]
fn check_zero_line_requires_grid() {
    in_app(|| {
        let mut chart = zero_line_chart(vstackedbarchart::Flags::ShowZeroLineOnYAxis, 20, 12);
        chart.add_stack([6, -5]);
        chart.set_yaxis_show_grid(false);
        paint_axis(&mut chart, 20, 12);
        for x in 8..20 {
            assert_eq!(chart.surface.char(x, 6).unwrap().code, ' ');
        }
    });
}

#[test]
fn check_zero_line_at_bottom_for_positive_only_data() {
    in_app(|| {
        let mut chart = zero_line_chart(vstackedbarchart::Flags::ShowZeroLineOnYAxis, 20, 12);
        chart.add_stack([10]);
        paint_axis(&mut chart, 20, 12);
        for x in 8..20 {
            assert_eq!(chart.surface.char(x, 11).unwrap().code, '\u{2500}');
        }
        assert_eq!(chart.surface.char(6, 11).unwrap().code, '0');
    });
}

#[test]
fn check_zero_line_hidden_when_zero_is_outside_range() {
    in_app(|| {
        let mut chart = zero_line_chart(vstackedbarchart::Flags::ShowZeroLineOnYAxis, 20, 12);
        chart.add_stack([5]);
        chart.add_stack([10]);
        chart.set_stacks_scale(StackBarScale::FitData);
        paint_axis(&mut chart, 20, 12);
        for y in 0..12 {
            for x in 8..20 {
                assert_ne!(chart.surface.char(x, y).unwrap().code, '\u{2500}');
            }
        }
    });
}

#[test]
fn check_zero_line_skipped_for_degenerate_range() {
    in_app(|| {
        let mut chart = zero_line_chart(vstackedbarchart::Flags::ShowZeroLineOnYAxis, 20, 12);
        paint_axis(&mut chart, 20, 12);
        chart.add_stack([0, 0]);
        paint_axis(&mut chart, 20, 12);
        for y in 0..12 {
            for x in 8..20 {
                assert_ne!(chart.surface.char(x, y).unwrap().code, '\u{2500}');
            }
        }
    });
}

#[test]
fn check_zero_line_with_xaxis_labels() {
    in_app(|| {
        let mut chart = zero_line_chart(vstackedbarchart::Flags::ShowZeroLineOnYAxis, 20, 12);
        chart.add_stack([6, -5]);
        chart.set_xaxis_label_mode(XAxisLabelMode::Index(0));
        paint_axis(&mut chart, 20, 12);
        let mut found = 0;
        for y in 0..10 {
            if chart.surface.char(8, y).unwrap().code == '\u{2500}' {
                found += 1;
            }
        }
        assert_eq!(found, 1);
    });
}

#[test]
fn check_zero_line_label_uses_percentage_format() {
    in_app(|| {
        let mut chart = zero_line_chart(vstackedbarchart::Flags::ShowZeroLineOnYAxis, 20, 11);
        chart.set_stacks_scale(StackBarScale::Percentage);
        chart.add_stack([1, -1]);
        paint_axis(&mut chart, 20, 11);
        for x in 8..20 {
            assert_eq!(chart.surface.char(x, 5).unwrap().code, '\u{2500}');
        }
        assert_eq!(chart.surface.char(5, 5).unwrap().code, '0');
        assert_eq!(chart.surface.char(6, 5).unwrap().code, '%');
    });
}

#[test]
fn check_zero_line_is_painted_below_the_bars() {
    in_app(|| {
        let mut chart = zero_line_chart(vstackedbarchart::Flags::ShowZeroLineOnYAxis, 20, 12);
        chart.add_stack([6, -5]);
        paint_axis(&mut chart, 20, 12);
        chart.paint_stacks(&plot_colors(), 20, 11);
        assert_eq!(chart.surface.char(9, 6).unwrap().code, '\u{2588}');
        assert_eq!(chart.surface.char(12, 6).unwrap().code, '\u{2500}');
    });
}

#[test]
fn check_percentage_axis_labels_have_percent_suffix() {
    in_app(|| {
        let mut chart = zero_line_chart(vstackedbarchart::Flags::None, 20, 11);
        chart.set_stacks_scale(StackBarScale::Percentage);
        chart.add_stack([1, 1]);
        paint_axis(&mut chart, 20, 11);
        assert_eq!(row_of(&chart.surface, 10, 4, 6), " 0%");
        assert_eq!(row_of(&chart.surface, 7, 4, 6), "30%");
        assert_eq!(row_of(&chart.surface, 4, 4, 6), "60%");
        assert_eq!(row_of(&chart.surface, 1, 4, 6), "90%");
    });
}

#[test]
fn check_non_percentage_axis_labels_have_no_percent_suffix() {
    in_app(|| {
        let mut chart = zero_line_chart(vstackedbarchart::Flags::None, 20, 11);
        chart.add_stack([10]);
        paint_axis(&mut chart, 20, 11);
        for y in 0..11 {
            assert!(!row_of(&chart.surface, y, 0, 6).contains('%'));
        }
    });
}

#[test]
fn check_axis_labels_follow_scale_changes() {
    in_app(|| {
        let mut chart = zero_line_chart(vstackedbarchart::Flags::None, 20, 11);
        chart.add_stack([1, 1]);
        chart.set_stacks_scale(StackBarScale::Percentage);
        paint_axis(&mut chart, 20, 11);
        assert!(row_of(&chart.surface, 10, 0, 6).contains('%'));
        chart.set_stacks_scale(StackBarScale::FromZero);
        paint_axis(&mut chart, 20, 11);
        for y in 0..11 {
            assert!(!row_of(&chart.surface, y, 0, 6).contains('%'));
        }
        chart.set_stacks_scale(StackBarScale::Percentage);
        paint_axis(&mut chart, 20, 11);
        assert!(row_of(&chart.surface, 10, 0, 6).contains('%'));
    });
}

#[test]
fn check_percentage_axis_uses_percentage_format() {
    in_app(|| {
        let mut chart = zero_line_chart(vstackedbarchart::Flags::None, 20, 11);
        chart.set_stacks_scale(StackBarScale::Percentage);
        chart.set_percentage_format(FormatNumber::new(10).suffix("pct"));
        chart.add_stack([1, 1]);
        paint_axis(&mut chart, 20, 11);
        assert_eq!(row_of(&chart.surface, 7, 2, 6), "30pct");
    });
}

#[test]
fn check_percentage_axis_ignores_number_format() {
    in_app(|| {
        let mut chart = zero_line_chart(vstackedbarchart::Flags::None, 20, 11);
        chart.set_stacks_scale(StackBarScale::Percentage);
        chart.set_number_format(FormatNumber::new(10).suffix("units"));
        chart.add_stack([1, 1]);
        paint_axis(&mut chart, 20, 11);
        assert_eq!(row_of(&chart.surface, 7, 4, 6), "30%");
    });
}

#[test]
fn check_number_axis_ignores_percentage_format() {
    in_app(|| {
        let mut chart = zero_line_chart(vstackedbarchart::Flags::None, 20, 11);
        chart.set_percentage_format(FormatNumber::new(10).suffix("pct"));
        chart.add_stack([10]);
        paint_axis(&mut chart, 20, 11);
        for y in 0..11 {
            assert!(!row_of(&chart.surface, y, 0, 6).contains("pct"));
        }
    });
}

fn hash_chart(flags: vstackedbarchart::Flags) -> VStackedBarChart<i32> {
    VStackedBarChart::<i32>::new(layout!("d:f"), flags)
}

fn add_positive_stacks(chart: &mut VStackedBarChart<i32>) {
    chart.add_stack(BarsStackBuilder::new().label("A").bars([3, 2]).build());
    chart.add_stack(BarsStackBuilder::new().label("B").bars([5, 1]).build());
    chart.add_stack(BarsStackBuilder::new().label("C").bars([4, 4, 2]).build());
    chart.add_stack(BarsStackBuilder::new().label("D").bars([6]).build());
}

fn add_negative_stacks(chart: &mut VStackedBarChart<i32>) {
    chart.add_stack(BarsStackBuilder::new().label("A").bars([-3, -2]).build());
    chart.add_stack(BarsStackBuilder::new().label("B").bars([-5, -1]).build());
    chart.add_stack(BarsStackBuilder::new().label("C").bars([-4, -4, -2]).build());
    chart.add_stack(BarsStackBuilder::new().label("D").bars([-6]).build());
}

fn add_mixed_stacks(chart: &mut VStackedBarChart<i32>) {
    chart.add_stack(BarsStackBuilder::new().label("A").bars([4, -2, 3]).build());
    chart.add_stack(BarsStackBuilder::new().label("B").bars([-5, 2]).build());
    chart.add_stack(BarsStackBuilder::new().label("C").bars([6, -1, -3]).build());
    chart.add_stack(BarsStackBuilder::new().label("D").bars([2, 2, 2, 2, 2, 2]).build());
}

fn check_hash(hash: u64, build: impl FnOnce() -> VStackedBarChart<i32> + 'static) {
    let script = format!(
        "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x{hash:X})
    "
    );
    App::new()
        .size(Size::new(50, 16))
        .debug_script(&script)
        .window(move || {
            let chart = build();
            let mut w = window!("Test,d:f");
            w.add(chart);
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_hash_all_positive() {
    check_hash(0xF781B8428265DA81, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        add_positive_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_all_negative() {
    check_hash(0x83050BDE5C568442, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        add_negative_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_mixed() {
    check_hash(0x2320A53E13C8684C, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        add_mixed_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_draw_mode_fill() {
    check_hash(0xC02F95A2AB87FBE0, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_default_bar_width(3);
        chart.set_default_bar_drawmode(StackBarDrawMode::Fill(BarFillType::Shade50));
        add_mixed_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_draw_mode_rectangle() {
    check_hash(0x53297C629425A1D3, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_default_bar_width(3);
        chart.set_default_bar_drawmode(rect());
        add_mixed_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_draw_mode_filled_rectangle() {
    check_hash(0xE69AFA45E478C25C, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_default_bar_width(3);
        chart.set_default_bar_drawmode(filled_rect());
        add_mixed_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_draw_modes_per_bar() {
    check_hash(0x741280BFDEE16811, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_default_bar_width(3);
        chart.add_stack(
            BarsStackBuilder::new()
                .bar(StackBarBuilder::new(4).draw_mode(rect()).build())
                .bar(StackBarBuilder::new(3).draw_mode(filled_rect()).build())
                .bar(StackBarBuilder::new(2).build())
                .build(),
        );
        chart.add_stack(
            BarsStackBuilder::new()
                .bar(StackBarBuilder::new(-3).draw_mode(filled_rect()).build())
                .bar(StackBarBuilder::new(-4).draw_mode(rect()).build())
                .build(),
        );
        chart
    });
}

#[test]
fn check_hash_scale_from_zero() {
    check_hash(0x2320A53E13C8684C, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_stacks_scale(StackBarScale::FromZero);
        add_mixed_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_scale_from_zero_min_range() {
    check_hash(0xBB381BBFFD0E57CF, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_stacks_scale(StackBarScale::FromZeroMinRange { min: -10, max: 20 });
        add_mixed_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_scale_fit_data() {
    check_hash(0x1E5A0B4F8D23F18A, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_stacks_scale(StackBarScale::FitData);
        add_positive_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_scale_fit_data_mixed() {
    check_hash(0x2320A53E13C8684C, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_stacks_scale(StackBarScale::FitData);
        add_mixed_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_scale_fixed() {
    check_hash(0xC2853F5FFB096992, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_stacks_scale(StackBarScale::Fixed { min: -3, max: 6 });
        add_mixed_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_scale_percentage() {
    check_hash(0x895F6859162D148F, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_stacks_scale(StackBarScale::Percentage);
        add_positive_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_scale_percentage_mixed() {
    check_hash(0x2918A56A539EBC5F, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::ShowZeroLineOnYAxis);
        chart.set_stacks_scale(StackBarScale::Percentage);
        add_mixed_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_thickness_1() {
    check_hash(0x2320A53E13C8684C, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_default_bar_width(1);
        add_mixed_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_thickness_2() {
    check_hash(0x891E781D8F01699C, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_default_bar_width(2);
        add_mixed_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_thickness_3() {
    check_hash(0xAF5E0C41AE1659CC, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_default_bar_width(3);
        add_mixed_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_colors_by_bar_index() {
    check_hash(0xFD2AB15539291F1B, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_default_bar_width(2);
        chart.add_stack([2, 2, 2, 2, 2, 2]);
        chart.add_stack([3, 3, 3, 3, 3]);
        chart.add_stack([1, 4, 1, 4, 1, 4]);
        chart
    });
}

#[test]
fn check_hash_colors_with_default_and_bar_attr() {
    check_hash(0x49E58B4A2373B38E, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_default_bar_width(2);
        chart.set_default_bar_attr(charattr!("aqua,black"));
        chart.add_stack(BarsStackBuilder::new().bar(3).bar(StackBarBuilder::new(3).attr(charattr!("red,black")).build()).bar(3).build());
        chart.add_stack([4, 4, 4]);
        chart
    });
}

#[test]
fn check_hash_zero_line() {
    check_hash(0xDA6A2835257C4AD4, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::ShowZeroLineOnYAxis);
        add_mixed_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_xaxis_stack_labels() {
    check_hash(0x5B80C8C154532C48, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_default_bar_width(3);
        chart.set_xaxis_label_mode(XAxisLabelMode::StackLabels);
        add_mixed_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_xaxis_index() {
    check_hash(0x63FF04D378FF70CC, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_xaxis_label_mode(XAxisLabelMode::Index(1));
        add_mixed_stacks(&mut chart);
        chart
    });
}

#[test]
fn check_hash_yaxis_hidden() {
    check_hash(0xC014CA0EC56497C5, || {
        let mut chart = hash_chart(vstackedbarchart::Flags::None);
        chart.set_yaxis_visible(false);
        add_mixed_stacks(&mut chart);
        chart
    });
}

fn scroll_chart(flags: vstackedbarchart::Flags, stacks: usize) -> VStackedBarChart<i32> {
    let mut chart = VStackedBarChart::<i32>::new(layout!("x:0,y:0,w:30,h:8"), flags);
    chart.deref_mut().layout.update(60, 20);
    for i in 0..stacks {
        chart.add_stack([(i as i32 % 4) + 1]);
    }
    chart
}

#[test]
fn check_scroll_max_left_scroll_zero_when_content_fits() {
    in_app(|| {
        let chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 3);
        assert_eq!(chart.max_left_scroll(), 0);
    });
}

#[test]
fn check_scroll_max_left_scroll_when_content_is_wider() {
    in_app(|| {
        let chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        let expected = chart.stacks_width as i32 + chart.x_axis_left_margin() + 1 - chart.size().width as i32;
        assert!(expected > 0);
        assert_eq!(chart.max_left_scroll(), expected);
    });
}

#[test]
fn check_scroll_left_scroll_is_clamped_after_repaint() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        let max = chart.max_left_scroll();
        chart.left_scroll = max + 50;
        chart.update_scroll_state();
        assert_eq!(chart.left_scroll, max);
        chart.left_scroll = -5;
        chart.update_scroll_state();
        assert_eq!(chart.left_scroll, 0);
    });
}

#[test]
fn check_scroll_left_scroll_is_clamped_when_stacks_are_removed() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        chart.left_scroll = chart.max_left_scroll();
        chart.clear();
        assert_eq!(chart.left_scroll, 0);
        assert_eq!(chart.first_visible_stack, 0);
    });
}

#[test]
fn check_scroll_first_visible_stack_follows_left_scroll() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        chart.left_scroll = 0;
        chart.update_scroll_state();
        assert_eq!(chart.first_visible_stack, 0);
        for scroll in 1..=chart.max_left_scroll() {
            chart.left_scroll = scroll;
            chart.update_scroll_state();
            let first = chart.first_visible_stack as usize;
            assert!(chart.stacks[first].pos <= scroll || first == 0);
            if first + 1 < chart.stacks.len() {
                assert!(chart.stacks[first + 1].pos >= scroll);
            }
        }
    });
}

#[test]
fn check_scroll_scrollbar_index_follows_left_scroll() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        chart.left_scroll = 4;
        chart.update_scroll_state();
        assert_eq!(chart.scrollbars.horizontal_index(), 4);
    });
}

#[test]
fn check_scroll_position_from_scrollbars() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        chart.scrollbars.set_indexes(7, 0);
        chart.update_scroll_pos_from_scrollbars();
        assert_eq!(chart.left_scroll, 7);
        assert_eq!(chart.scrollbars.horizontal_index(), 7);
    });
}

#[test]
fn check_scroll_resize_keeps_scroll_in_range() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        chart.left_scroll = chart.max_left_scroll();
        chart.on_resize(Size::new(30, 8), Size::new(60, 8));
        assert!(chart.left_scroll <= chart.max_left_scroll());
        assert_eq!(chart.surface.size().width, 60);
    });
}

fn thick_chart(stacks: usize) -> VStackedBarChart<i32> {
    let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 0);
    for _ in 0..stacks {
        chart.add_stack(BarsStackBuilder::new().thickness(40).bars([3]).build());
    }
    chart
}

#[test]
fn check_ensure_visible_out_of_range_is_ignored() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        chart.ensure_visible(20);
        assert_eq!(chart.left_scroll, 0);
    });
}

#[test]
fn check_ensure_visible_already_visible_does_not_scroll() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        chart.ensure_visible(0);
        assert_eq!(chart.left_scroll, 0);
        chart.left_scroll = 6;
        chart.update_scroll_state();
        chart.ensure_visible(5);
        assert_eq!(chart.left_scroll, 6);
    });
}

#[test]
fn check_ensure_visible_scrolls_right_to_last_stack() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        chart.ensure_visible(19);
        let plot_width = chart.size().width as i32 - chart.x_axis_left_margin();
        let last_right = chart.stacks[19].pos + chart.stacks[19].stack.actual_thickness() as i32;
        assert_eq!(chart.left_scroll, last_right - plot_width);
        assert!(chart.left_scroll <= chart.max_left_scroll());
        assert_eq!(chart.scrollbars.horizontal_index(), chart.left_scroll as u64);
    });
}

#[test]
fn check_ensure_visible_makes_stack_fit_in_view() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        chart.ensure_visible(10);
        let plot_width = chart.size().width as i32 - chart.x_axis_left_margin();
        let left = chart.stacks[10].pos - chart.left_scroll;
        let right = left + chart.stacks[10].stack.actual_thickness() as i32;
        assert!(left >= 0);
        assert!(right <= plot_width);
        assert_eq!(chart.stacks[10].pos + chart.stacks[10].stack.actual_thickness() as i32 - plot_width, chart.left_scroll);
    });
}

#[test]
fn check_ensure_visible_scrolls_back_left() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        chart.ensure_visible(19);
        chart.ensure_visible(2);
        assert_eq!(chart.left_scroll, chart.stacks[2].pos);
        assert_eq!(chart.first_visible_stack, 1);
    });
}

#[test]
fn check_ensure_visible_updates_first_visible_stack() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        chart.ensure_visible(19);
        assert!(chart.first_visible_stack > 0);
        chart.ensure_visible(0);
        assert_eq!(chart.left_scroll, chart.stacks[0].pos);
        assert_eq!(chart.first_visible_stack, 0);
    });
}

#[test]
fn check_ensure_visible_wider_than_view_partially_visible_is_kept() {
    in_app(|| {
        let mut chart = thick_chart(3);
        chart.left_scroll = chart.stacks[1].pos + 5;
        chart.update_scroll_state();
        let before = chart.left_scroll;
        chart.ensure_visible(1);
        assert_eq!(chart.left_scroll, before);
    });
}

#[test]
fn check_ensure_visible_wider_than_view_scrolls_to_left_edge() {
    in_app(|| {
        let mut chart = thick_chart(3);
        chart.ensure_visible(2);
        assert_eq!(chart.left_scroll, chart.stacks[2].pos.min(chart.max_left_scroll()));
    });
}

fn press(chart: &mut VStackedBarChart<i32>, key: u16) -> EventProcessStatus {
    chart.on_key_pressed(Key::from(key), '\0')
}

#[test]
fn check_keys_right_and_left_scroll_one_cell() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        assert!(matches!(press(&mut chart, key!("Right")), EventProcessStatus::Processed));
        assert_eq!(chart.left_scroll, 1);
        press(&mut chart, key!("Right"));
        assert_eq!(chart.left_scroll, 2);
        press(&mut chart, key!("Left"));
        assert_eq!(chart.left_scroll, 1);
        assert_eq!(chart.scrollbars.horizontal_index(), 1);
    });
}

#[test]
fn check_keys_left_at_start_does_nothing() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        assert!(matches!(press(&mut chart, key!("Left")), EventProcessStatus::Processed));
        assert_eq!(chart.left_scroll, 0);
    });
}

#[test]
fn check_keys_right_stops_at_max() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        let max = chart.max_left_scroll();
        for _ in 0..max + 10 {
            press(&mut chart, key!("Right"));
        }
        assert_eq!(chart.left_scroll, max);
    });
}

#[test]
fn check_keys_home_and_end() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        press(&mut chart, key!("End"));
        assert_eq!(chart.left_scroll, chart.max_left_scroll());
        assert!(chart.first_visible_stack > 0);
        press(&mut chart, key!("Home"));
        assert_eq!(chart.left_scroll, 0);
        assert_eq!(chart.first_visible_stack, 0);
    });
}

#[test]
fn check_keys_ctrl_right_aligns_to_next_stack() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        assert_eq!(chart.left_scroll, 0);
        press(&mut chart, key!("Ctrl+Right"));
        assert_eq!(chart.left_scroll, chart.stacks[0].pos);
        press(&mut chart, key!("Ctrl+Right"));
        assert_eq!(chart.left_scroll, chart.stacks[1].pos);
        press(&mut chart, key!("Ctrl+Right"));
        assert_eq!(chart.left_scroll, chart.stacks[2].pos);
        assert_eq!(chart.scrollbars.horizontal_index(), chart.left_scroll as u64);
    });
}

#[test]
fn check_keys_ctrl_right_from_between_stacks() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        chart.left_scroll = chart.stacks[3].pos + 1;
        chart.update_scroll_state();
        press(&mut chart, key!("Ctrl+Right"));
        assert_eq!(chart.left_scroll, chart.stacks[4].pos);
    });
}

#[test]
fn check_keys_ctrl_right_stops_at_max_scroll() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        for _ in 0..40 {
            press(&mut chart, key!("Ctrl+Right"));
        }
        assert_eq!(chart.left_scroll, chart.max_left_scroll());
    });
}

#[test]
fn check_keys_ctrl_left_aligns_to_previous_stack() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        chart.left_scroll = chart.stacks[5].pos;
        chart.update_scroll_state();
        press(&mut chart, key!("Ctrl+Left"));
        assert_eq!(chart.left_scroll, chart.stacks[4].pos);
        chart.left_scroll += 1;
        chart.update_scroll_state();
        press(&mut chart, key!("Ctrl+Left"));
        assert_eq!(chart.left_scroll, chart.stacks[4].pos);
        chart.left_scroll = chart.stacks[0].pos;
        chart.update_scroll_state();
        press(&mut chart, key!("Ctrl+Left"));
        assert_eq!(chart.left_scroll, chart.stacks[0].pos);
    });
}

#[test]
fn check_keys_ctrl_right_on_empty_chart() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 0);
        assert!(matches!(press(&mut chart, key!("Ctrl+Right")), EventProcessStatus::Processed));
        assert_eq!(chart.left_scroll, 0);
    });
}

#[test]
fn check_keys_unknown_key_is_ignored() {
    in_app(|| {
        let mut chart = scroll_chart(vstackedbarchart::Flags::ScrollBars, 20);
        assert!(matches!(press(&mut chart, key!("A")), EventProcessStatus::Ignored));
        assert_eq!(chart.left_scroll, 0);
    });
}

fn run_script(script: &str, size: Size, build: impl FnOnce() -> VStackedBarChart<i32> + 'static) {
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

fn wide_chart() -> VStackedBarChart<i32> {
    let mut chart = hash_chart(vstackedbarchart::Flags::ScrollBars);
    chart.set_xaxis_label_mode(XAxisLabelMode::Index(0));
    for i in 0..30 {
        chart.add_stack(BarsStackBuilder::new().bars([(i % 5) + 1, (i % 3) + 1]).build());
    }
    chart
}

#[test]
fn check_hash_scroll_wider_than_view() {
    let script = "
        Paint.Enable(false)
        Paint('1. Wider than the view')
        CheckHash(0xC7699FA889F036D0)
    ";
    run_script(script, Size::new(50, 16), wide_chart);
}

#[test]
fn check_hash_scroll_scrolled() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(Right,5)
        Paint('1. Scrolled 5 cells')
        CheckHash(0xA635DE772CC88CE4)
        Key.Pressed(Left,2)
        Paint('2. Back 2 cells')
        CheckHash(0x17D6EB6DF2881173)
        Key.Pressed(End)
        Paint('3. End')
        CheckHash(0x40044F5048424A19)
        Key.Pressed(Home)
        Paint('4. Home')
        CheckHash(0xC7699FA889F036D0)
        Key.Pressed(Ctrl+Right,3)
        Paint('5. Aligned to stack 3')
        CheckHash(0xA635DE772CC88CE4)
    ";
    run_script(script, Size::new(50, 16), wide_chart);
}

#[test]
fn check_hash_scroll_resized() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(End)
        Paint('1. End')
        CheckHash(0x40044F5048424A19)
        Resize(80,16)
        Paint('2. Wider terminal')
        CheckHash(0x1B158929D18E3847)
        Resize(30,12)
        Paint('3. Narrower terminal')
        CheckHash(0x78095E1AF82D5AF9)
    ";
    run_script(script, Size::new(50, 16), wide_chart);
}
