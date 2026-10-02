use super::traits::{
    CustomEvents, EventProcessStatus, ExpandedDirection, GenericBackgroundTaskEvents, OnExpand, OnWindowRegistered, TimerEvents,
};
use super::{ContainerBase, Number, NumberFormat};
use crate::graphics::{Point, Size};
use crate::system::{App, Handle, RuntimeManager};
use crate::ui::{Button, Label, Layout};

/// Implements the event traits without overriding any method, so each call hits the default body.
struct DefaultEventHost;

impl OnExpand for DefaultEventHost {}
impl OnWindowRegistered for DefaultEventHost {}
impl CustomEvents for DefaultEventHost {}
impl TimerEvents for DefaultEventHost {}
impl GenericBackgroundTaskEvents for DefaultEventHost {}

#[track_caller]
fn assert_exact<T>(value: T, expected: f64)
where
    T: Number + PartialEq + std::fmt::Debug,
{
    assert_eq!(value.to_f64(), expected, "{}", std::any::type_name::<T>());
    assert_eq!(T::from_f64(expected), value, "{}", std::any::type_name::<T>());
}

#[track_caller]
fn assert_from<T>(input: f64, expected: T)
where
    T: Number + PartialEq + std::fmt::Debug,
{
    assert_eq!(T::from_f64(input), expected, "{}", std::any::type_name::<T>());
}

fn formatted<T: Number>(value: T, format: NumberFormat) -> String {
    let mut output = String::from("stale");
    value.write_to_string(&mut output, format);
    output
}

#[test]
fn is_float_matches_the_numeric_kind() {
    assert!(!i8::is_float());
    assert!(!i16::is_float());
    assert!(!i32::is_float());
    assert!(!i64::is_float());
    assert!(!i128::is_float());
    assert!(!isize::is_float());

    assert!(!u8::is_float());
    assert!(!u16::is_float());
    assert!(!u32::is_float());
    assert!(!u64::is_float());
    assert!(!u128::is_float());
    assert!(!usize::is_float());

    assert!(f32::is_float());
    assert!(f64::is_float());
}

#[test]
fn signed_integers_roundtrip_when_the_value_is_an_exact_f64() {
    assert_exact(0i8, 0.0);
    assert_exact(42i8, 42.0);
    assert_exact(-7i8, -7.0);
    assert_exact(i8::MIN, -128.0);
    assert_exact(i8::MAX, 127.0);

    assert_exact(0i16, 0.0);
    assert_exact(-1000i16, -1000.0);
    assert_exact(i16::MIN, -32768.0);
    assert_exact(i16::MAX, 32767.0);

    assert_exact(0i32, 0.0);
    assert_exact(1_000_000i32, 1_000_000.0);
    assert_exact(i32::MIN, -2_147_483_648.0);
    assert_exact(i32::MAX, 2_147_483_647.0);

    assert_exact(0i64, 0.0);
    assert_exact(-42i64, -42.0);
    assert_exact(1i64 << 53, 9_007_199_254_740_992.0);
    assert_exact(-(1i64 << 53), -9_007_199_254_740_992.0);
    assert_exact(i64::MIN, -(2.0f64.powi(63)));

    assert_exact(0i128, 0.0);
    assert_exact(-42i128, -42.0);
    assert_exact(1i128 << 53, 9_007_199_254_740_992.0);
    assert_exact(-(1i128 << 53), -9_007_199_254_740_992.0);
    assert_exact(i128::MIN, -(2.0f64.powi(127)));

    assert_exact(0isize, 0.0);
    assert_exact(-15isize, -15.0);
    assert_exact(1_000_000isize, 1_000_000.0);
}

#[test]
fn unsigned_integers_roundtrip_when_the_value_is_an_exact_f64() {
    assert_exact(0u8, 0.0);
    assert_exact(u8::MAX, 255.0);

    assert_exact(0u16, 0.0);
    assert_exact(u16::MAX, 65_535.0);

    assert_exact(0u32, 0.0);
    assert_exact(u32::MAX, 4_294_967_295.0);

    assert_exact(0u64, 0.0);
    assert_exact(1u64 << 53, 9_007_199_254_740_992.0);

    assert_exact(0u128, 0.0);
    assert_exact(1u128 << 53, 9_007_199_254_740_992.0);

    assert_exact(0usize, 0.0);
    assert_exact(50usize, 50.0);
}

