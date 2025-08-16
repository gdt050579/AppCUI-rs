use crate::prelude::*;

#[test]
fn check_create() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0x89DCEE13ABCD4574)  
        Key.Pressed(Space)
        Paint('2. Expanded (3 rows)')   
        CheckHash(0xDDEF06221F6E1482)  
        Key.Pressed(Space)
        Paint('3. Back to initial state')   
        CheckHash(0x89DCEE13ABCD4574)  
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:40,h:9"), window::Flags::None);
    w.add(CharPicker::new(Some('a'),LayoutBuilder::new().x(1).y(1).width(20).build()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_create_macro() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0x89DCEE13ABCD4574)  
        Key.Pressed(Space)
        Paint('2. Expanded (3 rows)')   
        CheckHash(0x460C52EC7CFAF066)  
        Key.Pressed(Space)
        Paint('3. Back to initial state')   
        CheckHash(0x89DCEE13ABCD4574)  
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:9");
    w.add(charpicker!("a,x:1,y:1,w:20"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_create_macro_with_code() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0x89DCEE13ABCD4574)  
        Key.Pressed(Space)
        Paint('2. Expanded (3 rows)')   
        CheckHash(0x460C52EC7CFAF066)  
        Key.Pressed(Space)
        Paint('3. Back to initial state')   
        CheckHash(0x89DCEE13ABCD4574)  
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:9");
    w.add(charpicker!("x:1,y:1,w:20,code: 97"));
    a.add_window(w);
    a.run();
}


#[test]
fn check_ctor_one_set() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xC8F1ACF560D85904)  
        Key.Pressed(Space)
        Paint('2. Expanded down')   
        CheckHash(0xBA5C4FBD4C2E24E6)  
        Key.Pressed(Down)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:8");
    let set = charpicker::Set::new("Vowals", "AEIOUaeiou").unwrap();
    w.add(CharPicker::with_set(Some('e'), layout!("x:1,y:1,w:20"), set));
    a.add_window(w);
    a.run();
}

