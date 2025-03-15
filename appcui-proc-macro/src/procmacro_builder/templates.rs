pub(crate) static IMPORTS: &str = "
use $(ROOT)::prelude::*;
";

pub(crate) static IMPORTS_INTERNAL: &str = "
use crate::utils::*;
use crate::ui::common::*;
";

pub(crate) static MODAL_WINDOW_METHODS: &str = "
impl$(TEMPLATE_TYPE) ModalWindowMethods<$(MODAL_RESULT_TYPE)> for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn show(self) -> Option<$(MODAL_RESULT_TYPE)> {
        ModalWindow::show(self)
    }

    fn exit_with(&mut self, result: $(MODAL_RESULT_TYPE)) {
        self.base.exit_with(result);
    }

    fn exit(&mut self) {
        self.base.exit();
    }
    
    fn close(&mut self) {
        self.base.exit();
    }
}
";

pub(crate) static DEREF_TRAIT: &str = "
impl$(TEMPLATE_TYPE) std::ops::Deref for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    type Target = $(BASE);
    fn deref(&self) -> &Self::Target { return &self.base; }
}
impl$(TEMPLATE_TYPE) std::ops::DerefMut for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn deref_mut(&mut self) -> &mut Self::Target { return &mut self.base; }
}
";

pub(crate) static ON_PAINT_TRAIT: &str = "
impl$(TEMPLATE_TYPE) OnPaint for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme)  { self.base.on_paint(surface, theme); }
}
";

pub(crate) static ON_KEY_PRESSED_TRAIT: &str = "
impl$(TEMPLATE_TYPE) OnKeyPressed for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn on_key_pressed(&mut self, key: Key, character: char)->EventProcessStatus { return self.base.on_key_pressed(key, character); }
}
";

pub(crate) static ON_MOUSE_EVENT_TRAIT: &str = "
impl$(TEMPLATE_TYPE) OnMouseEvent for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn on_mouse_event(&mut self, event: &MouseEvent)->EventProcessStatus { return self.base.on_mouse_event(event); }
}
";

pub(crate) static ON_SIBLING_SELECTED: &str = "
impl$(TEMPLATE_TYPE) OnSiblingSelected for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn on_sibling_selected(&mut self, handle: Handle<UIElement>)  { self.base.on_sibling_selected(handle); }
}
";

pub(crate) static ON_DEFAULT_ACTION_TRAIT: &str = "
impl$(TEMPLATE_TYPE) OnDefaultAction for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn on_default_action(&mut self){ self.base.on_default_action(); }
}
";

pub(crate) static ON_RESIZE_TRAIT: &str = "
impl$(TEMPLATE_TYPE) OnResize for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn on_resize(&mut self, old: Size, new: Size)  { self.base.on_resize(old, new); }
}
";

pub(crate) static ON_FOCUS_TRAIT: &str = "
impl$(TEMPLATE_TYPE) OnFocus for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn on_focus(&mut self)  { self.base.on_focus(); }
    fn on_lose_focus(&mut self)  { self.base.on_lose_focus(); }
}
";

pub(crate) static ON_EXPAND_TRAIT: &str = "
impl$(TEMPLATE_TYPE) OnExpand for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn on_expand(&mut self, direction: ExpandedDirection) { self.base.on_expand(direction); }
    fn on_pack(&mut self) { self.base.on_pack(); }
}
";

pub(crate) static ON_THEME_CHANGED_TRAIT: &str = "
impl$(TEMPLATE_TYPE) OnThemeChanged for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn on_theme_changed(&mut self, theme: &Theme)  { self.base.on_theme_changed(theme); }
}
";

pub(crate) static ON_WINDOW_REGISTERED_TRAIT: &str = "
impl$(TEMPLATE_TYPE) OnWindowRegistered for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn on_registered(&mut self)  { self.base.on_registered(); }
}
";

pub(crate) static COMMANDS_TEMPLATE: &str = "
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

