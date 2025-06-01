use crate::prelude::*;

#[test]
fn check_create() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')
        CheckHash(0xBE767D638014E39A)
    ";

    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Dates,d:c,w:50,h:11");
    w.add(DatePicker::new("2024-06-13", Layout::new("x:1,y:1,w:19")));
    w.add(DatePicker::new("2024-06-13", Layout::new("x:1,y:3,w:16")));
    w.add(DatePicker::new("2024-06-13", Layout::new("x:1,y:5,w:14")));
    w.add(DatePicker::new("2024-06-13", Layout::new("x:1,y:7,w:10")));
    w.add(DatePicker::new("2024-06-13", Layout::new("x:23,y:1,w:23")));
    w.add(DatePicker::new("2024-06-13", Layout::new("x:23,y:3,w:7")));
    w.add(DatePicker::new("2024-06-13", Layout::new("x:23,y:5,w:6")));
    a.add_window(w);
    a.run();
}

#[test]
fn check_on_hover() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')
        CheckHash(0xBE767D638014E39A)
        Mouse.Move(10,8)
        Paint('Mouse hover')
        CheckHash(0x5F940267F4247C82)
        Mouse.Move(31,6)
        Paint('Mouse hover 2')
        CheckHash(0x93797768E0658943)
        
    ";

    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Dates,d:c,w:50,h:11");
    w.add(DatePicker::new("2024-06-13", Layout::new("x:1,y:1,w:19")));
    w.add(DatePicker::new("2024-06-13", Layout::new("x:1,y:3,w:16")));
    w.add(DatePicker::new("2024-06-13", Layout::new("x:1,y:5,w:14")));
    w.add(DatePicker::new("2024-06-13", Layout::new("x:1,y:7,w:10")));
    w.add(DatePicker::new("2024-06-13", Layout::new("x:23,y:1,w:23")));
    w.add(DatePicker::new("2024-06-13", Layout::new("x:23,y:3,w:7")));
    w.add(DatePicker::new("2024-06-13", Layout::new("x:23,y:5,w:6")));
    a.add_window(w);
    a.run();
}

#[test]
fn check_expand() {
    let script = "
        Paint.Enable(false)
        Paint('Init')
        CheckHash(0x1100da21cab3453)
        Mouse.Move(25,9)
        Mouse.Drag(25,9,25,2)
        Mouse.Move(30,5)
        Mouse.Click(30,5,left)
        Paint('State_expand_down')
        CheckHash(0x6ccfdecb94f80585)
        Mouse.Move(34,2)
        Mouse.Drag(34,2,34,17)
        Mouse.Move(37,20)
        Mouse.Click(37,20,left)
        Paint('State_expand_top')
        CheckHash(0x5f66995970a9aa50)

    ";

    let mut a = App::debug(60, 25, script).build().unwrap();
    let mut w = window!("Dates,d:c,w:25,h:6");
    w.add(DatePicker::new("2024-06-13", Layout::new("d:c,w:19")));
    a.add_window(w);
    a.run();
}

