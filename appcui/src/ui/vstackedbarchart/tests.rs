use crate::prelude::*;
use crate::ui::vstackedbarchart::{BarsStack, BarsStackBuilder, StackBar, StackBarBuilder, StackBarDrawMode, StackBarScale};
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
