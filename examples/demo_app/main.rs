use appcui::prelude::*;
use appcui::ui::treeview::Item;
use appcui::graphics::{LineType, CharAttribute};
use appcui::system::{App, Theme};
use chrono::NaiveDate;

#[derive(ListItem)]
struct FileItem {
    #[Column(name = "&Name", width = 25)]
    name: String,
    #[Column(name = "&Size", width = 12, align = right, render = size, format = auto)]
    size: u64,
    #[Column(name = "&Modified", width = 12, align = center, render = date, format = YearMonthDay)]
    modified: NaiveDate,
}

#[derive(Clone)]
enum WindowLayout {
    Grid,
    Vertical,
    Cascade,
}

#[derive(ListItem)]
struct TreeNode {
    #[Column(name = "Name", width = 25)]
    name: String,
}

#[Window(events: MenuEvents, commands: FileExplorer+Controls+TreeView+ImageView+Settings+DefaultTheme+DarkTheme+GridLayout+VerticalLayout+CascadeLayout+About)]
struct DemoApp {
    h_examples: Handle<Menu>,
    h_theme: Handle<Menu>,
    h_window: Handle<Menu>,
    layout: WindowLayout,
}

impl DemoApp {
    fn new() -> Self {
        let mut w = Self {
            base: window!("'AppCUI Demo',d:c,Flags:Sizeable"),
            h_examples: Handle::None,
            h_theme: Handle::None,
            h_window: Handle::None,
            layout: WindowLayout::Grid,
        };

        // Create menus
        w.h_examples = w.register_menu(menu!(
            "&Examples,class:DemoApp,items=[
                {'FileExplorer',F1,cmd:FileExplorer},
                {'ControlsDemo',F2,cmd:Controls},
                {'TreeView',F3,cmd:TreeView},
                {'ImageView',F4,cmd:ImageView},
                {'Settings',F5,cmd:Settings}
            ]"
        ));

