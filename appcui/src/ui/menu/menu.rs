use super::{
    events::*, menu_button_state::MenuButtonState, menu_item::MenuItem, mouse_position_info::MousePositionInfo, CheckBox, Command, MenuItemWrapper,
    Separator, SingleChoice, SubMenu,
};
use crate::{
    graphics::{Character, ClipArea, LineType, Rect, Size, SpecialChar, Surface, TextAlignament, TextFormatBuilder, WrapType},
    input::{Key, KeyCode, MouseWheelDirection},
    prelude::KeyModifier,
    system::{Handle, HandleSupport, RuntimeManager, Theme},
    ui::common::traits::EventProcessStatus,
    utils::{Caption, ExtractHotKeyMethod, Strategy, VectorIndex},
};
use appcui_proc_macro::key;
use std::sync::atomic::{AtomicUsize, Ordering};
const MAX_ITEMS: usize = 128;
static GLOBAL_MENUITEM_ID: AtomicUsize = AtomicUsize::new(0);

/// A container for menu items that can be displayed over existing controls.
///
/// A menu is a list of items (commands, checkboxes, single choice elements) that
/// can be displayed over existing controls. Menus can be added to a menu bar or
/// displayed as popup menus.
///
/// # Examples
///
/// Creating a menu with various items:
///
/// ```
/// use appcui::prelude::*;
///
/// // Define a window with menu events and commands
/// #[Window(events = MenuEvents, commands = New+Open+Save+Exit)]
/// struct MyWindow {
///     file_menu: Handle<Menu>,
/// }
///
/// impl MyWindow {
///     fn new() -> Self {
///         let mut w = MyWindow {
///             base: window!("Example,d:c,w:40,h:10"),
///             file_menu: Handle::None,
///         };
///         
///         // Create a menu and add items to it
///         let mut file_menu = Menu::new("&File");
///         file_menu.add(menu::Command::new("&New", key!("Ctrl+N"), mywindow::Commands::New));
///         file_menu.add(menu::Command::new("&Open", key!("Ctrl+O"), mywindow::Commands::Open));
///         file_menu.add(menu::Command::new("&Save", key!("Ctrl+S"), mywindow::Commands::Save));
///         file_menu.add(menu::Separator::new());
///         file_menu.add(menu::Command::new("E&xit", key!("Alt+F4"), mywindow::Commands::Exit));
///         
///         // Register the menu with the window
///         w.file_menu = w.register_menu(file_menu);
///         
///         w
///     }
/// }
///
/// // Implement menu event handlers
/// impl MenuEvents for MyWindow {
///     fn on_update_menubar(&self, menubar: &mut MenuBar) {
///         // Add the menu to the menu bar
///         menubar.add(self.file_menu);
///     }
///     
///     fn on_command(&mut self, _menu: Handle<Menu>, _item: Handle<menu::Command>, command: mywindow::Commands) {
///         match command {
///             mywindow::Commands::New => { /* Handle New command */ },
///             mywindow::Commands::Open => { /* Handle Open command */ },
///             mywindow::Commands::Save => { /* Handle Save command */ },
///             mywindow::Commands::Exit => { /* Handle Exit command */ },
///         }
///     }
/// }
/// ```
///
/// Using the `menu!` macro for more concise menu creation:
///
/// ```
/// use appcui::prelude::*;
///
/// #[Window(events = MenuEvents, commands = New+Open+Save+Exit)]
/// struct MyWindow {
///     file_menu: Handle<Menu>,
/// }
///
/// impl MyWindow {
///     fn new() -> Self {
///         let mut w = MyWindow {
///             base: window!("Example,d:c,w:40,h:10"),
///             file_menu: Handle::None,
///         };
///         
///         // Create a menu using the menu! macro
///         w.file_menu = w.register_menu(menu!("&File,class:MyWindow,items=[
///             { &New,Ctrl+N,cmd:New },
///             { &Open,Ctrl+O,cmd:Open },
///             { &Save,Ctrl+S,cmd:Save },
///             { --- },
///             { E&xit,Alt+F4,cmd:Exit }
///         ]"));
///         
///         w
///     }
/// }
/// // Implement menu event handlers
/// impl MenuEvents for MyWindow {
///     fn on_update_menubar(&self, menubar: &mut MenuBar) {
///         // Add the menu to the menu bar
///         menubar.add(self.file_menu);
///     }
///     
///     fn on_command(&mut self, _menu: Handle<Menu>, _item: Handle<menu::Command>, command: mywindow::Commands) {
///         match command {
///             mywindow::Commands::New => { /* Handle New command */ },
///             mywindow::Commands::Open => { /* Handle Open command */ },
///             mywindow::Commands::Save => { /* Handle Save command */ },
///             mywindow::Commands::Exit => { /* Handle Exit command */ },
///         }
///     }
/// }
/// ```
pub struct Menu {
    pub(super) caption: Caption,
    pub(super) items: Vec<MenuItemWrapper>,
    pub(super) current: VectorIndex,
    pub(super) width: u16,
    pub(super) text_width: u16,
    pub(super) first_visible_item: u32,
    pub(super) visible_items_count: u32,
    pub(super) button_up: MenuButtonState,
    pub(super) button_down: MenuButtonState,
    pub(super) clip: ClipArea,
    pub(super) handle: Handle<Menu>,
    pub(super) parent_handle: Handle<Menu>,
    pub(super) receiver_control_handle: Handle<()>,
}