#[test]
fn from_f64_truncates_fractions_toward_zero() {
    assert_from(1.9, 1i8);
    assert_from(-1.9, -1i8);
    assert_from(0.9, 0i8);
    assert_from(-0.9, 0i8);

    assert_from(1.9, 1i16);
    assert_from(-1.9, -1i16);
    assert_from(1.9, 1i32);
    assert_from(-3.14159, -3i32);
    assert_from(9.99, 9i64);
    assert_from(-9.99, -9i64);
    assert_from(100.1, 100i128);
    assert_from(-100.1, -100i128);
    assert_from(4.8, 4isize);
    assert_from(-4.8, -4isize);

    assert_from(1.9, 1u8);
    assert_from(255.9, 255u8);
    assert_from(0.1, 0u16);
    assert_from(1_000.75, 1_000u32);
    assert_from(42.9, 42u64);
    assert_from(7.2, 7u128);
    assert_from(8.8, 8usize);
}

#[test]
fn from_f64_saturates_outside_the_integer_range() {
    assert_from(128.0, i8::MAX);
    assert_from(-129.0, i8::MIN);
    assert_from(f64::INFINITY, i8::MAX);
    assert_from(f64::NEG_INFINITY, i8::MIN);
    assert_from(f64::NAN, 0i8);

    assert_from(256.0, u8::MAX);
    assert_from(-0.1, 0u8);
    assert_from(f64::INFINITY, u8::MAX);
    assert_from(f64::NEG_INFINITY, 0u8);
    assert_from(f64::NAN, 0u8);

    assert_from(32_768.0, i16::MAX);
    assert_from(-32_769.0, i16::MIN);
    assert_from(65_536.0, u16::MAX);
    assert_from(f64::NAN, 0i16);
    assert_from(f64::NEG_INFINITY, 0u16);

    assert_from(2_147_483_648.0, i32::MAX);
    assert_from(-2_147_483_649.0, i32::MIN);
    assert_from(4_294_967_296.0, u32::MAX);
    assert_from(f64::INFINITY, i32::MAX);
    assert_from(f64::NAN, 0u32);

    assert_from(2.0f64.powi(63), i64::MAX);
    assert_from(-(2.0f64.powi(64)), i64::MIN);
    assert_from(2.0f64.powi(64), u64::MAX);
    assert_from(-1.0, 0u64);
    assert_from(f64::NAN, 0i64);
    assert_from(f64::INFINITY, u64::MAX);
    assert_from(f64::NEG_INFINITY, 0u64);

    assert_from(2.0f64.powi(127), i128::MAX);
    assert_from(-(2.0f64.powi(128)), i128::MIN);
    assert_from(2.0f64.powi(128), u128::MAX);
    assert_from(-1.0, 0u128);
    assert_from(f64::NAN, 0i128);
    assert_from(f64::INFINITY, i128::MAX);
    assert_from(f64::NEG_INFINITY, i128::MIN);
    assert_from(f64::NEG_INFINITY, 0u128);

    assert_from(f64::NAN, 0isize);
    assert_from(f64::NAN, 0usize);
    assert_from(f64::INFINITY, isize::MAX);
    assert_from(f64::NEG_INFINITY, isize::MIN);
    assert_from(f64::INFINITY, usize::MAX);
    assert_from(f64::NEG_INFINITY, 0usize);
    assert_from(-1.0, 0usize);
}

#[test]
fn integers_past_the_f64_mantissa_lose_low_bits() {
    let past_i64 = (1i64 << 53) + 1;
    assert_eq!(past_i64.to_f64(), 9_007_199_254_740_992.0);
    assert_eq!(i64::from_f64(past_i64.to_f64()), 1i64 << 53);

    let past_u64 = (1u64 << 53) + 1;
    assert_eq!(past_u64.to_f64(), 9_007_199_254_740_992.0);
    assert_eq!(u64::from_f64(past_u64.to_f64()), 1u64 << 53);

    let past_i128 = (1i128 << 53) + 1;
    assert_eq!(past_i128.to_f64(), 9_007_199_254_740_992.0);
    assert_eq!(i128::from_f64(past_i128.to_f64()), 1i128 << 53);

    let past_u128 = (1u128 << 53) + 1;
    assert_eq!(past_u128.to_f64(), 9_007_199_254_740_992.0);
    assert_eq!(u128::from_f64(past_u128.to_f64()), 1u128 << 53);

    assert_eq!(i64::MAX.to_f64(), 2.0f64.powi(63));
    assert_eq!(i64::from_f64(i64::MAX.to_f64()), i64::MAX);
    assert_eq!(u64::MAX.to_f64(), 2.0f64.powi(64));
    assert_eq!(u64::from_f64(u64::MAX.to_f64()), u64::MAX);

    assert_eq!(i128::MAX.to_f64(), 2.0f64.powi(127));
    assert_eq!(i128::from_f64(i128::MAX.to_f64()), i128::MAX);
    assert_eq!(u128::MAX.to_f64(), 2.0f64.powi(128));
    assert_eq!(u128::from_f64(u128::MAX.to_f64()), u128::MAX);
}

