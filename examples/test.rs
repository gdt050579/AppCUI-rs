use appcui::prelude::*;

#[Window(events = MenuEvents, commands=A+B+C)]
struct MyWin {
    m_file: Handle<Menu>,
    m_edit: Handle<Menu>,
    m_help: Handle<Menu>,
}
impl MyWin {
    fn new() -> Self {
        let mut w = MyWin {
            base: window!("Test,d:c,w:40,h:8"),
            m_file: Handle::None,
            m_help: Handle::None,
            m_edit: Handle::None,
        };
        let mut m = Menu::new("&File");
        m.add(menuitem!("&New,F1,'mywin::A'"));
        m.add(menuitem!("&Save,F2,'mywin::A'"));
        m.add(menuitem!("&Open,F2,'mywin::A'"));
        m.add(menuitem!("---"));
        m.add(menu::Command::new("E&xit", key!("Alt+F4"), mywin::Commands::A));
        m.add(menuitem!("---"));
        m.add(menuitem!("Feature_1,None,B,class:myWin,checked: true"));
        m.add(menuitem!("Feature_2,None,B,class:myWin,checked: true"));
        m.add(menuitem!("Feature_3,None,B,class:myWin,checked: false"));
        m.add(menuitem!("Feature_4,None,B,class:myWin,checked: true"));
        m.add(menuitem!("---"));
        m.add(menuitem!("
        Sub-menu,items=[
            {cmd1,None,'mywin::A'},
            {cmd2,None,'mywin::A'},
            {cmd3,None,'mywin::A'},
            {---},
            {cmd1,None,'mywin::A'},
            {---},
            {Colors,items=[
                {Red,None,'mywin::B'},
                {Green,None,'mywin::B'},
                {Blue,None,'mywin::B'},
            ]}
        ]
        "));

        w.m_file = w.register_menu(m);

        let mut m = Menu::new("&Edit");
        m.add(menu::Command::new("&Copy", Key::None, mywin::Commands::B));
        m.add(menu::Command::new("C&ut", Key::None, mywin::Commands::B));
        m.add(menu::Command::new("&Paste", Key::None, mywin::Commands::B));
        m.add(menu::Separator::new());
        m.add(menuitem!("'Paste only text',None,B,class:myWin,selected: true"));
        m.add(menuitem!("'Paste only images',None,B,class:myWin,selected: false"));
        m.add(menuitem!("'Paste everything',None,B,class:myWin,selected: false"));
        w.m_edit = w.register_menu(m);

        let mut m = Menu::new("&Help");
        m.add(menu::Command::new("About", Key::None, mywin::Commands::C));
        w.m_help = w.register_menu(m);
        w
    }
}
impl MenuEvents for MyWin {
    fn on_menu_open(&self, menu: &mut Menu) {}

    fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, command: mywin::Commands) {}

    fn on_check(&mut self, menu: Handle<Menu>, item: Handle<menu::CheckBox>, command: mywin::Commands, checked: bool) {}

    fn on_select(&mut self, menu: Handle<Menu>, item: Handle<menu::SingleChoice>, command: mywin::Commands) {}

    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.add(self.m_file);
        menubar.add(self.m_edit);
        menubar.add(self.m_help);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(60,15)).menu().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())

    // //let mut a = App::new().size(Size::new(80,25)).build()?;
    // // let script = "
    // //     Paint()
    // //     Key.Pressed(Up)
    // //     Paint()
    // // ";
    // // let mut a = App::debug(80, 20, script).build()?;

    // let mut w = window!("Title,d:c,w:60,h:14,flags:Sizeable");
    // let mut c = Canvas::new(Size::new(80,40),Layout::new("l:15,t:0,b:0,r:0"),canvas::Flags::ScrollBars);
    // let s = c.get_drawing_surface();
    // s.clear(Character::with_color(Color::White, Color::Black));
    // //s.draw_rect(Rect::new(0, 0, 39, 39), LineType::Single, CharAttribute::with_color(Color::Aqua, Color::Black));
    // for x in 1..39 {
    //     for y in 1..39 {
    //         let c = (((x+y)+30) as u8) as char;
    //         s.write_char(x, y, Character::with_char(c));
    //     }
    // }
    // w.add(c);
    // w.add(button!("Test,l:1,t:1,a:tl    ,w:10"));
    // // let g = w.get_toolbar().create_group(toolbar::GroupPosition::BottomLeft);
    // // w.add(ColorPicker::new(Color::DarkGreen,Layout::new("x:1,y:1,w:32")));
    // // w.add(colorpicker!("Red,x:1,y:3,w:7"));
    // // w.add(colorpicker!("color:green,x:20,y:3,w:7"));
    // // w.add(ColorPicker::new(Color::Red,Layout::new("x:1,y:3,w:7")));
    // // w.add(ColorPicker::new(Color::Green,Layout::new("x:20,y:3,w:7")));
    // a.add_window(w);
    // //a.add_window(MyWin::new());
    // a.run();
    // Ok(())
}
