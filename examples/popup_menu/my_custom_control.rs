use std::char::CharTryFromError;

use appcui::prelude::*;

#[CustomControl(events = MenuEvents, overwrite = OnPaint+OnMouseEvent, commands = Red+Green+Black)]
pub struct MyCustomControl {
    col: Color,
    h_menu: Handle<Menu>,
}
impl MyCustomControl {
    pub fn new(layout: Layout) -> Self {
        let mut obj = Self {
            base: ControlBase::new(layout, true),
            col: Color::Red,
            h_menu: Handle::None,
        };
        let m = menu!(
            "ColorControl,class:MyCustomControl,items=[
            {Red,selected:true,cmd:Red},
            {Green,selected:false,cmd:Green},
            {Black,selected:false,cmd:Black}
        ]"
        );
        obj.h_menu = obj.register_menu(m);
        obj
    }
}
impl OnPaint for MyCustomControl {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(Character::new(' ', Color::Black, self.col, CharFlags::None));
        let sz = self.get_client_size();
        let attr = CharAttribute::with_fore_color(Color::White);
        let line = if self.has_focus() { LineType::Double } else { LineType::Single };
        let r = Rect::with_size(0, 0, sz.width as u16, sz.height as u16);
        surface.draw_rect(r, line, attr);
    }
}
impl MenuEvents for MyCustomControl {
    fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<menu::SingleChoice>, command: mycustomcontrol::Commands) {
        match command {
            mycustomcontrol::Commands::Red => self.col = Color::DarkRed,
            mycustomcontrol::Commands::Green => self.col = Color::DarkGreen,
            mycustomcontrol::Commands::Black => self.col = Color::Black,
        }
    }
}
impl OnMouseEvent for MyCustomControl {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if let MouseEvent::Pressed(ev) = event {
            if ev.button == MouseButton::Right {
                self.show_menu(self.h_menu, 1, 1);
                return EventProcessStatus::Processed;
            }
        }
        EventProcessStatus::Ignored
    }
}
