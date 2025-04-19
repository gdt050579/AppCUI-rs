use crate::{prelude::*, ui::numericselector::Format};
use super::number::Number;
use crate::ui::numericselector::NumericSelector;


#[test]
fn check_creation() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x9C2D78D1EAF6A3C)
    ";
    let mut a = App::debug(70, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:68,h:9");
    w.add(NumericSelector::<i32>::new(
        5,
        1,
        8,
        1,
        Layout::new("x:1,y:1,w:10"),
        numericselector::Flags::None,
    ));
    w.add(NumericSelector::<u32>::new(
        5,
        1,
        8,
        1,
        Layout::new("x:1,y:3,w:15"),
        numericselector::Flags::HideButtons,
    ));
    w.add(NumericSelector::<f32>::new(
        5.5,
        1.0,
        8.0,
        0.5,
        Layout::new("x:1,y:5,w:16"),
        numericselector::Flags::None,
    ));

    w.add(NumericSelector::<i32>::with_format(
        1000,
        1000,
        8000,
        100,
        Layout::new("x:22,y:1,w:20"),
        numericselector::Flags::None,
        numericselector::Format::DigitGrouping,
    ));
    w.add(NumericSelector::<u8>::with_format(
        50,
        0,
        100,
        1,
        Layout::new("x:22,y:3,w:20"),
        numericselector::Flags::None,
        numericselector::Format::Percentage,
    ));
    w.add(NumericSelector::<i8>::with_format(
        -50,
        -100,
        100,
        1,
        Layout::new("x:22,y:5,w:20"),
        numericselector::Flags::ReadOnly,
        numericselector::Format::Decimal,
    ));

    w.add(NumericSelector::<u128>::with_format(
        1_000_000,
        1000,
        80_000_000,
        1024,
        Layout::new("x:43,y:1,w:20"),
        numericselector::Flags::None,
        numericselector::Format::Size,
    ));
    w.add(NumericSelector::<u32>::with_format(
        0xFFEE,
        0,
        100000,
        1,
        Layout::new("x:43,y:3,w:20"),
        numericselector::Flags::None,
        numericselector::Format::Hex,
    ));

    a.add_window(w);
    a.run();
}

#[test]
fn check_create_procmacro() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x9C2D78D1EAF6A3C)
    ";
    let mut a = App::debug(70, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:68,h:9");

    w.add(numericselector!("i32,5,1,8,1,x:1,y:1,w:10"));
    w.add(numericselector!("u32,5,1,8,1,x:1,y:3,w:15,flags:HideButtons"));
    w.add(numericselector!("f32,5.5,1.0,8.0,0.5,x:1,y:5,w:16"));

    w.add(numericselector!("i32,1000,1000,8000,100,x:22,y:1,w:20,format:DigitGrouping"));
    w.add(numericselector!("u8,50,0,100,1,x:22,y:3,w:20,format:Percentage"));
    w.add(numericselector!("i8,-50,-100,100,1,x:22,y:5,w:20,flags:ReadOnly,format:Decimal"));

    w.add(numericselector!("u128,1000000,1000,80000000,1024,x:43,y:1,w:20,format:Size"));
    w.add(numericselector!("u32,0xFFEE,0,100000,1,x:43,y:3,w:20,format:Hex"));

    a.add_window(w);
    a.run();
}

#[test]
fn check_create_procmacro_defaults() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x9C2D78D1EAF6A3C)
    ";
    let mut a = App::debug(70, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:68,h:9");

    w.add(numericselector!("i32,5,x:1,y:1,w:10"));
    w.add(numericselector!("u32,5,x:1,y:3,w:15,flags:HideButtons"));
    w.add(numericselector!("f32,5.5,x:1,y:5,w:16"));

    w.add(numericselector!("i32,1000,x:22,y:1,w:20,format:DigitGrouping,min:1000"));
    w.add(numericselector!("u8,50,x:22,y:3,w:20,format:Percentage"));
    w.add(numericselector!("i8,-50,x:22,y:5,w:20,flags:ReadOnly,format:Decimal"));

    w.add(numericselector!("u128,1000000,1024,x:43,y:1,w:20,format:Size"));
    w.add(numericselector!("u32,0xFFEE,x:43,y:3,w:20,format:Hex"));

    a.add_window(w);
    a.run();
}