#[test]
fn pointer_sized_integers_follow_the_target_width() {
    #[cfg(target_pointer_width = "64")]
    {
        assert_exact(1isize << 53, 9_007_199_254_740_992.0);
        assert_exact(-(1isize << 53), -9_007_199_254_740_992.0);
        assert_exact(isize::MIN, -(2.0f64.powi(63)));
        assert_eq!(isize::MAX.to_f64(), 2.0f64.powi(63));
        assert_eq!(isize::from_f64(isize::MAX.to_f64()), isize::MAX);

        assert_exact(1usize << 53, 9_007_199_254_740_992.0);
        let past = (1usize << 53) + 1;
        assert_eq!(past.to_f64(), 9_007_199_254_740_992.0);
        assert_eq!(usize::from_f64(past.to_f64()), 1usize << 53);
        assert_eq!(usize::MAX.to_f64(), 2.0f64.powi(64));
        assert_eq!(usize::from_f64(usize::MAX.to_f64()), usize::MAX);
    }

    #[cfg(target_pointer_width = "32")]
    {
        assert_exact(isize::MIN, -2_147_483_648.0);
        assert_exact(isize::MAX, 2_147_483_647.0);
        assert_from(2_147_483_648.0, isize::MAX);
        assert_from(-2_147_483_649.0, isize::MIN);

        assert_exact(usize::MAX, 4_294_967_295.0);
        assert_from(4_294_967_296.0, usize::MAX);
    }
}

#[test]
fn f32_converts_through_f64() {
    assert_exact(0.0f32, 0.0);
    assert_exact(1.5f32, 1.5);
    assert_exact(-2.25f32, -2.25);
    assert_eq!(f32::from_f64(f32::MAX.to_f64()), f32::MAX);
    assert_eq!(f32::from_f64(f32::MIN.to_f64()), f32::MIN);

    let third = f32::from_f64(1.0 / 3.0);
    assert_eq!(third, (1.0f64 / 3.0) as f32);
    assert_eq!(third.to_f64(), f64::from(third));

    assert_eq!(f32::from_f64(f64::MAX), f32::INFINITY);
    assert_eq!(f32::from_f64(f64::MIN), f32::NEG_INFINITY);
    assert_eq!(f32::from_f64(f64::INFINITY), f32::INFINITY);
    assert_eq!(f32::INFINITY.to_f64(), f64::INFINITY);
    assert_eq!(f32::from_f64(f64::NEG_INFINITY), f32::NEG_INFINITY);
    assert_eq!(f32::NEG_INFINITY.to_f64(), f64::NEG_INFINITY);
    assert!(f32::from_f64(f64::NAN).is_nan());
    assert!(f32::NAN.to_f64().is_nan());
}

#[test]
fn f64_to_f64_and_from_f64_are_identity() {
    for value in [0.0, 1.5, -2.25, 1.0e20, f64::MIN, f64::MAX, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(value.to_f64(), value);
        assert_eq!(f64::from_f64(value), value);
    }
    assert!(f64::NAN.to_f64().is_nan());
    assert!(f64::from_f64(f64::NAN).is_nan());
}

