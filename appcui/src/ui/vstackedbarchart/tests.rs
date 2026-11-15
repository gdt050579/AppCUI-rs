use crate::prelude::*;
use crate::ui::vstackedbarchart::{BarsStack, BarsStackBuilder, StackBar, StackBarBuilder, StackBarDrawMode, StackBarScale, XAxisLabelMode};
use super::XAxisLabelFormat;
use crate::ui::components::BarFillType;

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
    let chart = new_chart();
    assert_eq!(chart.stacks_count(), 0);
    assert!(chart.get_stack(0).is_none());
}

#[test]
fn check_creation_with_scrollbars() {
    let chart = VStackedBarChart::<f64>::new(
        layout!("x:1,y:1,w:10,h:5"),
        vstackedbarchart::Flags::ScrollBars | vstackedbarchart::Flags::ShowZeroLineOnYAxis,
    );
    assert_eq!(chart.stacks_count(), 0);
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
    stack.add_bars(&[5, 6]);
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
    let mut chart = new_chart();
    chart.add_stack(BarsStackBuilder::<i32>::new().bars([1, 2, 3]).build());
    assert_eq!(chart.stacks_count(), 1);
    assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![1, 2, 3]);
}

#[test]
fn check_add_stack_from_values() {
    let mut chart = new_chart();
    chart.add_stack([1, 2, 3]);
    chart.add_stack(&[4, 5]);
    chart.add_stack(vec![6]);
    chart.add_stack(&vec![7, 8]);
    chart.add_stack(&[9, 10, 11][..]);
    assert_eq!(chart.stacks_count(), 5);
    assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![1, 2, 3]);
    assert_eq!(values_of(chart.get_stack(1).unwrap()), vec![4, 5]);
    assert_eq!(values_of(chart.get_stack(2).unwrap()), vec![6]);
    assert_eq!(values_of(chart.get_stack(3).unwrap()), vec![7, 8]);
    assert_eq!(values_of(chart.get_stack(4).unwrap()), vec![9, 10, 11]);
}

#[test]
fn check_add_stack_keeps_negative_and_positive_values_in_order() {
    let mut chart = new_chart();
    chart.add_stack([5, -3, 2, -1]);
    assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![5, -3, 2, -1]);
}

#[test]
fn check_add_stack_flat_list_is_one_stack() {
    let mut chart = new_chart();
    chart.add_stack(&[1, 2, 3]);
    assert_eq!(chart.stacks_count(), 1);
    assert_eq!(chart.get_stack(0).unwrap().count(), 3);
}

#[test]
fn check_add_stack_computes_thickness() {
    let mut chart = new_chart();
    let mut stack = BarsStack::<i32>::new();
    stack.add_bar(StackBarBuilder::new(1).draw_mode(filled_rect()).build());
    chart.add_stack(stack);
    chart.add_stack([1, 2]);
    assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 3);
    assert_eq!(chart.get_stack(1).unwrap().actual_thickness(), 1);
}

#[test]
fn check_add_stack_uses_requested_thickness() {
    let mut chart = new_chart();
    chart.add_stack(BarsStackBuilder::<i32>::new().bars([1, 2]).thickness(4).build());
    assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 4);
}

#[test]
fn check_add_stacks_from_stacks() {
    let mut chart = new_chart();
    let stacks = vec![BarsStack::from([1, 2]), BarsStack::from([3]), BarsStack::from([4, 5, 6])];
    chart.add_stacks(stacks);
    assert_eq!(chart.stacks_count(), 3);
    assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![1, 2]);
    assert_eq!(values_of(chart.get_stack(1).unwrap()), vec![3]);
    assert_eq!(values_of(chart.get_stack(2).unwrap()), vec![4, 5, 6]);
}

#[test]
fn check_add_stacks_from_stack_references() {
    let mut chart = new_chart();
    let stacks = vec![BarsStack::from([1, 2]), BarsStack::from([3])];
    chart.add_stacks(&stacks);
    assert_eq!(chart.stacks_count(), 2);
    assert_eq!(stacks.len(), 2);
    assert_eq!(values_of(chart.get_stack(1).unwrap()), vec![3]);
}

#[test]
fn check_add_stacks_from_array_matrix() {
    let mut chart = new_chart();
    chart.add_stacks([[1, 2, 3], [4, 5, 6]]);
    assert_eq!(chart.stacks_count(), 2);
    assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![1, 2, 3]);
    assert_eq!(values_of(chart.get_stack(1).unwrap()), vec![4, 5, 6]);
}

