use appcui::prelude::*;

#[Window(events = MenuEvents, commands=A+B+C)]
struct MyWin {
    m_file: Handle<Menu>,
    // m_edit: Handle<Menu>,
    // m_help: Handle<Menu>,
}
impl MyWin {
    fn new() -> Self {
        let mut w = MyWin {
            base: window!("Test,d:c,w:40,h:8"),
            m_file: Handle::None,
            // m_help: Handle::None,
            // m_edit: Handle::None,
        };
        // let m = menu!("&File,class:MyWin,items=[
        //     {&New,F1,A},
        //     {&Open,F3,B},
        //     {&Save,F2,C},
        //     {'Save &as...',None,A},
        //     {---},
        //     {Options,items=[
        //         {Feature_1,None,B,checked:true},
        //         {Feature_2,None,B,checked:true},
        //         {Feature_3,None,B,checked:true},
        //         {Feature_4,None,B,checked:true},
        //         {---},
        //         {'Color', enabled: false, items=[
        //             {Red,None,C,selected: false},
        //             {Green,None,C,selected: false},
        //             {Blue,None,C,selected: true},
        //             {--},
        //             {Km,None,C,selected: false},
        //             {Inch,None,C,selected: true},
        //             {Giga,None,C,selected: false},
        //         ]}
        //     ]},
        //     {---},
        //     {'Exit',Alt+F4,A}
        // ]
        // ");
        let m = menu!("&File,class: MyWin, items=[
            {&New,cmd:A},
            {&Open,F3,cmd:A},
            {&Save,F2,cmd:A},
            {'Save &as...',cmd:A},
            {---},
            {Options,items=[
                {'Enable advanced search' ,checked: true,cmd: B},
                {'Search within archives' ,checked: false,cmd: B},
                {---},
                {'By &Name',Alt+F1,selected: true,cmd: B},
                {'By &Size',Alt+F2,selected: false,cmd: B},
                {'By &Time',Alt+F3,selected: false,cmd: B},
            ]},
            {E&xit,Alt+F4,cmd:A}
        ]");
        w.m_file = w.register_menu(m);

        // let mut m = Menu::new("&Edit");
        // m.add(menu::Command::new("&Copy", Key::None, mywin::Commands::B));
        // m.add(menu::Command::new("C&ut", Key::None, mywin::Commands::B));
        // m.add(menu::Command::new("&Paste", Key::None, mywin::Commands::B));
        // m.add(menu::Separator::new());
        // m.add(menuitem!("'Paste only text',None,B,class:myWin,selected: true"));
        // m.add(menuitem!("'Paste only images',None,B,class:myWin,selected: false"));
        // m.add(menuitem!("'Paste everything',None,B,class:myWin,selected: false"));
        // w.m_edit = w.register_menu(m);

        // let mut m = Menu::new("&Help");
        // m.add(menu::Command::new("About", Key::None, mywin::Commands::C));
        // w.m_help = w.register_menu(m);
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
        // menubar.add(self.m_edit);
        // menubar.add(self.m_help);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(80,24)).menu().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