impl Menu {
    /// Creates a new menu with the specified name.
    ///
    /// The name can include the special character `&`, which designates the next
    /// character as a hotkey to activate the menu (e.g., "&File" makes 'F' the hotkey,
    /// typically activated with Alt+F).
    ///
    /// # Parameters
    /// * `name` - The text to display for the menu. If empty, a default caption is used.
    ///
    /// # Returns
    /// A new `Menu` instance.
    pub fn new(name: &str) -> Self {
        Self {
            caption: if name.is_empty() {
                Caption::default()
            } else {
                Caption::new(name, ExtractHotKeyMethod::AltPlusKey)
            },
            items: Vec::with_capacity(4),
            current: VectorIndex::Invalid,
            width: 1,
            text_width: 0,
            first_visible_item: 0,
            visible_items_count: 0,
            button_up: MenuButtonState::Normal,
            button_down: MenuButtonState::Normal,
            clip: ClipArea::new(0, 0, 1, 1),
            handle: Handle::None,
            parent_handle: Handle::None,
            receiver_control_handle: Handle::None,
        }
    }

    /// Adds a new menu item to the existing menu.
    ///
    /// # Parameters
    /// * `menuitem` - The menu item to add. This can be a Command, CheckBox,
    ///   SingleChoice, Separator, or SubMenu.
    ///
    /// # Returns
    /// A handle to the added menu item, which can be used to access or modify
    /// the item later.
    #[allow(private_bounds)]
    pub fn add<T>(&mut self, mut menuitem: T) -> Handle<T>
    where
        T: MenuItem,
    {
        let id = (GLOBAL_MENUITEM_ID.fetch_add(1, Ordering::SeqCst) as u32) % 0xFFFF_FFFE;
        let h: Handle<T> = Handle::with_id(id, self.items.len() as u32);
        menuitem.update_handles(self.handle, h.cast());
        self.items.push(menuitem.into_menuitem());
        h
    }

    /// Gets an immutable reference to a menu item by its handle.
    ///
    /// # Parameters
    /// * `menuitem_hamdle` - Handle to the menu item to retrieve.
    ///
    /// # Returns
    /// An `Option` containing a reference to the menu item if found, or `None` if
    /// the item doesn't exist or the handle is invalid.
    #[allow(private_bounds)]
    pub fn get<T>(&self, menuitem_hamdle: Handle<T>) -> Option<&T>
    where
        T: MenuItem,
    {
        let idx = menuitem_hamdle.index();
        if idx >= self.items.len() {
            return None;
        }
        let item = &self.items[idx];
        if item.get_handle() != menuitem_hamdle {
            return None;
        }
        match item {
            MenuItemWrapper::Command(obj) => Some(unsafe { &(*((obj as *const Command) as *const T)) }),
            MenuItemWrapper::CheckBox(obj) => Some(unsafe { &(*((obj as *const CheckBox) as *const T)) }),
            MenuItemWrapper::SingleChoice(obj) => Some(unsafe { &(*((obj as *const SingleChoice) as *const T)) }),
            MenuItemWrapper::Separator(obj) => Some(unsafe { &(*((obj as *const Separator) as *const T)) }),
            MenuItemWrapper::SubMenu(obj) => Some(unsafe { &(*((obj as *const SubMenu) as *const T)) }),
        }
    }