#[test]
fn check_add_stacks_from_ragged_vec_matrix() {
    let mut chart = new_chart();
    chart.add_stacks(vec![vec![1, 2, 3], vec![4], vec![]]);
    assert_eq!(chart.stacks_count(), 3);
    assert_eq!(chart.get_stack(0).unwrap().count(), 3);
    assert_eq!(chart.get_stack(1).unwrap().count(), 1);
    assert_eq!(chart.get_stack(2).unwrap().count(), 0);
}

#[test]
fn check_add_stacks_from_slice_of_slices() {
    let mut chart = new_chart();
    let rows: Vec<&[i32]> = vec![&[1, 2][..], &[3, 4, 5][..]];
    chart.add_stacks(rows.iter().copied());
    assert_eq!(chart.stacks_count(), 2);
    assert_eq!(values_of(chart.get_stack(1).unwrap()), vec![3, 4, 5]);
}

#[test]
fn check_add_stacks_empty_iterator() {
    let mut chart = new_chart();
    chart.add_stacks(Vec::<BarsStack<i32>>::new());
    assert_eq!(chart.stacks_count(), 0);
}

#[test]
fn check_add_stacks_appends_after_existing() {
    let mut chart = new_chart();
    chart.add_stack([1]);
    chart.add_stacks([[2], [3]]);
    chart.add_stack([4]);
    assert_eq!(chart.stacks_count(), 4);
    let first: Vec<i32> = (0..4).map(|i| chart.get_stack(i).unwrap().get(0).unwrap().value()).collect();
    assert_eq!(first, vec![1, 2, 3, 4]);
}

#[test]
fn check_add_stacks_computes_thickness_for_every_stack() {
    let mut chart = new_chart();
    let mut rect_stack = BarsStack::<i32>::new();
    rect_stack.add_bar(StackBarBuilder::new(1).draw_mode(rect()).build());
    chart.add_stacks(vec![rect_stack, BarsStack::from([1, 2])]);
    assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 2);
    assert_eq!(chart.get_stack(1).unwrap().actual_thickness(), 1);
}

#[test]
fn check_get_stack_out_of_range() {
    let mut chart = new_chart();
    chart.add_stack([1]);
    assert!(chart.get_stack(0).is_some());
    assert!(chart.get_stack(1).is_none());
    assert!(chart.get_stack(usize::MAX).is_none());
}

#[test]
fn check_update_stack_modifies_values() {
    let mut chart = new_chart();
    chart.add_stack([1, 2, 3]);
    chart.update_stack(0, |stack| {
        stack.get_mut(1).unwrap().set_value(20);
        stack.add_bar(-4);
    });
    assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![1, 20, 3, -4]);
}

#[test]
fn check_update_stack_recomputes_thickness() {
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
}

#[test]
fn check_update_stack_thickness_request() {
    let mut chart = new_chart();
    chart.add_stack([1, 2]);
    chart.update_stack(0, |stack| {
        stack.set_thickness(6);
    });
    assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 6);
    assert_eq!(chart.get_stack(0).unwrap().thickness(), Some(6));
}

#[test]
fn check_update_stack_touches_only_the_selected_stack() {
    let mut chart = new_chart();
    chart.add_stacks([[1, 2], [3, 4]]);
    chart.update_stack(1, |stack| {
        stack.get_mut(0).unwrap().set_value(30).set_draw_mode(filled_rect());
    });
    assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![1, 2]);
    assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 1);
    assert_eq!(values_of(chart.get_stack(1).unwrap()), vec![30, 4]);
    assert_eq!(chart.get_stack(1).unwrap().actual_thickness(), 3);
}

#[test]
fn check_update_stack_out_of_range_does_nothing() {
    let mut chart = new_chart();
    chart.add_stack([1, 2]);
    let mut called = false;
    chart.update_stack(5, |_| called = true);
    assert!(!called);
    assert_eq!(chart.stacks_count(), 1);
    assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![1, 2]);
}

#[test]
fn check_update_stack_on_empty_chart() {
    let mut chart = new_chart();
    let mut called = false;
    chart.update_stack(0, |_| called = true);
    assert!(!called);
    assert_eq!(chart.stacks_count(), 0);
}

#[test]
fn check_clear() {
    let mut chart = new_chart();
    chart.add_stacks([[1, 2], [3, 4]]);
    assert_eq!(chart.stacks_count(), 2);
    chart.clear();
    assert_eq!(chart.stacks_count(), 0);
    assert!(chart.get_stack(0).is_none());
    chart.add_stack([9]);
    assert_eq!(chart.stacks_count(), 1);
    assert_eq!(values_of(chart.get_stack(0).unwrap()), vec![9]);
}

#[test]
fn check_clear_on_empty_chart() {
    let mut chart = new_chart();
    chart.clear();
    assert_eq!(chart.stacks_count(), 0);
}

