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