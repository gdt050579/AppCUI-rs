use appcui::prelude::*;
use appcui::ui::appbar::*;

#[CustomControl(events = MenuEvents+AppBarEvents, overwrite = OnPaint, commands = Red+Green+Blue)]
pub struct ColorCustomControl {
    col: Color,
    h_menu: Handle<MenuButton>,
}
impl ColorCustomControl {
    pub fn new(layout: Layout) -> Self {
        let mut obj = Self {
            base: ControlBase::new(layout, true),
            col: Color::Red,
            h_menu: Handle::None,
        };
        let m = menu!(
            "class:ColorCustomControl,items=[
            {Red,F1,selected:true,cmd:Red},
            {Green,F2,selected:false,cmd:Green},
            {Blue,F3,selected:false,cmd:Blue}
        ]"
        );
        obj.h_menu = obj.appbar().add(MenuButton::new("ColorControl", m, 0, Side::Left));
        obj
    }
}
impl OnPaint for ColorCustomControl {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(Character::new(' ', Color::Black, self.col, CharFlags::None));
        if self.has_focus() {
            surface.write_string(1, 0, "Focus", CharAttribute::with_fore_color(Color::Yellow), false);
        }
    }
}
impl MenuEvents for ColorCustomControl {
    fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<menu::SingleChoice>, command: colorcustomcontrol::Commands) {
        match command {
            colorcustomcontrol::Commands::Red => self.col = Color::Red,
            colorcustomcontrol::Commands::Green => self.col = Color::Green,
            colorcustomcontrol::Commands::Blue => self.col = Color::Blue,
        }
    }
}
impl AppBarEvents for ColorCustomControl {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.h_menu);
    }
}