#[test]
fn write_to_string_formats_every_numeric_type() {
    assert_eq!(formatted(42i8, NumberFormat::Decimal), "42");
    assert_eq!(formatted(i8::MIN, NumberFormat::Decimal), "-128");
    assert_eq!(formatted(-1000i16, NumberFormat::DigitGrouping), "-1,000");
    assert_eq!(formatted(1_000_000i32, NumberFormat::Decimal), "1000000");
    assert_eq!(formatted(-42i64, NumberFormat::Decimal), "-42");
    assert_eq!(formatted(-42i128, NumberFormat::Hex), "-0x2A");
    assert_eq!(formatted(-15isize, NumberFormat::Decimal), "-15");

    assert_eq!(formatted(255u8, NumberFormat::Hex), "0xFF");
    assert_eq!(formatted(u16::MAX, NumberFormat::Decimal), "65535");
    assert_eq!(formatted(1_000_000u32, NumberFormat::DigitGrouping), "1,000,000");
    assert_eq!(formatted(1024u64, NumberFormat::Size), "1 KB");
    assert_eq!(formatted(42u128, NumberFormat::Decimal), "42");
    assert_eq!(formatted(50usize, NumberFormat::Percentage), "50%");

    assert_eq!(formatted(1.5f32, NumberFormat::Decimal), "1.50");
    assert_eq!(formatted(-2.25f64, NumberFormat::Decimal), "-2.25");
    assert_eq!(formatted(0.75f64, NumberFormat::Percentage), "75.00");
    assert_eq!(formatted(255.0f64, NumberFormat::Hex), "0xFF");
    assert_eq!(formatted(2048.0f32, NumberFormat::Size), "2 KB");
}

#[test]
fn on_expand_default_accepts_both_directions_and_pack() {
    let mut host = DefaultEventHost;
    OnExpand::on_expand(&mut host, ExpandedDirection::OnTop);
    OnExpand::on_expand(&mut host, ExpandedDirection::OnBottom);
    OnExpand::on_pack(&mut host);
}

#[test]
fn on_window_registered_default_does_nothing() {
    let mut host = DefaultEventHost;
    OnWindowRegistered::on_registered(&mut host);
}

#[test]
fn custom_events_default_ignores_the_event() {
    let mut host = DefaultEventHost;
    let status = CustomEvents::on_event(&mut host, Handle::<()>::None, 0xA11C, 7);
    assert!(status == EventProcessStatus::Ignored);

    let status = CustomEvents::on_event(&mut host, Handle::new(3), u64::MAX, 0);
    assert!(status == EventProcessStatus::Ignored);
}

#[test]
fn timer_events_default_ignores_every_phase() {
    let mut host = DefaultEventHost;
    assert!(TimerEvents::on_start(&mut host) == EventProcessStatus::Ignored);
    assert!(TimerEvents::on_resume(&mut host, 0) == EventProcessStatus::Ignored);
    assert!(TimerEvents::on_pause(&mut host, 1) == EventProcessStatus::Ignored);
    assert!(TimerEvents::on_update(&mut host, 42) == EventProcessStatus::Ignored);
    assert!(TimerEvents::on_update(&mut host, u64::MAX) == EventProcessStatus::Ignored);
}

#[test]
fn generic_background_task_events_default_ignores_every_phase() {
    let mut host = DefaultEventHost;
    let handle = Handle::<()>::None;
    assert!(GenericBackgroundTaskEvents::on_start(&mut host, handle) == EventProcessStatus::Ignored);
    assert!(GenericBackgroundTaskEvents::on_update(&mut host, handle) == EventProcessStatus::Ignored);
    assert!(GenericBackgroundTaskEvents::on_finish(&mut host, handle) == EventProcessStatus::Ignored);
    assert!(GenericBackgroundTaskEvents::on_query(&mut host, handle) == EventProcessStatus::Ignored);

    let handle = Handle::new(1);
    assert!(GenericBackgroundTaskEvents::on_start(&mut host, handle) == EventProcessStatus::Ignored);
    assert!(GenericBackgroundTaskEvents::on_update(&mut host, handle) == EventProcessStatus::Ignored);
    assert!(GenericBackgroundTaskEvents::on_finish(&mut host, handle) == EventProcessStatus::Ignored);
    assert!(GenericBackgroundTaskEvents::on_query(&mut host, handle) == EventProcessStatus::Ignored);
}

fn laid_out_container(accept_input: bool) -> ContainerBase {
    let mut container = ContainerBase::new(Layout::absolute(2, 3, 40, 10), accept_input);
    container.layout.update(80, 25);
    container
}

#[test]
fn container_new_starts_visible_enabled_and_empty() {
    let input = laid_out_container(true);
    assert!(input.is_visible());
    assert!(input.is_enabled());
    assert!(input.is_active());
    assert!(input.can_receive_input());
    assert!(!input.has_focus());
    assert!(!input.is_mouse_over());
    assert!(input.children.is_empty());
    assert!(input.handle.is_none());
    assert!(input.parent.is_none());
    assert_eq!(input.margins.left, 0);
    assert_eq!(input.margins.top, 0);
    assert_eq!(input.margins.right, 0);
    assert_eq!(input.margins.bottom, 0);
    assert_eq!(input.position(), Point::new(2, 3));
    assert_eq!(input.size(), Size::new(40, 10));
    assert_eq!(input.client_size(), Size::new(40, 10));

    let display_only = laid_out_container(false);
    assert!(display_only.is_visible());
    assert!(display_only.is_enabled());
    assert!(display_only.is_active());
    assert!(!display_only.can_receive_input());
}

