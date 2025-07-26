use crate::prelude::*;

use super::EnumSelector;

#[derive(Copy, Clone, Eq, PartialEq, EnumSelector)]
enum Options {
    A,
    B,
    C,
}

#[derive(Copy, Clone, Eq, PartialEq, EnumSelector)]
enum Cars {
    #[VariantInfo(description="list of cars :)")]
    Dacia,
    #[VariantInfo(description="list of cars :)")]
    Toyota,
    #[VariantInfo(description="list of cars :)", name: "BMW")]
    Bmw,
    #[VariantInfo(description="list of cars :)")]
    Mazda,
    #[VariantInfo(description="list of cars :)")]
    Mercedes,
    #[VariantInfo(description="list of cars :)")]
    Ford,
    #[VariantInfo(description="list of cars :)")]
    Ferrari,
    #[VariantInfo(description="list of cars :)")]
    Lamborghini,
    #[VariantInfo(description="list of cars :)")]
    Skoda,
    #[VariantInfo(description="list of cars :)")]
    Renault,
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
    let mut w = window!("Title,a:c,w:36,h:7");
    w.add(Selector::<Options>::new(
        Some(Options::B),
        layout!("x:1,y:1,w:10"),
        selector::Flags::None,
    ));
    w.add(Selector::<Options>::new(
        None,
        layout!("x:1,y:3,w:10"),
        selector::Flags::AllowNoneVariant,
    ));
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
    let mut w = window!("Title,a:c,w:36,h:7");
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
    let mut w = window!("Title,a:c,w:36,h:7");
    w.add(selector!("Options,value:B,x:1,y:1,w:20"));
    w.add(selector!("Cars,value:Ferrari,x:1,y:3,w:20"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_movement_keys_expand() {
    let script = "
        Paint.Enable(false)
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

#[test]
fn check_quick_search_expanded() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(Space)
        Paint('Mazda,Mercedes,Ford,[Ferari]')   
        CheckHash(0xEFFDEFE5806F6E75)
        Key.Pressed(M)
        Paint('[Mazda],Mercedes,Ford,Ferari')   
        CheckHash(0x6056BD2A0F71A6A3)
        Key.Pressed(M)
        Paint('Mazda,[Mercedes],Ford,Ferari')   
        CheckHash(0xDD4C5FFCF03E4BD6)
        Key.Pressed(M)
        Paint('[Mazda],Mercedes,Ford,Ferari')   
        CheckHash(0x6056BD2A0F71A6A3)
        Key.Pressed(F)
        Paint('Mazda,Mercedes,[Ford],Ferari')   
        CheckHash(0x2B2EC2641591323B)
        Key.Pressed(F)
        Paint('Mazda,Mercedes,Ford,[Ferari]')   
        CheckHash(0xEFFDEFE5806F6E75)
        Key.Pressed(R)
        Paint('Ferari,Lamborghini,Skoda,[Renault]')   
        CheckHash(0x385754AAEE74FE3A)
        Key.Pressed(D)
        Paint('[Dacia],Toyota,BMW,Mazda')   
        CheckHash(0x668B915504B8FAAC)
        Key.Pressed(X)
        Paint('[Dacia],Toyota,BMW,Mazda - no car with `X`')   
        CheckHash(0x668B915504B8FAAC)

    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:36,h:7");
    w.add(selector!("Cars,value:Ferrari,x:1,y:0,w:30"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_quick_search_packed() {
    let script = "
        Paint.Enable(false)
        Paint('1.Ferari')   
        CheckHash(0x4D5DD26CD625E51C)
        Key.Pressed(M)
        Paint('2.Mazda')   
        CheckHash(0xD93C83FFFEE3151A)
        Key.Pressed(M)
        Paint('3.Mercedes')   
        CheckHash(0x139A7D66A92482B3)
        Key.Pressed(M)
        Paint('4.Mazda')   
        CheckHash(0xD93C83FFFEE3151A)
        Key.Pressed(F)
        Paint('5.Ford')   
        CheckHash(0xF6AF2FA25E408A22)
        Key.Pressed(F)
        Paint('6.Ferari')   
        CheckHash(0x4D5DD26CD625E51C)
        Key.Pressed(R)
        Paint('7.Renault')   
        CheckHash(0x3FC6F7D52AD87990)
        Key.Pressed(D)
        Paint('8.Dacia')   
        CheckHash(0xF3BF1B9540CD5A5B)
        Key.Pressed(X)
        Paint('9.Dacia - no car with `X`')   
        CheckHash(0xF3BF1B9540CD5A5B)

    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:36,h:7");
    w.add(selector!("Cars,value:Ferrari,x:1,y:0,w:30"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_escape_key() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (closed)')   
        CheckHash(0x4D5DD26CD625E51C)
        Key.Pressed(Enter)
        Paint('Expanded')   
        CheckHash(0xEFFDEFE5806F6E75)
        Key.Pressed(Escape)
        Paint('Back to initial state')   
        CheckHash(0x4D5DD26CD625E51C)
        Key.Pressed(Escape)
        Paint('Now the window closes (empty desktop)')   
        CheckHash(0xAB06844D69595285)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:36,h:7");
    w.add(selector!("Cars,value:Ferrari,x:1,y:0,w:30"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_events() {
    #[Window(events = SelectorEvents<Cars>,internal: true)]
    struct MyWin {}
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("x:1,y:1,w:30,h:8,caption:Win"),
            };
            w.add(selector!("enum:Cars,value:Renault,x:1,y:1,w:20,flags:AllowNoneVariant"));
            w
        }
    }
    impl SelectorEvents<Cars> for MyWin {
        fn on_selection_changed(&mut self, _: Handle<Selector<Cars>>, value: Option<Cars>) -> EventProcessStatus {
            if let Some(val) = value {
                self.set_title(val.name());
            } else {
                self.set_title("None");
            }
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0x9881715A3EAA1684)
        Key.Pressed(Up)
        Paint('Window title: Skoda')   
        CheckHash(0x11FDA2E612720F15)
        Key.Pressed(Space)
        Paint('Window title: Skoda, expanded')   
        CheckHash(0xAFFC1660D08EBD8C)
        Key.Pressed(Down)
        Paint('Window title: Renault, expanded')   
        CheckHash(0x152BB3E5C33E4764)
        Key.Pressed(Down)
        Paint('Window title: None, expanded')   
        CheckHash(0x8077DD177D7FB53B)
        Key.Pressed(Down,4)
        Paint('Window title: None, expanded (nothing changes)')   
        CheckHash(0x8077DD177D7FB53B)
        Key.Pressed(Escape)
        Paint('Window title: None')   
        CheckHash(0xD99A74E9CD19C9BE)
        Key.Pressed(Home)
        Paint('Window title: Dacia')   
        CheckHash(0xD22ED284CAF9BD9)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_none_value_scenario() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (closed)')   
        CheckHash(0x4D5DD26CD625E51C)
        Key.Pressed(Enter)
        Paint('Expanded')   
        CheckHash(0xEFFDEFE5806F6E75)
        Key.Pressed(End)
        Paint('Lamborghini,Skoda,Renault,[None]')   
        CheckHash(0x3542CEA13D858F82)
        Key.Pressed(Down,10)
        Paint('Lamborghini,Skoda,Renault,[None] (nothing changes)')   
        CheckHash(0x3542CEA13D858F82)
        Key.Pressed(Up)
        Paint('Lamborghini,Skoda,[Renault],None')   
        CheckHash(0x136ABAE2F97FAC5D)
        Key.Pressed(Escape)
        Paint('Renault')
        CheckHash(0x3FC6F7D52AD87990)
        Key.Pressed(Down)
        Paint('None')
        CheckHash(0xA2FD5F80F94CD11B)        
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:36,h:7");
    w.add(selector!("Cars,value:Ferrari,x:1,y:0,w:30,flags:AllowNoneVariant"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_up_down_buttons() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (closed)')   
        CheckHash(0x4D5DD26CD625E51C)
        Mouse.Click(16,1,left)     
        Paint('Expanded (Mazda,Mercedes,Ford,[Ferrari])')   
        CheckHash(0xEFFDEFE5806F6E75)
        Mouse.Move(16,4)
        Paint('Hovered over Mercedes (Mazda,Mercedes,Ford,[Ferrari])')   
        CheckHash(0xF4C3E1C47C2FFB45)
        Mouse.Move(17,2)
        Paint('Hovered over Up Button (Mazda,Mercedes,Ford,[Ferrari])')   
        CheckHash(0x19079C5374C7B9FD)
        Mouse.Hold(16,2,left)
        Paint('Pressed Up Button (Mazda,Mercedes,[Ford],Ferrari)')   
        CheckHash(0x59696BB058528C5F)
        Mouse.Release(16,2,left)
        Paint('Release Up Button (Mazda,Mercedes,[Ford],Ferrari)')   
        CheckHash(0x258A40181DFDFE03)
        Mouse.Click(16,2,left)
        Mouse.Click(17,2,left)
        Mouse.Click(18,2,left)
        Paint('Now ([BMW],Mazda,Mercedes,Ford)')   
        CheckHash(0x5ACB598F5ECEC9D9)
        Mouse.Click(17,2,left)
        Paint('Now ([Toyota],BMW,Mazda,Mercedes)')   
        CheckHash(0x8ADAE9B5E15D42A6)
        Mouse.Click(17,2,left)
        Paint('Up button inactive ([Dacia],Toyota,BMW,Mazda)')   
        CheckHash(0x668B915504B8FAAC)
        Mouse.Click(17,2,left)
        Paint('Up button inactive ([Dacia],Toyota,BMW,Mazda) - nothing changes')   
        CheckHash(0x668B915504B8FAAC)
        Key.Pressed(PageDown,2)
        Paint('Ford,Ferrari,Lamborghini,[Skoda]')   
        CheckHash(0x6FE834D546EB9957)
        Mouse.Click(17,7,left)
        Paint('Ferrari,Lamborghini,Skoda,[Renault]')   
        CheckHash(0xC6C48AF71E8F54DA)
        Mouse.Click(17,7,left)
        Paint('DownButton inactive - Lamborghini,Skoda,Renault,[None]')   
        CheckHash(0x3542CEA13D858F82)
        Mouse.Click(17,7,left)
        Paint('DownButton inactive - Lamborghini,Skoda,Renault,[None] - nothing changes')   
        CheckHash(0x3542CEA13D858F82)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:36,h:7");
    w.add(selector!("Cars,value:Ferrari,x:1,y:0,w:30,flags:AllowNoneVariant"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_click() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (closed)')   
        CheckHash(0x4D5DD26CD625E51C)
        Mouse.Click(16,1,left)     
        Paint('Expanded (Mazda,Mercedes,Ford,[Ferrari])')   
        CheckHash(0xEFFDEFE5806F6E75)
        Mouse.Click(16,3,left)
        Paint('Packed -> [Mazda],Mercedes,Ford,Ferrari)')   
        CheckHash(0xD93C83FFFEE3151A)
        Mouse.Click(16,1,left)
        Paint('Expanded -> [Mazda],Mercedes,Ford,Ferrari)')   
        CheckHash(0x6056BD2A0F71A6A3)
        Mouse.Move(16,4)
        Paint('Hovered over Mercedes -> [Mazda],Mercedes,Ford,Ferrari)')   
        CheckHash(0x1F679526B9D9C0B3)
        Mouse.Click(16,4,left)
        Paint('Packed -> Mazda,[Mercedes],Ford,Ferrari)')   
        CheckHash(0x139A7D66A92482B3)
        Mouse.Click(30,1,left)
        Paint('Expanded -> Mazda,[Mercedes],Ford,Ferrari)')   
        CheckHash(0xDD4C5FFCF03E4BD6)
        Mouse.Click(0,0,left)
        Paint('Packed -> Mazda,[Mercedes],Ford,Ferrari)')   
        CheckHash(0x139A7D66A92482B3)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:36,h:7");
    w.add(selector!("Cars,value:Ferrari,x:1,y:0,w:30,flags:AllowNoneVariant"));
    a.add_window(w);
    a.run();
}


#[test]
fn check_mouse_wheel() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (closed)')   
        CheckHash(0x4D5DD26CD625E51C)
        Mouse.Click(16,1,left)     
        Paint('Expanded (Mazda,Mercedes,Ford,[Ferrari])')   
        CheckHash(0xEFFDEFE5806F6E75)
        Mouse.Move(20,4)
        Mouse.Wheel(20,4,up,1)
        Paint('Mazda,Mercedes,[Ford],Ferrari)')   
        CheckHash(0x9DD4AFF02416E60B)    
        Mouse.Wheel(20,4,up,1)
        Paint('Mazda,[Mercedes],Ford,Ferrari)')   
        CheckHash(0xDD4C5FFCF03E4BD6)    
        Mouse.Wheel(20,4,up,10)
        Paint('[Dacia],Toyota,BMW,Mazda')   
        CheckHash(0x668B915504B8FAAC)    
        Mouse.Wheel(20,4,down,1)
        Paint('Dacia,[Toyota],BMW,Mazda')   
        CheckHash(0x243C9F25A37EE0EA)    
        Mouse.Wheel(20,4,down,3)
        Paint('Toyota,BMW,Mazda,[Mercedes]')   
        CheckHash(0xC7E09CB60639C054)    
        Mouse.Wheel(20,4,down,10)
        Paint('Lamborghini,Skoda,Renault,[None]')   
        CheckHash(0x3542CEA13D858F82)    
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:36,h:7");
    w.add(selector!("Cars,value:Ferrari,x:1,y:0,w:30,flags:AllowNoneVariant"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_description_on_hover() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (closed)')   
        CheckHash(0x371A30F83F6AE4E) 
        Mouse.Move(20,1)
        Paint('Tootltip with description for cars')   
        CheckHash(0x58E1F498C9A61D16) 
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:36,h:7");
    w.add(selector!("Cars,value:Ferrari,x:1,y:0,w:30,flags:AllowNoneVariant"));
    w.add(selector!("enum: Options,x:1,y:3,w:20,flags: AllowNoneVariant"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_value_set_value_clear_value() {
    #[derive(Copy, Clone, Eq, PartialEq, EnumSelector)]
    enum TestEnum {
        #[VariantInfo(description = "First option", name = "First")]
        Option1,
        
        #[VariantInfo(description = "Second option", name = "Second")]
        Option2,
        
        #[VariantInfo(description = "Third option", name = "Third")]
        Option3,
    }
    
    let script = "
        Paint.Enable(false)
        Paint('Initial state with Option2 (Second) selected')
        CheckHash(0x70EB221DBE8E6B0D)
        Key.Pressed(F1)
        Paint('Value changed to Option1 (First)')
        CheckHash(0x9FD131F018F686AE)
        Key.Pressed(F2)
        Paint('Value changed to Option3 (Third)')
        CheckHash(0x48FE8E14449A2776)
        Key.Pressed(F3)
        Paint('Value cleared (None selected)')
        CheckHash(0xB31AEF8F0870D4EA)
        Key.Pressed(F4)
        Paint('Value set back to Option2 (Second)')
        CheckHash(0x70EB221DBE8E6B0D)
    ";

    #[Window(events=CommandBarEvents+SelectorEvents<TestEnum>,commands:A+B+C+D, internal:true)]
    struct MyWin {
        selector_handle: Handle<Selector<TestEnum>>,
        info_handle: Handle<Label>,
    }
    
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,x:1,y:1,w:78,h:10"),
                selector_handle: Handle::None,
                info_handle: Handle::None,
            };
            
            // Create selector with Option2 as initial value and AllowNoneVariant flag
            let sel = Selector::<TestEnum>::new(
                Some(TestEnum::Option2),
                layout!("x:1,y:1,w:30"),
                selector::Flags::AllowNoneVariant
            );
            
            // Create a label to display value information
            let l = Label::new("", layout!("x:1,y:3,w:76,h:5"));
            
            w.selector_handle = w.add(sel);
            w.info_handle = w.add(l);
            
            // Update the info label with initial values
            w.update_info_label();
            
            w
        }
        
        fn update_info_label(&mut self) {
            let h = self.selector_handle;
            let mut info_text = String::new();
            
            if let Some(sel) = self.control(h) {
                // Check if we have a value using try_value()
                match sel.try_value() {
                    Some(value) => {
                        // We have a value, so we can also use value() safely
                        info_text.push_str(&format!("try_value(): Some({})\n", value.name()));
                        info_text.push_str(&format!("value(): {}\n", sel.value().name()));
                        info_text.push_str(&format!("Description: {}", value.description()));
                    },
                    None => {
                        info_text.push_str("try_value(): None\n");
                        info_text.push_str("value(): Would panic!\n");
                        info_text.push_str("Description: N/A");
                    }
                }
            } else {
                info_text = "Error: Selector not found".to_string();
            }
            
            // Update the label
            let h = self.info_handle;
            if let Some(label) = self.control_mut(h) {
                label.set_caption(&info_text);
            }
        }
        
        // Set value to Option1
        fn set_to_option1(&mut self) {
            let h = self.selector_handle;
            if let Some(sel) = self.control_mut(h) {
                sel.set_value(TestEnum::Option1);
            }
            self.update_info_label();
        }
        
        // Set value to Option3
        fn set_to_option3(&mut self) {
            let h = self.selector_handle;
            if let Some(sel) = self.control_mut(h) {
                sel.set_value(TestEnum::Option3);
            }
            self.update_info_label();
        }
        
        // Clear the value
        fn clear_value(&mut self) {
            let h = self.selector_handle;
            if let Some(sel) = self.control_mut(h) {
                sel.clear_value();
            }
            self.update_info_label();
        }
        
        // Set value back to Option2
        fn set_to_option2(&mut self) {
            let h = self.selector_handle;
            if let Some(sel) = self.control_mut(h) {
                sel.set_value(TestEnum::Option2);
            }
            self.update_info_label();
        }
    }
    
    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Set Option1", mywin::Commands::A);
            commandbar.set(key!("F2"), "Set Option3", mywin::Commands::B);
            commandbar.set(key!("F3"), "Clear Value", mywin::Commands::C);
            commandbar.set(key!("F4"), "Set Option2", mywin::Commands::D);
        }
        
        fn on_event(&mut self, command_id: mywin::Commands) {
            match command_id {
                mywin::Commands::A => self.set_to_option1(),
                mywin::Commands::B => self.set_to_option3(),
                mywin::Commands::C => self.clear_value(),
                mywin::Commands::D => self.set_to_option2(),
            }
        }
    }
    
    impl SelectorEvents<TestEnum> for MyWin {
        fn on_selection_changed(&mut self, _handle: Handle<Selector<TestEnum>>, _value: Option<TestEnum>) -> EventProcessStatus {
            self.update_info_label();
            EventProcessStatus::Processed
        }
    }
    
    let mut a = App::debug(80, 12, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}


#[test]
#[should_panic(expected = "You can not instantiate a selector with `None` value without setting the flags `AllowNoneVariant`. Have you forgot to do this ?")]
fn check_create_with_panic() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x0)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,a:c,w:36,h:7");
    let s = Selector::<Options>::new(
        None,
        layout!("x:1,y:1,w:10"),
        selector::Flags::None,
    );
    w.add(s);
    a.add_window(w);
    a.run();
}

#[test]
#[should_panic(expected = "You can not clear the value of a selector unless flag `AllowNoneVariant` was set. Have you forgot to do this ?")]
fn check_clear_value_with_panic() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x0)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,a:c,w:36,h:7");
    let mut s = Selector::<Options>::new(
        Some(Options::B),
        layout!("x:1,y:1,w:10"),
        selector::Flags::None,
    );
    s.clear_value(); // should panic here
    w.add(s);
    a.add_window(w);
    a.run();
}

#[test]
fn check_on_default_action() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (closed)')   
        CheckHash(0x686CC81380DCE865) 
        Key.Pressed(Alt+S)
        Paint('2. Selector is opened (default action)')   
        CheckHash(0xEFFDEFE5806F6E75) 
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:36,h:7");
    let mut s = selector!("Cars,value:Ferrari,x:1,y:0,w:30,flags:AllowNoneVariant");
    s.set_hotkey(key!("Alt+S"));
    w.add(s);
    w.add(button!("Button,x:1,y:2,w:10"));
    a.add_window(w);
    a.run();
}