#[test]
fn check_float_chart_with_mixed_signs() {
    let mut chart = VStackedBarChart::<f64>::new(layout!("x:1,y:1,w:10,h:5"), vstackedbarchart::Flags::None);
    chart.add_stack([1.5, -2.5, 0.0]);
    chart.add_stacks(vec![vec![-1.0, 2.0], vec![3.25]]);
    assert_eq!(chart.stacks_count(), 3);
    let first = chart.get_stack(0).unwrap();
    assert_eq!(first.get(0).unwrap().value(), 1.5);
    assert_eq!(first.get(1).unwrap().value(), -2.5);
    assert_eq!(first.get(2).unwrap().value(), 0.0);
    assert_eq!(chart.get_stack(2).unwrap().get(0).unwrap().value(), 3.25);
}

#[test]
fn check_zero_values_are_kept() {
    let mut chart = new_chart();
    chart.add_stack([0, 0, 0]);
    assert_eq!(chart.get_stack(0).unwrap().count(), 3);
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
    let mut chart = new_chart();
    chart.add_stack(BarsStackBuilder::<i32>::new().label("A").bars([1]).build());
    chart.update_stack(0, |stack| {
        stack.set_label("B");
    });
    assert_eq!(chart.get_stack(0).unwrap().label(), "B");
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
}

#[test]
fn check_set_stacks_scale() {
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
}

#[test]
fn check_set_number_format() {
    let mut chart = new_chart();
    chart.set_number_format(FormatNumber::new(10).decimals(3));
    let custom = FormatNumber::new(10).decimals(3);
    assert_eq!(format_float(&chart.number_format, 1.0), format_float(&custom, 1.0));
    assert_eq!(format_float(&chart.number_format, 2.5), format_float(&custom, 2.5));
    let default_chart = new_chart();
    assert_ne!(format_float(&default_chart.number_format, 1.0), format_float(&custom, 1.0));
}

#[test]
fn check_default_number_format_depends_on_value_type() {
    let int_chart = new_chart();
    let float_chart = VStackedBarChart::<f64>::new(layout!("x:1,y:1,w:10,h:5"), vstackedbarchart::Flags::None);
    assert_ne!(format_float(&int_chart.number_format, 1.0), format_float(&float_chart.number_format, 1.0));
}

#[test]
fn check_set_default_bar_width() {
    let mut chart = new_chart();
    chart.set_default_bar_width(4);
    assert_eq!(chart.defaults.thickness, 4);
    chart.set_default_bar_width(0);
    assert_eq!(chart.defaults.thickness, 1);
}

#[test]
fn check_set_default_bar_width_refreshes_every_stack() {
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
}

#[test]
fn check_set_default_bar_spacing() {
    let mut chart = new_chart();
    chart.set_default_bar_spacing(3);
    assert_eq!(chart.defaults.spacing, 3);
    chart.set_default_bar_spacing(0);
    assert_eq!(chart.defaults.spacing, 0);
}

#[test]
fn check_set_default_bar_drawmode() {
    let mut chart = new_chart();
    chart.set_default_bar_drawmode(rect());
    assert_eq!(chart.defaults.draw_mode, rect());
    chart.set_default_bar_drawmode(filled_rect());
    assert_eq!(chart.defaults.draw_mode, filled_rect());
}

#[test]
fn check_set_default_bar_drawmode_refreshes_every_stack() {
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
}

#[test]
fn check_stacks_added_after_defaults_change_use_new_defaults() {
    let mut chart = new_chart();
    chart.set_default_bar_width(3);
    chart.set_default_bar_drawmode(rect());
    chart.add_stack([1, 2]);
    assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 3);
    chart.set_default_bar_width(1);
    assert_eq!(chart.get_stack(0).unwrap().actual_thickness(), 2);
}

#[test]
fn check_set_default_bar_attr() {
    let mut chart = new_chart();
    assert!(chart.defaults.attr.is_none());
    chart.set_default_bar_attr(CharAttribute::with_fore_color(Color::Yellow));
    assert!(chart.defaults.attr.is_some());
}

#[test]
fn check_set_yaxis_visible_and_grid() {
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
}

#[test]
fn check_set_yaxis_width() {
    let mut chart = new_chart();
    chart.set_yaxis_width(9);
    assert_eq!(chart.yaxis.width, 9);
    chart.set_yaxis_width(0);
    assert_eq!(chart.yaxis.width, 1);
}

#[test]
fn check_set_yaxis_step() {
    let mut chart = new_chart();
    chart.set_yaxis_step(5);
    assert_eq!(chart.yaxis.step, 5);
    chart.set_yaxis_step(0);
    assert_eq!(chart.yaxis.step, 1);
}