#[test]
fn check_min_max_by_mouse() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x21E8E7F45798DD63)
        Mouse.Click(4,3,left)
        Paint('value: 4 (hover on minus)')   
        CheckHash(0xE4A9807564EFFBF8)
        Mouse.Click(4,3,left)
        Paint('value: 3, minus button disabled')   
        CheckHash(0xAB2ADD34B5706401)
        Mouse.Click(4,3,left)
        Paint('value: 3, nothing changes')   
        CheckHash(0xAB2ADD34B5706401)
        Mouse.Click(20,3,left)
        Paint('value: 4 (hover on plus)')   
        CheckHash(0x17E44C83F40145FC)
        Mouse.Click(20,3,left)
        Mouse.Click(20,3,left)
        Mouse.Click(20,3,left)
        Paint('value: 7, hover on plus')   
        CheckHash(0x7C58ADC98EA500A3)
        Mouse.Click(20,3,left)
        Paint('value: 8, right button disabled')   
        CheckHash(0x3F03BA5FA9F80BAA)
        Mouse.Click(20,3,left)
        Paint('value: 8, nothing changes')   
        CheckHash(0x3F03BA5FA9F80BAA)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,w:38,h:5,x:1,y:1,");
    w.add(numericselector!("i32,5,min:3,max:8,step:1,x:1,y:1,w:20"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_min_max_by_keyboard() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x21E8E7F45798DD63)
        Key.Pressed(Left)
        Paint('value: 4')   
        CheckHash(0x8C7E9FBD384B89EE)
        Key.Pressed(Down)
        Paint('value: 3 (minus button disabled)')   
        CheckHash(0xAB2ADD34B5706401)
        Key.Pressed(Left)
        Paint('value: 3, nothing changes')   
        CheckHash(0xAB2ADD34B5706401)
        Key.Pressed(Right)
        Paint('value: 4')   
        CheckHash(0x8C7E9FBD384B89EE)
        Key.Pressed(Up)
        Key.Pressed(Right)
        Key.Pressed(Up)
        Paint('value: 7')   
        CheckHash(0x1EF0AF28867D456D)
        Key.Pressed(Right)
        Paint('value: 8, right button disabled')   
        CheckHash(0x3F03BA5FA9F80BAA)
        Key.Pressed(Up)
        Paint('value: 8, nothing changes')   
        CheckHash(0x3F03BA5FA9F80BAA)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,w:38,h:5,x:1,y:1,");
    w.add(numericselector!("i32,5,min:3,max:8,step:1,x:1,y:1,w:20"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_min_max_by_home_end() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x21E8E7F45798DD63)
        Key.Pressed(Home)
        Paint('value: 3 (minus button disabled)')   
        CheckHash(0xAB2ADD34B5706401)
        Key.Pressed(End)
        Paint('value: 8, right button disabled')   
        CheckHash(0x3F03BA5FA9F80BAA)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,w:38,h:5,x:1,y:1,");
    w.add(numericselector!("i32,5,min:3,max:8,step:1,x:1,y:1,w:20"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_readonly() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0xBE164F0A76471873)
        Key.Pressed(Home)
        Paint('nothng changes')   
        CheckHash(0xBE164F0A76471873)
        Key.Pressed(Right)
        Key.Pressed(Up)
        Paint('nothng changes')   
        CheckHash(0xBE164F0A76471873)
        Key.Pressed(End)
        Paint('nothng changes')   
        CheckHash(0xBE164F0A76471873)
        Key.Pressed(Left)
        Key.Pressed(Down)
        Paint('nothng changes')   
        CheckHash(0xBE164F0A76471873)
        Mouse.Click(4,3,left)
        Paint('nothng changes')   
        CheckHash(0xBE164F0A76471873)
        Mouse.Click(20,3,left)
        Paint('nothng changes')   
        CheckHash(0xBE164F0A76471873)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,w:38,h:5,x:1,y:1,");
    w.add(numericselector!("i32,5,min:3,max:8,step:1,x:1,y:1,w:20,flags:ReadOnly"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_edit_mode() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x21E8E7F45798DD63)
        Key.Pressed(Enter)
        Paint('Enter edit mode')   
        CheckHash(0xE9BCB46477286639)
        CheckCursor(5,3)
        Key.Pressed(Backspace)
        Paint('no value')   
        CheckHash(0x8E1A6A4BBEB2260C)
        CheckCursor(4,3)
        Key.TypeText('56')
        Paint('edit mode -> 56')   
        CheckHash(0xA91E775973377A37)
        CheckCursor(6,3)
        Key.Pressed(Enter)
        Paint('exit edit mode -> 56')   
        CheckHash(0x534C873C7A674B61)
        CheckCursor(hidden)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,w:38,h:5,x:1,y:1,");
    w.add(numericselector!("i32,5,min:3,max:80,step:1,x:1,y:1,w:20"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_exit_edit_mode() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x21E8E7F45798DD63)
        Key.Pressed(Enter)
        Paint('Enter edit mode')   
        CheckHash(0xE9BCB46477286639)
        CheckCursor(5,3)
        Key.Pressed(Escape)
        Paint('exit edit mode')   
        CheckHash(0x21E8E7F45798DD63)
        CheckCursor(hidden)
        Key.Pressed(Escape)
        Paint('window closed')
        CheckHash(0xAB06844D69595285)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,w:38,h:5,x:1,y:1,");
    w.add(numericselector!("i32,5,min:3,max:80,step:1,x:1,y:1,w:20"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_events() {
    #[Window(events: NumericSelectorEvents<i32>, internal: true)]
    struct MyWin {}
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Title,w:38,h:5,x:1,y:1,"),
            };
            w.add(numericselector!("i32,5,min:3,max:8,step:1,x:1,y:1,w:20"));
            w
        }
    }
    impl NumericSelectorEvents<i32> for MyWin {
        fn on_value_changed(&mut self, _handle: Handle<NumericSelector<i32>>, value: i32) -> EventProcessStatus {
            let s = format!("val = {}", value);
            self.base.set_title(&s);
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x21E8E7F45798DD63)
        Mouse.Click(4,3,left)
        Paint('New-value: 4')   
        CheckHash(0x50AB32FB27060802)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn test_number_signed_integers_write_to_string() {
    // Create a string to hold the formatted output
    let mut output = String::with_capacity(128);
    
    // Test i8
    let i8_value: i8 = 42;
    i8_value.write_to_string(&mut output, Format::Decimal);
    assert_eq!(output, "42");
    
    i8_value.write_to_string(&mut output, Format::Percentage);
    assert_eq!(output, "42%");
    
    i8_value.write_to_string(&mut output, Format::Hex);
    assert_eq!(output, "0x2A");
    
    // Test i16
    let i16_value: i16 = -1000;
    i16_value.write_to_string(&mut output, Format::Decimal);
    assert_eq!(output, "-1000");
    
    i16_value.write_to_string(&mut output, Format::DigitGrouping);
    assert_eq!(output, "-1,000");
    
    // Test i32
    let i32_value: i32 = 1_000_000;
    i32_value.write_to_string(&mut output, Format::Size);
    assert_eq!(output, "976 KB");
    
    // Test negative values
    let negative: i32 = -42;
    negative.write_to_string(&mut output, Format::Decimal);
    assert_eq!(output, "-42");
    
    negative.write_to_string(&mut output, Format::Hex);
    assert_eq!(output, "-0x2A");
}

#[test]
fn test_number_unsigned_integers_write_to_string() {
    // Create a string to hold the formatted output
    let mut output = String::with_capacity(128);
    
    // Test u8
    let u8_value: u8 = 255;
    u8_value.write_to_string(&mut output, Format::Decimal);
    assert_eq!(output, "255");
    
    u8_value.write_to_string(&mut output, Format::Hex);
    assert_eq!(output, "0xFF");
    
    // Test u32
    let u32_value: u32 = 1_000_000;
    u32_value.write_to_string(&mut output, Format::DigitGrouping);
    assert_eq!(output, "1,000,000");
    
    // Test u64
    let u64_value: u64 = 0xDEADBEEF;
    u64_value.write_to_string(&mut output, Format::Hex);
    assert_eq!(output, "0xDEADBEEF");
    
    // Test usize
    let usize_value: usize = 50;
    usize_value.write_to_string(&mut output, Format::Percentage);
    assert_eq!(output, "50%");
}

#[test]
fn test_number_floating_point_write_to_string() {
    // Create a string to hold the formatted output
    let mut output = String::with_capacity(128);
    
    // Test f32
    let f32_value: f32 = 3.14159;
    f32_value.write_to_string(&mut output, Format::Decimal);
    assert_eq!(output, "3.14");
    
    // Test f64
    let f64_value: f64 = 2.71828;
    f64_value.write_to_string(&mut output, Format::Decimal);
    assert_eq!(output, "2.71");
    
    
    // Test negative float
    let negative_float: f64 = -1.5;
    negative_float.write_to_string(&mut output, Format::Decimal);
    assert_eq!(output, "-1.50");
}

#[test]
fn test_number_size_format() {
    // Create a string to hold the formatted output
    let mut output = String::with_capacity(128);
    
    // Test bytes
    let bytes: u32 = 900;
    bytes.write_to_string(&mut output, Format::Size);
    assert_eq!(output, "900 B");
    
    // Test kilobytes
    let kilobytes: u32 = 2 * 1024;
    kilobytes.write_to_string(&mut output, Format::Size);
    assert_eq!(output, "2 KB");
    
    // Test megabytes
    let megabytes: u32 = 3 * 1024 * 1024;
    megabytes.write_to_string(&mut output, Format::Size);
    assert_eq!(output, "3 MB");
    
    // Test gigabytes (using u64 to handle larger values)
    let gigabytes: u64 = 4u64 * 1024 * 1024 * 1024;
    gigabytes.write_to_string(&mut output, Format::Size);
    assert_eq!(output, "4 GB");
    
    // Test terabytes (using u64 to handle larger values)
    let terabytes: u64 = 5u64 * 1024 * 1024 * 1024 * 1024;
    terabytes.write_to_string(&mut output, Format::Size);
    assert_eq!(output, "5 TB");
}

#[test]
fn test_number_large_numbers() {
    // Create a string to hold the formatted output
    let mut output = String::with_capacity(128);
    
    // Test large i128
    let large_i128: i128 = i128::MAX / 2;
    large_i128.write_to_string(&mut output, Format::DigitGrouping);
    assert!(output.contains(","));
    
    // Test large u128
    let large_u128: u128 = u128::MAX / 2;
    large_u128.write_to_string(&mut output, Format::Decimal);
    assert!(output.len() > 30);
}

#[test]
fn test_number_boundary_values() {
    // Create a string to hold the formatted output
    let mut output = String::with_capacity(128);
    
    // Test minimum values
    let min_i8: i8 = i8::MIN;
    min_i8.write_to_string(&mut output, Format::Decimal);
    assert_eq!(output, "-128");
    
    let min_i32: i32 = i32::MIN;
    min_i32.write_to_string(&mut output, Format::Decimal);
    assert_eq!(output, "-2147483648");
    
    // Test maximum values
    let max_u8: u8 = u8::MAX;
    max_u8.write_to_string(&mut output, Format::Decimal);
    assert_eq!(output, "255");
    
    let max_u16: u16 = u16::MAX;
    max_u16.write_to_string(&mut output, Format::Decimal);
    assert_eq!(output, "65535");
}

