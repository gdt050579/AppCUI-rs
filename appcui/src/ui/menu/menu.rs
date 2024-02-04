use std::sync::atomic::{AtomicUsize, Ordering};

use super::{
    events::*, menu_button_state::MenuButtonState, mouse_position_info::MousePositionInfo,
    CheckBox, Command, MenuItem, SingleChoice, SubMenu,
    MousePressedResult, menu_item::IntoMenuItem,
};
use crate::{
    graphics::{
        Character, ClipArea, LineType, Rect, Size, SpecialChar, Surface, TextAlignament,
        TextFormat, TextWrap,
    },
    input::{Key, KeyCode, MouseWheelDirection},
    system::{Handle, HandleSupport, RuntimeManager, Theme},
    ui::common::{traits::EventProcessStatus, UIElement},
    utils::{Caption, Strategy, VectorIndex},
};
const MAX_ITEMS: usize = 128;
static GLOBAL_MENUITEM_ID: AtomicUsize = AtomicUsize::new(0);
pub struct Menu {
    pub(super) caption: Caption,
    pub(super) items: Vec<MenuItem>,
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
    pub(super) receiver_control_handle: Handle<UIElement>,
}
impl Menu {
    pub fn new(name: &str) -> Self {
        Self {
            caption: if name.len() == 0 {
                Caption::default()
            } else {
                Caption::new(name, true)
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

    pub fn add<T>(&mut self, mut menuitem: T) -> Handle<T> where T: IntoMenuItem {
        let id = (GLOBAL_MENUITEM_ID.fetch_add(1, Ordering::SeqCst) as u32) % 0xFFFF_FFFE;
        let h: Handle<T> = Handle::with_id(id, self.items.len() as u32);
        menuitem.update_handles(self.handle, h.cast());
        self.items.push(menuitem.into_menuitem());
        h
    }

    pub(crate) fn is_on_menu(&self, x: i32, y: i32) -> bool {
        MousePositionInfo::new(x - self.clip.left, y - self.clip.top, &self).is_on_menu
    }
    #[inline(always)]
    pub(crate) fn set_receiver_control_handle(&mut self, handle: Handle<UIElement>) {
        self.receiver_control_handle = handle;
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
            for tr in 0..idx_count {
                let diff = if idx[tr] < self.current.index() {
                    self.current.index() - idx[tr]
                } else {
                    idx[tr] - self.current.index()
                };
                if diff < best_diff {
                    best_diff = diff;
                    current_idx = VectorIndex::with_value(tr);
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
                    current_idx.sub(
                        self.visible_items_count as usize,
                        idx_count,
                        Strategy::Clamp,
                    );
                }
                KeyCode::PageDown => {
                    current_idx.add(
                        self.visible_items_count as usize,
                        idx_count,
                        Strategy::Clamp,
                    );
                }
                KeyCode::Home => current_idx = VectorIndex::First,
                KeyCode::End => current_idx = VectorIndex::last(idx_count),
                _ => {}
            }
            self.current
                .set(idx[current_idx.index()] as usize, self.items.len(), false);
        }

        self.update_first_visible_item();
    }

    fn process_shortcut(&mut self, key: Key) -> bool {
        for (index, item) in self.items.iter_mut().enumerate() {
            if !item.is_enabled() {
                continue;
            }
            if let Some(shortcut) = item.get_shortcut() {
                if shortcut == key {
                    match item {
                        MenuItem::Command(item) => {
                            let evnt = MenuEvent::Command(MenuCommandEvent {
                                command_id: item.command_id,
                                menu: self.handle,
                                control_receiver_handle: self.receiver_control_handle,
                            });
                            self.send_event(evnt);
                            return true;
                        }
                        MenuItem::CheckBox(item) => {
                            item.checked = !item.checked;
                            let evnt =
                                MenuEvent::CheckBoxStateChanged(MenuCheckBoxStateChangedEvent {
                                    command_id: item.command_id,
                                    menu: self.handle,
                                    checked: item.checked,
                                    control_receiver_handle: self.receiver_control_handle,
                                });
                            self.send_event(evnt);
                            return true;
                        }
                        MenuItem::SingleChoice(item) => {
                            let evnt = MenuEvent::RadioBoxSelected(MenuRadioBoxSelectedEvent {
                                command_id: item.command_id,
                                menu: self.handle,
                                control_receiver_handle: self.receiver_control_handle,
                            });
                            self.check_radio_item(index);
                            self.send_event(evnt);
                            return true;
                        }
                        MenuItem::Separator(_) => {}
                        MenuItem::SubMenu(item) => {
                            if let Some(submenu) = RuntimeManager::get()
                                .get_menus()
                                .get_mut(item.submenu_handle)
                            {
                                if submenu.process_shortcut(key) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        return false;
    }

    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme, active: bool) {
        let col = if active {
            &theme.menu
        } else {
            &theme.parent_menu
        };
        surface.set_clip(
            self.clip.left,
            self.clip.top,
            self.clip.right,
            self.clip.bottom,
        );
        surface.set_origin(self.clip.left, self.clip.top);
        surface.clear(Character::with_attributes(' ', col.text.normal));
        surface.draw_rect(
            Rect::new(
                0,
                0,
                self.clip.right - self.clip.left,
                self.clip.bottom - self.clip.top,
            ),
            LineType::Single,
            col.text.normal,
        );
        // draw scroll buttons if case
        if (self.visible_items_count as usize) < self.items.len() {
            // top button
            let c = self.button_up.get_color(self.first_visible_item == 0, col);
            let x = (self.width >> 1) as i32;
            surface.fill_horizontal_line(x, 0, x + 2, Character::with_attributes(' ', c));
            surface.write_char(
                x + 1,
                0,
                Character::with_attributes(SpecialChar::TriangleUp, c),
            );

            // bottom button
            // this->FirstVisibleItem + this->VisibleItemsCount >= this->ItemsCount
            let c = self.button_down.get_color(
                (self.first_visible_item + self.visible_items_count) as usize > self.items.len(),
                col,
            );
            let y = self.clip.bottom - self.clip.top;
            surface.fill_horizontal_line(x, y, x + 2, Character::with_attributes(' ', c));
            surface.write_char(
                x + 1,
                y,
                Character::with_attributes(SpecialChar::TriangleDown, c),
            );
        }
        // write items
        let mut format = TextFormat::default();
        format.multi_line = false;
        format.align = TextAlignament::Left;
        format.text_wrap = TextWrap::None;
        format.width = Some(self.text_width);

        let start = self.first_visible_item as usize;
        let end = self
            .items
            .len()
            .min((self.first_visible_item + self.visible_items_count) as usize);
        if end <= start {
            return;
        }
        for idx in start..end {
            let item = &self.items[idx as usize];
            format.y += 1;
            item.paint(
                surface,
                &mut format,
                self.width,
                idx == self.current.index(),
                col,
            );
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
            if (mpi.is_on_down_button)
                && ((self.visible_items_count + self.first_visible_item) as usize)
                    < self.items.len()
            {
                self.button_down = MenuButtonState::Hovered;
                return EventProcessStatus::Processed;
            }
        }
        EventProcessStatus::Ignored
    }
    pub(crate) fn on_mouse_move(&mut self, x: i32, y: i32) -> EventProcessStatus {
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
            return EventProcessStatus::Processed;
            // if mpi.is_on_menu {
            //     return EventProcessStatus::Processed;
            // } else {
            //     return EventProcessStatus::Update;
            // }
        } else {
            return EventProcessStatus::Ignored
            // if mpi.is_on_menu {
            //     return EventProcessStatus::Cancel;
            // } else {
            //     return EventProcessStatus::Ignored;
            // }
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
        if (direction == MouseWheelDirection::Down)
            && (((self.visible_items_count + self.first_visible_item) as usize) < self.items.len())
        {
            self.first_visible_item += 1;
            return EventProcessStatus::Processed;
        }
        return EventProcessStatus::Ignored;
    }
    pub(crate) fn on_mouse_pressed(&mut self, x: i32, y: i32) -> MousePressedResult {
        let x = x - self.clip.left;
        let y = y - self.clip.top;
        let mpi = MousePositionInfo::new(x, y, self);
        // check buttons
        if (self.visible_items_count as usize) < self.items.len() {
            if (mpi.is_on_up_button) && (self.first_visible_item > 0) {
                self.button_up = MenuButtonState::Pressed;
                self.on_mouse_wheel(MouseWheelDirection::Up);
                //return EventProcessStatus::Processed;
                return MousePressedResult::Repaint;
            }
            if (mpi.is_on_down_button)
                && ((self.visible_items_count + self.first_visible_item) as usize)
                    < self.items.len()
            {
                self.button_down = MenuButtonState::Pressed;
                self.on_mouse_wheel(MouseWheelDirection::Down);
                return MousePressedResult::Repaint;
            }
        }
        // if click on a valid item, apply the action and close the menu
        if mpi.item_index.is_valid() {
            self.run_item_action(mpi.item_index.index());
            return MousePressedResult::Repaint;
        }

        // is it's on the menu -> do nothing
        if mpi.is_on_menu {
            return MousePressedResult::None;
        }
        // if it's outsize, check if mouse is on one of its parens
        return MousePressedResult::CheckParent;
    }

    fn check_radio_item(&mut self, index: usize) {
        // safety checks
        let count = self.items.len();
        if index >= count {
            return;
        }
        if !self.items[index].is_radiobox() {
            return;
        }
        let mut idx = index;
        while (idx > 0) && (self.items[idx].is_radiobox()) {
            self.items[idx].set_checked(false);
            idx -= 1;
        }
        if (idx == 0) && (self.items[0].is_radiobox()) {
            self.items[0].set_checked(false);
        }
        idx = index;
        while (idx < count) && (self.items[idx].is_radiobox()) {
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
    fn run_item_action(&mut self, index: usize) {
        if index >= self.items.len() {
            return;
        }
        if !self.items[index].is_enabled() {
            return;
        }
        match &mut self.items[index] {
            MenuItem::Command(item) => {
                let evnt = MenuEvent::Command(MenuCommandEvent {
                    command_id: item.command_id,
                    menu: self.handle,
                    control_receiver_handle: self.receiver_control_handle,
                });
                self.send_event(evnt);
            }
            MenuItem::CheckBox(item) => {
                item.checked = !item.checked;
                let evnt = MenuEvent::CheckBoxStateChanged(MenuCheckBoxStateChangedEvent {
                    command_id: item.command_id,
                    menu: self.handle,
                    checked: item.checked,
                    control_receiver_handle: self.receiver_control_handle,
                });
                self.send_event(evnt);
            }
            MenuItem::SingleChoice(item) => {
                let evnt = MenuEvent::RadioBoxSelected(MenuRadioBoxSelectedEvent {
                    command_id: item.command_id,
                    menu: self.handle,
                    control_receiver_handle: self.receiver_control_handle,
                });
                self.check_radio_item(index);
                self.send_event(evnt);
            }
            MenuItem::Separator(_) => {}
            MenuItem::SubMenu(item) => {
                RuntimeManager::get().show_menu(
                    item.submenu_handle,
                    self.receiver_control_handle,
                    (self.width as i32) + self.clip.left,
                    self.clip.top + 1 + ((index as u32 - self.first_visible_item) as i32),
                    Size::new(0, 0),
                );

                /*
                            itm->SubMenu->Show(
                      Width + ScreenClip.ScreenPosition.X, ScreenClip.ScreenPosition.Y + 1 + itemIndex - FirstVisibleItem);
                // transfer owner
                (reinterpret_cast<MenuContext*>(itm->SubMenu->Context))->Owner = this->Owner;
                    */
            }
        };
    }

    pub(crate) fn on_key_pressed(&mut self, key: Key) -> EventProcessStatus {
        match key.code {
            KeyCode::Up
            | KeyCode::Down
            | KeyCode::Home
            | KeyCode::End
            | KeyCode::PageUp
            | KeyCode::PageDown => {
                self.move_currentitem_to(key);
                return EventProcessStatus::Processed;
            }
            KeyCode::Enter | KeyCode::Space => {
                self.run_item_action(self.current.index());
                return EventProcessStatus::Processed;
            }
            KeyCode::Escape => {
                self.close();
                return EventProcessStatus::Processed;
            }
            KeyCode::Left => {
                if self.parent_handle.is_none() {
                    return EventProcessStatus::Ignored;
                }
                RuntimeManager::get().activate_opened_menu_parent();
                return EventProcessStatus::Processed;
            }
            KeyCode::Right => {
                if self.current.in_range(self.items.len()) {
                    let item = &self.items[self.current.index()];
                    if (item.is_enabled()) && (item.is_submenu()) {
                        self.run_item_action(self.current.index());
                        return EventProcessStatus::Processed;
                    }
                }
                return EventProcessStatus::Ignored;
            }
            _ => {}
        }
        // check short keys
        let count = self.items.len();
        let mut idx = 0usize;
        while idx < count {
            let item = &self.items[idx];
            if item.is_enabled() {
                if let Some(hotkey) = item.get_hotkey() {
                    if hotkey == key {
                        self.current = VectorIndex::with_value(idx);
                        self.update_first_visible_item();
                        self.run_item_action(idx);
                        return EventProcessStatus::Processed;
                    }
                }
            }
            idx += 1;
        }
        return EventProcessStatus::Ignored;
    }

    pub(crate) fn compute_position(
        &mut self,
        x: i32,
        y: i32,
        max_size: Size,
        term_size: Size,
    ) -> bool {
        if (term_size.width < 5) || (term_size.height < 5) {
            // can not display if terminal is less than 5 x 5
            return false;
        }
        /*
                void MenuContext::Show(
              Controls::Menu* me, Reference<Controls::Control> relativeControl, int x, int y, const Graphics::Size& maxSize)
        {
            // compute abosolute position
            while (relativeControl.IsValid())
            {
                x += relativeControl->GetX();
                y += relativeControl->GetY();
                // move to parent
                relativeControl = relativeControl->GetParent();
                // add parent margins
                if (relativeControl.IsValid())
                {
                    x += ((ControlContext*) relativeControl->Context)->Margins.Left;
                    y += ((ControlContext*) relativeControl->Context)->Margins.Top;
                }
            }
            */

        // compute best width
        let mut max_width_left = 0usize;
        let mut max_hot_key_width = 0usize;
        for item in &self.items {
            let mut w_left = item.get_caption_chars_count() + 4;
            let mut w_right = 0usize;
            if item.is_checkable() {
                w_left += 2;
            }
            if let Some(shortcut) = item.get_shortcut() {
                w_right += shortcut.code.get_name().len();
                w_right += shortcut.modifier.get_name().len();
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
                self.clip
                    .set_with_size(x, y, menu_width as u16, menu_height as u16);
            } else {
                self.clip.set_with_size(
                    x,
                    y + 1 - (menu_height as i32),
                    menu_width as u16,
                    menu_height as u16,
                );
            }
        } else {
            if to_bottom {
                self.clip.set_with_size(
                    x + 1 - (menu_width as i32),
                    y,
                    menu_width as u16,
                    menu_height as u16,
                );
            } else {
                self.clip.set_with_size(
                    x + 1 - (menu_width as i32),
                    y + 1 - (menu_height as i32),
                    menu_width as u16,
                    menu_height as u16,
                );
            }
        }

        // clear selection & buttons
        self.first_visible_item = 0;
        self.current = VectorIndex::Invalid;
        self.button_up = MenuButtonState::Normal;
        self.button_down = MenuButtonState::Normal;

        return true;
    }
    #[inline(always)]
    pub(crate) fn get_handle(&self) -> Handle<Menu> {
        self.handle
    }
    pub(crate) fn set_handle(&mut self, handle: Handle<Menu>) {
        self.handle = handle;
    }
    #[inline(always)]
    pub(crate) fn get_parent_handle(&self) -> Handle<Menu> {
        self.parent_handle
    }
}

impl HandleSupport<Menu> for Menu {
    fn get_handle(&self) -> Handle<Menu> {
        self.handle.cast()
    }

    fn set_handle(&mut self, handle: Handle<Menu>) {
        self.handle = handle;
        self.update_children_with_parent_handle();
    }
}

/*


MenuContext::MenuContext()
{
    this->Parent            = nullptr;
    this->Owner             = nullptr;
    this->Cfg               = Application::GetAppConfig();
    this->FirstVisibleItem  = 0;
    this->VisibleItemsCount = 0;
    this->CurrentItem       = NO_MENUITEM_SELECTED;
    this->Width             = 0;
    this->TextWidth         = 0;
    this->ItemsCount        = 0;
    this->ButtonUp          = MenuButtonState::Normal;
    this->ButtonDown        = MenuButtonState::Normal;
}


//=====================================================================================[Menu]====
Menu::Menu()
{
    this->Context = new MenuContext();
}

ItemHandle Menu::AddCommandItem(const ConstString& text, int CommandID, Input::Key shortcutKey)
{
    return CTX->AddItem(std::make_unique<MenuItem>(MenuItemType::Command, text, CommandID, false, shortcutKey));
}
ItemHandle Menu::AddCheckItem(const ConstString& text, int CommandID, bool checked, Input::Key shortcutKey)
{
    return CTX->AddItem(std::make_unique<MenuItem>(MenuItemType::Check, text, CommandID, checked, shortcutKey));
}
ItemHandle Menu::AddRadioItem(const ConstString& text, int CommandID, bool checked, Input::Key shortcutKey)
{
    return CTX->AddItem(std::make_unique<MenuItem>(MenuItemType::Radio, text, CommandID, checked, shortcutKey));
}
ItemHandle Menu::AddSeparator()
{
    return CTX->AddItem(std::make_unique<MenuItem>());
}
ItemHandle Menu::AddSubMenu(const ConstString& text)
{
    try
    {
        Menu* SubMenu                               = new Menu();
        ((MenuContext*) (SubMenu->Context))->Parent = this;
        return CTX->AddItem(std::make_unique<MenuItem>(text, SubMenu));
    }
    catch (...)
    {
        return InvalidItemHandle; // could not allocate
    }
}
bool Menu::SetEnable(ItemHandle menuItem, bool status)
{
    CHECK_VALID_ITEM(false);
    CTX->Items[(uint32) menuItem]->Enabled = status;
    return true;
}
bool Menu::SetChecked(ItemHandle menuItem, bool status)
{
    CHECK_VALID_ITEM(false);
    return CTX->SetChecked((uint32) menuItem, status);
}

Reference<Menu> Menu::GetSubMenu(ItemHandle menuItem)
{
    CHECK_VALID_ITEM(nullptr);
    return Reference<Menu>(CTX->Items[(uint32) menuItem]->SubMenu);
}

void Menu::Show(int x, int y, const Graphics::Size& maxSize)
{
    CTX->Show(this, nullptr, x, y, maxSize);
}
void Menu::Show(Reference<Control> parent, int relativeX, int relativeY, const Graphics::Size& maxSize)
{
    CTX->Show(this, parent, relativeX, relativeY, maxSize);
}

bool Menu::ProcessShortcutKey(Input::Key keyCode)
{
    return CTX->ProcessShortCut(keyCode);
}
} // namespace AppCUI

#undef CTX
*/
