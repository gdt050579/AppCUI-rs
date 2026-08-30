use crate::prelude::*;
use crate::ui::hslider::HSlider;
use crate::ui::hslider::Type;

#[test]
fn check_creation_all_types() {
    let script = "
        Paint.Enable(false)
        Paint('all five visual types')
        CheckHash(0x7E04695FCF180859)
    ";
    let mut a = App::new().size(Size::new(60, 12)).debug_script(script).run().unwrap();
    let mut w = window!("Title,x:0,y:0,w:58,h:11");

    w.add(HSlider::new(0, 10, 1, Type::Standard, layout!("x:1,y:0,w:24"), hslider::Flags::None));
    w.add(HSlider::new(0, 10, 1, Type::ProgressBar, layout!("x:1,y:1,w:24"), hslider::Flags::None));
    w.add(HSlider::new(0, 10, 1, Type::Inline, layout!("x:1,y:2,w:24"), hslider::Flags::None));
    w.add(HSlider::new(0, 10, 1, Type::Blocks, layout!("x:1,y:3,w:24"), hslider::Flags::None));
    w.add(HSlider::new(0, 10, 1, Type::Ruler, layout!("x:1,y:4,w:24"), hslider::Flags::None));

    let mut s = HSlider::new(0, 10, 1, Type::Standard, layout!("x:28,y:0,w:24"), hslider::Flags::None);
    s.set_value(5);
    w.add(s);
    let mut s = HSlider::new(0, 10, 1, Type::ProgressBar, layout!("x:28,y:1,w:24"), hslider::Flags::None);
    s.set_value(5);
    w.add(s);
    let mut s = HSlider::new(0, 10, 1, Type::Inline, layout!("x:28,y:2,w:24"), hslider::Flags::None);
    s.set_value(5);
    w.add(s);
    let mut s = HSlider::new(0, 10, 1, Type::Blocks, layout!("x:28,y:3,w:24"), hslider::Flags::None);
    s.set_value(5);
    w.add(s);
    let mut s = HSlider::new(0, 10, 1, Type::Ruler, layout!("x:28,y:4,w:24"), hslider::Flags::None);
    s.set_value(10);
    w.add(s);

    a.add_window(w);
    a.run();
}

#[test]
fn check_creation_all_flags_and_numeric_types() {
    let script = "
        Paint.Enable(false)
        Paint('every flag combination over several numeric types')
        CheckHash(0x9E2DF4B2EE3B3EE5)
    ";
    let mut a = App::new().size(Size::new(60, 12)).debug_script(script).run().unwrap();
    let mut w = window!("Title,x:0,y:0,w:58,h:11");

    w.add(HSlider::new(
        0,
        100,
        5,
        Type::Standard,
        layout!("x:1,y:0,w:26"),
        hslider::Flags::ShowValue,
    ));
    w.add(HSlider::new(
        0,
        100,
        5,
        Type::Standard,
        layout!("x:1,y:1,w:26"),
        hslider::Flags::ValueAsMarker,
    ));
    w.add(HSlider::new(
        0,
        100,
        5,
        Type::Standard,
        layout!("x:1,y:2,w:26"),
        hslider::Flags::ShowValue | hslider::Flags::ValueAsMarker,
    ));
    let mut s = HSlider::new(
        0,
        100,
        5,
        Type::Inline,
        layout!("x:1,y:3,w:26"),
        hslider::Flags::ShowValue | hslider::Flags::Ticks | hslider::Flags::ValueAsMarker,
    );
    s.set_ticks(5);
    w.add(s);

    w.add(HSlider::new(
        0u8,
        255u8,
        5u8,
        Type::Inline,
        layout!("x:29,y:0,w:26"),
        hslider::Flags::ShowValue,
    ));
    w.add(HSlider::new(
        -100i16,
        100i16,
        10i16,
        Type::Ruler,
        layout!("x:29,y:1,w:26"),
        hslider::Flags::ShowValue,
    ));
    w.add(HSlider::new(
        0i64,
        1000i64,
        50i64,
        Type::Blocks,
        layout!("x:29,y:2,w:26"),
        hslider::Flags::ShowValue,
    ));
    w.add(HSlider::new(
        0.0f32,
        1.0f32,
        0.1f32,
        Type::Inline,
        layout!("x:29,y:3,w:26"),
        hslider::Flags::ShowValue,
    ));
    w.add(HSlider::new(
        0.0f64,
        10.0f64,
        0.5f64,
        Type::Ruler,
        layout!("x:29,y:4,w:26"),
        hslider::Flags::ShowValue,
    ));
    w.add(HSlider::new(
        0u32,
        4000u32,
        100u32,
        Type::ProgressBar,
        layout!("x:29,y:5,w:26"),
        hslider::Flags::ShowValue,
    ));

    a.add_window(w);
    a.run();
}

