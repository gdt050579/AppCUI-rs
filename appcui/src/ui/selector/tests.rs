use crate::prelude::*;

use super::EnumSelector;

#[derive(Copy,Clone,Eq,PartialEq)]
enum Options { A, B, C}
impl EnumSelector for Options {
    const COUNT: u32 = 3;

    fn from_index(index: u32) -> Option<Self> where Self: Sized {
        match index {
            0 => Some(Options::A),
            1 => Some(Options::B),
            2 => Some(Options::C),
            _ => None
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Options::A => "A",
            Options::B => "B",
            Options::C => "C",
        }
    }
}

#[test]
fn check_creation() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x8E402A80F606DBF1)
        Key.Pressed(Up)
        Paint('B si C (focus on second)')   
        CheckHash(0xEB02B5C40538168)
        Key.Pressed(Up,4)
        Paint('B si A (focus on second)')   
        CheckHash(0xF7DF3DA7C92F6506)
        Key.Pressed(Down,4)
        Paint('B si None (focus on second)')   
        CheckHash(0x8E402A80F606DBF1)
        Key.Pressed(Tab)
        Paint('B si None (focus on first)')   
        CheckHash(0x4C0C850934401A61)
        Key.Pressed(Up)
        Paint('A si None (focus on first)')   
        CheckHash(0xBDA211EA6A9555A2)
        Key.Pressed(Up)
        Paint('A si None (focus on first) - nothing changes')   
        CheckHash(0xBDA211EA6A9555A2)
        Key.Pressed(Down,4)
        Paint('C si None (focus on first)')   
        CheckHash(0xBC99794D98A96264)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:36,h:7");
    w.add(Selector::<Options>::new(Some(Options::B),Layout::new("x:1,y:1,w:10"),selector::Flags::None));
    w.add(Selector::<Options>::new(None,Layout::new("x:1,y:3,w:10"),selector::Flags::AllowNoneVariant));
    a.add_window(w);
    a.run();
}

#[test]
fn check_create_with_macro() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x8E402A80F606DBF1)
        Key.Pressed(Up)
        Paint('B si C (focus on second)')   
        CheckHash(0xEB02B5C40538168)
        Key.Pressed(Up,4)
        Paint('B si A (focus on second)')   
        CheckHash(0xF7DF3DA7C92F6506)
        Key.Pressed(Down,4)
        Paint('B si None (focus on second)')   
        CheckHash(0x8E402A80F606DBF1)
        Key.Pressed(Tab)
        Paint('B si None (focus on first)')   
        CheckHash(0x4C0C850934401A61)
        Key.Pressed(Up)
        Paint('A si None (focus on first)')   
        CheckHash(0xBDA211EA6A9555A2)
        Key.Pressed(Up)
        Paint('A si None (focus on first) - nothing changes')   
        CheckHash(0xBDA211EA6A9555A2)
        Key.Pressed(Down,4)
        Paint('C si None (focus on first)')   
        CheckHash(0xBC99794D98A96264)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:36,h:7");
    w.add(selector!("Options,value:B,x:1,y:1,w:10"));
    w.add(selector!("Options,x:1,y:3,w:10,flags: AllowNoneVariant"));
    a.add_window(w);
    a.run();
}