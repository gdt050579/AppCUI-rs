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
#[derive(Copy, Clone, Eq, PartialEq)]
enum Cars {
    Dacia,
    Toyota,
    BMW,
    Mazda,
    Mercedes,
    Ford,
    Ferrari,
    Lamborghini,
    Skoda,
    Renault,
}
impl EnumSelector for Cars {
    const COUNT: u32 = 10;

    fn from_index(index: u32) -> Option<Self>
    where
        Self: Sized,
    {
        match index {
            0 => Some(Cars::Dacia),
            1 => Some(Cars::Toyota),
            2 => Some(Cars::BMW),
            3 => Some(Cars::Mazda),
            4 => Some(Cars::Mercedes),
            5 => Some(Cars::Ford),
            6 => Some(Cars::Ferrari),
            7 => Some(Cars::Lamborghini),
            8 => Some(Cars::Skoda),
            9 => Some(Cars::Renault),

            _ => None,
        }
    }

    fn name(&self) -> &'static str {
        match self {    
            Cars::Dacia => "Dacia",
            Cars::Toyota => "Toyota",
            Cars::BMW => "BMW",
            Cars::Mazda => "Mazda",
            Cars::Mercedes => "Mercedes",
            Cars::Ford => "Ford",
            Cars::Ferrari => "Ferrari",
            Cars::Lamborghini => "Lamborghini",
            Cars::Skoda => "Skoda",
            Cars::Renault => "Renault",
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

#[test]
fn check_expand_pack() {
    let script = "
        Paint.Enable(false)
        Paint('Ferari (focus), B (not-focus)')   
        CheckHash(0xEEC6C631F8745D86)
        Key.Pressed(Tab)
        Paint('Ferari (not-focus), B (focus)')   
        CheckHash(0xFDB5C05259D5E41A)
        Key.Pressed(Space)
        Paint('Ferari (not-focus), B (focus+expanded)')   
        CheckHash(0x1F5694B15DBA6047)
        Key.Pressed(Down)
        Paint('Ferari (not-focus), C (focus+expanded)')   
        CheckHash(0x252F7D4697BB1DF)
        Key.Pressed(Tab)
        // control C should be packed
        Paint('Ferari (focus), C (not-focus)')   
        CheckHash(0xDB04242D9F2D9B3)
        Key.Pressed(Enter)
        Paint('Ferari (focus-expanded), C (not-focus)')   
        CheckHash(0xC9B87A3C51E97477)
        Key.Pressed(Home)
        Paint('Dacia (focus-expanded), C (not-focus)')   
        CheckHash(0xC560A1D708693D8C)
        Key.Pressed(Tab)
        // Control Dacia shoud be packed
        Paint('Dacia (not-focus), C (focus)')   
        CheckHash(0xE947715F06C97410)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:36,h:7");
    w.add(selector!("Options,value:B,x:1,y:1,w:20"));
    w.add(selector!("Cars,value:Ferrari,x:1,y:3,w:20"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_movement_keys_expand() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.Pressed(Space)
        Paint('Mazda,Mercedes,Ford,[Ferari]')   
        CheckHash(0xEFFDEFE5806F6E75)
        Key.Pressed(Up)
        Paint('Mazda,Mercedes,[Ford],Ferari')   
        CheckHash(0x2B2EC2641591323B)
        Key.Pressed(Up,2)
        Paint('[Mazda],Mercedes,Ford,Ferari')   
        CheckHash(0x6056BD2A0F71A6A3)
        Key.Pressed(Up,2)
        Paint('[Toyota],BMW,Mazda,Mercedes')   
        CheckHash(0x7806AE51A75CF8FE)
        Key.Pressed(Down,3)
        Paint('Toyota,BMW,Mazda,[Mercedes]')   
        CheckHash(0xC7E09CB60639C054)
        Key.Pressed(Down)
        Paint('BMW,Mazda,Mercedes,[Ford]')   
        CheckHash(0x40729635B9A03C0E)
        Key.Pressed(Home)
        Paint('[Dacia],Toyota,BMW,Mazda')   
        CheckHash(0x668B915504B8FAAC)
        Key.Pressed(Up)
        Paint('[Dacia],Toyota,BMW,Mazda (nothing changes)')   
        CheckHash(0x668B915504B8FAAC)
        Key.Pressed(End)
        Paint('Ferarri,Lamborghini,Skoda,[Renault]')   
        CheckHash(0x385754AAEE74FE3A)
        Key.Pressed(Down)
        Paint('Ferarri,Lamborghini,Skoda,[Renault] (nothing changes)')   
        CheckHash(0x385754AAEE74FE3A)
        Key.Pressed(Up,2)
        Paint('Ferarri,[Lamborghini],Skoda,Renault')   
        CheckHash(0xB22AD47B8725FAAD)
        Key.Pressed(PageUp)
        Paint('[Mazda],Mercedes,Ford,Ferrar')   
        CheckHash(0x6056BD2A0F71A6A3)
        Key.Pressed(PageUp)
        Paint('[Dacia],Toyota,BMW,Mazda')   
        CheckHash(0x668B915504B8FAAC)
        Key.Pressed(PageDown)
        Paint('Toyota,BMW,Mazda,[Mercedes]')   
        CheckHash(0xC7E09CB60639C054)
        Key.Pressed(PageDown)
        Paint('Ford,Ferrari,Lamborghini,[Skoda]')   
        CheckHash(0x6FE834D546EB9957)
        Key.Pressed(PageDown)
        Paint('Ferrari,Lamborghini,Skoda,[Renault]')   
        CheckHash(0x385754AAEE74FE3A)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:36,h:7");
    w.add(selector!("Cars,value:Ferrari,x:1,y:0,w:30"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_movement_keys_packed() {
    let script = "
        Paint.Enable(false)
        Paint('1.Ferari')   
        CheckHash(0x4D5DD26CD625E51C)
        Key.Pressed(Up)
        Paint('2.Ford')   
        CheckHash(0xF6AF2FA25E408A22)
        Key.Pressed(Up,2)
        Paint('3.Mazda')   
        CheckHash(0xD93C83FFFEE3151A)
        Key.Pressed(Up,2)
        Paint('4.Toyota')   
        CheckHash(0x412BB7B9C2C736F1)
        Key.Pressed(Down,3)
        Paint('5.Mercedes')   
        CheckHash(0x139A7D66A92482B3)
        Key.Pressed(Down)
        Paint('6.Ford')   
        CheckHash(0xF6AF2FA25E408A22)
        Key.Pressed(Home)
        Paint('7.Dacia')   
        CheckHash(0xF3BF1B9540CD5A5B)
        Key.Pressed(Up)
        Paint('8.Dacia (nothing changes)')   
        CheckHash(0xF3BF1B9540CD5A5B)
        Key.Pressed(End)
        Paint('9.Renault')   
        CheckHash(0x3FC6F7D52AD87990)
        Key.Pressed(Down)
        Paint('10.Renault (nothing changes)')   
        CheckHash(0x3FC6F7D52AD87990)
        Key.Pressed(Up,2)
        Paint('11.Lamborghini')   
        CheckHash(0xA441EC0C730FA1CF)
        Key.Pressed(PageUp)
        // pack mode --> PageUp moves one element at time
        Paint('12.Ferrari')   
        CheckHash(0x4D5DD26CD625E51C)
        Key.Pressed(PageUp)
        Paint('13.Ford')   
        CheckHash(0xF6AF2FA25E408A22)
        Key.Pressed(PageDown,2)
        // pack mode --> PageDown moves one element at time
        Paint('14.Lamborghini')   
        CheckHash(0xA441EC0C730FA1CF)
        Key.Pressed(PageDown)
        Paint('15.Skoda')   
        CheckHash(0xD46BF7EA6F08B293)
        Key.Pressed(PageDown,10)
        Paint('16.Renault')   
        CheckHash(0x3FC6F7D52AD87990)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:36,h:7");
    w.add(selector!("Cars,value:Ferrari,x:1,y:0,w:30"));
    a.add_window(w);
    a.run();
}


#[test]
fn check_scroll_view() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(Space)
        Paint('Mazda,Mercedes,Ford,[Ferari]')   
        CheckHash(0xEFFDEFE5806F6E75)
        Key.Pressed(Ctrl+Down)
        Paint('Mercedes,Ford,[Ferari],Lamborghini')   
        CheckHash(0xD56D26E736D7B5DC)
        Key.Pressed(Ctrl+Down,2)
        Paint('[Ferari],Lamborghini,Skoda,Renault')   
        CheckHash(0x6650D85CD5812D3A)
        Key.Pressed(Ctrl+Down,2)
        Paint('Ferari,Lamborghini,[Skoda],Renault')   
        CheckHash(0xC148E101DB80BD9)
        Key.Pressed(Ctrl+Up)
        Paint('Ford,Ferari,Lamborghini,[Skoda]')   
        CheckHash(0x6FE834D546EB9957)
        Key.Pressed(Ctrl+Up)
        Paint('Mercedes,Ford,Ferari,[Lamborghini]')   
        CheckHash(0x53D0DC3BE6A4E553)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:36,h:7");
    w.add(selector!("Cars,value:Ferrari,x:1,y:0,w:30"));
    a.add_window(w);
    a.run();
}




// search keys (packed si unpacked)
// esc to be tested (packs)
// events -> on value changed
// mouse --> a lot of scenarios
// test with None value as well
// suport de iconite (1 sau 2 caractere)
// descriere pentru mouse over