#[test]
fn check_creation_procmacro() {
    let script = "
        Paint.Enable(false)
        Paint('controls built through the hslider! macro')
        CheckHash(0xD6E790A46F62F0CB)
    ";
    let mut a = App::new().size(Size::new(60, 12)).debug_script(script).run().unwrap();
    let mut w = window!("Title,x:0,y:0,w:58,h:11");

    w.add(hslider!("i32,0,10,1,x:1,y:0,w:24"));
    w.add(hslider!("class:i32,min:0,max:10,step:1,x:1,y:1,w:24,type:ProgressBar"));
    w.add(hslider!("i32,0,10,1,x:1,y:2,w:24,type:Inline"));
    w.add(hslider!("i32,0,10,1,x:1,y:3,w:24,type:Blocks"));
    w.add(hslider!("i32,0,10,1,x:1,y:4,w:24,type:Ruler"));

    w.add(hslider!("i32,0,100,5,x:28,y:0,w:26,flags:ShowValue"));

    let mut s = hslider!("i32,0,100,5,x:28,y:1,w:26,flags:Ticks,type:Ruler");
    s.set_ticks(5);
    w.add(s);

    w.add(hslider!("i32,0,100,5,x:28,y:2,w:26,flags:ValueAsMarker"));

    let mut s = hslider!("i32,0,100,5,x:28,y:3,w:26,flags:ShowValue|Ticks|ValueAsMarker,type:Inline");
    s.set_ticks(5);
    w.add(s);

    w.add(hslider!("f32,0f32,10f32,1.5f32,x:28,y:4,w:26,flags:ShowValue"));

    a.add_window(w);
    a.run();
}

#[test]
fn check_setters_and_getters() {
    let script = "
        Paint.Enable(false)
        Paint('after the setters ran')
        CheckHash(0x4CD4A4E8C2993CC6)
    ";
    let mut a = App::new().size(Size::new(60, 10)).debug_script(script).run().unwrap();
    let mut w = window!("Title,x:0,y:0,w:58,h:9");

    let mut s = HSlider::new(0, 10, 1, Type::Standard, layout!("x:1,y:0,w:26"), hslider::Flags::None);
    assert_eq!(s.min(), 0);
    assert_eq!(s.max(), 10);
    assert_eq!(s.step(), 1);
    assert_eq!(s.value(), 0);
    assert_eq!(s.ticks(), 0);

    s.set_value(7);
    assert_eq!(s.value(), 7);
    s.set_value(999);
    assert_eq!(s.value(), 10);
    s.set_value(-999);
    assert_eq!(s.value(), 0);

    s.set_step(3);
    assert_eq!(s.step(), 3);

    s.set_value(5);
    s.set_min(8);
    assert_eq!(s.min(), 8);
    assert_eq!(s.value(), 8);

    s.set_max(20);
    assert_eq!(s.max(), 20);
    s.set_value(20);
    s.set_max(15);
    assert_eq!(s.max(), 15);
    assert_eq!(s.value(), 15);

    s.set_ticks(6);
    assert_eq!(s.ticks(), 6);
    w.add(s);

    let mut s = HSlider::new(0, 100, 5, Type::Inline, layout!("x:1,y:2,w:26"), hslider::Flags::Ticks);
    s.set_ticks(5);
    s.set_value(37);
    assert_eq!(s.value(), 25);
    s.set_value(60);
    assert_eq!(s.value(), 50);
    s.set_value(100);
    assert_eq!(s.value(), 100);
    w.add(s);

    let mut s = HSlider::new(0, 100, 5, Type::Inline, layout!("x:1,y:4,w:26"), hslider::Flags::Ticks);
    s.set_ticks(1);
    assert_eq!(s.ticks(), 1);
    s.set_value(37);
    assert_eq!(s.value(), 37);
    s.set_ticks(0);
    assert_eq!(s.ticks(), 0);
    s.set_value(42);
    assert_eq!(s.value(), 42);
    w.add(s);

    let mut s = HSlider::new(0.0f32, 10.0f32, 0.5f32, Type::Ruler, layout!("x:1,y:6,w:26"), hslider::Flags::ShowValue);
    s.set_value(2.5);
    assert_eq!(s.value(), 2.5);
    s.set_value(100.0);
    assert_eq!(s.value(), 10.0);
    s.set_min(-5.0);
    assert_eq!(s.min(), -5.0);
    s.set_step(0.25);
    assert_eq!(s.step(), 0.25);
    w.add(s);

    a.add_window(w);
    a.run();
}

