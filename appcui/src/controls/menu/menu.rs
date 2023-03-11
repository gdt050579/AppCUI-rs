use super::{
    menu_button_state::MenuButtonState, menu_item_type::MenuItemType,
    mouse_position_info::MousePositionInfo, MenuItem,
};
use crate::{
    graphics::{
        Character, ClipArea, LineType, Rect, SpecialChar, Surface, TextAlignament, TextFormat,
        TextWrap,
    },
    input::{Key, KeyCode},
    system::Theme,
};
const MAX_ITEMS: usize = 128;
pub struct Menu {
    pub(super) items: Vec<MenuItem>,
    pub(super) current: u32,
    pub(super) width: u16,
    pub(super) text_with: u16,
    pub(super) first_visible_item: u32,
    pub(super) visible_items_count: u32,
    pub(super) button_up: MenuButtonState,
    pub(super) button_down: MenuButtonState,
    pub(super) clip: ClipArea,
}
impl Menu {
    pub fn add(&mut self, item: MenuItem) {
        self.items.push(item);
    }
    fn is_on_menu(&self, x: i32, y: i32) -> bool {
        MousePositionInfo::new(x, y, &self).is_on_menu
    }
    fn update_first_visible_item(&mut self) {
        if self.current as usize >= self.items.len() {
            return;
        }
        self.first_visible_item = self.first_visible_item.min(self.current);
        if (self.current - self.first_visible_item) > self.visible_items_count {
            self.first_visible_item = (self.current + 1) - self.visible_items_count;
        }
    }

    fn move_currentitem_to(&mut self, key: Key) {
        let mut idx: [usize; MAX_ITEMS] = [0usize; MAX_ITEMS];
        let mut idx_count = 0usize;
        let items_count = self.items.len();
        for i in 0usize..items_count {
            let item = &self.items[i];
            if !item.enabled {
                continue;
            }
            match item.item_type {
                MenuItemType::Check
                | MenuItemType::Command
                | MenuItemType::Radio
                | MenuItemType::SubMenu => {
                    idx[idx_count] = i;
                    idx_count += 1;
                }
                _ => {}
            }
            if idx_count >= items_count {
                break;
            }
        }
        if idx_count == 0 {
            // no items or all items are disabled
            self.current = MenuItem::INVALID_INDEX;
            return;
        }
        // if CurrentItem is MenuItem::INVALID_INDEX ==> select the first available item
        if self.current as usize >= items_count {
            self.current = idx[0] as u32;
        } else {
            // make sure that this->CurrentItem is part of the list
            let mut current_idx = MenuItem::INVALID_INDEX;
            let mut best_diff = 0xFFFFFFFFu32;
            for tr in 0..idx_count {
                {
                    let diff = if idx[tr] < self.current as usize {
                        self.current - (idx[tr] as u32)
                    } else {
                        (idx[tr] as u32) - self.current
                    };
                    if diff < best_diff {
                        best_diff = diff;
                        current_idx = tr as u32;
                    }
                }
            }
            // sanity check
            if current_idx as usize >= idx_count {
                // no item is selected
                self.current = MenuItem::INVALID_INDEX;
                return;
            }
            match key.code {
                KeyCode::Up => {
                    if current_idx > 0 {
                        current_idx -= 1;
                    } else {
                        current_idx = (idx_count as u32) - 1;
                    }
                }
                KeyCode::Down => {
                    current_idx += 1;
                    if current_idx >= idx_count as u32 {
                        current_idx = 0;
                    }
                }
                KeyCode::PageUp => {
                    if current_idx >= self.visible_items_count {
                        current_idx -= self.visible_items_count;
                    } else {
                        current_idx = 0;
                    }
                }
                KeyCode::PageDown => {
                    current_idx = self.visible_items_count;
                    if current_idx >= idx_count as u32 {
                        current_idx = (idx_count as u32) - 1;
                    }
                }
                KeyCode::Home => current_idx = 0,
                KeyCode::End => current_idx = (idx_count as u32) - 1,
                _ => {}
            }
            self.current = idx[current_idx as usize] as u32;
        }
        
        self.update_first_visible_item();
    }