#[test]
fn check_hovers() {
    let script = "
        Paint.Enable(false)
        Mouse.Drag(30,9,30,4)
        Paint('State_init')
        CheckHash(0x953c1eccb3b87233)
        Mouse.Click(29,7,left)
        Mouse.Move(24,9)
        Paint('State_double_arrow_left')
        CheckHash(0xa616044e29413d05)
        Mouse.Move(26,7)
        Mouse.Click(26,7,left)
        Mouse.Move(27,7)
        Mouse.Click(27,7,left)
        Mouse.Move(26,9)
        Paint('State_arrow_left_year')
        CheckHash(0x46af07784f59f86d)
        Mouse.Move(27,7)
        Mouse.Click(27,7,left)
        Mouse.Move(28,7)
        Mouse.Click(28,7,left)
        Mouse.Move(33,9)
        Paint('State_arrow_right_year')
        CheckHash(0xd9f41585513d720d)
        Mouse.Move(38,7)
        Mouse.Click(38,7,left)
        Mouse.Move(37,7)
        Mouse.Click(37,7,left)
        Mouse.Move(36,9)
        Paint('State_double_arrow_right')
        CheckHash(0x5b9bf42dfc65635)
        Mouse.Move(33,7)
        Mouse.Click(33,7,left)
        Mouse.Click(33,7,left)
        Mouse.Move(41,9)
        Paint('State_arrow_month_left')
        CheckHash(0xd0c18993aa112d2d)
        Mouse.Move(36,7)
        Mouse.Click(36,7,left)
        Mouse.Move(38,7)
        Mouse.Click(38,7,left)
        Mouse.Move(47,9)
        Paint('State_arrow_month_right')
        CheckHash(0x9eea1c0f81538e1d)
        Mouse.Move(38,7)
        Mouse.Click(38,7,left)
        Mouse.Click(38,7,left)
        Mouse.Move(40,16)
        Paint('State_hover_date')
        CheckHash(0x1125aa027ca09f95)
    ";

    let mut a = App::debug(60, 25, script).build().unwrap();
    let mut w = window!("Dates,d:c,w:25,h:6");
    w.add(DatePicker::new("2024-06-13", Layout::new("d:c,w:19")));
    a.add_window(w);
    a.run();
}