#[test]
fn check_keyboard_step() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('initial state, value 5')
        CheckHash(0x556B92331A5FB8F5)
        CheckCursor(hidden)
        Key.Pressed(Right)
        Paint('value 6')
        CheckHash(0x667E0BA40D9C3E7D)
        Key.Pressed(Right,3)
        Paint('value 9')
        CheckHash(0x4F8396875AB1AC4A)
        Key.Pressed(Right)
        Paint('value 10, clamped to max')
        CheckHash(0x3E2CEB4F3ECC826A)
        Key.Pressed(Right)
        Paint('still 10, nothing changes')
        CheckHash(0x3E2CEB4F3ECC826A)
        Key.Pressed(Left,5)
        Paint('value 5')
        CheckHash(0x556B92331A5FB8F5)
        Key.Pressed(Left,5)
        Paint('value 0, clamped to min')
        CheckHash(0xB3A3C3AEE8ADEB8C)
        Key.Pressed(Left)
        Paint('still 0, nothing changes')
        CheckHash(0xB3A3C3AEE8ADEB8C)
        Key.Pressed(A)
        Paint('unhandled key is ignored, nothing changes')
        CheckHash(0xB3A3C3AEE8ADEB8C)
    ";
    let mut a = App::new().size(Size::new(40, 10)).debug_script(script).run().unwrap();
    let mut w = window!("Title,x:1,y:1,w:38,h:5");
    let mut s = HSlider::new(0, 10, 1, Type::Standard, layout!("x:1,y:1,w:20"), hslider::Flags::ShowValue);
    s.set_value(5);
    w.add(s);
    a.add_window(w);
    a.run();
}

#[test]
fn check_keyboard_ticks() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('initial state, tick 0 -> value 0')
        CheckHash(0xBF3013E8676292DC)
        Key.Pressed(Right)
        Paint('tick 1 -> value 25')
        CheckHash(0x415A61B9805C1482)
        Key.Pressed(Right)
        Paint('tick 2 -> value 50')
        CheckHash(0xF2F9358408A8CD14)
        Key.Pressed(Right,2)
        Paint('tick 4 -> value 100')
        CheckHash(0x76B25ED943AC2F62)
        Key.Pressed(Right)
        Paint('already on the last tick, nothing changes')
        CheckHash(0x76B25ED943AC2F62)
        Key.Pressed(Left,4)
        Paint('back on tick 0 -> value 0')
        CheckHash(0xBF3013E8676292DC)
        Key.Pressed(Left)
        Paint('already on the first tick, nothing changes')
        CheckHash(0xBF3013E8676292DC)
    ";
    let mut a = App::new().size(Size::new(40, 10)).debug_script(script).run().unwrap();
    let mut w = window!("Title,x:1,y:1,w:38,h:5");
    let mut s = HSlider::new(
        0,
        100,
        5,
        Type::Inline,
        layout!("x:1,y:1,w:20"),
        hslider::Flags::Ticks | hslider::Flags::ShowValue,
    );
    s.set_ticks(5);
    w.add(s);
    a.add_window(w);
    a.run();
}

