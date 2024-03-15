pub(crate) static IMPORTS: &str = "
use $(ROOT)::prelude::*;
";

pub(crate) static IMPORTS_INTERNAL: &str = "
use crate::utils::*;
use crate::ui::common::*;
";

pub(crate) static MODAL_WINDOW_METHODS: &str = "
impl ModalWindowMethods<$(MODAL_RESULT_TYPE)> for $(STRUCT_NAME) {
    fn show(self) -> Option<$(MODAL_RESULT_TYPE)> {
        ModalWindow::show(self)
    }

    fn exit_with(&mut self, result: $(MODAL_RESULT_TYPE)) {
        self.base.exit_with(result);
    }

    fn exit(&mut self) {
        self.base.exit();
    }
}
";

pub(crate) static DEREF_TRAIT: &str = "
impl std::ops::Deref for $(STRUCT_NAME) {
    type Target = $(BASE);
    fn deref(&self) -> &Self::Target { return &self.base; }
}
impl std::ops::DerefMut for $(STRUCT_NAME) {
    fn deref_mut(&mut self) -> &mut Self::Target { return &mut self.base; }
}
";

pub(crate) static ON_PAINT_TRAIT: &str = "
impl OnPaint for $(STRUCT_NAME) {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme)  { self.base.on_paint(surface, theme); }
}
";

pub(crate) static ON_KEY_PRESSED_TRAIT: &str = "
impl OnKeyPressed for $(STRUCT_NAME) {
    fn on_key_pressed(&mut self, key: Key, character: char)->EventProcessStatus { return self.base.on_key_pressed(key, character); }
}
";

pub(crate) static ON_MOUSE_EVENT_TRAIT: &str = "
impl OnMouseEvent for $(STRUCT_NAME) {
    fn on_mouse_event(&mut self, event: &MouseEvent)->EventProcessStatus { return self.base.on_mouse_event(event); }
}
";

pub(crate) static ON_SIBLING_SELECTED: &str = "
impl OnSiblingSelected for $(STRUCT_NAME) {
    fn on_sibling_selected(&mut self, handle: Handle<UIElement>)  { self.base.on_sibling_selected(handle); }
}
";

pub(crate) static ON_DEFAULT_ACTION_TRAIT: &str = "
impl OnDefaultAction for $(STRUCT_NAME) {
    fn on_default_action(&mut self){ self.base.on_default_action(); }
}
";

pub(crate) static ON_RESIZE_TRAIT: &str = "
impl OnResize for $(STRUCT_NAME) {
    fn on_resize(&mut self, old: Size, new: Size)  { self.base.on_resize(old, new); }
}
";

pub(crate) static ON_FOCUS_TRAIT: &str = "
impl OnFocus for $(STRUCT_NAME) {
    fn on_focus(&mut self)  { self.base.on_focus(); }
    fn on_lose_focus(&mut self)  { self.base.on_lose_focus(); }
}
";

pub(crate) static ON_EXPAND_TRAIT: &str = "
impl OnExpand for $(STRUCT_NAME) {
    fn on_expand(&mut self, direction: ExpandedDirection) { self.base.on_expand(direction); }
    fn on_pack(&mut self) { self.base.on_pack(); }
}
";

pub(crate) static ON_WINDOW_REGISTERED_TRAIT: &str = "
impl OnWindowRegistered for $(STRUCT_NAME) {
    fn on_registered(&mut self)  { self.base.on_registered(); }
}
";

pub(crate) static COMMANDS_TEMPLATE: &str = "
mod $(MOD_NAME)
{
    use $(ROOT)::prelude::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Eq,PartialEq,Debug)]
    pub enum Commands {
        $(COMMANDS_IDS)
    }
    impl CommandID for Commands {}
    impl TryFrom<u32> for Commands {
        type Error = ();

        fn try_from(value: u32) -> Result<Self, Self::Error> {
            match value {
                $(U32_TO_COMMANDS)
                _ => Err(())
            }
        }
    }
    impl From<Commands> for u32 {
        fn from(value: Commands)->u32 {
            match value {
                $(COMMANDS_TO_U32)
            }
        }
    }
}
";
pub(crate) static COMMANDBAR_EVENTS: &str = "
trait CommandBarEvents {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar);
    fn on_event(&mut self, command_id: $(MOD_NAME)::Commands);
}
impl GenericCommandBarEvents for $(STRUCT_NAME) {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        CommandBarEvents::on_update_commandbar(self, commandbar);
    }
    fn on_event(&mut self, command_id: u32) {
        if let Ok(command) = $(MOD_NAME)::Commands::try_from(command_id) {
            CommandBarEvents::on_event(self, command);
        } else {
            panic!(\"Invalid internal state (can not convert value: {} into $(MOD_NAME)::Commands\",command_id);
        }
    }
}
";
pub(crate) static MENU_EVENTS: &str = "
trait MenuEvents {
    fn on_menu_open(&self, menu: &mut Menu) {}
    fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, command: $(MOD_NAME)::Commands) {}
    fn on_check(&mut self, menu: Handle<Menu>, item: Handle<menu::CheckBox>, command: $(MOD_NAME)::Commands, checked: bool) {}
    fn on_select(&mut self, menu: Handle<Menu>, item: Handle<menu::SingleChoice>, command: $(MOD_NAME)::Commands) {}
    fn on_update_menubar(&self, menubar: &mut MenuBar) {}
}
impl GenericMenuEvents for $(STRUCT_NAME) {
    fn on_menu_open(&self, menu: &mut Menu) {
        MenuEvents::on_menu_open(self, menu);
    }
    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        MenuEvents::on_update_menubar(self, menubar);
    }
    fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, command_id: u32) {
        if let Ok(command) = $(MOD_NAME)::Commands::try_from(command_id) {
            MenuEvents::on_command(self, menu, item, command);
        } else {
            panic!(\"Invalid internal state (can not convert value: {} into $(MOD_NAME)::Commands\",command_id);
        }
    }
    fn on_check(&mut self, menu: Handle<Menu>, item: Handle<menu::CheckBox>, command_id: u32, checked: bool) {
        if let Ok(command) = $(MOD_NAME)::Commands::try_from(command_id) {
            MenuEvents::on_check(self, menu, item, command, checked);
        } else {
            panic!(\"Invalid internal state (can not convert value: {} into $(MOD_NAME)::Commands\",command_id);
        }
    }
    fn on_select(&mut self, menu: Handle<Menu>, item: Handle<menu::SingleChoice>, command_id: u32) {
        if let Ok(command) = $(MOD_NAME)::Commands::try_from(command_id) {
            MenuEvents::on_select(self, menu, item, command);
        } else {
            panic!(\"Invalid internal state (can not convert value: {} into $(MOD_NAME)::Commands\",command_id);
        }
    }
}
";