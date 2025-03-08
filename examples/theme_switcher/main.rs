use appcui::prelude::*;
use chrono::NaiveDate;

#[derive(ListItem)]
struct FileInformation {
    #[Column(name: "&Name", width: 25)]
    name: &'static str,
    #[Column(name: "&Size", width: 12, align: right, render: size, format: auto)]
    size: u64,
    #[Column(name: "&Created", width: 12, align: center, render: date, format: YearMonthDay)]
    created: NaiveDate,
}

#[Window(events : MenuEvents, commands  : New+Save+Open+Exit+DefaultTheme+DarkGrayTheme)]
struct MyWindow {
    h_file: Handle<Menu>,
    h_theme: Handle<Menu>,
}
impl MyWindow {
    fn new() -> Self {
        let mut w = Self {
            base: window!("'Theme Switcher',r:1,b:1,w:100,h:20, Flags: Sizeable"),
            h_file: Handle::None,
            h_theme: Handle::None,
        };
        // construct a popup menu
        w.h_file = w.register_menu(menu!(
            "&File,class: MyWindow, items=[
            {New,F1,cmd:New},
            {&Save,F2,cmd:Save},
            {&Open,F3,cmd:Open},
            {-},
            {E&xit,Alt+F4,cmd:Exit}
        ]"
        ));
        w.h_theme = w.register_menu(menu!(
            "&Theme,class: MyWindow, items=[
            {&Default,cmd:DefaultTheme,selected: true},
            {'Dark Gray',cmd:DarkGrayTheme, selected: false},
        ]"
        ));

        let mut splitter = vsplitter!("d:c,w:100%,h:100%,pos:55");
        let mut p_basic = panel!("'Basic controls',l:1,t:1,r:1,h:8");
        p_basic.add(checkbox!("'Checkbox unchecked',x:1,y:1,w:23"));
        p_basic.add(checkbox!("'Checkbox &selected',x:1,y:2,w:23,checked:true"));
        p_basic.add(checkbox!("'Checkbox disabled',x:1,y:3,w:23,enabled:false"));
        p_basic.add(radiobox!("'RadioBox &1',x:27,y:1,w:15,selected: true"));
        p_basic.add(radiobox!("'RadioBox &2',x:27,y:2,w:15"));
        p_basic.add(radiobox!("'RadioBox disabled',x:27,y:3,w:202,enabled:false"));
        p_basic.add(label!("'A text field editor:',x:1,y:5,w:23"));
        p_basic.add(textfield!("'Text field',l:26,y:5,r:1"));
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
        let mut lv = listview!("class: FileInformation,d:c,w:100%,h:100%,flags: ScrollBars+SearchBar+LargeIcons+Checkboxes,lsm:2");
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

        splitter.add(vsplitter::Panel::Right,button!("'Inactive',x:1,y:15,w:19,enabled:false"));
        splitter.add(vsplitter::Panel::Right,button!("'&Button',x:22,y:15,w:19"));

        w.add(splitter);
        w.request_focus_for_control(c_focus);
        w
    }
}
impl MenuEvents for MyWindow {
    fn on_command(&mut self, _: Handle<Menu>, _: Handle<menu::Command>, cmd: mywindow::Commands) {
        if cmd == mywindow::Commands::Exit {
            self.close();
        }
    }
    fn on_select(&mut self, _: Handle<Menu>, _: Handle<menu::SingleChoice>, cmd: mywindow::Commands) {
        let stock_theme = match cmd {
            mywindow::Commands::DefaultTheme => Some(Themes::Default),
            mywindow::Commands::DarkGrayTheme => Some(Themes::DarkGray),
            _ => None,
        };
        if let Some(theme) = stock_theme {
            App::set_theme(Theme::new(theme));
        }
    }

    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.add(self.h_file);
        menubar.add(self.h_theme);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(120, 24)).menu_bar().build()?;
    a.add_window(MyWindow::new());
    a.run();
    Ok(())  
}
