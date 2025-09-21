use appcui::prelude::*;

#[Window(events = [AppBarEvents, MenuEvents], 
         commands=[New,Save,SaveAs,Open,Exit,Copy,Cut,Paste,Time,Date,Convert,About,Welcome])]
pub(crate) struct Win {
    h_file: Handle<appbar::MenuButton>,
    h_edit: Handle<appbar::MenuButton>,
    h_help: Handle<appbar::MenuButton>,
}
impl Win {
    pub(crate) fn new() -> Self {
        let mut w = Win {
            base: window!("'Simple menus',a:c,w:40,h:8,Flags: Sizeable"),
            h_file: Handle::None,
            h_edit: Handle::None,
            h_help: Handle::None,
        };

        let m = menu!(
            "class: Win, items=[
                { &New, cmd: New , key: Ctrl+N },
                { &Save, cmd: Save, key: Ctrl+S },
                { 'Save &as...', cmd: SaveAs },
                { &Open, cmd: Open, key: Ctrl+O },
                { --- },
                { E&xit, cmd: Exit, key: Alt+F4 },
            ]"
        );
        w.h_file = w.appbar().add(appbar::MenuButton::new("&File", m, 1, appbar::Side::Left));
        let m = menu!(
            "class: Win, items=[
                { &Copy, cmd: Copy , key: Ctrl+C },
                { C&ut, cmd: Cut, key: Ctrl+X },
                { &Paste, cmd: Paste, key: Ctrl+V },
                { --- },
                { 'Sub menu one', items = [
                      { &Time, cmd: Time, Key: F1 },
                      { &Date, cmd: Date, Key: F2 },
                      { &Convert, items = [
                           { 'From milliseconds', cmd: Convert, key: Ctrl+1 },
                           { 'From seconds', cmd: Convert, key: Ctrl+2 },
                        ] 
                       }
                   ] 
                }
            ]"
        );
        w.h_edit = w.appbar().add(appbar::MenuButton::new("&Edit", m, 1, appbar::Side::Left));
        let m = menu!(
            "class: Win, items=[
                { &About, cmd: About },
                { Welcome, cmd: Welcome },
            ]"
        );
        w.h_help = w.appbar().add(appbar::MenuButton::new("&Help", m, 1, appbar::Side::Right));

        w
    }
}
impl AppBarEvents for Win {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.h_file);
        appbar.show(self.h_edit);
        appbar.show(self.h_help);
    }
}
impl MenuEvents for Win {
    fn on_command(&mut self, _menu:Handle<Menu>, _item:Handle<menu::Command>, command:win::Commands){
        match command {
            win::Commands::New => { /* Handle New command */ },
            win::Commands::Save => { /* Handle Save command */ },
            win::Commands::SaveAs => { /* Handle SaveAs command */ },
            win::Commands::Open => { /* Handle Open command */ },
            win::Commands::Exit => { /* Handle Exit command */ },
            win::Commands::Copy => { /* Handle Copy command */ },
            win::Commands::Cut => { /* Handle Cut command */ },
            win::Commands::Paste => { /* Handle Paste command */ },
            win::Commands::Time => { /* Handle Time command */ },
            win::Commands::Date => { /* Handle Date command */ },
            win::Commands::Convert => { /* Handle Convert command */ },
            win::Commands::About => { /* Handle About command */ },
            win::Commands::Welcome => { /* Handle Welcome command */ },
        }
    }
}



fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().app_bar().build()?;
    app.add_window(Win::new());
    app.run();
    Ok(())
}