#[test]
fn check_keyboard_ticks_shorter_than_one_unit() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('tick 0 -> value 0')
        CheckHash(0x36931940B35D8A40)
        Key.Pressed(Right)
        Paint('tick 1 -> marker moved even though the value barely changed')
        CheckHash(0x3FB19E50A7D23F2)
        Key.Pressed(Right)
        Paint('tick 2')
        CheckHash(0xE9EC1E31272521C)
        Key.Pressed(Right)
        Paint('tick 3')
        CheckHash(0xB999FE93954B5AF7)
        Key.Pressed(Right)
        Paint('tick 4 -> value 3')
        CheckHash(0x7FDAC326A3F25224)
        Key.Pressed(Right)
        Paint('last tick, nothing changes')
        CheckHash(0x7FDAC326A3F25224)
    ";
    let mut a = App::new().size(Size::new(40, 10)).debug_script(script).run().unwrap();
    let mut w = window!("Title,x:1,y:1,w:38,h:5");
    let mut s = HSlider::new(
        0,
        3,
        1,
        Type::Ruler,
        layout!("x:1,y:1,w:20"),
        hslider::Flags::Ticks | hslider::Flags::ShowValue,
    );
    s.set_ticks(5);
    w.add(s);
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_click_and_drag() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('initial state, value 0')
        CheckHash(0xB3A3C3AEE8ADEB8C)
        Mouse.Hold(12,3,left)
        Paint('button held near the middle, marker is pressed')
        CheckHash(0xD7202032234A5BF)
        Mouse.Release(12,3,left)
        Paint('released, marker no longer pressed')
        CheckHash(0x667E0BA40D9C3E7D)
        Mouse.Click(20,3,left)
        Paint('clicked on the right end -> value 10')
        CheckHash(0x3E2CEB4F3ECC826A)
        Mouse.Click(5,3,left)
        Paint('clicked on the left end -> value 0')
        CheckHash(0xB3A3C3AEE8ADEB8C)
        Mouse.Click(3,3,left)
        Paint('clicked on the left cap, clamped to min')
        CheckHash(0xB3A3C3AEE8ADEB8C)
        Mouse.Click(22,3,left)
        Paint('clicked on the right cap, clamped to max')
        CheckHash(0x3E2CEB4F3ECC826A)
        Mouse.Drag(5,3,12,3)
        Paint('dragged from the left end to the middle')
        CheckHash(0x667E0BA40D9C3E7D)
        Mouse.Drag(12,3,20,3)
        Paint('dragged on to the right end')
        CheckHash(0x3E2CEB4F3ECC826A)
    ";
    let mut a = App::new().size(Size::new(40, 10)).debug_script(script).run().unwrap();
    let mut w = window!("Title,x:1,y:1,w:38,h:5");
    w.add(HSlider::new(0, 10, 1, Type::Standard, layout!("x:1,y:1,w:20"), hslider::Flags::ShowValue));
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_ticks() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('initial state, tick 0')
        CheckHash(0xBF3013E8676292DC)
        Mouse.Click(11,3,left)
        Paint('clicked the middle tick -> value 50')
        CheckHash(0xF2F9358408A8CD14)
        Mouse.Click(11,3,left)
        Paint('clicked the same tick again, nothing changes')
        CheckHash(0xF2F9358408A8CD14)
        Mouse.Click(13,3,left)
        Paint('clicked between two ticks, snapped to the nearer one -> value 75')
        CheckHash(0x190CB01261E0420C)
        Mouse.Click(18,3,left)
        Paint('clicked the last tick -> value 100')
        CheckHash(0x76B25ED943AC2F62)
        Mouse.Click(3,3,left)
        Paint('clicked the first tick -> value 0')
        CheckHash(0xBF3013E8676292DC)
        Mouse.Drag(3,3,22,3)
        Paint('dragged past the right end, clamped onto the last tick')
        CheckHash(0x76B25ED943AC2F62)
    ";
    let mut a = App::new().size(Size::new(40, 10)).debug_script(script).run().unwrap();
    let mut w = window!("Title,x:1,y:1,w:38,h:5");
    let mut s = HSlider::new(
        0,
        100,
        5,
        Type::Inline,
        layout!("x:1,y:1,w:20"),
        hslider::Flags::Ticks | hslider::Flags::ShowValue,
    );
    s.set_ticks(5);
    w.add(s);
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_enter_leave_and_wheel() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('initial state, value 5, mouse outside')
        CheckHash(0x556B92331A5FB8F5)
        Mouse.Move(12,3)
        Paint('mouse entered the control, hovered look + tooltip')
        CheckHash(0x90097CE53B4FC504)
        Mouse.Move(12,7)
        Paint('mouse left the control, back to the normal look')
        CheckHash(0x556B92331A5FB8F5)
        Mouse.Wheel(12,3,up,1)
        Paint('wheel up -> value 6')
        CheckHash(0xD7202032234A5BF)
        Mouse.Wheel(12,3,left,1)
        Paint('wheel left -> value 7')
        CheckHash(0x2E046F87EB731190)
        Mouse.Wheel(12,3,down,1)
        Paint('wheel down -> value 6')
        CheckHash(0xD7202032234A5BF)
        Mouse.Wheel(12,3,right,1)
        Paint('wheel right -> value 5')
        CheckHash(0x90097CE53B4FC504)
        Mouse.Wheel(12,3,up,10)
        Paint('wheel up ten times -> clamped to max')
        CheckHash(0x6E0FA6DF27ABD4EF)
        Mouse.Wheel(12,3,down,20)
        Paint('wheel down twenty times -> clamped to min')
        CheckHash(0x3C2BB3D7B28F3268)
        Mouse.Hold(12,3,left)
        Paint('button held, marker is pressed')
        CheckHash(0xD7202032234A5BF)
        Mouse.Move(12,7)
        Paint('left the control while holding, pressed state is cleared')
        CheckHash(0xD7202032234A5BF)
    ";
    let mut a = App::new().size(Size::new(40, 10)).debug_script(script).run().unwrap();
    let mut w = window!("Title,x:1,y:1,w:38,h:5");
    let mut s = HSlider::new(0, 10, 1, Type::Standard, layout!("x:1,y:1,w:20"), hslider::Flags::ShowValue);
    s.set_value(5);
    w.add(s);
    a.add_window(w);
    a.run();
}

