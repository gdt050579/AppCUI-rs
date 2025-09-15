use crate::prelude::*;
use appcui_proc_macro::*;
use chrono::NaiveDate;

#[test]
fn check_theme_update() {
    #[CustomControl(overwrite:OnThemeChanged+OnPaint, internal: true)]
    struct TestControl {
        attr: CharAttribute,
    }
    impl TestControl {
        fn new() -> Self {
            let mut obj = Self {
                base: ControlBase::new(layout!("l:1,r:1,t:1,b:1"), true),
                attr: CharAttribute::default(),
            };
            obj.attr = obj.theme().window.normal;
            obj
        }
    }
    impl OnPaint for TestControl {
        fn on_paint(&self, surface: &mut crate::prelude::Surface, _theme: &Theme) {
            surface.clear(Character::with_attributes('X', self.attr));
        }
    }
    impl OnThemeChanged for TestControl {
        fn on_theme_changed(&mut self, theme: &Theme) {
            self.attr = theme.window.normal;
        }
    }

    #[Window(events = CommandBarEvents, internal: true, commands: ChangeTheme)]
    struct TestWindow {}
    impl TestWindow {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,l:2,t:2,b:2,r:2"),
            };
            w.add(TestControl::new());
            w
        }
    }
    impl CommandBarEvents for TestWindow {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Change Theme", testwindow::Commands::ChangeTheme);
        }

        fn on_event(&mut self, command_id: testwindow::Commands) {
            if command_id == testwindow::Commands::ChangeTheme {
                let mut theme = Theme::new(Themes::Default);
                // make the window dark greem
                theme.window.normal = CharAttribute::new(Color::White, Color::DarkGreen, CharFlags::None);
                App::set_theme(theme);
            }
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xFAE7599256AB44D1) 
        Key.Pressed(F1)
        Paint('Theme changed to green')   
        CheckHash(0xFD09CB2F64B6B15) 
    ";
    let mut a = App::debug(60, 10, script).command_bar().build().unwrap();
    a.add_window(TestWindow::new());
    a.run();
}

#[derive(ListItem)]
struct FileInformation {
    #[Column(name: "&Name", width: 25)]
    name: &'static str,
    #[Column(name: "&Size", width: 12, align: right, render: size, format: auto)]
    size: u64,
    #[Column(name: "&Created", width: 12, align: center, render: date, format: YearMonthDay)]
    created: NaiveDate,
}

#[Window(events : MenuEvents, commands  : New+Save+Open+Exit+DefaultTheme+DarkGrayTheme+LightTheme, internal: true)]
struct WindowWithTheme {
    h_file: Handle<menubar::MenuEntry>,
}
impl WindowWithTheme {
    fn new() -> Self {
        let mut w = Self {
            base: window!("'Theme Switcher',r:1,b:1,w:100,h:20, Flags: Sizeable"),
            h_file: Handle::None,
        };
        // construct a popup menu
        w.h_file = w.menubar_mut().add(menubar::MenuEntry::new(
            "&File",
            menu!(
                "class: WindowWithTheme, items=[
                    {New,F1,cmd:New},
                    {&Save,F2,cmd:Save},
                    {&Open,F3,cmd:Open},
                    {-},
                    {E&xit,Alt+F4,cmd:Exit}
                ]"
            ),
            0,
            menubar::MenuBarPosition::Left,
        ));

        let mut splitter = vsplitter!("d:f,pos:55");
        let mut p_basic = panel!("'Basic controls',l:1,t:1,r:1,h:8");
        p_basic.add(checkbox!("'Checkbox unchecked',x:1,y:1,w:23"));
        p_basic.add(checkbox!("'Checkbox &selected',x:1,y:2,w:23,checked:true"));
        p_basic.add(checkbox!("'Checkbox disabled',x:1,y:3,w:23,enabled:false"));
        p_basic.add(radiobox!("'RadioBox &1',x:27,y:1,w:15,selected: true"));
        p_basic.add(radiobox!("'RadioBox &2',x:27,y:2,w:15"));
        p_basic.add(radiobox!("'RadioBox disabled',x:27,y:3,w:202,enabled:false"));
        p_basic.add(label!("'A text field editor:',x:1,y:5,w:23"));
        p_basic.add(textfield!("'Text field',l:26,y:5,r:1,pivot:center"));
        splitter.add(vsplitter::Panel::Left, p_basic);