        w.h_theme = w.register_menu(menu!(
            "&Theme,class:DemoApp,items=[
                {&Default,cmd:DefaultTheme,selected:true},
                {'Dark',cmd:DarkTheme,selected:false}
            ]"
        ));

        w.h_window = w.register_menu(menu!(
            "&Window,class:DemoApp,items=[
                {'Grid',cmd:GridLayout,selected:true},
                {'Vertical',cmd:VerticalLayout,selected:false},
                {'Cascade',cmd:CascadeLayout,selected:false},
                {-},
                {'About',cmd:About}
            ]"
        ));

        // Create the AppCUI logo
        let mut canvas = canvas!("'120x30',d:c,w:100%,h:100%");
        let s = canvas.get_drawing_surface();

        // Draw "AppCUI" text in a stylized way
        let text = "AppCUI";
        let x = 55;
        let y = 15;

        s.draw_horizontal_line(x - 2, x + text.len() as i32 + 2, y - 2, LineType::Single, CharAttribute::default());
        s.draw_horizontal_line(x - 2, x + text.len() as i32 + 2, y + 2, LineType::Single, CharAttribute::default());
        s.draw_vertical_line(y - 2, y + 2, x - 2, LineType::Single, CharAttribute::default());
        s.draw_vertical_line(y - 2, y + 2, x + text.len() as i32 + 2, LineType::Single, CharAttribute::default());

        s.write_string(x, y, text, CharAttribute::default(), false);

        w.base.add(canvas);
        w
    }

    fn create_file_explorer(&mut self) -> Result<(), Error> {
        let mut w = window!("'File Explorer',d:c,w:60,h:20,Flags:Sizeable");
        let mut lv = listview!("class:FileItem,d:c,w:100%,h:100%,flags:ScrollBars+SearchBar");
        
        // Add some sample files
        lv.add_item(listview::Item::new(
            FileItem {
                name: "Documents".to_string(),
                size: 0,
                modified: NaiveDate::from_ymd_opt(2025, 3, 1).unwrap(),
            },
            false,
            None,
            ['📁', ' '],
            listview::Group::None,
        ));

        lv.add_item(listview::Item::new(
            FileItem {
                name: "report.pdf".to_string(),
                size: 1024 * 1024 * 2,
                modified: NaiveDate::from_ymd_opt(2025, 3, 15).unwrap(),
            },
            false,
            None,
            ['📄', ' '],
            listview::Group::None,
        ));

        w.add(lv);
        self.base.add(w);
        Ok(())
    }

    fn create_controls_demo(&mut self) -> Result<(), Error> {
        let mut w = window!("'Controls Demo',d:c,w:70,h:25,Flags:Sizeable");
        let mut tab = tab!("d:c,w:100%,h:100%,tabs:[&Basic,&Input,&Selection]");

        // Basic tab
        let mut basic = panel!("d:c,w:100%,h:100%");
        basic.add(button!("'Click Me!',x:2,y:2,w:15"));
        basic.add(label!("'A sample label',x:2,y:4,w:20"));
        basic.add(checkbox!("'Enable feature',x:2,y:6,w:20"));
        tab.add(0, basic);

        // Input tab
        let mut input = panel!("d:c,w:100%,h:100%");
        input.add(label!("'Name:',x:2,y:2,w:10"));
        input.add(textfield!("x:12,y:2,w:30"));
        input.add(label!("'Description:',x:2,y:4,w:10"));
        input.add(textfield!("x:12,y:4,w:50,h:3"));
        tab.add(1, input);

        // Selection tab
        let mut selection = panel!("d:c,w:100%,h:100%");
        selection.add(radiobox!("'Option 1',x:2,y:2,w:15,selected:true"));
        selection.add(radiobox!("'Option 2',x:2,y:4,w:15"));
        selection.add(radiobox!("'Option 3',x:2,y:6,w:15"));
        tab.add(2, selection);

        w.add(tab);
        self.base.add(w);
        Ok(())
    }

    fn create_tree_view(&mut self) -> Result<(), Error> {
        let mut w = window!("'Tree View',d:c,w:50,h:20,Flags:Sizeable");
        let mut tv = treeview!("type:TreeNode,d:c,w:100%,h:100%");

        // Add tree items and ignore unused variables
        tv.add_item(Item::new(TreeNode { name: "Root".to_string() }, false, None, ['📁', ' ']));
        tv.add_item(Item::new(TreeNode { name: "Documents".to_string() }, false, None, ['📁', ' ']));
        tv.add_item(Item::new(TreeNode { name: "Pictures".to_string() }, false, None, ['📁', ' ']));
        tv.add_item(Item::new(TreeNode { name: "Source".to_string() }, false, None, ['📁', ' ']));
        tv.add_item(Item::new(TreeNode { name: "main.rs".to_string() }, false, None, ['📄', ' ']));
        tv.add_item(Item::new(TreeNode { name: "lib.rs".to_string() }, false, None, ['📄', ' ']));

        w.add(tv);
        self.base.add(w);
        Ok(())
    }

    fn create_image_view(&mut self) -> Result<(), Error> {
        let mut w = window!("'Ferris View',d:c,w:40,h:15,Flags:Sizeable");
        let mut canvas = canvas!("'40x15',d:c,w:100%,h:100%");
        let s = canvas.get_drawing_surface();

        // ASCII art of Ferris
        let ferris = vec![
            "    \\\\          //    ",
            "     \\\\        //     ",
            "      \\\\      //      ",
            "       \\\\||||//       ",
            "       (o  o)         ",
            "        \\~~\\          ",
            "         \\__\\         ",
        ];

        for (i, line) in ferris.iter().enumerate() {
            s.write_string(2, i as i32 + 2, line, CharAttribute::default(), false);
        }

        w.add(canvas);
        self.base.add(w);
        Ok(())
    }

    fn create_settings(&mut self) -> Result<(), Error> {
        let mut w = window!("'Settings',d:c,w:60,h:20,Flags:Sizeable");
        let mut panel = panel!("d:c,w:100%,h:100%");

        panel.add(label!("'Display Settings',x:2,y:2,w:20"));
        panel.add(combobox!("x:2,y:4,w:25,items:[1920x1080,1600x900,1280x720],index:0"));
        
        panel.add(label!("'Volume:',x:2,y:6,w:10"));
        panel.add(numericselector!("class:i32,value:50,min:0,max:100,step:5,x:12,y:6,w:15"));
        
        panel.add(label!("'Theme:',x:2,y:8,w:10"));
        panel.add(combobox!("x:12,y:8,w:25,items:[Light,Dark,System],index:0"));

        w.add(panel);
        self.base.add(w);
        Ok(())
    }

    fn show_about(&self) {
        let mut mb = window!("'About',d:c,w:40,h:10");
        mb.add(label!("'AppCUI Demo\nVersion 1.0.0\n\nA demonstration of AppCUI capabilities.',x:1,y:1,w:38,h:8"));
        self.base.add(mb);
    }

    fn arrange_windows(&self, layout: WindowLayout) {
        match layout {
            WindowLayout::Grid => self.base.arrange_windows_in_grid(),
            WindowLayout::Vertical => self.base.arrange_windows_vertically(),
            WindowLayout::Cascade => self.base.arrange_windows_in_cascade(),
        }
    }
}

impl MenuEvents for DemoApp {
    fn on_command(&mut self, _: Handle<Menu>, _: Handle<menu::Command>, cmd: demoapp::Commands) {
        match cmd {
            demoapp::Commands::FileExplorer => { let _ = self.create_file_explorer(); }
            demoapp::Commands::Controls => { let _ = self.create_controls_demo(); }
            demoapp::Commands::TreeView => { let _ = self.create_tree_view(); }
            demoapp::Commands::ImageView => { let _ = self.create_image_view(); }
            demoapp::Commands::Settings => { let _ = self.create_settings(); }
            demoapp::Commands::About => self.show_about(),
            _ => (),
        }
    }

    fn on_select(&mut self, _: Handle<Menu>, _: Handle<menu::SingleChoice>, cmd: demoapp::Commands) {
        match cmd {
            demoapp::Commands::DefaultTheme => {
                App::set_theme(Theme::default());
            }
            demoapp::Commands::DarkTheme => {
                App::set_theme(Theme::dark());
            }
            demoapp::Commands::GridLayout => {
                self.layout = WindowLayout::Grid;
                self.arrange_windows(self.layout.clone());
            }
            demoapp::Commands::VerticalLayout => {
                self.layout = WindowLayout::Vertical;
                self.arrange_windows(self.layout.clone());
            }
            demoapp::Commands::CascadeLayout => {
                self.layout = WindowLayout::Cascade;
                self.arrange_windows(self.layout.clone());
            }
            _ => (),
        }
    }

    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.add(self.h_examples);
        menubar.add(self.h_theme);
        menubar.add(self.h_window);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(DemoApp::new());
    app.run();
    Ok(())
}