#[test]
fn check_focus_and_disabled() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('initial state, the second slider is disabled')
        CheckHash(0x6C762A7443D358D5)
        Key.Pressed(Tab)
        Paint('focus wrapped around to the first slider')
        CheckHash(0x32DD4D6D63848DE5)
        Key.Pressed(Right)
        Paint('the first slider moved, the other two did not')
        CheckHash(0x65D04004BDCD8CE7)
        Mouse.Click(8,4,left)
        Paint('clicked the disabled slider, nothing changes')
        CheckHash(0x65D04004BDCD8CE7)
        Mouse.Move(8,4)
        Paint('hovering the disabled slider, still inactive')
        CheckHash(0x65D04004BDCD8CE7)
        Key.Pressed(Tab)
        Paint('tab skipped the disabled slider, focus is on the third one')
        CheckHash(0xF3DE8C8ED1D59147)
        Key.Pressed(Right)
        Paint('the third slider moved, the disabled one is still at 5')
        CheckHash(0xF9B3690B8000C592)
    ";
    let mut a = App::new().size(Size::new(40, 12)).debug_script(script).run().unwrap();
    let mut w = window!("Title,x:1,y:1,w:38,h:8");

    w.add(HSlider::new(0, 10, 1, Type::Standard, layout!("x:1,y:1,w:20"), hslider::Flags::ShowValue));

    let mut s = HSlider::new(0, 10, 1, Type::Standard, layout!("x:1,y:2,w:20"), hslider::Flags::ShowValue);
    s.set_value(5);
    s.set_enabled(false);
    w.add(s);

    w.add(HSlider::new(0, 10, 1, Type::Inline, layout!("x:1,y:3,w:20"), hslider::Flags::ShowValue));

    a.add_window(w);
    a.run();
}

#[test]
fn check_resize() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('initial width')
        CheckHash(0xCCA9DC1597CDF7F0)
        Resize(70,12)
        Paint('wider terminal, ticks spread out')
        CheckHash(0xF3F97D3A892C9938)
        Resize(30,12)
        Paint('narrow terminal, ticks pack together')
        CheckHash(0xA963F0DB82EDAF92)
        Resize(12,12)
        Paint('very narrow, the track nearly disappears')
        CheckHash(0x3FE22FDA69E28DA6)
        Key.Pressed(Right,2)
        Paint('the third slider stepped twice by step, still usable while narrow')
        CheckHash(0x1AF368FD72E68773)
        Resize(50,12)
        Paint('back to a usable width, the value survived the resize')
        CheckHash(0xAEA61BA6D4EDBA06)
    ";
    let mut a = App::new().size(Size::new(50, 12)).debug_script(script).run().unwrap();
    let mut w = window!("Test,x:0,y:0,w:100%,h:100%,flags: Sizeable");

    let mut s = HSlider::new(
        0,
        100,
        5,
        Type::Inline,
        layout!("x:0,y:0,w:100%"),
        hslider::Flags::Ticks | hslider::Flags::ShowValue,
    );
    s.set_ticks(5);
    w.add(s);

    let mut s = HSlider::new(
        0,
        100,
        5,
        Type::Standard,
        layout!("x:0,y:2,w:100%"),
        hslider::Flags::Ticks | hslider::Flags::ValueAsMarker,
    );
    s.set_ticks(11);
    w.add(s);

    w.add(HSlider::new(0, 100, 5, Type::Ruler, layout!("x:0,y:4,w:100%"), hslider::Flags::ShowValue));

    a.add_window(w);
    a.run();
}

