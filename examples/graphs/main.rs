use appcui::prelude::*;
mod settings;
mod graph_window;
mod tree;
mod circular;
mod bipartite;

use graph_window::GraphWindow;

const LOGO: [&str; 6] = [
    " ██████╗ ██████╗  █████╗ ██████╗ ██╗  ██╗██╗   ██╗██╗███████╗██╗    ██╗",
    "██╔════╝ ██╔══██╗██╔══██╗██╔══██╗██║  ██║██║   ██║██║██╔════╝██║    ██║",
    "██║  ███╗██████╔╝███████║██████╔╝███████║██║   ██║██║█████╗  ██║ █╗ ██║",
    "██║   ██║██╔══██╗██╔══██║██╔═══╝ ██╔══██║╚██╗ ██╔╝██║██╔══╝  ██║███╗██║",
    "╚██████╔╝██║  ██║██║  ██║██║     ██║  ██║ ╚████╔╝ ██║███████╗╚███╔███╔╝",
    " ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝  ╚═╝  ╚═══╝  ╚═╝╚══════╝ ╚══╝╚══╝ "                                                                          
];

#[Desktop(events    = [MenuEvents,DesktopEvents], 
          overwrite = OnPaint, 
          commands  = [ShowTree, ShowCircular, ShowBipartite,
                       Exit, About, 
                       NoArrange, Cascade, Vertical, Horizontal, Grid])]
struct MyDesktop {
    index: u32,
    arrange_method: Option<desktop::ArrangeWindowsMethod>,
    menu_arrange: Handle<Menu>,
    menu_examples: Handle<Menu>,
    menu_help: Handle<Menu>,
}

impl MyDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new(),
            index: 1,
            arrange_method: None,
            menu_arrange: Handle::None,
            menu_examples: Handle::None,
            menu_help: Handle::None,
        }
    }
    
    fn update_arrange_windows_method(&mut self, method: Option<desktop::ArrangeWindowsMethod>) {
        self.arrange_method = method;
        if let Some(method) = method {
            self.arrange_windows(method);
        }
    }  
}

impl OnPaint for MyDesktop {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let attr = CharAttribute::with_color(theme.desktop.character.foreground, theme.desktop.character.background);
        surface.clear(Character::with_attributes(' ', attr));
        let x = (surface.size().width as i32) - 70;
        let mut y = (surface.size().height as i32) - 7;
        for line in LOGO {
            surface.write_string(x, y, line, attr, false);
            y += 1;
        }
    }
}

impl DesktopEvents for MyDesktop {
    fn on_update_window_count(&mut self, _count: usize) {
        let m = self.arrange_method;
        if let Some(method) = m {
            self.arrange_windows(method);
        }
    }
    
    fn on_start(&mut self) { 
        // define and register menus
        self.menu_arrange = self.register_menu(menu!("
            &Windows,class: MyDesktop, items:[
                {'&No arrangement',cmd: NoArrange, select: true},
                {&Cascade,cmd: Cascade, select: false},
                {&Vertical,cmd: Vertical, select: false},
                {&Horizontal,cmd: Horizontal, select: false},
                {&Grid,cmd: Grid, select: false},
            ]
        "));
        
        self.menu_examples = self.register_menu(menu!("
            &Examples,class: MyDesktop, items:[
                {'&Tree Graph',cmd: ShowTree},
                {'&Circular Graph',cmd: ShowCircular},
                {'&Bipartite Graph',cmd: ShowBipartite},
            ]
        "));
        
        self.menu_help = self.register_menu(menu!("
            &Help,class: MyDesktop, items:[
                {&About,cmd: About},
                {E&xit,cmd: Exit},
            ]
        "));
    }  
}

impl MenuEvents for MyDesktop {
    fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<menu::SingleChoice>, command: mydesktop::Commands) {
        match command {
            mydesktop::Commands::NoArrange => self.update_arrange_windows_method(None),
            mydesktop::Commands::Cascade => self.update_arrange_windows_method(Some(desktop::ArrangeWindowsMethod::Cascade)),
            mydesktop::Commands::Vertical => self.update_arrange_windows_method(Some(desktop::ArrangeWindowsMethod::Vertical)),
            mydesktop::Commands::Horizontal => self.update_arrange_windows_method(Some(desktop::ArrangeWindowsMethod::Horizontal)),
            mydesktop::Commands::Grid => self.update_arrange_windows_method(Some(desktop::ArrangeWindowsMethod::Grid)),
            _ => {}
        }
    }
    
    fn on_command(&mut self, _menu: Handle<Menu>, _item: Handle<menu::Command>, command: mydesktop::Commands) {
        match command {
            mydesktop::Commands::ShowTree => { 
                let (graph, settings) = tree::create();
                self.add_window(GraphWindow::new(graph, settings));
            },
            mydesktop::Commands::ShowCircular => { 
                let (graph, settings) = circular::create();
                self.add_window(GraphWindow::new(graph, settings));
            },
            mydesktop::Commands::ShowBipartite => { 
                let (graph, settings) = bipartite::create();
                self.add_window(GraphWindow::new(graph, settings));
            },
            mydesktop::Commands::Exit => self.close(),
            mydesktop::Commands::About => {
                dialogs::message("GraphView Example", "This is an example demonstrating the GraphView control in AppCUI.\n\nFeatures:\n• Tree graphs with hierarchical layout\n• Circular graphs with radial arrangement\n• Bipartite graphs with two-layer structure\n• Interactive graph configuration options\n\nEach graph window includes a Graph menu with options to:\n• Change node arrangement algorithms\n• Toggle arrow heads and edge highlighting\n• Modify edge line types and routing");
            },     
            _ => { }      
        }
    }

    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.add(self.menu_examples, 0);
        menubar.add(self.menu_arrange, 0);
        menubar.add(self.menu_help, 0);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().desktop(MyDesktop::new()).menu_bar().build()?.run();
    Ok(())
}