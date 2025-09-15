use appcui::prelude::*;
use appcui::ui::appbar::*;

#[CustomControl(events = MenuEvents, overwrite = OnPaint, commands = Red+Green+Blue)]
pub struct TextCustomControl {
    text: &'static str,
    h_menu: Handle<MenuEntry>,
}
impl TextCustomControl {
    pub fn new(layout: Layout) -> Self {
        let mut obj = Self {
            base: ControlBase::new(layout, true),
            text: "Red",
            h_menu: Handle::None,
        };
        let m = menu!(
            "class:TextCustomControl,items=[
                {'Text->Red',F1,selected:true,cmd:Red},
                {'Text->Green',F2,selected:false,cmd:Green},
                {'Text->Blue',F3,selected:false,cmd:Blue}
            ]"
        );
        obj.h_menu = obj.appbar_mut().add(MenuEntry::new("Text", m,0,AppBarPosition::Left));
        obj
    }
}
impl OnPaint for TextCustomControl {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        if self.has_focus() {
            surface.clear(Character::new(' ', Color::Black, Color::Black, CharFlags::None));
            surface.write_string(1, 0, self.text, CharAttribute::with_fore_color(Color::Yellow), false);
        } else {
            surface.clear(Character::new(' ', Color::Blue, Color::Blue, CharFlags::None));
            surface.write_string(1, 0, self.text, CharAttribute::with_fore_color(Color::Yellow), false);
        }
    }
}
impl MenuEvents for TextCustomControl {
    fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<menu::SingleChoice>, command: textcustomcontrol::Commands) {
        match command {
            textcustomcontrol::Commands::Red => self.text = "Red",
            textcustomcontrol::Commands::Green => self.text = "Green",
            textcustomcontrol::Commands::Blue =>self.text = "Blue",
        }
    }

    fn on_update_menubar(&self, menubar: &mut AppBar) {
        menubar.show(self.h_menu);
    }
}