";
pub(crate) static COMMANDBAR_EVENTS: &str = "
trait CommandBarEvents {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar);
    fn on_event(&mut self, command_id: $(MOD_NAME)::Commands);
}
impl$(TEMPLATE_TYPE) GenericCommandBarEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {
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
impl$(TEMPLATE_TYPE) GenericMenuEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {
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

pub(crate) static EMIT_EVENTS_TEMPLATE: &str = "
    #[repr(u32)]
    #[derive(Copy,Clone,Eq,PartialEq,Debug)]
    pub enum Events {
        $(EVENTS_IDS)
    }
    impl TryFrom<u32> for Events {
        type Error = ();

        fn try_from(value: u32) -> Result<Self, Self::Error> {
            match value {
                $(U32_TO_EVENTS)
                _ => Err(())
            }
        }
    }
    impl From<Events> for u32 {
        fn from(value: Events)->u32 {
            match value {
                $(EVENTS_TO_U32)
            }
        }
    }

";
pub(crate) static RAISE_EVENTS_TEMPLATE: &str = "
trait RaiseEvents {
    fn raise_event(&self, event: $(MOD_NAME)::Events);
}
impl$(TEMPLATE_TYPE) RaiseEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn raise_event(&self, event: $(MOD_NAME)::Events) {
        self.raise_custom_event($(STRUCT_NAME_HASH),u32::from(event));
    }
}
";

pub(crate) static CUSTOM_EVENTS: &str = "
impl$(TEMPLATE_TYPE) CustomEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn on_event(&mut self, handle: Handle<()>, class_hash: u64, event_id: u32) -> EventProcessStatus {
        match class_hash {
            $(CUSTOM_EVENT_CLASS_PROXY_CALL)
            _ => EventProcessStatus::Ignored
        }        
    }
}
";

pub(crate) static CUSTOM_EVENT_CONVERTOR: &str = "
if let Ok(event) = $(MOD_NAME)::Events::try_from(event_id) {
    $(STRUC_NAME)Events::on_event(self, unsafe { handle.unsafe_cast() }, event)
} else {
    panic!(\"Invalid internal state (can not convert value: {} into $(MOD_NAME)::Events\",event_id);
}
";

pub(crate) static CUSTOM_TRAIT_DEF: &str = "
trait $(TRAIT_NAME) {
    fn on_event(&mut self, handle: Handle<$(STRUC_NAME)>, event:  $(MOD_NAME)::Events) -> EventProcessStatus;
}
";

pub(crate) static SELECTOR_TRAIT_DEF: &str = "
trait SelectorEvents<T: Copy+Clone+EnumSelector+Eq+'static> {
    fn on_selection_changed(&mut self, handle: Handle<Selector<T>>, value: Option<T>) -> EventProcessStatus;
}
impl$(TEMPLATE_TYPE) GenericSelectorEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn on_selection_changed(&mut self, handle: Handle<()>, type_id: std::any::TypeId) -> EventProcessStatus {
        $(TYPE_ID_TRANSLATION_FOR_SELECTOR)
        return EventProcessStatus::Ignored;
    }
}
";
pub(crate) static SELECT_ON_SELECTION_CHANGE_DEF: &str = "
if std::any::TypeId::of::<$(TYPE)>() == type_id {
    let h: Handle<Selector<$(TYPE)>> = unsafe { handle.unsafe_cast() };
    if let Some(obj) = self.control(h) {
        let value = obj.try_value();
        return SelectorEvents::<$(TYPE)>::on_selection_changed(self, h, value);
    }
    return EventProcessStatus::Ignored;
}
";

pub(crate) static DROPDOWNLIST_TRAIT_DEF: &str = "
trait DropDownListEvents<T: DropDownListType+'static> {
    fn on_selection_changed(&mut self, handle: Handle<DropDownList<T>>) -> EventProcessStatus;
}
impl$(TEMPLATE_TYPE) GenericDropDownListEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn on_selection_changed(&mut self, handle: Handle<()>, type_id: std::any::TypeId) -> EventProcessStatus {
        $(TYPE_ID_TRANSLATION_FOR_DROPDOWNLIST)
        return EventProcessStatus::Ignored;
    }
}
";
pub(crate) static SELECT_ON_DROPDOWNLIST_CHANGE_DEF: &str = "
if std::any::TypeId::of::<$(TYPE)>() == type_id {
    let h: Handle<DropDownList<$(TYPE)>> = unsafe { handle.unsafe_cast() };
    return DropDownListEvents::<$(TYPE)>::on_selection_changed(self, h);
}
";

