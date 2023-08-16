pub trait MenuEvents {
    fn on_menu_open(&self, _menu: &mut Menu) {}
    fn on_event(&mut self, _event: MenuEvent) {}
    fn on_update_menubar(&self, _menubar: &mut MenuBar) {}
}