    fn paint(&self, surface: &mut Surface, theme: &Theme, active: bool) {
        let col = if active {
            &theme.menu
        } else {
            &theme.parent_menu
        };
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
            let c = self.button_up.get_color(
                (self.first_visible_item + self.visible_items_count) as usize > self.items.len(),
                col,
            );
            let y = self.clip.bottom - self.clip.top;
            surface.fill_horizontal_line(x, y, x + 2, Character::with_attributes(' ', c));
            surface.write_char(
                x + 1,
                y,
                Character::with_attributes(SpecialChar::TriangleUp, c),
            );
        }
        // write items
        let mut format = TextFormat::default();
        format.multi_line = false;
        format.align = TextAlignament::Left;
        format.text_wrap = TextWrap::None;
        format.width = Some(self.text_with);

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
            item.paint(
                surface,
                &mut format,
                self.width,
                idx == self.current as usize,
                col,
            );
        }
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
ItemHandle MenuContext::AddItem(unique_ptr<MenuItem> itm)
{
    if (itm->Type == MenuItemType::Invalid)
        return InvalidItemHandle;
    CHECK(this->ItemsCount < MAX_NUMBER_OF_MENU_ITEMS,
          InvalidItemHandle,
          "A maximum of 256 items can be added to a Menu");

    auto res                = ItemHandle{ (uint32) this->ItemsCount };
    Items[this->ItemsCount] = std::move(itm);
    this->ItemsCount++;
    return res;
}
void MenuContext::Paint(Graphics::Renderer& renderer, bool activ)
{
}
bool MenuContext::SetChecked(uint32 menuIndex, bool status)
{
    CHECK(menuIndex < ItemsCount,
          false,
          "Invalid menu index (%u) , should be between 0 and less than %u",
          menuIndex,
          ItemsCount);
    auto i = this->Items[menuIndex].get();
    CHECK((i->Type == MenuItemType::Check) || (i->Type == MenuItemType::Radio),
          false,
          "Only Check and Radio item can change their state");
    if (i->Type == MenuItemType::Radio)
    {
        // radio menu item -> uncheck all items that are radioboxes
        uint32 index = menuIndex;
        while (((index < this->ItemsCount)) && (this->Items[index]->Type == MenuItemType::Radio))
        {
            this->Items[index]->Checked = false;
            index--;
        }
        index = menuIndex + 1;
        while ((index < this->ItemsCount) && (this->Items[index]->Type == MenuItemType::Radio))
        {
            this->Items[index]->Checked = false;
            index++;
        }
    }
    i->Checked = status;
    return true;
}
void MenuContext::ComputeMousePositionInfo(int x, int y, MenuMousePositionInfo& mpi)
{
    // done
}
bool MenuContext::OnMouseMove(int x, int y, bool& repaint)
{
    MenuMousePositionInfo mpi;
    ComputeMousePositionInfo(x, y, mpi);
    auto buttonUpStatus   = mpi.IsOnUpButton ? MenuButtonState::Hovered : MenuButtonState::Normal;
    auto buttonDownStatus = mpi.IsOnDownButton ? MenuButtonState::Hovered : MenuButtonState::Normal;
    auto processed        = mpi.IsOnMenu;
    if (buttonUpStatus != this->ButtonUp)
    {
        this->ButtonUp = buttonUpStatus;
        repaint        = true;
    }
    if (buttonDownStatus != this->ButtonDown)
    {
        this->ButtonDown = buttonDownStatus;
        repaint          = true;
    }
    if (CurrentItem != mpi.ItemIndex)
    {
        CurrentItem = mpi.ItemIndex;
        repaint     = true;
    }
    return processed;
}
MousePressedResult MenuContext::OnMousePressed(int x, int y)
{
    MenuMousePositionInfo mpi;
    ComputeMousePositionInfo(x, y, mpi);
    // check buttons
    if (this->VisibleItemsCount < this->ItemsCount)
    {
        if ((mpi.IsOnUpButton) && (this->FirstVisibleItem > 0))
        {
            this->ButtonUp = MenuButtonState::Pressed;
            OnMouseWheel(x, y, MouseWheel::Up);
            return MousePressedResult::Repaint;
        }
        if ((mpi.IsOnDownButton) && (this->FirstVisibleItem + this->VisibleItemsCount < this->ItemsCount))
        {
            this->ButtonDown = MenuButtonState::Pressed;
            OnMouseWheel(x, y, MouseWheel::Down);
            return MousePressedResult::Repaint;
        }
    }
    // if click on a valid item, apply the action and close the menu
    if (mpi.ItemIndex != NO_MENUITEM_SELECTED)
    {
        RunItemAction(mpi.ItemIndex);
        // other type of items
        return MousePressedResult::Repaint;
    }
    // is it's on the menu -> do nothing
    if (mpi.IsOnMenu)
        return MousePressedResult::None;
    // if it's outsize, check if mouse is on one of its parens
    return MousePressedResult::CheckParent;
}
bool MenuContext::OnMouseReleased(int x, int y)
{
    MenuMousePositionInfo mpi;
    ComputeMousePositionInfo(x, y, mpi);
    // check buttons
    if (this->VisibleItemsCount < this->ItemsCount)
    {
        if ((mpi.IsOnUpButton) && (this->FirstVisibleItem > 0))
        {
            this->ButtonUp = MenuButtonState::Hovered;
            return true;
        }
        if ((mpi.IsOnDownButton) && (this->FirstVisibleItem + this->VisibleItemsCount < this->ItemsCount))
        {
            this->ButtonDown = MenuButtonState::Hovered;
            return true;
        }
    }
    return false;
}

