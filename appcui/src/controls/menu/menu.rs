use crate::graphics::ClipArea;
use super::{MenuItem, menu_button_state::MenuButtonState, mouse_position_info::MousePositionInfo};

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
    fn is_on_menu(&self, x: i32, y: i32)->bool {
        MousePositionInfo::new(x,y,&self).is_on_menu
    }
}
/*


#include "ControlContext.hpp"

namespace AppCUI
{
using namespace Graphics;
using namespace Controls;
using namespace Input;

MenuItem::MenuItem()
{
    Type         = MenuItemType::Line;
    Enabled      = true;
    Checked      = true;
    SubMenu      = nullptr;
    CommandID    = -1;
    HotKey       = Input::Key::None;
    ShortcutKey  = Input::Key::None;
    HotKeyOffset = CharacterBuffer::INVALID_HOTKEY_OFFSET;
}
MenuItem::MenuItem(MenuItemType type, const ConstString& text, int cmdID, bool checked, Input::Key shortcutKey)
{
    Type = MenuItemType::Invalid;
    if (Name.SetWithHotKey(text, HotKeyOffset, HotKey))
    {
        Type        = type;
        Enabled     = true;
        Checked     = checked;
        SubMenu     = nullptr;
        CommandID   = cmdID;
        ShortcutKey = shortcutKey;
    }
}
MenuItem::MenuItem(const ConstString& text, Menu* subMenu)
{
    Type = MenuItemType::Invalid;
    if (Name.SetWithHotKey(text, HotKeyOffset, HotKey))
    {
        Type        = MenuItemType::SubMenu;
        Enabled     = true;
        Checked     = false;
        ShortcutKey = Input::Key::None;
        SubMenu     = subMenu;
    }
}

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
    const auto* col = activ ? (&Cfg->Menu) : (&Cfg->ParentMenu);

    WriteTextParams textParams(
          WriteTextFlags::SingleLine | WriteTextFlags::OverwriteColors | WriteTextFlags::HighlightHotKey |
                WriteTextFlags::ClipToWidth | WriteTextFlags::FitTextToWidth,
          TextAlignament::Left);
    textParams.Width = this->TextWidth;

    renderer.Clear(' ', col->Text.Normal);
    renderer.DrawRectSize(
          0, 0, ScreenClip.ClipRect.Width, ScreenClip.ClipRect.Height, col->Text.Normal, LineType::Single);
    // draw scroll buttons if case
    if (this->VisibleItemsCount < this->ItemsCount)
    {
        ColorPair c;
        // top button
        if (this->FirstVisibleItem == 0)
            c = col->Text.Inactive;
        else
        {
            switch (this->ButtonUp)
            {
            case MenuButtonState::Normal:
                c = col->Text.Normal;
                break;
            case MenuButtonState::Hovered:
                c = col->Text.Hovered;
                break;
            case MenuButtonState::Pressed:
                c = col->Text.PressedOrSelected;
                break;
            }
        }

        renderer.FillHorizontalLineSize(this->Width / 2, 0, 3, ' ', c);
        renderer.WriteSpecialCharacter(1 + this->Width / 2, 0, SpecialChars::TriangleUp, c);

        // bottom button
        if (this->FirstVisibleItem + this->VisibleItemsCount >= this->ItemsCount)
            c = col->Text.Inactive;
        else
        {
            switch (this->ButtonDown)
            {
            case MenuButtonState::Normal:
                c = col->Text.Normal;
                break;
            case MenuButtonState::Hovered:
                c = col->Text.Hovered;
                break;
            case MenuButtonState::Pressed:
                c = col->Text.PressedOrSelected;
                break;
            }
        }

        renderer.FillHorizontalLineSize(this->Width / 2, ScreenClip.ClipRect.Height - 1, 3, ' ', c);
        renderer.WriteSpecialCharacter(
              1 + this->Width / 2, ScreenClip.ClipRect.Height - 1, SpecialChars::TriangleDown, c);
    }

    // draw items
    for (uint32 tr = 1; tr <= this->VisibleItemsCount; tr++)
    {
        uint32 actualIndex = this->FirstVisibleItem + tr - 1;
        if (actualIndex >= ItemsCount)
            break;
        MenuItem* item = this->Items[actualIndex].get();
        ColorPair shortCutCol, checkColor;
        if (item->Enabled == false)
        {
            textParams.Color       = col->Text.Inactive;
            textParams.HotKeyColor = col->HotKey.Inactive;
            shortCutCol            = col->ShortCut.Inactive;
            checkColor             = col->Symbol.Inactive;
        }
        else
        {
            if (actualIndex == this->CurrentItem)
            {
                textParams.Color       = col->Text.Hovered;
                textParams.HotKeyColor = col->HotKey.Hovered;
                shortCutCol            = col->ShortCut.Hovered;
                checkColor             = col->Symbol.Hovered;
                renderer.FillHorizontalLine(1, tr, Width, ' ', col->Text.Hovered);
            }
            else
            {
                textParams.Color       = col->Text.Normal;
                textParams.HotKeyColor = col->HotKey.Normal;
                shortCutCol            = col->ShortCut.Normal;
                checkColor             = col->Symbol.Normal;
            }
        }

        textParams.HotKeyPosition = item->HotKeyOffset;
        textParams.Y              = tr;

        switch (item->Type)
        {
        case MenuItemType::Line:
            renderer.DrawHorizontalLine(1, tr, this->Width, col->Text.Normal);
            break;
        case MenuItemType::Command:
            textParams.X = 2;
            renderer.WriteText(item->Name, textParams);
            break;
        case MenuItemType::Check:
            textParams.X = 4;
            renderer.WriteText(item->Name, textParams);
            if (item->Checked)
                renderer.WriteSpecialCharacter(2, tr, SpecialChars::CheckMark, checkColor);
            break;
        case MenuItemType::Radio:
            textParams.X = 4;
            renderer.WriteText(item->Name, textParams);
            if (item->Checked)
                renderer.WriteSpecialCharacter(2, tr, SpecialChars::CircleFilled, checkColor);
            else
                renderer.WriteSpecialCharacter(2, tr, SpecialChars::CircleEmpty, checkColor);
            break;
        case MenuItemType::SubMenu:
            textParams.X = 2;
            renderer.WriteText(item->Name, textParams);
            renderer.WriteSpecialCharacter(this->Width - 1, tr, SpecialChars::TriangleRight, textParams.Color);
            break;
        }
        if (item->ShortcutKey != Key::None)
        {
            auto k_n = KeyUtils::GetKeyName(item->ShortcutKey);
            auto m_n = KeyUtils::GetKeyModifierName(item->ShortcutKey);
            renderer.WriteSingleLineText(this->Width - (uint32) k_n.size(), tr, k_n, shortCutCol);
            renderer.WriteSingleLineText(this->Width - (uint32) (k_n.size() + m_n.size()), tr, m_n, shortCutCol);
        }
    }
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
    // assume indexes is valid and has a size of MAX_NUMBER_OF_MENU_ITEMS
    count = 0;
    for (uint32 tr = 0; tr < ItemsCount; tr++)
    {
        if (!Items[tr]->Enabled)
            continue;
        if ((Items[tr]->Type == MenuItemType::Command) || (Items[tr]->Type == MenuItemType::Check) ||
            (Items[tr]->Type == MenuItemType::Radio) || (Items[tr]->Type == MenuItemType::SubMenu))
        {
            indexes[count++] = tr;
        }
    }
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
    // if no current item -> exit
    if (this->CurrentItem >= this->ItemsCount)
        return;
    if (this->CurrentItem < this->FirstVisibleItem)
        this->FirstVisibleItem = this->CurrentItem;
    if ((this->CurrentItem - this->FirstVisibleItem) >= this->VisibleItemsCount)
        this->FirstVisibleItem = (this->CurrentItem - this->VisibleItemsCount) + 1;
}
void MenuContext::MoveCurrentItemTo(Input::Key keyCode)
{
    uint32 idx[MAX_NUMBER_OF_MENU_ITEMS];
    uint32 idxCount;
    CreateAvailableItemsList(idx, idxCount);
    if (idxCount == 0)
    {
        // no items or all items are disabled
        this->CurrentItem = NO_MENUITEM_SELECTED;
        return;
    }
    // if CurrentItem is NO_MENUITEM_SELECTED ==> select the first available item
    if (this->CurrentItem >= this->ItemsCount)
    {
        this->CurrentItem = idx[0];
    }
    else
    {
        // make sure that this->CurrentItem is part of the list
        uint32 currentIdx = 0xFFFFFFFF;
        uint32 bestDiff   = 0xFFFFFFFF;
        for (uint32 tr = 0; tr < idxCount; tr++)
        {
            uint32 diff;
            if (idx[tr] < this->CurrentItem)
                diff = this->CurrentItem - idx[tr];
            else
                diff = idx[tr] - this->CurrentItem;
            if (diff < bestDiff)
            {
                bestDiff   = diff;
                currentIdx = tr;
            }
        }
        // sanity check
        if (currentIdx >= idxCount)
        {
            // no item is selected
            this->CurrentItem = NO_MENUITEM_SELECTED;
            return;
        }
        // compute the new position
        switch (keyCode)
        {
        case Key::Up:
            if (currentIdx > 0)
                currentIdx--;
            else
                currentIdx = idxCount - 1;
            break;
        case Key::Down:
            currentIdx++;
            if (currentIdx >= idxCount)
                currentIdx = 0;
            break;
        case Key::PageUp:
            if (currentIdx >= this->VisibleItemsCount)
                currentIdx -= this->VisibleItemsCount;
            else
                currentIdx = 0;
            break;
        case Key::PageDown:
            currentIdx += this->VisibleItemsCount;
            if (currentIdx >= idxCount)
                currentIdx = idxCount - 1;
            break;
        case Key::Home:
            currentIdx = 0;
            break;
        case Key::End:
            currentIdx = idxCount - 1;
            break;
        }
        this->CurrentItem = idx[currentIdx];
    }
    UpdateFirstVisibleItem();
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