#[test]
fn check_procmacro() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')
        CheckHash(0xBE767D638014E39A)
    ";

    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Dates,d:c,w:50,h:11");
    w.add(datepicker!("2024-06-13, x:1,y:1,w:19"));
    w.add(datepicker!("2024-06-13, x:1,y:3,w:16"));
    w.add(datepicker!("2024-06-13, x:1,y:5,w:14"));
    w.add(datepicker!("2024-06-13, x:1,y:7,w:10"));
    w.add(datepicker!("2024-06-13, x:23,y:1,w:23"));
    w.add(datepicker!("2024-06-13, x:23,y:3,w:7"));
    w.add(datepicker!("2024-06-13, x:23,y:5,w:6"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_buttons() {
    let script = "
        Paint.Enable(false)
        Mouse.Move(28,9)
        Mouse.Drag(28,9,28,3)
        Mouse.Move(29,6)
        Mouse.Click(29,6,left)
        Mouse.Move(24,8)
        Mouse.Click(24,8,left)
        Paint('Button_years_1')
        CheckHash(0xe5df54cad2d7be8e)
        Mouse.Move(25,6)
        Mouse.Click(25,6,left)
        Mouse.Click(25,6,left)
        Mouse.Move(26,8)
        Mouse.Click(26,8,left)
        Paint('Button_years_2')
        CheckHash(0xb7402fa256f0885e)
        Mouse.Move(27,6)
        Mouse.Click(27,6,left)
        Mouse.Click(27,6,left)
        Mouse.Move(33,8)
        Mouse.Click(33,8,left)
        Paint('Button_years_3')
        CheckHash(0xec33bab3a1c76b84)
        Mouse.Move(31,6)
        Mouse.Click(31,6,left)
        Mouse.Click(31,6,left)
        Mouse.Move(35,8)
        Mouse.Click(35,8,left)
        Mouse.Move(36,8)
        Paint('Button_years_4')
        CheckHash(0xe2ef0a41903eb27c)
        Mouse.Move(32,6)
        Mouse.Click(32,6,left)
        Mouse.Release(32,6,left)
        Mouse.Click(32,6,left)
        Mouse.Move(41,8)
        Mouse.Click(41,8,left)
        Paint('Button_months_1')
        CheckHash(0xa2d4145f01441587)
        Mouse.Move(32,6)
        Mouse.Click(32,6,left)
        Mouse.Release(32,6,left)
        Mouse.Click(32,6,left)
        Mouse.Move(47,8)
        Mouse.Click(47,8,left)
        Paint('Button_months_2')
        CheckHash(0x6ad910e6be120a15)
        Mouse.Move(33,6)
        Mouse.Click(33,6,left)
        Mouse.Click(33,6,left)
        Mouse.Move(31,15)
        Mouse.Click(31,15,left)
        Paint('Button_date')
        CheckHash(0x9bd1097ed71b821d)
    ";

    let mut a = App::debug(60, 25, script).build().unwrap();
    let mut w = window!("Dates,d:c,w:25,h:6");
    w.add(DatePicker::new("2024-06-13", Layout::new("d:c,w:19")));
    a.add_window(w);
    a.run();
}

#[test]
fn check_packed_keys() {
    let script = "
    Paint.Enable(false)
    Resize(60,25)
    Paint('State_1')
    CheckHash(0x1100da21cab3453)
    Paint('State_3')
    CheckHash(0x1100da21cab3453)
    Key.Pressed(Up)
    Paint('State_4')
    CheckHash(0x8a5f1c5d44ddbd70)
    Key.Pressed(Up)
    Paint('State_5')
    CheckHash(0x486f45de1a093741)
    Key.Pressed(Up)
    Paint('State_6')
    CheckHash(0x9201d6d9671d051e)
    Key.Pressed(Up)
    Paint('State_7')
    CheckHash(0x70d20e4672d0298f)
    Key.Pressed(Down)
    Paint('State_8')
    CheckHash(0x9201d6d9671d051e)
    Key.Pressed(Down)
    Paint('State_9')
    CheckHash(0x486f45de1a093741)
    Key.Pressed(Down)
    Paint('State_10')
    CheckHash(0x8a5f1c5d44ddbd70)
    Key.Pressed(Down)
    Paint('State_11')
    CheckHash(0x1100da21cab3453)
    Key.Pressed(Down)
    Paint('State_12')
    CheckHash(0x4347a05666d63b72)
    Key.Pressed(Down)
    Paint('State_13')
    CheckHash(0xbf03d735c59c8355)
    Key.Pressed(Down)
    Paint('State_14')
    CheckHash(0x12b8b4485c2c2b14)
    Key.Pressed(Down)
    Paint('State_15')
    CheckHash(0xbcb94524b640ed7c)
    Key.Pressed(Down)
    Paint('State_16')
    CheckHash(0x538fcda7ee7c777d)
    Key.Pressed(Down)
    Paint('State_17')
    CheckHash(0x3ada7835304564fe)
    Key.Pressed(Down)
    Paint('State_18')
    CheckHash(0xd8ecec627c94ba6f)
    Key.Pressed(Down)
    Paint('State_19')
    CheckHash(0x531515056b92a9d0)
    Key.Pressed(Down)
    Paint('State_20')
    CheckHash(0x8e4e7909f2d93ea1)
    Key.Pressed(Down)
    Paint('State_21')
    CheckHash(0xf782130a12dc2852)
    Key.Pressed(Down)
    Paint('State_22')
    CheckHash(0x50dac46c5a940e33)
    Key.Pressed(Down)
    Paint('State_23')
    CheckHash(0xde1dce9ebbd7b7f4)
    Key.Pressed(Down)
    Paint('State_24')
    CheckHash(0x8696d26566118c7b)
    Key.Pressed(Down)
    Paint('State_25')
    CheckHash(0x2eedc14c470e6fda)
    Key.Pressed(Shift+Up)
    Paint('State_26')
    CheckHash(0xaa0ad7959204eaa6)
    Key.Pressed(Shift+Up)
    Paint('State_27')
    CheckHash(0x11c517fa6a448dd8)
    Key.Pressed(Shift+Up)
    Paint('State_28')
    CheckHash(0xe934ddad3ca5c220)
    Key.Pressed(Shift+Up)
    Paint('State_29')
    CheckHash(0x40864e27d22926f5)
    Key.Pressed(Shift+Up)
    Paint('State_30')
    CheckHash(0xf2481b5d169da1b3)
    Key.Pressed(Shift+Up)
    Paint('State_31')
    CheckHash(0x6042a9fb842361b0)
    Key.Pressed(Shift+Up)
    Paint('State_32')
    CheckHash(0xf70bbe5f8c6b8841)
    Key.Pressed(Shift+Up)
    Paint('State_33')
    CheckHash(0x69f9108008ba328b)
    Key.Pressed(Shift+Up)
    Paint('State_34')
    CheckHash(0xe6ebf4bac681c62e)
    Key.Pressed(Ctrl+Down)
    Paint('State_35')
    CheckHash(0x5915415d04f28f5f)
    Key.Pressed(Ctrl+Down)
    Paint('State_36')
    CheckHash(0x71d917de2e5e8958)
    Key.Pressed(Ctrl+Down)
    Paint('State_37')
    CheckHash(0xb0191cb7115bace9)
    Key.Pressed(Ctrl+Down)
    Paint('State_38')
    CheckHash(0x35b5bfc5e7e09e1a)
    Key.Pressed(Ctrl+Down)
    Paint('State_39')
    CheckHash(0x808d23ec345beb7b)
    Key.Pressed(Ctrl+Down)
    Paint('State_40')
    CheckHash(0x37e5a2ac791da22d)
    Key.Pressed(Ctrl+Down)
    Paint('State_41')
    CheckHash(0xe76afb8a808c240c)
    Key.Pressed(Ctrl+Down)
    Paint('State_42')
    CheckHash(0x38f2a1df2ad483e3)
    Key.Pressed(Ctrl+Down)
    Paint('State_43')
    CheckHash(0xb8f5d4e51a377ac2)
    Key.Pressed(Ctrl+Down)
    Paint('State_44')
    CheckHash(0x877a0e7421f19111)
    Key.Pressed(Ctrl+Down)
    Paint('State_45')
    CheckHash(0xfe56df4e7c77e940)
    Key.Pressed(Ctrl+Down)
    Paint('State_46')
    CheckHash(0x41420ea1cca5aaa7)
    Key.Pressed(Ctrl+Down)
    Paint('State_47')
    CheckHash(0x5125510978aba116)
    Key.Pressed(Ctrl+Shift+Up)
    Paint('State_48')
    CheckHash(0xb0191cb7115bace9)
    Key.Pressed(Ctrl+Shift+Up)
    Paint('State_49')
    CheckHash(0x86a25844622cbb78)
    Key.Pressed(Ctrl+Shift+Up)
    Paint('State_50')
    CheckHash(0x7957e45928a353bb)
    Key.Pressed(Ctrl+Shift+Down)
    Paint('State_51')
    CheckHash(0x86a25844622cbb78)
    Key.Pressed(Ctrl+Shift+Down)
    Paint('State_52')
    CheckHash(0xb0191cb7115bace9)
    Key.Pressed(Ctrl+Shift+Down)
    Paint('State_53')
    CheckHash(0x5125510978aba116)
    Key.Pressed(A)
    Paint('State_54')
    CheckHash(0x1904255a88485720)
    Key.Pressed(A)
    Paint('State_55')
    CheckHash(0x670e7bb8ed141550)
    Key.Pressed(M)
    Paint('State_56')
    CheckHash(0x67e98dfc1c62e335)
    Key.Pressed(M)
    Paint('State_57')
    CheckHash(0xee51e96bc5d0ad3a)
    Key.Pressed(Shift+M)
    Paint('State_58')
    CheckHash(0x67e98dfc1c62e335)
    Key.Pressed(J)
    Paint('State_59')
    CheckHash(0x8cc4515a78949ce6)
    Key.Pressed(Shift+J)
    Paint('State_60')
    CheckHash(0x7f77f66cca8704ea)
    Key.Pressed(J)
    Paint('State_61')
    CheckHash(0x8cc4515a78949ce6)
    Key.Pressed(J)
    Paint('State_62')
    CheckHash(0xdfbb09efae7d458)
    Key.Pressed(Shift+J)
    Paint('State_63')
    CheckHash(0x8cc4515a78949ce6)
    Key.Pressed(D)
    Paint('State_64')
    CheckHash(0x5c84e746f4680a91)
    Key.Pressed(Escape)
    Paint('State_65')
    CheckHash(0x4252fa8abad3e54d)
    ";

    let mut a = App::debug(60, 25, script).build().unwrap();
    let mut w = window!("Dates,d:c,w:25,h:6");
    w.add(DatePicker::new("2024-06-13", Layout::new("d:c,w:19")));
    a.add_window(w);
    a.run();
}

#[test]
fn check_expanded_keys() {
    let script = "
    Paint.Enable(false)
    Resize(60,25)
    Mouse.Move(29,9)
    Mouse.Drag(29,9,29,3)
    Paint('State_1')
    CheckHash(0x13d32a8761cd7033)
    Paint('State_3')
    CheckHash(0x13d32a8761cd7033)
    Key.Pressed(Enter)
    Paint('State_4')
    CheckHash(0x7ba9684aa85df685)
    Key.Pressed(Left)
    Paint('State_5')
    CheckHash(0xc18b536cf84fd265)
    Key.Pressed(Up)
    Paint('State_6')
    CheckHash(0x3b41b5df2fd8c685)
    Key.Pressed(Left)
    Paint('State_7')
    CheckHash(0xa5360475a47b7c85)
    Key.Pressed(Down)
    Paint('State_8')
    CheckHash(0xa6b9c781b751ec65)
    Key.Pressed(Down)
    Paint('State_9')
    CheckHash(0xe276cb54653b57e5)
    Key.Pressed(Down)
    Paint('State_10')
    CheckHash(0x1080db736de5fd95)
    Key.Pressed(Down)
    Paint('State_11')
    CheckHash(0xd005460bb14d52ad)
    Key.Pressed(Down)
    Paint('State_12')
    CheckHash(0x710e9e2caa655a6d)
    Key.Pressed(Shift+Left)
    Paint('State_13')
    CheckHash(0xe5d1a9a47eda0b05)
    Key.Pressed(Shift+Left)
    Paint('State_14')
    CheckHash(0x3ef79649247970ef)
    Key.Pressed(Shift+Left)
    Paint('State_15')
    CheckHash(0x510bde8ffa1c9d47)
    Key.Pressed(Ctrl+Left)
    Paint('State_16')
    CheckHash(0xc661e7ef58d8b860)
    Key.Pressed(Ctrl+Left)
    Paint('State_17')
    CheckHash(0xfa32e9cacb6a5855)
    Key.Pressed(Ctrl+Left)
    Paint('State_18')
    CheckHash(0x280f5e913d0ee956)
    Key.Pressed(Ctrl+Shift+Left)
    Paint('State_19')
    CheckHash(0xc2989d78a7d882f1)
    Key.Pressed(Ctrl+Shift+Left)
    Paint('State_20')
    CheckHash(0x739b2ff47f6245ac)
    Key.Pressed(Ctrl+Shift+Right)
    Paint('State_21')
    CheckHash(0xc2989d78a7d882f1)
    Key.Pressed(Ctrl+Shift+Right)
    Paint('State_22')
    CheckHash(0x280f5e913d0ee956)
    Key.Pressed(D)
    Paint('State_23')
    CheckHash(0x2de63365f2bc06e9)
    Key.Pressed(J)
    Paint('State_24')
    CheckHash(0x1795412288e42c86)
    Key.Pressed(Shift+J)
    Paint('State_25')
    CheckHash(0xfea66a172c72aa3c)
    Key.Pressed(Shift+J)
    Paint('State_26')
    CheckHash(0x8ad1f1b3f7b6c9e8)
    Key.Pressed(Enter)
    Paint('State_27')
    CheckHash(0x58521978a17913b9)
    Key.Pressed(Escape)
    Paint('State_28')
    CheckHash(0x4252fa8abad3e54d)
    ";

    let mut a = App::debug(60, 25, script).build().unwrap();
    let mut w = window!("Dates,d:c,w:25,h:6");
    w.add(DatePicker::new("2024-06-13", Layout::new("d:c,w:19")));
    a.add_window(w);
    a.run();
}
