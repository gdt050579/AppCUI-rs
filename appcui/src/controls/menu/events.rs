use super::{MenuBar, Menu};

pub trait MenuEvents {
    fn on_menu_open(&self, _menu: &mut Menu) {}
    fn on_item_clicked(&mut self, _command: u32) {}
    fn on_update_menubar(&self, _menubar: &mut MenuBar) {}
}