        let mut p_selectors = panel!("'Selectors',l:1,t:10,r:1,h:7");
        p_selectors.add(combobox!("x:1,y:1,w:23,items=[Delphin,Zebra,Lion],index:1"));
        p_selectors.add(colorpicker!("x:27,y:1,w:20,color:Red"));
        p_selectors.add(datepicker!("x:1,y:3,w:23,date:2024-12-31"));
        p_selectors.add(numericselector!("class:i32,value:50,min:0,max:99,step:1,x:27,y:3,w:20"));
        splitter.add(vsplitter::Panel::Left, p_selectors);

        let mut tab = tab!("l:1,t:1,r:1,h:5,tabs:[&First,&Second,&Third]");
        tab.add(
            0,
            label!("'A tabulator control allows grouping of other controls under different tabs.',l:1,t:1,r:1,h:2"),
        );
        splitter.add(vsplitter::Panel::Right, tab);

        let mut p_list = panel!("'Lists',l:1,t:7,r:1,h:6");
        let mut lv = listview!("class: FileInformation,d:f,flags: ScrollBars+SearchBar+LargeIcons+Checkboxes,lsm:2");
        lv.add_item(listview::Item::new(
            FileInformation {
                name: "Applicatons",
                size: 0,
                created: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            },
            false,
            None,
            ['📁', ' '],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            FileInformation {
                name: "MyBackup.zip",
                size: 1024 * 1024 * 10,
                created: NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
            },
            true,
            None,
            ['📦', ' '],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            FileInformation {
                name: "document.docx",
                size: 1024 * 1024,
                created: NaiveDate::from_ymd_opt(2023, 10, 9).unwrap(),
            },
            false,
            None,
            ['📄', ' '],
            listview::Group::None,
        ));
        lv.sort(0, true);
        let c_focus = p_list.add(lv);
        splitter.add(vsplitter::Panel::Right, p_list);

        splitter.add(vsplitter::Panel::Right, button!("'Inactive',x:1,y:15,w:19,enabled:false"));
        splitter.add(vsplitter::Panel::Right, button!("'&Button',x:22,y:15,w:19"));

        w.add(splitter);
        w.request_focus_for_control(c_focus);
        w
    }
}
impl MenuEvents for WindowWithTheme {
    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.show(self.h_file);
    }
}

#[test]
fn check_default_theme() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(3,0,left)
        Mouse.Move(3,2)
        Paint('Initial state')   
        CheckHash(0x1CEEE10D9F754FA4) 
    ";
    let mut a = App::debug(120, 24, script)
        .theme(Theme::new(Themes::Default))
        .command_bar()
        .menu_bar()
        .build()
        .unwrap();
    a.add_window(WindowWithTheme::new());
    a.run();
}

#[test]
fn check_darkgray_theme() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(3,0,left)
        Mouse.Move(3,2)
        Paint('Initial state')   
        CheckHash(0x44BB8888C3906018) 
    ";
    let mut a = App::debug(120, 24, script)
        .theme(Theme::new(Themes::DarkGray))
        .command_bar()
        .menu_bar()
        .build()
        .unwrap();
    a.add_window(WindowWithTheme::new());
    a.run();
}

#[test]
fn check_light_theme() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(3,0,left)
        Mouse.Move(3,2)
        Paint('Initial state')   
        CheckHash(0xE906AAA8E7503FBC) 
    ";
    let mut a = App::debug(120, 24, script)
        .theme(Theme::new(Themes::Light))
        .command_bar()
        .menu_bar()
        .build()
        .unwrap();
    a.add_window(WindowWithTheme::new());
    a.run();
}