#[test]
fn check_degenerate_geometry() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('degenerate sliders')
        CheckHash(0xD64325E17A48E995)
        Mouse.Click(4,3,left)
        Paint('clicked the one-column slider, nothing to move')
        CheckHash(0x23E0712CEC8F4ADD)
        Key.Pressed(Right)
        Paint('stepping the one-column slider changes nothing on screen')
        CheckHash(0x23E0712CEC8F4ADD)
        Mouse.Click(10,5,left)
        Paint('clicked the min == max slider, the value cannot move')
        CheckHash(0x54F6262E566323ED)
        Mouse.Click(10,7,left)
        Paint('clicked the zero-step slider, the raw position is used')
        CheckHash(0x941C5E48DCFCD47E)
        Key.Pressed(Right)
        Paint('stepping by zero changes nothing')
        CheckHash(0x941C5E48DCFCD47E)
    ";
    let mut a = App::new().size(Size::new(40, 12)).debug_script(script).run().unwrap();
    let mut w = window!("Title,x:1,y:1,w:38,h:9");

    w.add(HSlider::new(0, 10, 1, Type::Standard, layout!("x:1,y:1,w:3"), hslider::Flags::Ticks));
    w.add(HSlider::new(0, 10, 1, Type::Standard, layout!("x:5,y:1,w:4"), hslider::Flags::None));
    w.add(HSlider::new(0, 10, 1, Type::Inline, layout!("x:10,y:1,w:1"), hslider::Flags::None));
    w.add(HSlider::new(0, 10, 1, Type::Blocks, layout!("x:12,y:1,w:2"), hslider::Flags::None));

    let mut s = HSlider::new(
        5,
        5,
        1,
        Type::Standard,
        layout!("x:1,y:3,w:24"),
        hslider::Flags::Ticks | hslider::Flags::ShowValue,
    );
    s.set_ticks(5);
    w.add(s);

    w.add(HSlider::new(0, 10, 0, Type::Inline, layout!("x:1,y:5,w:24"), hslider::Flags::ShowValue));

    a.add_window(w);
    a.run();
}

#[test]
fn check_more_ticks_than_columns() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('forty ticks over a twenty column track')
        CheckHash(0xAEC2A472A58F7F66)
        Key.Pressed(Right,5)
        Paint('five ticks forward')
        CheckHash(0xAF15122073FA6E68)
        Key.Pressed(Right,40)
        Paint('walked past the end, sitting on the last tick')
        CheckHash(0x76B762C8B9ACDEFF)
        Key.Pressed(Left,40)
        Paint('walked back, sitting on the first tick')
        CheckHash(0xAEC2A472A58F7F66)
        Mouse.Click(12,3,left)
        Paint('clicked in the middle, snapped to the closest tick')
        CheckHash(0x69833AD0E1D3385C)
        Mouse.Drag(3,3,22,3)
        Paint('dragged across the whole track')
        CheckHash(0x76B762C8B9ACDEFF)
    ";
    let mut a = App::new().size(Size::new(40, 10)).debug_script(script).run().unwrap();
    let mut w = window!("Title,x:1,y:1,w:38,h:5");
    let mut s = HSlider::new(
        0,
        200,
        1,
        Type::Ruler,
        layout!("x:1,y:1,w:20"),
        hslider::Flags::Ticks | hslider::Flags::ShowValue,
    );
    s.set_ticks(40);
    w.add(s);
    a.add_window(w);
    a.run();
}

