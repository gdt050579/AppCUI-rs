use crate::{utils::Caption, controls::events::{EventProcessStatus, Event}, input::Key};

use super::{MenuBarItem, Menu};

pub(super) struct MenuBar {
    items: Vec<MenuBarItem>,
    x: i32,
    y: i32,
    width: u32,
}

impl MenuBar {
    fn update_positions(&mut self) {
        let mut x = 0;
        for item in &mut self.items {
            item.x = x;
            x += 2 + (item.caption.get_chars_count() as i32);
        }
    }
    fn mouse_position_to_index(&self, x: i32, y: i32) -> Option<usize> {
        let mut x = x - self.x;
        let mut y = y - self.y;

        if y != 0 {
            return None;
        }
        for (index, item) in self.items.iter().enumerate() {
            if (x >= item.x) && (x < (item.x + 2 + (item.caption.get_chars_count() as i32))) {
                return Some(index);
            }
        }
        return None;
    }
    fn set_position(&mut self, x: i32, y: i32, width: u32) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.update_positions();
    }
    fn add(&mut self, menu: Menu, caption: Caption) {
        self.items.push(MenuBarItem { caption, menu, x: 0 });
        self.update_positions();
    }

    fn on_mouse_pressed(&mut self, x: i32, y: i32)->EventProcessStatus {
        if let Some(idx) = self.mouse_position_to_index(x, y) {
            self.open(idx as u32);
            return EventProcessStatus::Processed;
        }
        return EventProcessStatus::Ignored;
    }
    fn on_mouse_move(&mut self, x: i32, y: i32)->EventProcessStatus {
        if let Some(idx) = self.mouse_position_to_index(x, y) {

        }
        return EventProcessStatus::Ignored;
/*
    uint32 idx = MousePositionToItem(x, y);
    if (idx != this->HoveredItem)
    {
        this->HoveredItem = idx;
        repaint           = true;
        // if MenuBar is already opened, moving a mouse over another menu will implicetely open that menu
        if ((this->OpenedItem != NO_ITEM_SELECTED) && (idx != NO_ITEM_SELECTED))
            Open(idx);
        return true;
    }
    if (y == this->Y)
    {
        repaint = false;
        return true;
    }
    return false;

*/        
    }

    fn close(&mut self) {
        // this->OpenedItem  = NO_ITEM_SELECTED;
        // this->HoveredItem = NO_ITEM_SELECTED;
    }
    fn open(&mut self, index: u32) {
/*
    this->OpenedItem = menuIndex;
    if (menuIndex < ItemsCount)
    {
        Items[menuIndex]->Mnu.Show(this->Parent, this->X + Items[menuIndex]->X, this->Y + 1);
        // set the owner
        ((MenuContext*) (Items[menuIndex]->Mnu.Context))->Owner = this;
    }

 */
    }
    #[inline(always)]
    fn is_opened(&self) -> bool {
        false
        //return this->OpenedItem != NO_ITEM_SELECTED;
    }
    fn on_key_event(&mut self, key: Key) -> EventProcessStatus {
        if self.is_opened() {
/*
        switch (keyCode)
        {
        case Key::Left:
            if (this->OpenedItem > 0)
                Open(this->OpenedItem - 1);
            else
                Open(this->ItemsCount - 1);
            return true;
        case Key::Right:
            if (this->OpenedItem + 1 < ItemsCount)
                Open(this->OpenedItem + 1);
            else
                Open(0);
            return true;
        default:
            break;
        }

 */            
        } else {
            for (index,item) in self.items.iter().enumerate() {
                if item.caption.get_hotkey() == key {
                    self.open(index as u32);
                    return EventProcessStatus::Processed;
                }
            }
        }
/*

    // check recursivelly if a shortcut key was not pressed
    for (uint32 tr = 0; tr < this->ItemsCount; tr++)
    {
        if (this->Items[tr]->Mnu.ProcessShortcutKey(keyCode))
        {
            Close();
            return true;
        }
    }
    */
    // nothing to process
    return EventProcessStatus::Ignored;      
    }
}

/*


#include "ControlContext.hpp"

namespace AppCUI::Internal
{
constexpr uint32 NO_ITEM_SELECTED = 0xFFFFFFFFU;

MenuBarItem::MenuBarItem()
{
    this->HotKey       = Input::Key::None;
    this->HotKeyOffset = Graphics::CharacterBuffer::INVALID_HOTKEY_OFFSET;
    this->X            = 0;
}
MenuBar::MenuBar(Controls::Control* parent, int x, int y)
{
    this->ItemsCount  = 0;
    this->Width       = 0;
    this->OpenedItem  = NO_ITEM_SELECTED;
    this->HoveredItem = NO_ITEM_SELECTED;
    this->Cfg         = Application::GetAppConfig();
    this->X           = x;
    this->Y           = y;
    this->Parent      = parent;
}
Menu* MenuBar::GetMenu(ItemHandle itemHandle)
{
    CHECK((uint32) itemHandle < this->ItemsCount,
          nullptr,
          "Invalid item handle (%08X)",
          (uint32) itemHandle);
    return &Items[(uint32) itemHandle]->Mnu;
}
ItemHandle MenuBar::AddMenu(const ConstString& name)
{
    // done
}
void MenuBar::RecomputePositions()
{
    // done
}
void MenuBar::SetWidth(uint32 value)
{
    // done
}
bool MenuBar::OnMouseMove(int x, int y, bool& repaint)
{
    // done
}
uint32 MenuBar::MousePositionToItem(int x, int y)
{
    // done
}
void MenuBar::Open(uint32 menuIndex)
{
    // done
}
bool MenuBar::OnMousePressed(int x, int y, Input::MouseButton /*button*/)
{
    // done
}
void MenuBar::Close()
{
    // oone
}
bool MenuBar::IsOpened()
{
    // done
}
bool MenuBar::OnKeyEvent(Input::Key keyCode)
{
    // done
}
void MenuBar::Paint(Graphics::Renderer& renderer)
{
    renderer.FillHorizontalLine(this->X, this->Y, this->X + Width - 1, ' ', Cfg->Menu.Text.Normal);
    WriteTextParams params(
          WriteTextFlags::SingleLine | WriteTextFlags::LeftMargin | WriteTextFlags::RightMargin |
                WriteTextFlags::OverwriteColors | WriteTextFlags::HighlightHotKey,
          TextAlignament::Left);
    params.Y = this->Y;

    for (uint32 tr = 0; tr < this->ItemsCount; tr++)
    {
        params.X              = this->X + Items[tr]->X + 1;
        params.HotKeyPosition = Items[tr]->HotKeyOffset;

        if (tr == this->OpenedItem)
        {
            params.Color       = Cfg->Menu.Text.PressedOrSelected;
            params.HotKeyColor = Cfg->Menu.HotKey.PressedOrSelected;
        }
        else if (tr == this->HoveredItem)
        {
            params.Color       = Cfg->Menu.Text.Hovered;
            params.HotKeyColor = Cfg->Menu.HotKey.Hovered;
        }
        else
        {
            params.Color       = Cfg->Menu.Text.Normal;
            params.HotKeyColor = Cfg->Menu.HotKey.Normal;
        }

        renderer.WriteText(Items[tr]->Name, params);
    }
}
} // namespace AppCUI::Internal


*/