#[test]
fn check_set_xaxis_label_mode_simple_modes() {
    let mut chart = new_chart();
    chart.set_xaxis_label_mode(XAxisLabelMode::Index(5));
    assert!(chart.xaxis.label_format == XAxisLabelFormat::Index(5));
    chart.set_xaxis_label_mode(XAxisLabelMode::StackLabels);
    assert!(chart.xaxis.label_format == XAxisLabelFormat::StackLabels);
    chart.set_xaxis_label_mode(XAxisLabelMode::None);
    assert!(chart.xaxis.label_format == XAxisLabelFormat::None);
}

#[test]
fn check_set_xaxis_label_mode_custom_sorts_spans() {
    let mut chart = new_chart();
    let spans = [BarSpan::new(6, 2, "C"), BarSpan::new(0, 3, "A"), BarSpan::new(3, 2, "B")];
    chart.set_xaxis_label_mode(XAxisLabelMode::Custom(&spans));
    assert!(chart.xaxis.label_format == XAxisLabelFormat::Custom);
    let starts: Vec<u32> = chart.xaxis.spans.iter().map(|span| span.start).collect();
    assert_eq!(starts, vec![0, 3, 6]);
}

#[test]
fn check_set_xaxis_label_mode_custom_drops_overlapping_spans() {
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
}

#[test]
fn check_set_xaxis_label_mode_custom_replaces_previous_spans() {
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
}

#[test]
fn check_setters_do_not_change_stacks_data() {
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
}

#[test]
fn check_percentage_format_default() {
    let chart = new_chart();
    let expected = FormatNumber::new(10).suffix("%");
    assert_eq!(format_float(&chart.percentage_format, 50.0), format_float(&expected, 50.0));
    let text = format_float(&chart.percentage_format, 50.0).unwrap();
    assert!(text.ends_with('%'));
    assert!(text.starts_with("50"));
}

#[test]
fn check_percentage_format_has_no_decimals() {
    let chart = new_chart();
    let text = format_float(&chart.percentage_format, 33.333).unwrap();
    assert!(!text.contains('.'));
    assert!(text.ends_with('%'));
    let float_chart = VStackedBarChart::<f64>::new(layout!("x:1,y:1,w:10,h:5"), vstackedbarchart::Flags::None);
    let float_text = format_float(&float_chart.percentage_format, 33.333).unwrap();
    assert!(!float_text.contains('.'));
    assert!(float_text.ends_with('%'));
}

#[test]
fn check_set_percentage_format() {
    let mut chart = new_chart();
    chart.set_percentage_format(FormatNumber::new(10).decimals(1).suffix(" pct"));
    let custom = FormatNumber::new(10).decimals(1).suffix(" pct");
    assert_eq!(format_float(&chart.percentage_format, 12.5), format_float(&custom, 12.5));
    let default_chart = new_chart();
    assert_ne!(format_float(&default_chart.percentage_format, 12.5), format_float(&chart.percentage_format, 12.5));
}

#[test]
fn check_set_percentage_format_keeps_number_format() {
    let mut chart = new_chart();
    let before = format_float(&chart.number_format, 1234.0);
    chart.set_percentage_format(FormatNumber::new(10).decimals(2).suffix("%"));
    assert_eq!(format_float(&chart.number_format, 1234.0), before);
}

#[test]
fn check_yaxis_format_follows_scale() {
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
}

#[test]
fn check_yaxis_format_uses_custom_formats() {
    let mut chart = new_chart();
    chart.set_number_format(FormatNumber::new(10).decimals(3));
    chart.set_percentage_format(FormatNumber::new(10).decimals(1).suffix("%"));
    let number = format_float(chart.yaxis_format(), 2.0);
    assert_eq!(number, format_float(&FormatNumber::new(10).decimals(3), 2.0));
    chart.set_stacks_scale(StackBarScale::Percentage);
    let percent = format_float(chart.yaxis_format(), 2.0);
    assert_eq!(percent, format_float(&FormatNumber::new(10).decimals(1).suffix("%"), 2.0));
    assert_ne!(number, percent);
}

#[test]
fn check_other_scales_use_number_format() {
    let mut chart = new_chart();
    let expected = format_float(&chart.number_format, 7.0);
    chart.set_stacks_scale(StackBarScale::FromZero);
    assert_eq!(format_float(chart.yaxis_format(), 7.0), expected);
    chart.set_stacks_scale(StackBarScale::FromZeroMinRange { min: -1, max: 1 });
    assert_eq!(format_float(chart.yaxis_format(), 7.0), expected);
    chart.set_stacks_scale(StackBarScale::Fixed { min: 0, max: 10 });
    assert_eq!(format_float(chart.yaxis_format(), 7.0), expected);
}