    /// Gets a mutable reference to a menu item by its handle.
    ///
    /// # Parameters
    /// * `menuitem_hamdle` - Handle to the menu item to retrieve.
    ///
    /// # Returns
    /// An `Option` containing a mutable reference to the menu item if found, or `None` if
    /// the item doesn't exist or the handle is invalid.
    #[allow(private_bounds)]
    pub fn get_mut<T>(&mut self, menuitem_hamdle: Handle<T>) -> Option<&mut T>
    where
        T: MenuItem,
    {
        let idx = menuitem_hamdle.index();
        if idx >= self.items.len() {
            return None;
        }
        let item = &mut self.items[idx];
        if item.get_handle() != menuitem_hamdle {
            return None;
        }
        match item {
            MenuItemWrapper::Command(obj) => Some(unsafe { &mut (*((obj as *mut Command) as *mut T)) }),
            MenuItemWrapper::CheckBox(obj) => Some(unsafe { &mut (*((obj as *mut CheckBox) as *mut T)) }),
            MenuItemWrapper::SingleChoice(obj) => Some(unsafe { &mut (*((obj as *mut SingleChoice) as *mut T)) }),
            MenuItemWrapper::Separator(obj) => Some(unsafe { &mut (*((obj as *mut Separator) as *mut T)) }),
            MenuItemWrapper::SubMenu(obj) => Some(unsafe { &mut (*((obj as *mut SubMenu) as *mut T)) }),
        }
    }

    pub(crate) fn is_on_menu(&self, x: i32, y: i32) -> bool {
        MousePositionInfo::new(x - self.clip.left, y - self.clip.top, self).is_on_menu
    }
    #[inline(always)]
    pub(crate) fn set_receiver_control_handle(&mut self, handle: Handle<()>) {
        self.receiver_control_handle = handle;
    }

    pub(crate) fn update_menuitems_menu_handle(&mut self) {
        for item in self.items.iter_mut() {
            item.update_menu_handle(self.handle);
        }
    }

    fn update_first_visible_item(&mut self) {
        if !self.current.in_range(self.items.len()) {
            return;
        }
        let cpoz = self.current.index() as u32;
        self.first_visible_item = self.first_visible_item.min(cpoz);
        if (cpoz - self.first_visible_item) > self.visible_items_count {
            self.first_visible_item = (cpoz + 1) - self.visible_items_count;
        }
    }

    fn move_currentitem_to(&mut self, key: Key) {
        let mut idx: [usize; MAX_ITEMS] = [0usize; MAX_ITEMS];
        let mut idx_count = 0usize;
        let items_count = self.items.len();
        for i in 0usize..items_count {
            let item = &self.items[i];
            if !item.is_enabled() {
                continue;
            }
            if item.can_be_selected() {
                idx[idx_count] = i;
                idx_count += 1;
            }
            if idx_count >= items_count {
                break;
            }
        }
        if idx_count == 0 {
            // no items or all items are disabled
            self.current = VectorIndex::Invalid;
            return;
        }
        // if CurrentItem is MenuItem::INVALID_INDEX ==> select the first available item
        if !self.current.in_range(items_count) {
            self.current.set(idx[0], self.items.len(), false);
        } else {
            // make sure that this->CurrentItem is part of the list
            let mut current_idx = VectorIndex::Invalid;
            let mut best_diff = usize::MAX;
            for (index, item) in idx.iter().enumerate().take(idx_count) {
                let val = *item;
                let diff = if val < self.current.index() {
                    self.current.index() - val
                } else {
                    val - self.current.index()
                };
                if diff < best_diff {
                    best_diff = diff;
                    current_idx = VectorIndex::with_value(index);
                }
            }
            // sanity check
            if !current_idx.in_range(idx_count) {
                // no item is selected
                self.current = VectorIndex::Invalid;
                return;
            }
            match key.code {
                KeyCode::Up => {
                    current_idx.sub(1, idx_count, Strategy::Rotate);
                }
                KeyCode::Down => {
                    current_idx.add(1, idx_count, Strategy::Rotate);
                }
                KeyCode::PageUp => {
                    current_idx.sub(self.visible_items_count as usize, idx_count, Strategy::Clamp);
                }
                KeyCode::PageDown => {
                    current_idx.add(self.visible_items_count as usize, idx_count, Strategy::Clamp);
                }
                KeyCode::Home => current_idx = VectorIndex::First,
                KeyCode::End => current_idx = VectorIndex::last(idx_count),
                _ => {}
            }
            self.current.set(idx[current_idx.index()], self.items.len(), false);
        }

        self.update_first_visible_item();
    }

