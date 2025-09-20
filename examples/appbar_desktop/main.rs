use appcui::prelude::*;

// this is a demo designed to create a appbar with different kind of inputs
// it is meant to allow to take a snapshot / picture (it does not work)

#[Desktop(events = [DesktopEvents,AppBarEvents], commands  = TempCommand)]
struct MyDesktop {
    h_file: Handle<appbar::MenuButton>,
    h_edit: Handle<appbar::MenuButton>,
    h_help: Handle<appbar::MenuButton>,
    h_sep1: Handle<appbar::Separator>,
    h_sep2: Handle<appbar::Separator>,
    h_sep3: Handle<appbar::Separator>,
    h_prev: Handle<appbar::Button>,
    h_next: Handle<appbar::Button>,
    h_lb: Handle<appbar::Label>,
    h_save: Handle<appbar::Button>,
    h_autosave: Handle<appbar::SwitchButton>,
    h_tg1: Handle<appbar::ToggleButton>,
    h_tg2: Handle<appbar::ToggleButton>,
    h_tg3: Handle<appbar::ToggleButton>,
    h_download: Handle<appbar::Label>,
}
impl MyDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new(),
            h_file: Handle::None,
            h_edit: Handle::None,
            h_help: Handle::None,
            h_sep1: Handle::None,
            h_sep2: Handle::None,
            h_sep3: Handle::None,
            h_prev: Handle::None,
            h_next: Handle::None,
            h_lb: Handle::None,
            h_save: Handle::None,
            h_autosave: Handle::None,
            h_tg1: Handle::None,
            h_tg2: Handle::None,
            h_tg3: Handle::None,
            h_download: Handle::None,
        }
    }
}

impl DesktopEvents for MyDesktop {
    fn on_start(&mut self) {
        self.h_file = self.appbar().add(appbar::MenuButton::new(
            "&File",
            menu!(
                " class: MyDesktop, items:[
                    {'&New',cmd: TempCommand, key: Ctrl+N},
                    {'&Save',cmd: TempCommand, key: Ctrl+S},
                    {'&Save As', cmd: TempCommand, key: Ctrl+Shift+S},
                    {'&Open',cmd: TempCommand, key: Ctrl+O},
                    { --- },
                    {'&Exit',cmd: TempCommand, key: Alt+F4},
                ]"
            ),
            0,
            appbar::Side::Left,
        ));
        self.h_edit = self.appbar().add(appbar::MenuButton::new(
            "&Edit",
            menu!(
                " class: MyDesktop, items:[
                    {'&Copy',cmd: TempCommand, key: Ctrl+C},
                    {'&Paste',cmd: TempCommand, key: Ctrl+V},
                    {'&Cut',cmd: TempCommand, key: Ctrl+X},
                ]"
            ),
            1,
            appbar::Side::Left,
        ));
        self.h_help = self.appbar().add(appbar::MenuButton::new(
            "&Help",
            menu!(
                " class: MyDesktop, items:[
                    {'&About',cmd: TempCommand, key: F1},
                ]"
            ),
            2,
            appbar::Side::Left,
        ));
        self.h_sep1 = self.appbar().add(appbar::Separator::new(3, appbar::Side::Left));
        self.h_save = self.appbar().add(appbar::Button::with_tooltip("💾  ", "Save", 4, appbar::Side::Left));
        self.h_autosave = self.appbar().add(appbar::SwitchButton::with_tooltip(
            "on  ",
            "off ",
            appbar::SwitchButtonSymbol::CheckBox,
            "Enable/Disable autosave",
            false,
            5,
            appbar::Side::Left,
        ));
        self.h_sep2 = self.appbar().add(appbar::Separator::new(6, appbar::Side::Left));
        self.h_prev = self.appbar().add(appbar::Button::with_tooltip(" < ", "Previous", 7, appbar::Side::Left));
        self.h_lb = self.appbar().add(appbar::Label::new("1/5", 8, appbar::Side::Left));
        self.h_next = self.appbar().add(appbar::Button::with_tooltip(" > ", "Next", 9, appbar::Side::Left));

        self.h_download = self.appbar().add(appbar::Label::new("[■■■■■■    ] 50% ", 0, appbar::Side::Right));
        self.h_sep3 = self.appbar().add(appbar::Separator::new(1, appbar::Side::Right));

        self.h_tg1 = self.appbar().add(appbar::ToggleButton::new(" ⚙  ", true, 10, appbar::Side::Right));
        self.h_tg2 = self.appbar().add(appbar::ToggleButton::new(" 🔧  ", false, 11, appbar::Side::Right));
        self.h_tg3 = self.appbar().add(appbar::ToggleButton::new(" 🖨  ", true, 12, appbar::Side::Right));
    }
}

impl AppBarEvents for MyDesktop {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.h_file);
        appbar.show(self.h_edit);
        appbar.show(self.h_help);
        appbar.show(self.h_sep1);
        appbar.show(self.h_save);
        appbar.show(self.h_autosave);
        appbar.show(self.h_sep2);
        appbar.show(self.h_prev);
        appbar.show(self.h_lb);
        appbar.show(self.h_next);
        appbar.show(self.h_download);
        appbar.show(self.h_sep3);
        appbar.show(self.h_tg1);
        appbar.show(self.h_tg2);
        appbar.show(self.h_tg3);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().desktop(MyDesktop::new()).app_bar().build()?.run();
    Ok(())
}