#[test]
fn check_expand_up_and_down() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (focus on second control)')   
        CheckHash(0x79B93797825FB3D4)  
        Key.Pressed(Space)
        Paint('2. Expanded up (second control)')   
        CheckHash(0x198BBD28407011)  
        Key.Pressed(Tab)
        Paint('3. Focus on first charpicker')   
        CheckHash(0x1D6ABC5029E07D2C)  
        Key.Pressed(Space)
        Paint('4. Expanded down (first control)')   
        CheckHash(0xD9C071575AE05055)  
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Title,d:f");
    w.add(charpicker!("l:1,t:1,r:1,char: 0"));
    w.add(charpicker!("l:1,b:1,r:1,code: 41"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigation() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xECFFDFB22EF3A9AF)  
        Key.Pressed(Space)
        Paint('2. Expanded down')   
        CheckHash(0x65004700C4283CED)  
        Key.Pressed(Down)
        Paint('3. Letter <E> selected')   
        CheckHash(0x39FF15AAEDD4F1C1)  
        Key.Pressed(Left)
        Paint('4. Letter <D> selected')   
        CheckHash(0x35E683866FA4AD79)  
        Key.Pressed(Right,3)
        Paint('5. Letter <G> selected')   
        CheckHash(0x6D1218E62C360CD)  
        Key.Pressed(Down,1)
        Paint('6. Letter <K> selected')   
        CheckHash(0x8126AA535DD818E0)  
        Key.Pressed(Down,3)
        Paint('7. Letter <W> selected')   
        CheckHash(0x1094432CC98C84D8)  
        Key.Pressed(Right,2)
        Paint('8. Letter <Y> selected')   
        CheckHash(0x1B312457A3405AE0)  
        Key.Pressed(Left,4)
        Key.Pressed(Up,3)
        Key.Pressed(Left,8)
        Paint('9. Letter <A> selected')   
        CheckHash(0x65004700C4283CED)  
        Key.Pressed(Right)
        Key.Pressed(PageUp)
        Paint('10. Char <6> selected (row:1, line:1)')   
        CheckHash(0x68F81497AFD3755A)  
        Key.Pressed(PageUp)
        Paint('11. Char <*> selected')   
        CheckHash(0x553786F42017271C)  
        Key.Pressed(PageUp)
        Paint('12. Char < > (space) selected')   
        CheckHash(0x4ECC9CE7E41B55DF)  
        Key.Pressed(Right,2)
        Key.Pressed(PageDown)
        Paint('13. Char <.> selected')   
        CheckHash(0x7950CC10C1D8B48C)  
        Key.Pressed(PageDown)
        Paint('14. Char <:> (line: 3, row: 3) selected')   
        CheckHash(0xE3994812C8D775BD)  
        Key.Pressed(PageDown)
        Paint('15. Letter <F> selected')   
        CheckHash(0x240DAE0EAD2556BD)  
        Key.Pressed(Home)
        Paint('16. Char < > (space) selected (back home)')   
        CheckHash(0x4ECC9CE7E41B55DF)  
        Key.Pressed(End)
        Paint('17. Char <~> (last char) selected')   
        CheckHash(0x6FBE8396B77AED3D)  
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:8");
    w.add(charpicker!("x:1,y:1,w:15,sets:[Ascii],char:A"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigation_scroll_view() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xECFFDFB22EF3A9AF)  
        Key.Pressed(Space)
        Paint('2. Expanded down')   
        CheckHash(0x65004700C4283CED)  
        Key.Pressed(Down)
        Paint('3. Letter <E> selected')   
        CheckHash(0x39FF15AAEDD4F1C1)  
        Key.Pressed(Alt+Down)
        Paint('4. Letter <E> selected, scroll starts from <E>')   
        CheckHash(0xA4B17004684B8ED)  
        Key.Pressed(Ctrl+Down,2)
        Paint('5. Letter <E> selected (not visible), scroll starts from <M>')   
        CheckHash(0x1708AF1B9438F911)  
        Key.Pressed(Ctrl+Up)
        Paint('6. Letter <E> selected (not visible), scroll starts from <I>')   
        CheckHash(0x3246559469614989)  
        Key.Pressed(Alt+Up,3)
        Paint('7. Letter <E> selected, scroll starts from <=>')   
        CheckHash(0xB3208D72921A80DD)  
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:8");
    w.add(charpicker!("x:1,y:1,w:15,sets:[Ascii],char:A"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigation_escape() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xECFFDFB22EF3A9AF)  
        Key.Pressed(Space)
        Paint('2. Expanded down')   
        CheckHash(0x65004700C4283CED)  
        Key.Pressed(Escape)
        Paint('3. Packed again')   
        CheckHash(0xECFFDFB22EF3A9AF)  
        Key.Pressed(Escape)
        Paint('4. Exit window')   
        CheckHash(0x97CFA8E09EF9879D)  
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:8");
    w.add(charpicker!("x:1,y:1,w:15,sets:[Ascii],char:A"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigation_chars() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xECFFDFB22EF3A9AF)  
        Key.Pressed(Space)
        Paint('2. Expanded down')   
        CheckHash(0x65004700C4283CED)  
        Key.TypeText('0')
        Paint('3. Character <0> is not set (row:1, col:1)')   
        CheckHash(0xD6AF1A0F970524E6)  
        Key.TypeText(':')
        Paint('4. Character <:> is not set (row:3, col:3)')   
        CheckHash(0xE3994812C8D775BD)  
        Key.TypeText('z')
        Paint('5. Character <z> is not set (row:3, col:3)')   
        CheckHash(0x5B5C442256534B1)  
        Key.TypeText('G')
        Paint('6. Character <G> is not set (row:1, col:3)')   
        CheckHash(0xD8A1E8478C65CC79)  
        Key.TypeText('▒')
        Paint('7. Nothing chagest as ▒ is not in the set')   
        CheckHash(0xD8A1E8478C65CC79)  
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:8");
    w.add(charpicker!("x:1,y:1,w:15,sets:[Ascii],char:A"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigation_change_sets() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xECFFDFB22EF3A9AF)  
        Key.Pressed(Space)
        Paint('2. Expanded down (current set: Ascii)')   
        CheckHash(0xD721C8C4FDA0F9F5)  
        Key.Pressed(Alt+Right)
        Paint('3. Move to set Braille')   
        CheckHash(0xB88956750EA2542B)  
        Key.Pressed(Ctrl+Right)
        Paint('4. Move to set Animals')   
        CheckHash(0x48BCC9693003963F)  
        Key.Pressed(Alt+Right)
        Paint('5. Last set (no change)')   
        CheckHash(0x48BCC9693003963F)  
        Key.Pressed(Ctrl+Left)
        Paint('6. Move to set Braille')   
        CheckHash(0xB88956750EA2542B)  
        Key.Pressed(Alt+Left)
        Paint('7. Move to set Ascii (first character (space) is selecte)')   
        CheckHash(0x2BD74D5589035BC7)          
        Key.Pressed(Alt+Left)
        Paint('8. Nothing changes (already at first set)')   
        CheckHash(0x2BD74D5589035BC7)          
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:8");
    w.add(charpicker!("x:1,y:1,w:15,sets:[Ascii,Braille,Animals],char:A"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigation_paste() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xECFFDFB22EF3A9AF)  
        Key.Pressed(Space)
        Paint('2. Expanded down (current set: Ascii)')   
        CheckHash(0xD721C8C4FDA0F9F5)  
        Key.Pressed(Ctrl+V)
        Paint('3. Rmains the same (nothing to paste)')   
        CheckHash(0xD721C8C4FDA0F9F5)  
        Clipboard.SetText('z')
        Key.Pressed(Ctrl+V)
        Paint('4. Seleced character is <z>')   
        CheckHash(0x2696253A2ED859F5)  
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:8");
    w.add(charpicker!("x:1,y:1,w:15,sets:[Ascii,Braille,Animals],char:A"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigation_copy() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xECFFDFB22EF3A9AF)  
        Key.Pressed(Space)
        Paint('2. Expanded down (current set: Ascii)')   
        CheckHash(0xD721C8C4FDA0F9F5)  
        Key.Pressed(Right,6)
        Paint('3. Current char is <G>')   
        CheckHash(0xB319CD11C2DD3B65)  
        Key.Pressed(Ctrl+C)
        Paint('4. Character copied')   
        CheckHash(0xB319CD11C2DD3B65) 
        CheckClipboardText('G') 
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:8");
    w.add(charpicker!("x:1,y:1,w:15,sets:[Ascii,Braille,Animals],char:A"));
    a.add_window(w);
    a.run();
}