use appcui::prelude::*;
use super::painter_control::PainterControl;

#[Window(events = MenuEvents + ColorPickerEvents + ButtonEvents, commands = ForegroundColor + BackgroundColor + Char25 + Char50 + Char75 + Char100)]
pub struct PainterWindow {
    painter: Handle<PainterControl>,
    menu: Handle<Menu>,
    fg_color_picker: Handle<ColorPicker>,
    bg_color_picker: Handle<ColorPicker>,
    clear_button: Handle<Button>,
}

impl PainterWindow {
    pub fn new(name: &str) -> Self {
        let mut w = Self {
            base: Window::new(name, Layout::new("d:c,w:60,h:20"), window::Flags::Sizeable),
            painter: Handle::None,
            menu: Handle::None,
            fg_color_picker: Handle::None,
            bg_color_picker: Handle::None,
            clear_button: Handle::None,
        };

        let m = menu!("
            &Options,class:PainterWindow,items:[
                {'&25% Block',1,cmd:Char25},
                {'&50% Block',2,cmd:Char50},
                {'&75% Block',3,cmd:Char75},
                {'&100% Block',4,cmd:Char100}
            ]
        ");
        w.menu = w.register_menu(m);
        w.add(label!("'ForeColor:',t:0,l:0,w:10,h:1"));
        w.add(label!("'BackColor:',t:0,l:23,w:10,h:1"));

        w.fg_color_picker = w.add(ColorPicker::new(Color::White, Layout::new("t:0,l:10,w:8,h:1")));
        w.bg_color_picker = w.add(ColorPicker::new(Color::Black, Layout::new("t:0,l:33,w:8,h:1")));

        w.clear_button = w.add(button!("Clear,t:0,l:46,w:9,h:1,type: Flat"));

        w.painter = w.add(PainterControl::new(Layout::new("t:1,l:0,r:0,b:0")));
        w
    }

    pub fn set_drawing_char(&mut self, ch: char) {
        let h = self.painter;
        if let Some(p) = self.control_mut(h) {
            p.set_drawing_char(ch);
        }
    }

    pub fn set_foreground_color(&mut self, color: Color) {
        let h = self.painter;
        if let Some(p) = self.control_mut(h) {
            p.set_foreground_color(color);
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        let h = self.painter;
        if let Some(p) = self.control_mut(h) {
            p.set_background_color(color);
        }
    }

    pub fn clear_surface(&mut self) {
        let h = self.painter;
        if let Some(p) = self.control_mut(h) {
            p.clear_surface();
        }
    }
}

impl MenuEvents for PainterWindow {
    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.add(self.menu);
    }

    fn on_command(&mut self, _menu: Handle<Menu>, _item: Handle<menu::Command>, command: painterwindow::Commands) {
        match command {
            painterwindow::Commands::Char25 => {
                self.set_drawing_char('░');
            }
            painterwindow::Commands::Char50 => {
                self.set_drawing_char('▒');
            }
            painterwindow::Commands::Char75 => {
                self.set_drawing_char('▓');
            }
            painterwindow::Commands::Char100 => {
                self.set_drawing_char('█');
            }
            _ => {}
        }
    }
}

impl ColorPickerEvents for PainterWindow {
    fn on_color_changed(&mut self, handle: Handle<ColorPicker>, color: Color) -> EventProcessStatus {
        if handle == self.fg_color_picker {
            self.set_foreground_color(color);
            EventProcessStatus::Processed
        } else if handle == self.bg_color_picker {
            self.set_background_color(color);
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

impl ButtonEvents for PainterWindow {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.clear_button {
            self.clear_surface();
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
} 