#[test]
fn check_events_i32() {
    #[Window(events: HSliderEvents<i32>, internal: true)]
    struct MyWin {}
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Title,x:1,y:1,w:38,h:5"),
            };
            let mut s = HSlider::new(0, 10, 1, Type::Standard, layout!("x:1,y:1,w:20"), hslider::Flags::ShowValue);
            s.set_value(5);
            w.add(s);
            w
        }
    }
    impl HSliderEvents<i32> for MyWin {
        fn on_value_changed(&mut self, _handle: Handle<HSlider<i32>>, value: i32) -> EventProcessStatus {
            let s = format!("val = {value}");
            self.base.set_title(&s);
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('initial state, value 5')
        CheckHash(0x556B92331A5FB8F5)
        Key.Pressed(Right)
        Paint('title reads val = 6')
        CheckHash(0xEAFD332D3AA37FDD)
        Mouse.Click(20,3,left)
        Paint('title reads val = 10')
        CheckHash(0xC10D5F94843C6D0A)
        Mouse.Click(20,3,left)
        Paint('already at max, no event is raised, title stays')
        CheckHash(0xC10D5F94843C6D0A)
        Mouse.Wheel(12,3,down,1)
        Paint('title reads val = 9')
        CheckHash(0xCB63B3FB3425E861)
    ";
    let mut a = App::new().size(Size::new(40, 10)).debug_script(script).run().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_events_f32_with_ticks() {
    #[Window(events: HSliderEvents<f32>, internal: true)]
    struct MyWin {}
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Title,x:1,y:1,w:38,h:5"),
            };
            let mut s = HSlider::new(
                0.0f32,
                1.0f32,
                0.1f32,
                Type::Inline,
                layout!("x:1,y:1,w:20"),
                hslider::Flags::Ticks | hslider::Flags::ShowValue,
            );
            s.set_ticks(5);
            w.add(s);
            w
        }
    }
    impl HSliderEvents<f32> for MyWin {
        fn on_value_changed(&mut self, _handle: Handle<HSlider<f32>>, value: f32) -> EventProcessStatus {
            let s = format!("val = {value}");
            self.base.set_title(&s);
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('initial state, value 0')
        CheckHash(0x1CF0F50F95BC27D3)
        Key.Pressed(Right)
        Paint('title reads val = 0.25')
        CheckHash(0x8D4B75DFA2D12856)
        Key.Pressed(Right,3)
        Paint('title reads val = 1')
        CheckHash(0xE759E3EBB51FB6D)
        Mouse.Click(3,3,left)
        Paint('back to the first tick, title reads val = 0')
        CheckHash(0xB211799884927B01)
    ";
    let mut a = App::new().size(Size::new(40, 10)).debug_script(script).run().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_char_sets() {
    let cs = Type::Standard.char_set();
    assert_eq!(cs.marker, 'X');
    assert_eq!(cs.left_marker, Some('['));
    assert_eq!(cs.right_marker, Some(']'));
    assert_eq!(cs.left_marker_line, '.');
    assert_eq!(cs.right_marker_line, '.');
    assert_eq!(cs.left_cap, Some('['));
    assert_eq!(cs.right_cap, Some(']'));
    assert_eq!(cs.tick, '|');

    let cs = Type::ProgressBar.char_set();
    assert_eq!(cs.marker, '>');
    assert_eq!(cs.left_marker, None);
    assert_eq!(cs.right_marker, None);
    assert_eq!(cs.left_marker_line, '=');
    assert_eq!(cs.right_marker_line, ' ');
    assert_eq!(cs.left_cap, Some('['));
    assert_eq!(cs.right_cap, Some(']'));

    let cs = Type::Inline.char_set();
    assert_eq!(cs.marker, '●');
    assert_eq!(cs.left_cap, None);
    assert_eq!(cs.right_cap, None);
    assert_eq!(cs.left_marker_line, '━');
    assert_eq!(cs.tick, '┿');
    assert_eq!(cs.left_tick, '┝');
    assert_eq!(cs.right_tick, '┥');

    let cs = Type::Blocks.char_set();
    assert_eq!(cs.marker, '█');
    assert_eq!(cs.left_marker_line, '█');
    assert_eq!(cs.right_marker_line, '░');
    assert_eq!(cs.left_cap, None);
    assert_eq!(cs.tick, '│');

    let cs = Type::Ruler.char_set();
    assert_eq!(cs.marker, '●');
    assert_eq!(cs.left_marker_line, '━');
    assert_eq!(cs.left_cap, None);
    assert_eq!(cs.tick, '┷');
    assert_eq!(cs.left_tick, '┕');
    assert_eq!(cs.right_tick, '┙');

    let t = Type::Ruler;
    let t2 = t;
    assert!(t == t2);
    assert!(Type::Standard != Type::Inline);
}