#[test]
fn container_default_has_no_input_and_no_children() {
    let container = ContainerBase::default();
    assert!(!container.is_visible());
    assert!(!container.is_enabled());
    assert!(!container.can_receive_input());
    assert!(container.children.is_empty());
    assert_eq!(container.size(), Size::new(0, 0));
    assert_eq!(container.client_size(), Size::new(0, 0));
}

#[test]
fn container_with_focus_overlay_reserves_right_and_bottom_margins_only_while_focused() {
    let mut overlay = ContainerBase::with_focus_overlay(Layout::absolute(0, 0, 10, 5));
    assert!(overlay.can_receive_input());
    assert!(overlay.should_increase_margins_on_focus().is_none());

    overlay.update_focus_flag(true);
    // Bit 0 reserves the right margin, bit 1 the bottom margin.
    assert_eq!(overlay.should_increase_margins_on_focus(), Some(0b11));

    overlay.update_focus_flag(false);
    assert!(overlay.should_increase_margins_on_focus().is_none());

    let mut plain = laid_out_container(true);
    plain.update_focus_flag(true);
    assert!(plain.should_increase_margins_on_focus().is_none());
}

#[test]
fn container_set_margins_shrinks_the_client_size() {
    let mut container = ContainerBase::new(Layout::absolute(2, 3, 40, 10), true);
    container.set_margins(1, 2, 3, 4);
    assert_eq!(container.margins.left, 1);
    assert_eq!(container.margins.top, 2);
    assert_eq!(container.margins.right, 3);
    assert_eq!(container.margins.bottom, 4);
    assert_eq!(container.client_size(), Size::new(0, 0));

    container.layout.update(80, 25);
    assert_eq!(container.size(), Size::new(40, 10));
    assert_eq!(container.client_size(), Size::new(36, 4));

    container.set_margins(30, 8, 30, 8);
    assert_eq!(container.size(), Size::new(40, 10));
    assert_eq!(container.client_size(), Size::new(0, 0));

    container.set_margins(0, 0, 0, 0);
    assert_eq!(container.client_size(), container.size());
}

fn check_container_add() {
    let mut container = ContainerBase::new(Layout::absolute(0, 0, 30, 12), true);
    container.handle = Handle::with_id(4, 8);

    let label = container.add(Label::new("Hint", Layout::absolute(1, 4, 10, 1)));
    assert!(!label.is_none());
    assert_eq!(container.children.len(), 1);
    assert!(container.get_focused_control().is_none());

    let button = container.add(Button::new("Ok", Layout::absolute(1, 1, 8, 2)));
    assert!(!button.is_none());
    assert!(button != label);
    assert_eq!(container.children.len(), 2);
    assert_eq!(container.children[1], button.cast::<()>());
    assert_eq!(container.get_focused_control(), button.cast::<()>());

    let stored = RuntimeManager::get().get_controls_mut().get(button.cast::<()>()).expect("button was registered");
    assert_eq!(stored.base().parent, container.handle);
    assert!(stored.base().can_receive_input());

    let note = container.add(Label::new("Note", Layout::absolute(1, 6, 10, 1)));
    assert_eq!(container.children.len(), 3);
    assert_eq!(container.get_focused_control(), button.cast::<()>());
    assert_eq!(container.children[2], note.cast::<()>());

    let cancel = container.add(Button::new("Cancel", Layout::absolute(12, 1, 10, 2)));
    assert_eq!(container.children.len(), 4);
    assert_eq!(container.get_focused_control(), cancel.cast::<()>());
    let cancel_parent = RuntimeManager::get()
        .get_controls_mut()
        .get(cancel.cast::<()>())
        .expect("cancel was registered")
        .base()
        .parent;
    assert_eq!(cancel_parent, container.handle);
}

#[test]
fn container_add_registers_children_and_focuses_the_latest_input_control() {
    App::new()
        .size(Size::new(40, 10))
        .debug_script("Paint.Enable(false)")
        .runtime_check(check_container_add)
        .run()
        .unwrap();
}