bool MenuContext::OnMouseWheel(int, int, Input::MouseWheel direction)
{
    if (this->VisibleItemsCount >= this->ItemsCount)
        return false; // nothing to scroll
    if ((direction == MouseWheel::Up) && (this->FirstVisibleItem > 0))
    {
        this->FirstVisibleItem--;
        return true;
    }
    if ((direction == MouseWheel::Down) && ((this->FirstVisibleItem + this->VisibleItemsCount) < this->ItemsCount))
    {
        this->FirstVisibleItem++;
        return true;
    }
    return false;
}
void MenuContext::CreateAvailableItemsList(uint32* indexes, uint32& count)
{
    // DONE
}
void MenuContext::RunItemAction(uint32 itemIndex)
{
    if (itemIndex >= this->ItemsCount)
        return;
    auto itm      = this->Items[itemIndex].get();
    int commandID = -1;
    switch (itm->Type)
    {
    case MenuItemType::Check:
        this->SetChecked(itemIndex, !itm->Checked);
        commandID = itm->CommandID;
        break;
    case MenuItemType::Radio:
        this->SetChecked(itemIndex, true);
        commandID = itm->CommandID;
        break;
    case MenuItemType::SubMenu:
        itm->SubMenu->Show(
              Width + ScreenClip.ScreenPosition.X, ScreenClip.ScreenPosition.Y + 1 + itemIndex - FirstVisibleItem);
        // transfer owner
        (reinterpret_cast<MenuContext*>(itm->SubMenu->Context))->Owner = this->Owner;
        break;
    case MenuItemType::Command:
        commandID = itm->CommandID;
        break;
    }
    if (commandID >= 0)
    {
        Application::GetApplication()->CloseContextualMenu();
        Application::GetApplication()->SendCommand(commandID);
    }
}
void MenuContext::CloseMenu()
{
    if (this->Parent)
        Application::GetApplication()->ShowContextualMenu(this->Parent);
    else
        Application::GetApplication()->CloseContextualMenu();
}
void MenuContext::UpdateFirstVisibleItem()
{
    // Done
}
void MenuContext::MoveCurrentItemTo(Input::Key keyCode)
{
}
// key events
bool MenuContext::OnKeyEvent(Input::Key keyCode)
{
    // check movement keys
    switch (keyCode)
    {
    case Key::Up:
    case Key::Down:
    case Key::Home:
    case Key::End:
    case Key::PageUp:
    case Key::PageDown:
        MoveCurrentItemTo(keyCode);
        return true;
    case Key::Enter:
    case Key::Space:
        RunItemAction(this->CurrentItem);
        return true;
    case Key::Escape:
        CloseMenu();
        return true;
    case Key::Right:
        if ((this->CurrentItem < ItemsCount) && (Items[this->CurrentItem]->Enabled) &&
            (Items[this->CurrentItem]->Type == MenuItemType::SubMenu))
        {
            RunItemAction(this->CurrentItem);
            return true;
        }
        return false;
    case Key::Left:
        if (this->Parent)
        {
            CloseMenu();
            return true;
        }
        return false;
    }
    // check short keys
    for (uint32 tr = 0; tr < ItemsCount; tr++)
    {
        if ((Items[tr]->HotKey != Key::None) && (Items[tr]->HotKey == keyCode) && (Items[tr]->Enabled))
        {
            this->CurrentItem = tr;
            UpdateFirstVisibleItem();
            RunItemAction(tr);
            return true;
        }
    }
    // no binding
    return false;
}
bool MenuContext::ProcessShortCut(Input::Key keyCode)
{
    for (uint32 tr = 0; tr < this->ItemsCount; tr++)
    {
        if (!Items[tr]->Enabled)
            continue;
        if ((Items[tr]->Type == MenuItemType::Command) || (Items[tr]->Type == MenuItemType::Check) ||
            (Items[tr]->Type == MenuItemType::Radio))
        {
            if (Items[tr]->ShortcutKey == keyCode)
            {
                if (Items[tr]->Type == MenuItemType::Check)
                    this->SetChecked(tr, !Items[tr]->Checked);
                if (Items[tr]->Type == MenuItemType::Radio)
                    this->SetChecked(tr, true);
                if (Items[tr]->CommandID >= 0)
                {
                    Application::GetApplication()->SendCommand(Items[tr]->CommandID);
                }
                return true; // key was processed
            }
        }
        if ((Items[tr]->Type == MenuItemType::SubMenu) && (Items[tr]->SubMenu))
        {
            MenuContext* ctx = reinterpret_cast<MenuContext*>(Items[tr]->SubMenu->Context);
            if (ctx->ProcessShortCut(keyCode))
                return true;
        }
    }
    // if nothing matched - return false;
    return false;
}

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
    // compute best width
    uint32 maxWidthLeft   = 0;
    uint32 maxHotKeyWidth = 0;
    for (uint32 tr = 0; tr < this->ItemsCount; tr++)
    {
        auto i         = this->Items[tr].get();
        uint32 w_left  = i->Name.Len() + 4;
        uint32 w_right = 0;
        if ((i->Type == MenuItemType::Radio) || (i->Type == MenuItemType::Check))
            w_left += 2;
        if (i->ShortcutKey != Key::None)
        {
            w_right += (uint32) KeyUtils::GetKeyName(i->ShortcutKey).size();
            w_right += (uint32) KeyUtils::GetKeyModifierName(i->ShortcutKey).size();
            if (w_right > 0)
                w_right += 2;
        }
        maxWidthLeft   = std::max<>(maxWidthLeft, w_left);
        maxHotKeyWidth = std::max<>(maxHotKeyWidth, w_right);
    }
    uint32 BestWidth = maxWidthLeft + maxHotKeyWidth;
    BestWidth = BestWidth | 1; // make sure it's not an odd number (this will help better position Arrow Up and Down)
    // Check agains app size
    Size appSize;
    if (!Application::GetApplicationSize(appSize))
    {
        LOG_WARNING("Unable to retrieve application size --> contextual menu will not be display !");
        return;
    }
    if ((appSize.Height < 5) || (appSize.Width < 10))
    {
        LOG_WARNING(
              "Current application size %d x %d is too small to display a contextual menu (a size of at least 10 x 5 "
              "is required)",
              appSize.Width,
              appSize.Height);
        return;
    }
    // adjust X and Y to be on the screen
    x = std::max<>(x, 0);
    y = std::max<>(y, 0);
    x = std::min<>(x, (int) appSize.Width);
    y = std::min<>(y, (int) appSize.Height);

    // validate max and min limits for menu width and height
    auto maxWidthForCurrentScreen =
          std::max<>((appSize.Width / 4), 37U); // use a non-odd number (31 / 33 / 35 --> bigger them 30)
    auto maxHeightForCurrentScreen = std::max<>((appSize.Height - 4), 5U);
    if (maxSize.Width >= 30)
        maxWidthForCurrentScreen = std::min<>(maxWidthForCurrentScreen, (maxSize.Width | 1));
    if (maxSize.Height >= 5)
        maxHeightForCurrentScreen = std::min<>(maxHeightForCurrentScreen, maxSize.Height);
    uint32 menuWidth  = std::min<>(BestWidth + 2, maxWidthForCurrentScreen);
    uint32 menuHeight = std::min<>(this->ItemsCount + 2, maxHeightForCurrentScreen);

    // Set direction
    bool toLeft, toBottom;
    if (x + menuWidth <= appSize.Width)
        toLeft = true; // best fit on left
    else if (x >= (int) menuWidth)
        toLeft = false; // best fit on right
    else
        toLeft = x < (int) (appSize.Width / 2); // if x is closest to right edge - expand to left, otherwise to right

    if (y + menuHeight <= appSize.Height)
        toBottom = true; // best fit on bottom
    else if (y >= (int) menuHeight)
        toBottom = false; // best fit on top
    else
    {
        toBottom = y < (int) (appSize.Height / 2); // if y is closest to top edge - expand to top, otherwise to bottom
        if (toBottom)
            menuHeight = std::max<>(appSize.Height - y, 5U);
        else
            menuHeight = std::max<>(y, 5); // y - 0 = y
    }

    VisibleItemsCount = menuHeight - 2;
    Width             = menuWidth - 2;
    TextWidth         = Width - (maxHotKeyWidth + 2);
    // set the actual clip
    if (toLeft)
    {
        if (toBottom)
            this->ScreenClip.Set(x, y, menuWidth, menuHeight);
        else
            this->ScreenClip.Set(x, y + 1 - (int) menuHeight, menuWidth, menuHeight);
    }
    else
    {
        if (toBottom)
            this->ScreenClip.Set(x + 1 - (int) menuWidth, y, menuWidth, menuHeight);
        else
            this->ScreenClip.Set(x + 1 - (int) menuWidth, y + 1 - (int) menuHeight, menuWidth, menuHeight);
    }
    // clear selection & buttons
    this->FirstVisibleItem = 0;
    this->CurrentItem      = NO_MENUITEM_SELECTED;
    this->ButtonUp         = MenuButtonState::Normal;
    this->ButtonDown       = MenuButtonState::Normal;
    // link to application
    auto* app = Application::GetApplication();
    if (app)
        app->ShowContextualMenu(me);
}

//=====================================================================================[Menu]====
Menu::Menu()
{
    this->Context = new MenuContext();
}

Menu::~Menu()
{
    if (this->Context)
        delete ((MenuContext*) Context);
    this->Context = nullptr;
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
