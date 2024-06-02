use crate::prelude::*;


#[test]
fn check_creation() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x401057116C41B768)
    ";
    let mut a = App::debug(70, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:68,h:9");
    w.add(NumericSelector::<i32>::new(5,1,8,1,Layout::new("x:1,y:1,w:10"),numericselector::Flags::None));
    w.add(NumericSelector::<u32>::new(5,1,8,1,Layout::new("x:1,y:3,w:15"),numericselector::Flags::HideButtons));
    w.add(NumericSelector::<f32>::new(5.5,1.0,8.0,0.5,Layout::new("x:1,y:5,w:14"),numericselector::Flags::None));

    w.add(NumericSelector::<i32>::with_format(1000,1000,8000,100,Layout::new("x:22,y:1,w:20"),numericselector::Flags::None, numericselector::Format::DigitGrouping));
    w.add(NumericSelector::<u8>::with_format(50,0,100,1,Layout::new("x:22,y:3,w:20"),numericselector::Flags::None, numericselector::Format::Percentage));
    w.add(NumericSelector::<i8>::with_format(-50,-100,100,1,Layout::new("x:22,y:5,w:20"),numericselector::Flags::ReadOnly, numericselector::Format::Decimal));

    w.add(NumericSelector::<u128>::with_format(1_000_000,1000,80_000_000,1024,Layout::new("x:43,y:1,w:20"),numericselector::Flags::None, numericselector::Format::Size));
    w.add(NumericSelector::<u32>::with_format(0xFFEE,0,100000,1,Layout::new("x:43,y:3,w:20"),numericselector::Flags::None, numericselector::Format::Hex));


    a.add_window(w);
    a.run();
}


#[test]
fn check_create_procmacro() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x401057116C41B768)
    ";
    let mut a = App::debug(70, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:68,h:9");

    w.add(numericselector!("i32,5,1,8,1,x:1,y:1,w:10"));
    w.add(numericselector!("u32,5,1,8,1,x:1,y:3,w:15,flags:HideButtons"));
    w.add(numericselector!("f32,5.5,1.0,8.0,0.5,x:1,y:5,w:14"));

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
        CheckHash(0x401057116C41B768)
    ";
    let mut a = App::debug(70, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:68,h:9");

    w.add(numericselector!("i32,5,x:1,y:1,w:10"));
    w.add(numericselector!("u32,5,x:1,y:3,w:15,flags:HideButtons"));
    w.add(numericselector!("f32,5.5,x:1,y:5,w:14"));

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
        CheckHash(0xD4AD9150F1EA4E67)
        Mouse.Click(4,3,left)
        Paint('value: 4 (hover on minus)')   
        CheckHash(0x6391031B571469C)
        Mouse.Click(4,3,left)
        Paint('value: 3, minus button disabled')   
        CheckHash(0x5627CC54036AA831)
        Mouse.Click(4,3,left)
        Paint('value: 3, nothing changes')   
        CheckHash(0x5627CC54036AA831)
        Mouse.Click(20,3,left)
        Paint('value: 4 (hover on plus)')   
        CheckHash(0x68B6023C171AB5E4)
        Mouse.Click(20,3,left)
        Mouse.Click(20,3,left)
        Mouse.Click(20,3,left)
        Paint('value: 7, hover on plus')   
        CheckHash(0x23F47536083C71F3)
        Mouse.Click(20,3,left)
        Paint('value: 8, right button disabled')   
        CheckHash(0xC049BAA97BC1B892)
        Mouse.Click(20,3,left)
        Paint('value: 8, nothing changes')   
        CheckHash(0xC049BAA97BC1B892)
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
        CheckHash(0xD4AD9150F1EA4E67)
        Key.Pressed(Left)
        Paint('value: 4')   
        CheckHash(0x154343482A32A0E)
        Key.Pressed(Down)
        Paint('value: 3 (minus button disabled)')   
        CheckHash(0x5627CC54036AA831)
        Key.Pressed(Left)
        Paint('value: 3, nothing changes')   
        CheckHash(0x5627CC54036AA831)
        Key.Pressed(Right)
        Paint('value: 4')   
        CheckHash(0x154343482A32A0E)
        Key.Pressed(Up)
        Key.Pressed(Right)
        Key.Pressed(Up)
        Paint('value: 7')   
        CheckHash(0xB51CDD4CCB7E5475)
        Key.Pressed(Right)
        Paint('value: 8, right button disabled')   
        CheckHash(0xC049BAA97BC1B892)
        Key.Pressed(Up)
        Paint('value: 8, nothing changes')   
        CheckHash(0xC049BAA97BC1B892)
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
        CheckHash(0xD4AD9150F1EA4E67)
        Key.Pressed(Home)
        Paint('value: 3 (minus button disabled)')   
        CheckHash(0x5627CC54036AA831)
        Key.Pressed(End)
        Paint('value: 8, right button disabled')   
        CheckHash(0xC049BAA97BC1B892)
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
        CheckHash(0x63F43E387E2F60B7)
        Key.Pressed(Home)
        Paint('nothng changes')   
        CheckHash(0x63F43E387E2F60B7)
        Key.Pressed(Right)
        Key.Pressed(Up)
        Paint('nothng changes')   
        CheckHash(0x63F43E387E2F60B7)
        Key.Pressed(End)
        Paint('nothng changes')   
        CheckHash(0x63F43E387E2F60B7)
        Key.Pressed(Left)
        Key.Pressed(Down)
        Paint('nothng changes')   
        CheckHash(0x63F43E387E2F60B7)
        Mouse.Click(4,3,left)
        Paint('nothng changes')   
        CheckHash(0x63F43E387E2F60B7)
        Mouse.Click(20,3,left)
        Paint('nothng changes')   
        CheckHash(0x63F43E387E2F60B7)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,w:38,h:5,x:1,y:1,");
    w.add(numericselector!("i32,5,min:3,max:8,step:1,x:1,y:1,w:20,flags:ReadOnly")); 
    a.add_window(w);
    a.run();
}