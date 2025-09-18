use appcui::prelude::*;
use appcui::ui::appbar::*;
mod file_navigator;
mod base_controls;
mod image_win;
mod animation;
mod tree_example;
mod color_palette;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const LOGO: [&str; 15] = [
    "▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒",
    "▒▒█████┐▒▒██████┐▒▒██████┐▒▒",
    "▒██┌──██┐▒██┌──██┐▒██┌──██┐▒",
    "▒███████│▒██████┌┘▒██████┌┘▒",
    "▒██┌──██│▒██┌───┘▒▒██┌───┘▒▒",
    "▒██│▒▒██│▒██│▒▒▒▒▒▒██│▒▒▒▒▒▒",
    "▒└─┘▒▒└─┘▒└─┘▒▒▒▒▒▒└─┘▒▒▒▒▒▒",
    "▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒",
    "▒▒▒▒█████┐▒▒██┐▒▒▒██┐▒██┐▒▒▒",
    "▒▒▒██┌──██┐▒██│▒▒▒██│▒██│▒▒▒",
    "▒▒▒██│▒▒└─┘▒██│▒▒▒██│▒██│▒▒▒",
    "▒▒▒██│▒▒██┐▒██│▒▒▒██│▒██│▒▒▒",
    "▒▒▒└█████┌┘▒└██████┌┘▒██│▒▒▒",
    "▒▒▒▒└────┘▒▒▒└─────┘▒▒└─┘▒▒▒",
    "▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒",
];

#[Desktop(events    = [CommandBarEvents,MenuEvents,DesktopEvents,AppBarEvents], 
          overwrite = OnPaint, 
          commands  = [Lists, BaseControls, Images, Animation, TreeExample, ColorPalette, Exit, NoArrange, Cascade, Vertical, Horizontal, Grid, DefaultTheme, DarkGrayTheme, LightTheme])]
struct MyDesktop {
    arrange_method: Option<desktop::ArrangeWindowsMethod>,
    menu_arrange: Handle<MenuButton>,
    menu_examples: Handle<MenuButton>,
    menu_theme: Handle<MenuButton>,
}
impl MyDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new(),
            arrange_method: None,
            menu_arrange: Handle::None,
            menu_examples: Handle::None,
            menu_theme: Handle::None,
        }
    }
}
impl OnPaint for MyDesktop {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.clear(theme.desktop.character);
        let attr = CharAttribute::with_color(theme.desktop.character.foreground,theme.desktop.character.background);
        let x = ((surface.size().width as i32) / 2 ) - 15;
        let mut y = ((surface.size().height as i32) / 2 ) - 7;
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
        // define and register a menu
        self.menu_examples = self.appbar().add(MenuButton::new("&Examples", menu!("
            class: MyDesktop, items:[
                { Lists, cmd: Lists}, 
                { 'Base Controls', cmd: BaseControls},
                { Images, cmd: Images},
                { Animation, cmd: Animation},
                { 'Tree Example', cmd: TreeExample},
                { 'Color Palette', cmd: ColorPalette}
            ]
        "),0,Side::Left));
        self.menu_arrange = self.appbar().add(MenuButton::new("&Windows", menu!("
            class: MyDesktop, items:[
                {'&No arrangament',cmd: NoArrange, select: true},
                {&Cascade,cmd: Cascade, select: false},
                {&Vertical,cmd: Vertical, select: false},
                {&Horizontal,cmd: Horizontal, select: false},
                {&Grid,cmd: Grid, select: false},
            ]
        "),0,Side::Left));
        self.menu_theme = self.appbar().add(MenuButton::new("&Theme", menu!("
            class: MyDesktop, items:[
                {&Default,cmd: DefaultTheme, select: true},
                {'Dark &Gray',cmd: DarkGrayTheme, select: false},
                {'&Light',cmd: LightTheme, select: false}
            ]
        "),0,Side::Left));
    }
        
}
impl CommandBarEvents for MyDesktop {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        commandbar.set(key!("Escape"), "Exit", mydesktop::Commands::Exit);
    }

    fn on_event(&mut self, command_id: mydesktop::Commands) {
        if command_id == mydesktop::Commands::Exit { self.close() }
    }
}
impl MenuEvents for MyDesktop {
    fn on_select(&mut self,_menu:Handle<Menu>,_item:Handle<menu::SingleChoice>,command:mydesktop::Commands){
        match command {
            mydesktop::Commands::NoArrange => self.arrange_method = None,
            mydesktop::Commands::Cascade => self.arrange_method = Some(desktop::ArrangeWindowsMethod::Cascade),
            mydesktop::Commands::Vertical => self.arrange_method = Some(desktop::ArrangeWindowsMethod::Vertical),
            mydesktop::Commands::Horizontal => self.arrange_method = Some(desktop::ArrangeWindowsMethod::Horizontal),
            mydesktop::Commands::Grid => self.arrange_method = Some(desktop::ArrangeWindowsMethod::Grid),
            mydesktop::Commands::DefaultTheme => App::set_theme(Theme::new(Themes::Default)),
            mydesktop::Commands::DarkGrayTheme => App::set_theme(Theme::new(Themes::DarkGray)),
            mydesktop::Commands::LightTheme => App::set_theme(Theme::new(Themes::Light)),
            _ => {}
        }
        let m = self.arrange_method;
        if let Some(method) = m {
            self.arrange_windows(method);
        }
    }    
    fn on_command(&mut self,_:Handle<Menu>,_:Handle<menu::Command>,command:mydesktop::Commands){
        match command {
            mydesktop::Commands::Lists => { self.add_window(file_navigator::Win::new()); },
            mydesktop::Commands::Images => { self.add_window(image_win::Win::new()); },
            mydesktop::Commands::BaseControls => { self.add_window(base_controls::Win::new()); },
            mydesktop::Commands::Animation => { self.add_window(animation::Win::new()); },
            mydesktop::Commands::TreeExample => { self.add_window(tree_example::Win::new()); },
            mydesktop::Commands::ColorPalette => { self.add_window(color_palette::Win::new()); },
            _ => {}
        }
        let m = self.arrange_method;
        if let Some(method) = m {
            self.arrange_windows(method);
        }
    }
}
impl AppBarEvents for MyDesktop {
    fn on_update(&self,appbar: &mut AppBar) {
        appbar.show(self.menu_examples);
        appbar.show(self.menu_arrange);
        appbar.show(self.menu_theme);
    }
}


#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), appcui::system::Error> {
    #[cfg(target_family = "windows")]
    App::with_backend(appcui::backend::Type::WindowsVT)
        .desktop(MyDesktop::new())
        .app_bar()
        .command_bar()
        .build()?
        .run();

    #[cfg(not(target_family = "windows"))]
    App::new().desktop(MyDesktop::new()).menu_bar().command_bar().build()?.run();
    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {    
    // Important for WASM: the project must be a lib that should be built with `wasm-pack build --target web`
    let app = App::new().desktop(MyDesktop::new()).menu_bar().command_bar().build().unwrap();
    app.run();
}
