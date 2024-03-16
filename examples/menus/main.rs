use appcui::prelude::*;

#[Window(events : MenuEvents, commands  : A)]
struct MyWindow {
    h_file: Handle<Menu>,
    h_edit: Handle<Menu>,
    h_help: Handle<Menu>,
    lb: Handle<Label>,
}
impl MyWindow {
    fn new() -> Self {
        let mut w = Self {
            base: window!("Test,d:c,w:40,h:8"),
            h_file: Handle::None,
            h_edit: Handle::None,
            h_help: Handle::None,
            lb: Handle::None,
        };
        w.lb = w.add(label!("None,d:c,w:30,h:1"));
        // construct a popup menu
        w.h_file = w.register_menu(menu!(
            "&File,class: MyWindow, items=[
            {New,F1,cmd:A},
            {&Save,F2,cmd:A},
            {'&Save As ...',Alt+F2,cmd:A},
            {&Open,F3,cmd:A},
            {-},
            {E&xit,Alt+F4,cmd:A}
        ]"
        ));
        w.h_edit = w.register_menu(menu!(
            "&Edit,class: MyWindow, items=[
            {&Copy,Ctrl+Ins,cmd:A},
            {&Paste,Shift+Ins,cmd:A},
            {&Cut,Ctrl+X,cmd:A},
            {-},
            {&Special,items=[
                {'Slot &1',Alt+1,cmd:A},
                {'Slot &2',Alt+2,cmd:A},
                {'Slot &3',Alt+3,cmd:A},
                {'Slot &4',Alt+4,cmd:A},
                {'Slot &5',Alt+5,cmd:A},
            ]}            
        ]"
        ));
        w.h_help = w.register_menu(menu!(
            "&Help,class: MyWindow, items=[
            {&About,Ctrl+Shift+A,cmd:A},
            {&Update,F10,cmd:A},
            {-},
            {&Tutorials,items=[
                {'&Usage',Alt+U,cmd:A},
                {'&Download',Ctrl+D,cmd:A},
                {&Time,items=[
                    {'Day &1',Ctrl+Alt+Shift+1,cmd:A},
                    {'Day &2',Ctrl+Alt+Shift+2,cmd:A},
                    {'Day &3',Ctrl+Alt+Shift+3,cmd:A},
                ]}            
            ]}            
        ]"
        ));
        w
    }
}
impl MenuEvents for MyWindow {
    fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, _: mywindow::Commands) {
        if let Some(i) = self.menuitem(menu, item) {
            let s = String::from(i.caption());
            let h = self.lb;
            if let Some(l) = self.control_mut(h) {
                l.set_caption(&s);
            }
        }
    }

    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.add(self.h_file);
        menubar.add(self.h_edit);
        menubar.add(self.h_help);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().menu_bar().build()?;
    a.add_window(MyWindow::new());
    a.run();
    Ok(())
}
