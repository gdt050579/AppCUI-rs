use crate::prelude::*;

#[test]
fn check_radiobox_creation() {
    let script = "
        Paint.Enable(false)
        Paint('some radioboxes')   
        CheckHash(0xDCBAB7277D546411)  
        CheckCursor(13,5) 
        Key.Pressed(Space)
        Paint('third radiobox checked')  
        CheckHash(0x9896DC05AC121749) 
        CheckCursor(13,5) 
        Key.Pressed(Tab,2)
        Paint('second radiobox selected')  
        CheckHash(0x16DDCF9DB5BA6AB1) 
        CheckCursor(13,4) 
        Key.Pressed(Space)
        Paint('second radiobox checked')  
        CheckHash(0x640BD0AB0F16893D)
        CheckCursor(13,4) 
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(RadioBox::new("Selection &One", Layout::new("x:1,y:1,w:20"), true));
    w.add(RadioBox::new("Selection &Two", Layout::new("x:1,y:2,w:40"), false));
    w.add(RadioBox::new("Selection T&hree", Layout::new("x:1,y:3,w:40"), false));

    a.add_window(w);
    a.run();
}

#[test]
fn check_radiobox_macro_creation() {
    let script = "
        Paint.Enable(false)
        Paint('some radioboxes')   
        CheckHash(0xDCBAB7277D546411)  
        CheckCursor(13,5) 
        Key.Pressed(Space)
        Paint('third radiobox checked')  
        CheckHash(0x9896DC05AC121749) 
        CheckCursor(13,5) 
        Key.Pressed(Tab,2)
        Paint('second radiobox selected')  
        CheckHash(0x16DDCF9DB5BA6AB1) 
        CheckCursor(13,4) 
        Key.Pressed(Space)
        Paint('second radiobox checked')  
        CheckHash(0x640BD0AB0F16893D)
        CheckCursor(13,4) 
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(radiobox!("'Selection &One',x:1,y:1,w:20,select: true"));
    w.add(radiobox!("'Selection &Two',x:1,y:2,w:40"));
    w.add(radiobox!("caption:'Selection T&hree',x:1,y:3,w:40,selected: false"));

    a.add_window(w);
    a.run();
}