pub(crate) static NUMERIC_SELECTOR_TRAIT_DEF: &str = "
trait NumericSelectorEvents<T: Number+'static> {
    fn on_value_changed(&mut self, handle: Handle<NumericSelector<T>>, value: T) -> EventProcessStatus;
}
impl$(TEMPLATE_TYPE) GenericNumericSelectorEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {
    fn on_value_changed(&mut self, handle: Handle<()>, type_id: std::any::TypeId) -> EventProcessStatus {
        $(TYPE_ID_TRANSLATION_FOR_NUMERIC_SELECTOR)
        return EventProcessStatus::Ignored;
    }
}
";
pub(crate) static NUMERIC_SELECT_ON_VALUE_CHANGE_DEF: &str = "
if std::any::TypeId::of::<$(TYPE)>() == type_id {
    let h: Handle<NumericSelector<$(TYPE)>> = unsafe { handle.unsafe_cast() };
    if let Some(obj) = self.control(h) {
        let value = obj.value();
        return NumericSelectorEvents::<$(TYPE)>::on_value_changed(self, h, value);
    }
    return EventProcessStatus::Ignored;
}
";


pub(crate) static LISTVIEW_ON_CURRENT_ITEM_CHANGED_DEF: &str = "
if std::any::TypeId::of::<$(TYPE)>() == type_id {
    let h: Handle<ListView<$(TYPE)>> = unsafe { handle.unsafe_cast() };
    return ListViewEvents::<$(TYPE)>::on_current_item_changed(self, h);
}
";

pub(crate) static LISTVIEW_ON_GROUP_EXPANDED_DEF: &str = "
if std::any::TypeId::of::<$(TYPE)>() == type_id {
    let h: Handle<ListView<$(TYPE)>> = unsafe { handle.unsafe_cast() };
    return ListViewEvents::<$(TYPE)>::on_group_expanded(self, h, group);
}
";

pub(crate) static LISTVIEW_ON_GROUP_COLLAPSED_DEF: &str = "
if std::any::TypeId::of::<$(TYPE)>() == type_id {
    let h: Handle<ListView<$(TYPE)>> = unsafe { handle.unsafe_cast() };
    return ListViewEvents::<$(TYPE)>::on_group_collapsed(self, h, group);
}
";

pub(crate) static LISTVIEW_ON_SELECTION_CHANGED_DEF: &str = "
if std::any::TypeId::of::<$(TYPE)>() == type_id {
    let h: Handle<ListView<$(TYPE)>> = unsafe { handle.unsafe_cast() };
    return ListViewEvents::<$(TYPE)>::on_selection_changed(self, h);
}
";

pub(crate) static LISTVIEW_ON_ITEM_ACTION_DEF: &str = "
if std::any::TypeId::of::<$(TYPE)>() == type_id {
    let h: Handle<ListView<$(TYPE)>> = unsafe { handle.unsafe_cast() };
    return ListViewEvents::<$(TYPE)>::on_item_action(self, h, index);
}
";