    pub(super) fn process_shortcut(&mut self, key: Key, receiver_control_handle: Handle<()>) -> bool {
        for (index, item) in self.items.iter_mut().enumerate() {
            if !item.is_enabled() {
                continue;
            }
            if let Some(shortcut) = item.shortcut() {
                if shortcut == key {
                    self.run_item_action(index, receiver_control_handle);
                    return true;
                }
            }
            // recursively check other menus
            if let Some(submenu_handle) = item.get_submenu() {
                if let Some(menu) = RuntimeManager::get().get_menu(submenu_handle) {
                    if menu.process_shortcut(key, receiver_control_handle) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme, active: bool) {
        let col = if active { &theme.menu } else { &theme.parent_menu };
        surface.set_clip(self.clip.left, self.clip.top, self.clip.right, self.clip.bottom);
        surface.set_origin(self.clip.left, self.clip.top);
        surface.clear(Character::with_attributes(' ', col.text.normal));
        surface.draw_rect(
            Rect::new(0, 0, self.clip.right - self.clip.left, self.clip.bottom - self.clip.top),
            LineType::Single,
            col.text.normal,
        );
        // draw scroll buttons if case
        if (self.visible_items_count as usize) < self.items.len() {
            // top button
            let c = self.button_up.get_color(self.first_visible_item == 0, col);
            let x = (self.width >> 1) as i32;
            surface.fill_horizontal_line(x, 0, x + 2, Character::with_attributes(' ', c));
            surface.write_char(x + 1, 0, Character::with_attributes(SpecialChar::TriangleUp, c));

            // bottom button
            // this->FirstVisibleItem + this->VisibleItemsCount >= this->ItemsCount
            let c = self
                .button_down
                .get_color((self.first_visible_item + self.visible_items_count) as usize >= self.items.len(), col);
            let y = self.clip.bottom - self.clip.top;
            surface.fill_horizontal_line(x, y, x + 2, Character::with_attributes(' ', c));
            surface.write_char(x + 1, y, Character::with_attributes(SpecialChar::TriangleDown, c));
        }
        // write items
        // let mut format = TextFormat {
        //     width: Some(self.text_width),
        //     align: TextAlignament::Left,
        //     text_wrap: TextWrap::Character,
        //     multi_line: false,
        //     ..Default::default()
        // };
        let mut format = TextFormatBuilder::new()
            .wrap_type(WrapType::SingleLineWrap(self.text_width))
            .align(TextAlignament::Left)
            .build();

        let start = self.first_visible_item as usize;
        let end = self.items.len().min((self.first_visible_item + self.visible_items_count) as usize);
        if end <= start {
            return;
        }
        for idx in start..end {
            let item = &self.items[idx];
            format.y += 1;
            item.paint(surface, &mut format, self.width, idx == self.current.index(), col);
        }
    }
    pub(crate) fn update_children_with_parent_handle(&self) {
        let menus = RuntimeManager::get().get_menus();
        for item in self.items.iter() {
            if let Some(child_handle) = item.get_submenu() {
                if let Some(menu) = menus.get_mut(child_handle) {
                    menu.parent_handle = self.handle;
                }
            }
        }
    }
    pub(crate) fn on_mouse_released(&mut self, x: i32, y: i32) -> EventProcessStatus {
        let x = x - self.clip.left;
        let y = y - self.clip.top;
        let mpi = MousePositionInfo::new(x, y, self);
        if (self.visible_items_count as usize) < self.items.len() {
            if (mpi.is_on_up_button) && (self.first_visible_item > 0) {
                self.button_up = MenuButtonState::Hovered;
                return EventProcessStatus::Processed;
            }
            if (mpi.is_on_down_button) && ((self.visible_items_count + self.first_visible_item) as usize) < self.items.len() {
                self.button_down = MenuButtonState::Hovered;
                return EventProcessStatus::Processed;
            }
        }
        EventProcessStatus::Ignored
    }
    pub(crate) fn on_mouse_move(&mut self, x: i32, y: i32) -> MouseMoveMenuResult {
        let x = x - self.clip.left;
        let y = y - self.clip.top;
        let mpi = MousePositionInfo::new(x, y, self);
        let button_up_status = if mpi.is_on_up_button {
            MenuButtonState::Hovered
        } else {
            MenuButtonState::Normal
        };
        let button_down_status = if mpi.is_on_down_button {
            MenuButtonState::Hovered
        } else {
            MenuButtonState::Normal
        };
        let mut need_repaint = false;
        if button_up_status != self.button_up {
            self.button_up = button_up_status;
            need_repaint = true;
        }
        if button_down_status != self.button_down {
            self.button_down = button_down_status;
            need_repaint = true;
        }
        if self.current != mpi.item_index {
            self.current = mpi.item_index;
            need_repaint = true;
        }
        if need_repaint {
            if mpi.is_on_menu {
                MouseMoveMenuResult::ProcessedAndRepaint
            } else {
                MouseMoveMenuResult::RepaintAndPass
            }
        } else if mpi.is_on_menu {
            return MouseMoveMenuResult::ProcessWithoutRepaint;
        } else {
            return MouseMoveMenuResult::Ignored;
        }
    }
    pub(crate) fn on_mouse_wheel(&mut self, direction: MouseWheelDirection) -> EventProcessStatus {
        if (self.visible_items_count as usize) >= self.items.len() {
            // nothing to scroll
            return EventProcessStatus::Ignored;
        }
        if (direction == MouseWheelDirection::Up) && (self.first_visible_item > 0) {
            self.first_visible_item -= 1;
            return EventProcessStatus::Processed;
        }
        if (direction == MouseWheelDirection::Down) && (((self.visible_items_count + self.first_visible_item) as usize) < self.items.len()) {
            self.first_visible_item += 1;
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
    pub(crate) fn on_mouse_pressed(&mut self, x: i32, y: i32) -> MousePressedMenuResult {
        let x = x - self.clip.left;
        let y = y - self.clip.top;
        let mpi = MousePositionInfo::new(x, y, self);
        // check buttons
        if (self.visible_items_count as usize) < self.items.len() {
            if (mpi.is_on_up_button) && (self.first_visible_item > 0) {
                self.button_up = MenuButtonState::Pressed;
                self.on_mouse_wheel(MouseWheelDirection::Up);
                //return EventProcessStatus::Processed;
                return MousePressedMenuResult::Repaint;
            }
            if (mpi.is_on_down_button) && ((self.visible_items_count + self.first_visible_item) as usize) < self.items.len() {
                self.button_down = MenuButtonState::Pressed;
                self.on_mouse_wheel(MouseWheelDirection::Down);
                return MousePressedMenuResult::Repaint;
            }
        }
        // if click on a valid item, apply the action and close the menu
        if mpi.item_index.is_valid() {
            self.run_item_action(mpi.item_index.index(), self.receiver_control_handle);
            return MousePressedMenuResult::Repaint;
        }

        // is it's on the menu -> do nothing
        if mpi.is_on_menu {
            return MousePressedMenuResult::None;
        }
        // if it's outsize, check if mouse is on one of its parens
        MousePressedMenuResult::CheckParent
    }

    pub(super) fn select_single_choice(&mut self, index: usize) {
        // safety checks
        let count = self.items.len();
        if index >= count {
            return;
        }
        if !self.items[index].is_singlechoice() {
            return;
        }
        let mut idx = index;
        while (idx > 0) && (self.items[idx].is_singlechoice()) {
            self.items[idx].set_checked(false);
            idx -= 1;
        }
        if (idx == 0) && (self.items[0].is_singlechoice()) {
            self.items[0].set_checked(false);
        }
        idx = index;
        while (idx < count) && (self.items[idx].is_singlechoice()) {
            self.items[idx].set_checked(false);
            idx += 1;
        }
        self.items[index].set_checked(true);
    }
    fn send_event(&mut self, event: MenuEvent) {
        let rm = RuntimeManager::get();
        rm.close_opened_menu();
        rm.set_menu_event(event);
    }
    fn close(&mut self) {
        RuntimeManager::get().activate_opened_menu_parent();
    }
    fn run_item_action(&mut self, index: usize, receiver_control_handle: Handle<()>) {
        if index >= self.items.len() {
            return;
        }
        if !self.items[index].is_enabled() {
            return;
        }
        match &mut self.items[index] {
            MenuItemWrapper::Command(item) => {
                let evnt = MenuEvent::Command(MenuCommandEvent {
                    command_id: item.command_id,
                    menu: self.handle,
                    item: item.handle,
                    control_receiver_handle: receiver_control_handle,
                });
                self.send_event(evnt);
            }
            MenuItemWrapper::CheckBox(item) => {
                item.checked = !item.checked;
                let evnt = MenuEvent::CheckBoxStateChanged(MenuCheckBoxStateChangedEvent {
                    command_id: item.command_id,
                    menu: self.handle,
                    item: item.handle,
                    checked: item.checked,
                    control_receiver_handle: receiver_control_handle,
                });
                self.send_event(evnt);
            }
            MenuItemWrapper::SingleChoice(item) => {
                let evnt = MenuEvent::SingleChoiceSelected(MenuRadioBoxSelectedEvent {
                    command_id: item.command_id,
                    menu: self.handle,
                    item: item.handle,
                    control_receiver_handle: receiver_control_handle,
                });
                self.select_single_choice(index);
                self.send_event(evnt);
            }
            MenuItemWrapper::Separator(_) => {}
            MenuItemWrapper::SubMenu(item) => {
                RuntimeManager::get().show_menu(
                    item.submenu_handle,
                    receiver_control_handle,
                    (self.width as i32) + self.clip.left,
                    self.clip.top + 1 + ((index as u32 - self.first_visible_item) as i32),
                    None,
                );
            }
        };
    }

    pub(crate) fn on_key_pressed(&mut self, key: Key) -> EventProcessStatus {
        match key.value() {
            key!("Up") | key!("Down") | key!("Home") | key!("End") | key!("PageUp") | key!("PageDown") => {
                self.move_currentitem_to(key);
                return EventProcessStatus::Processed;
            }
            key!("Enter") | key!("Space") => {
                self.run_item_action(self.current.index(), self.receiver_control_handle);
                return EventProcessStatus::Processed;
            }
            key!("Escape") => {
                self.close();
                return EventProcessStatus::Processed;
            }
            key!("Left") => {
                if self.parent_handle.is_none() {
                    return EventProcessStatus::Ignored;
                }
                RuntimeManager::get().activate_opened_menu_parent();
                return EventProcessStatus::Processed;
            }
            key!("Right") => {
                if self.current.in_range(self.items.len()) {
                    let item = &self.items[self.current.index()];
                    if (item.is_enabled()) && (item.is_submenu()) {
                        self.run_item_action(self.current.index(), self.receiver_control_handle);
                        return EventProcessStatus::Processed;
                    }
                }
                return EventProcessStatus::Ignored;
            }
            _ => {}
        }
        // check short keys -> only if a key is being pressed without any modifier
        if (key.code != KeyCode::None) && (key.modifier == KeyModifier::None) {
            let count = self.items.len();
            let mut idx = 0usize;
            while idx < count {
                let item = &self.items[idx];
                if item.is_enabled() {
                    if let Some(hotkey) = item.hotkey() {
                        if hotkey == key {
                            self.current = VectorIndex::with_value(idx);
                            self.update_first_visible_item();
                            self.run_item_action(idx, self.receiver_control_handle);
                            return EventProcessStatus::Processed;
                        }
                    }
                }
                idx += 1;
            }
        }
        EventProcessStatus::Ignored
    }

    pub(crate) fn compute_position(&mut self, x: i32, y: i32, max_size: Size, term_size: Size) -> bool {
        if (term_size.width < 5) || (term_size.height < 5) {
            // can not display if terminal is less than 5 x 5
            return false;
        }
        // compute best width
        let mut max_width_left = 0usize;
        let mut max_hot_key_width = 0usize;
        for item in &self.items {
            let mut w_left = item.get_caption_chars_count() + 4;
            let mut w_right = 0usize;
            if item.is_checkable() {
                w_left += 2;
            }
            if let Some(shortcut) = item.shortcut() {
                w_right += shortcut.code.name().len();
                w_right += shortcut.modifier.name().len();
                if w_right > 0 {
                    w_right += 2;
                }
            }
            max_width_left = max_width_left.max(w_left);
            max_hot_key_width = max_hot_key_width.max(w_right);
        }
        let best_width = (max_width_left + max_hot_key_width) | 1; // make sure it's not an odd number (this will help better position Arrow Up and Down)

        // adjust X and Y to be on the screen
        let x = x.clamp(0, term_size.width as i32);
        let y = y.clamp(0, term_size.height as i32);

        // validate max and min limits for menu width and height
        let mut max_width_for_current_screen = 37u32.max(term_size.width / 4); // use a non-odd number (31 / 33 / 35 --> bigger them 30)
        let mut max_height_for_current_screen = 5u32.max(term_size.height - 4);
        if max_size.width >= 30 {
            max_width_for_current_screen = max_width_for_current_screen.min(max_size.width | 1);
        }
        if max_size.height >= 5 {
            max_height_for_current_screen = max_height_for_current_screen.min(max_size.height);
        }
        let menu_width = max_width_for_current_screen.min((best_width as u32) + 2);
        let mut menu_height = max_height_for_current_screen.min((self.items.len() as u32) + 2);

        // Set direction
        let to_left = {
            if x + (menu_width as i32) <= (term_size.width as i32) {
                true // best fit on left
            } else if x >= (menu_width as i32) {
                false // best fit on right
            } else {
                x < ((term_size.width / 2) as i32) // if x is closest to right edge - expand to left, otherwise to right
            }
        };

        let to_bottom = {
            if y + (menu_height as i32) <= (term_size.height as i32) {
                true // best fit on bottom
            } else if y >= (menu_height as i32) {
                false // best fit on top
            } else {
                let result = y < ((term_size.height / 2) as i32); // if y is closest to top edge - expand to top, otherwise to bottom
                if result {
                    menu_height = 5u32.max(((term_size.height as i32) - y) as u32);
                } else {
                    menu_height = 5u32.max(y as u32); // y - 0 = y
                }
                result
            }
        };

        self.visible_items_count = menu_height - 2;
        self.width = (menu_width - 2) as u16;
        self.text_width = self.width - ((max_hot_key_width + 2) as u16);
        // set the actual clip
        if to_left {
            if to_bottom {
                self.clip.set_with_size(x, y, menu_width as u16, menu_height as u16);
            } else {
                self.clip
                    .set_with_size(x, y + 1 - (menu_height as i32), menu_width as u16, menu_height as u16);
            }
        } else if to_bottom {
            self.clip
                .set_with_size(x + 1 - (menu_width as i32), y, menu_width as u16, menu_height as u16);
        } else {
            self.clip.set_with_size(
                x + 1 - (menu_width as i32),
                y + 1 - (menu_height as i32),
                menu_width as u16,
                menu_height as u16,
            );
        }

        // clear selection & buttons
        self.first_visible_item = 0;
        self.current = VectorIndex::Invalid;
        self.button_up = MenuButtonState::Normal;
        self.button_down = MenuButtonState::Normal;

        true
    }

    #[inline(always)]
    pub(crate) fn get_parent_handle(&self) -> Handle<Menu> {
        self.parent_handle
    }
}

impl HandleSupport<Menu> for Menu {
    fn handle(&self) -> Handle<Menu> {
        self.handle.cast()
    }

    fn set_handle(&mut self, handle: Handle<Menu>) {
        self.handle = handle;
        self.update_children_with_parent_handle();
    }
}