pub(crate) static LISTVIEW_TRAIT_DEF: &str = "
trait ListViewEvents<T: listview::ListItem+'static> {
    fn on_current_item_changed(&mut self, handle: Handle<ListView<T>>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_group_collapsed(&mut self, handle: Handle<ListView<T>>, group: listview::Group) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_group_expanded(&mut self, handle: Handle<ListView<T>>, group: listview::Group) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_selection_changed(&mut self, handle: Handle<ListView<T>>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_item_action(&mut self, handle: Handle<ListView<T>>, item_index: usize) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl$(TEMPLATE_TYPE) GenericListViewEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {

    fn on_current_item_changed(&mut self, handle: Handle<()>, type_id: std::any::TypeId) -> EventProcessStatus {
        $(TYPE_ID_TRANSLATION_FOR_LISTVIEW_ON_CURRENT_ITEM_CHANGED)
        return EventProcessStatus::Ignored;
    }

    fn on_group_collapsed(&mut self, handle: Handle<()>, type_id: std::any::TypeId, group: listview::Group) -> EventProcessStatus {
        $(TYPE_ID_TRANSLATION_FOR_LISTVIEW_ON_GROUP_COLLAPSED)
        return EventProcessStatus::Ignored;
    }

    fn on_group_expanded(&mut self, handle: Handle<()>, type_id: std::any::TypeId, group: listview::Group) -> EventProcessStatus {
        $(TYPE_ID_TRANSLATION_FOR_LISTVIEW_ON_GROUP_EXPANDED)
        return EventProcessStatus::Ignored;
    }
    
    fn on_selection_changed(&mut self, handle: Handle<()>, type_id: std::any::TypeId) -> EventProcessStatus {
        $(TYPE_ID_TRANSLATION_FOR_LISTVIEW_ON_SELECTION_CHANGED)
        return EventProcessStatus::Ignored;
    }

    fn on_item_action(&mut self, handle: Handle<()>, type_id: std::any::TypeId, index: usize) -> EventProcessStatus {
        $(TYPE_ID_TRANSLATION_FOR_LISTVIEW_ON_ITEM_ACTION)
        return EventProcessStatus::Ignored;
    }    

}
";


pub(crate) static TREEVIEW_ON_CURRENT_ITEM_CHANGED_DEF: &str = "
if std::any::TypeId::of::<$(TYPE)>() == type_id {
    let h: Handle<TreeView<$(TYPE)>> = unsafe { handle.unsafe_cast() };
    let i: Handle<treeview::Item<$(TYPE)>> = unsafe { item_handle.unsafe_cast() };
    return TreeViewEvents::<$(TYPE)>::on_current_item_changed(self, h, i);
}
";

pub(crate) static TREEVIEW_ON_ITEM_EXPANDED_DEF: &str = "
if std::any::TypeId::of::<$(TYPE)>() == type_id {
    let h: Handle<TreeView<$(TYPE)>> = unsafe { handle.unsafe_cast() };
    let i: Handle<treeview::Item<$(TYPE)>> = unsafe { item_handle.unsafe_cast() };
    return TreeViewEvents::<$(TYPE)>::on_item_expanded(self, h, i, recursive);
}
";

pub(crate) static TREEVIEW_ON_ITEM_COLLAPSED_DEF: &str = "
if std::any::TypeId::of::<$(TYPE)>() == type_id {
    let h: Handle<TreeView<$(TYPE)>> = unsafe { handle.unsafe_cast() };
    let i: Handle<treeview::Item<$(TYPE)>> = unsafe { item_handle.unsafe_cast() };
    return TreeViewEvents::<$(TYPE)>::on_item_collapsed(self, h, i, recursive);
}
";

pub(crate) static TREEVIEW_ON_SELECTION_CHANGED_DEF: &str = "
if std::any::TypeId::of::<$(TYPE)>() == type_id {
    let h: Handle<TreeView<$(TYPE)>> = unsafe { handle.unsafe_cast() };
    return TreeViewEvents::<$(TYPE)>::on_selection_changed(self, h);
}
";

pub(crate) static TREEVIEW_ON_ITEM_ACTION_DEF: &str = "
if std::any::TypeId::of::<$(TYPE)>() == type_id {
    let h: Handle<TreeView<$(TYPE)>> = unsafe { handle.unsafe_cast() };
    let i: Handle<treeview::Item<$(TYPE)>> = unsafe { item_handle.unsafe_cast() };
    return TreeViewEvents::<$(TYPE)>::on_item_action(self, h, i);
}
";


pub(crate) static TREEVIEW_TRAIT_DEF: &str = "
trait TreeViewEvents<T: treeview::ListItem+'static> {

    fn on_current_item_changed(&mut self, handle: Handle<TreeView<T>>, item_handle: Handle<treeview::Item<T>>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_item_collapsed(&mut self, handle: Handle<TreeView<T>>, item_handle: Handle<treeview::Item<T>>, recursive: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_item_expanded(&mut self, handle: Handle<TreeView<T>>, item_handle: Handle<treeview::Item<T>>, recursive: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_item_action(&mut self, handle: Handle<TreeView<T>>, item_handle: Handle<treeview::Item<T>>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_selection_changed(&mut self, handle: Handle<TreeView<T>>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl$(TEMPLATE_TYPE) GenericTreeViewEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {

    fn on_current_item_changed(&mut self, handle: Handle<()>, type_id: std::any::TypeId, item_handle: Handle<()>) -> EventProcessStatus {
        $(TYPE_ID_TRANSLATION_FOR_TREEVIEW_ON_CURRENT_ITEM_CHANGED)
        EventProcessStatus::Ignored
    }
    fn on_item_collapsed(&mut self, handle: Handle<()>, type_id: std::any::TypeId, item_handle: Handle<()>, recursive: bool) -> EventProcessStatus {
        $(TYPE_ID_TRANSLATION_FOR_TREEVIEW_ON_ITEM_COLLAPSED)
        EventProcessStatus::Ignored
    }
    fn on_item_expanded(&mut self, handle: Handle<()>, type_id: std::any::TypeId, item_handle: Handle<()>, recursive: bool) -> EventProcessStatus {
        $(TYPE_ID_TRANSLATION_FOR_TREEVIEW_ON_ITEM_EXPANDED)
        EventProcessStatus::Ignored
    }
    fn on_selection_changed(&mut self, handle: Handle<()>, type_id: std::any::TypeId) -> EventProcessStatus {
        $(TYPE_ID_TRANSLATION_FOR_TREEVIEW_ON_SELECTION_CHANGED)
        EventProcessStatus::Ignored
    }
    fn on_item_action(&mut self, handle: Handle<()>, type_id: std::any::TypeId, item_handle: Handle<()>) -> EventProcessStatus {
        $(TYPE_ID_TRANSLATION_FOR_TREEVIEW_ON_ITEM_ACTION)
        EventProcessStatus::Ignored
    }
}
";

pub(crate) static BACKGROUNDTASK_ON_UPDATE_DEF: &str = "
if std::any::TypeId::of::<$(TYPE)>() == type_id {
    let h: Handle<TreeView<$(TYPE)>> = unsafe { handle.unsafe_cast() };
    let i: Handle<treeview::Item<$(TYPE)>> = unsafe { item_handle.unsafe_cast() };
    return TreeViewEvents::<$(TYPE)>::on_item_action(self, h, i);
}
";
pub(crate) static BACKGROUNDTASK_TRAIT_DEF: &str = "
trait BackgroundTaskEvents<T: Send+'static, R: Send+'static> {
    fn on_start(&mut self, task: &BackgroundTask<T,R>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_update(&mut self, value: T, task: &BackgroundTask<T,R>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_finish(&mut self, task: &BackgroundTask<T,R>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl$(TEMPLATE_TYPE) GenericBackgroundTaskEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {

    fn on_start(&mut self, id: u32) -> EventProcessStatus {
        $(TYPE_ID_TRANSLATION_FOR_BACKGROUNDTASK_ON_START)
        EventProcessStatus::Ignored
    }
    fn on_update(&mut self, id: u32) -> EventProcessStatus {
        $(TYPE_ID_TRANSLATION_FOR_BACKGROUNDTASK_ON_UPDATE)
        EventProcessStatus::Ignored
    }
    fn on_finish(&mut self, id: u32) -> EventProcessStatus {
        $(TYPE_ID_TRANSLATION_FOR_BACKGROUNDTASK_ON_FINISH)
        EventProcessStatus::Ignored
    }
}
";