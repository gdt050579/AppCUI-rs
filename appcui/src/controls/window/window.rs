use AppCUIProcMacro::AppCUIControl;

use super::DecoratorsList;
use super::DragStatus;
use super::WindowFlags;
use crate::controls::events::*;
use crate::controls::menu::MenuBar;
use crate::controls::*;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;


#[AppCUIControl(overwrite=OnPaint)]
pub struct Window {
    title: String,
    flags: WindowFlags,
    menu: Option<MenuBar>,
    decorators: DecoratorsList,
    resize_move_mode: bool,
    maximized: bool,
    drag_status: DragStatus,
    title_max_width: u16,
    title_left_margin: i32,
}

impl Window {
    pub fn new(title: &str, layout: Layout, flags: WindowFlags) -> Self {
        Window {
            base: ControlBase::new(
                layout,
                StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput,
            ),
            title: String::from(title),
            flags,
            menu: None,
            resize_move_mode: false,
            maximized: false,
            decorators: DecoratorsList::new(),
            drag_status: DragStatus::None,
            title_max_width: 0,
            title_left_margin: 0,
        }
    }
    pub fn add<T>(&mut self, control: T)
    where
        T: Control + 'static,
    {
        // GDT: only for test --> not the final implementation
        let c = ControlManager::new(control);
        //let v = c.get_version();
        self.children.push(c);
    }
}
impl OnPaint for Window {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let color_window = match () {
            _ if !self.has_focus() => theme.window.inactive,
            _ if self.flags.contains(WindowFlags::WarningWindow) => theme.window.warning,
            _ if self.flags.contains(WindowFlags::ErrorWindow) => theme.window.error,
            _ if self.flags.contains(WindowFlags::NotifyWindow) => theme.window.info,
            _ => theme.window.normal,
        };
        // set some colors
        let color_title: CharAttribute;
        let color_border: CharAttribute;
        let line_type: LineType;

        // initialization
        if self.has_focus() {
            color_title = theme.text.focused;
            color_border = match self.drag_status {
                DragStatus::None => theme.border.focused,
                _ => theme.border.pressed_or_selectd,
            };
            line_type = match self.drag_status {
                DragStatus::None => LineType::Double,
                _ => LineType::Single,
            };
        } else {
            color_title = theme.text.normal;
            color_border = theme.border.normal;
            line_type = LineType::Single;
        }

        surface.clear(Character::with_attributes(' ', color_window));
        surface.draw_rect(
            Rect::with_size(0, 0, self.get_width(), self.get_height()),
            line_type,
            color_border,
        );

        // paint decorators
        self.decorators
            .paint(surface, theme, self.has_focus(), self.maximized);

        // paint title
        if self.title_max_width >= 2 {
            let mut format = TextFormat::single_line(
                self.title_left_margin,
                0,
                color_title,
                TextAlignament::Center,
            );
            format.width = Some(self.title_max_width);
            surface.write_text(self.title.as_str(), &format);
        }
        // paint the menu
        if self.menu.is_some() {
            self.menu.as_ref().unwrap().paint(surface, theme);
        }
    }
}

/*

#include "ControlContext.hpp"

namespace AppCUI
{
constexpr uint8 NO_CONTROLBAR_ITEM   = 0xFF;
constexpr uint32 MAX_TAG_CHARS       = 8U;
constexpr int32 MOVE_TO_LOWER_MARGIN = -1000000;
constexpr int32 MOVE_TO_UPPER_MARGIN = 1000000;
constexpr uint32 INFINITE_DISTANCE   = 0xFFFFFFFF;

const static CharacterBuffer tempReferenceChBuf;

struct WindowControlBarLayoutData
{
    int Left, Right, Y;
    WindowBarItem* LeftGroup;
    WindowBarItem* RighGroup;
};
enum class MoveDirection
{
    ToLeft,
    ToRight,
    ToTop,
    ToBottom
};
Control* FindNextControl(Control* parent, bool forward, bool startFromCurrentOne, bool rootLevel, bool noSteps)
{
    if (parent == nullptr)
        return nullptr;
    CREATE_CONTROL_CONTEXT(parent, Members, nullptr);
    // daca am copii
    if (Members->ControlsCount != 0)
    {
        int start, end;
        if (startFromCurrentOne)
            start = Members->CurrentControlIndex;
        else
            start = -1;
        if (start < 0)
        {
            if (forward)
                start = 0;
            else
                start = Members->ControlsCount - 1;
        }
        // calculez si end
        if (forward)
            end = Members->ControlsCount;
        else
            end = -1;
        // sanity check
        if (((forward) && (start >= end)) || ((!forward) && (start <= end)))
            return nullptr;
        // ma plimb intre elemente
        bool firstElement = true;
        while (true)
        {
            Control* copil = Members->Controls[start];
            if ((copil != nullptr) && (copil->Context != nullptr))
            {
                ControlContext* cMembers = (ControlContext*) copil->Context;
                // am un element posibil ok
                if (((cMembers->Flags & (GATTR_VISIBLE | GATTR_ENABLE)) == (GATTR_VISIBLE | GATTR_ENABLE)))
                {
                    Control* res = FindNextControl(
                          copil, forward, startFromCurrentOne & firstElement, false, noSteps & firstElement);
                    if (res != nullptr)
                        return res;
                }
            }

            if (forward)
                start++;
            else
                start--;
            noSteps = false;
            if (start == end)
            {
                if ((!rootLevel) || (startFromCurrentOne == false))
                    return nullptr;
                // am ajuns la finalul listei si nu am gasit sau am parcurs toata lista
                // daca nu - parcurg si restul listei
                if (forward)
                {
                    start = 0;
                    end   = Members->CurrentControlIndex + 1;
                }
                else
                {
                    start = Members->ControlsCount - 1;
                    end   = Members->CurrentControlIndex + 1;
                }
                // sanity check
                if (((forward) && (start >= end)) || ((!forward) && (start <= end)))
                    return nullptr;
                rootLevel    = false;
                firstElement = false;
            }
            firstElement = false;
        }
    }
    // daca nu am copii
    if (((Members->Flags & GATTR_TABSTOP) != 0) && (noSteps == false))
        return parent;
    return nullptr;
}
uint32 PointToPointDistance(const Rect& originRect, const Rect& objectRect, MoveDirection dir)
{
    // computes the point to point square distance only if the direction is respected
    Point origin, object;

    switch (dir)
    {
    case AppCUI::MoveDirection::ToLeft:
        // we need to have <object>[space]<origin>
        // we compare <TOP,LEFT>
        object.Set(objectRect.GetRight(), objectRect.GetTop());
        origin.Set(originRect.GetLeft(), originRect.GetTop());
        if (object.X >= origin.X)
            return INFINITE_DISTANCE;
        break;
    case AppCUI::MoveDirection::ToRight:
        // we need to have <origin>[space]<object>
        object.Set(objectRect.GetLeft(), objectRect.GetTop());
        origin.Set(originRect.GetRight(), originRect.GetTop());
        if (object.X <= origin.X)
            return INFINITE_DISTANCE;
        break;
    case AppCUI::MoveDirection::ToTop:
        // we need to have <object>[space]<origin>
        object.Set(objectRect.GetLeft(), objectRect.GetBottom());
        origin.Set(originRect.GetLeft(), originRect.GetTop());
        if (object.Y >= origin.Y)
            return INFINITE_DISTANCE;
        break;
    case AppCUI::MoveDirection::ToBottom:
        // we need to have <origin>[space]<object>
        object.Set(objectRect.GetLeft(), objectRect.GetTop());
        origin.Set(originRect.GetLeft(), originRect.GetBottom());
        if (object.Y <= origin.Y)
            return INFINITE_DISTANCE;
        break;
    }
    return (object.X - origin.X) * (object.X - origin.X) + (object.Y - origin.Y) * (object.Y - origin.Y);
}
Control* FindClosestControl(Control* parent, MoveDirection dir, const Rect& origin)
{
    if (parent == nullptr)
        return nullptr;
    CREATE_CONTROL_CONTEXT(parent, Members, nullptr);
    // check my children and find the best fit
    Control* result = nullptr;
    uint32 best     = INFINITE_DISTANCE;
    for (auto idx = 0U; idx < Members->ControlsCount; idx++)
    {
        auto child = Members->Controls[idx];
        auto flags = ((ControlContext*) child->Context)->Flags;
        if ((flags & (GATTR_ENABLE | GATTR_VISIBLE)) != (GATTR_ENABLE | GATTR_VISIBLE))
            continue;
        if (child->GetChildrenCount() > 0)
        {
            // check its children
            child = FindClosestControl(child, dir, origin);
            if (child == nullptr)
                continue;
        }
        else
        {
            // if TABSTOP is not set ==> skip it (e.g. a label)
            if ((flags & GATTR_TABSTOP) == 0)
                continue;
        }

        auto r = child->GetAbsoluteRectangle();
        auto d = PointToPointDistance(origin, r, dir);
        // LOG_INFO(
        //       "%s => (%d,%d  %dx%d), D=%d",
        //       ((std::string) child->GetText()).c_str(),
        //       r.GetLeft(),
        //       r.GetTop(),
        //       r.GetWidth(),
        //       r.GetHeight(),
        //       d);
        if (d < best)
        {
            best   = d;
            result = child;
        }
    }
    return result;
}
Control* FindClosestControl(Control* parent, MoveDirection dir)
{
    if (parent == nullptr)
        return nullptr;
    CREATE_CONTROL_CONTEXT(parent, Members, nullptr);
    // first search current control
    auto child = parent;
    while (child != nullptr)
    {
        auto ctx = ((ControlContext*) (child->Context));
        if (ctx->CurrentControlIndex >= ctx->ControlsCount)
            break;
        auto prnt  = child;
        child      = ctx->Controls[ctx->CurrentControlIndex];
        auto flags = ((ControlContext*) child->Context)->Flags;
        if ((flags & (GATTR_ENABLE | GATTR_VISIBLE)) != (GATTR_ENABLE | GATTR_VISIBLE))
        {
            // current control is unreacheable --> move to parent and stop
            child = prnt;
            break;
        }
    }
    // if child is nullptr --> then we have an error (return)
    CHECK(child, nullptr, "");
    // now we have the current control --> create a center point
    Rect currenChild = child->GetAbsoluteRectangle();
    // Log info
    // LOG_INFO(
    //      "Current control (X=%d,Y=%d, Size=%dx%d)",
    //      currenChild.GetLeft(),
    //      currenChild.GetTop(),
    //      currenChild.GetWidth(),
    //      currenChild.GetHeight());

    // now we need to search the first child that is closest to childPos
    return FindClosestControl(parent, dir, currenChild);
}
bool ProcessHotKey(Control* ctrl, Input::Key KeyCode)
{
    if (ctrl == nullptr)
        return false;
    CREATE_CONTROL_CONTEXT(ctrl, Members, false);
    if (((Members->Flags & (GATTR_VISIBLE | GATTR_ENABLE)) != (GATTR_VISIBLE | GATTR_ENABLE)))
        return false;
    for (uint32 tr = 0; tr < Members->ControlsCount; tr++)
    {
        if (ProcessHotKey(Members->Controls[tr], KeyCode))
            return true;
    }
    if (ctrl->GetHotKey() == KeyCode)
    {
        ctrl->SetFocus();
        ctrl->OnHotKey();
        return true;
    }
    return false;
}
void UpdateWindowButtonPos(WindowBarItem* b, WindowControlBarLayoutData& layout, bool fromLeft)
{
    int next;

    bool partOfGroup = (b->Type == WindowBarItemType::Button) | (b->Type == WindowBarItemType::SingleChoice) |
                       (b->Type == WindowBarItemType::CheckBox) | (b->Type == WindowBarItemType::Text);
    WindowBarItem* group = nullptr;
    int extraX           = 0;

    if (fromLeft)
        group = layout.LeftGroup;
    else
        group = layout.RighGroup;

    // analyze current group
    if (partOfGroup)
    {
        if (group)
        {
            if (group->Type != b->Type)
            {
                if (fromLeft)
                    group->SetFlag(WindowBarItemFlags::RightGroupMarker); // new group, close previous one
                else
                    group->SetFlag(WindowBarItemFlags::LeftGroupMarker); // new group, close previous one
                group  = nullptr;
                extraX = 2;
            }
        }
        else
            extraX = 1;
    }
    else
    {
        if (group)
        {
            if (fromLeft)
                group->SetFlag(WindowBarItemFlags::RightGroupMarker); // close previous one
            else
                group->SetFlag(WindowBarItemFlags::LeftGroupMarker); // close previous one
            group = nullptr;
        }
    }
    if (fromLeft)
        layout.LeftGroup = group;
    else
        layout.RighGroup = group;

    b->Y = layout.Y;
    if (fromLeft)
    {
        b->X = layout.Left + extraX;
        next = b->X + b->Size + 1;
        if (next < layout.Right)
        {
            b->SetFlag(WindowBarItemFlags::Visible);
            layout.Left = next;
            if (partOfGroup)
            {
                if (layout.LeftGroup == nullptr)
                    b->SetFlag(WindowBarItemFlags::LeftGroupMarker);
                else
                    b->RemoveFlag(WindowBarItemFlags::LeftGroupMarker);
                layout.LeftGroup = b;
            }
        }
    }
    else
    {
        b->X = layout.Right - b->Size + 1;
        b->X -= extraX;
        next = b->X - 2;
        if (next > layout.Left)
        {
            b->SetFlag(WindowBarItemFlags::Visible);
            layout.Right = next;
            if (partOfGroup)
            {
                if (layout.RighGroup == nullptr)
                    b->SetFlag(WindowBarItemFlags::RightGroupMarker);
                else
                    b->RemoveFlag(WindowBarItemFlags::RightGroupMarker);
                layout.RighGroup = b;
            }
        }
    }
}
void UpdateWindowsButtonsPoz(WindowControlContext* wcc)
{
    for (uint32 tr = 0; tr < wcc->ControlBar.Count; tr++)
        wcc->ControlBar.Items[tr].RemoveFlag(WindowBarItemFlags::Visible);

    WindowControlBarLayoutData top, bottom;
    top.Left         = 1;
    bottom.Left      = 1;
    top.Y            = 0;
    bottom.Y         = wcc->Layout.Height - 1;
    top.Right        = wcc->Layout.Width - 2;
    bottom.Right     = wcc->Layout.Width - 1;
    top.LeftGroup    = nullptr;
    top.RighGroup    = nullptr;
    bottom.LeftGroup = nullptr;
    bottom.RighGroup = nullptr;

    auto* btn = wcc->ControlBar.Items;
    for (uint32 tr = 0; tr < wcc->ControlBar.Count; tr++, btn++)
    {
        if (btn->IsHidden())
            continue;
        switch (btn->Layout)
        {
        case WindowControlsBarLayout::TopBarFromLeft:
            UpdateWindowButtonPos(btn, top, true);
            break;
        case WindowControlsBarLayout::TopBarFromRight:
            UpdateWindowButtonPos(btn, top, false);
            break;
        case WindowControlsBarLayout::BottomBarFromLeft:
            UpdateWindowButtonPos(btn, bottom, true);
            break;
        case WindowControlsBarLayout::BottomBarFromRight:
            UpdateWindowButtonPos(btn, bottom, false);
            break;
        }
    }
    // group flags
    if (top.LeftGroup)
        top.LeftGroup->SetFlag(WindowBarItemFlags::RightGroupMarker);
    if (top.RighGroup)
        top.RighGroup->SetFlag(WindowBarItemFlags::LeftGroupMarker);
    if (bottom.LeftGroup)
        bottom.LeftGroup->SetFlag(WindowBarItemFlags::RightGroupMarker);
    if (bottom.RighGroup)
        bottom.RighGroup->SetFlag(WindowBarItemFlags::LeftGroupMarker);

    // set title space
    wcc->TitleLeftMargin = top.Left + 1;
    wcc->TitleMaxWidth   = top.Right - wcc->TitleLeftMargin;
    if (wcc->TitleMaxWidth <= 2)
        wcc->TitleMaxWidth = 0;

    if (wcc->menu)
        wcc->menu->SetWidth(wcc->Layout.Width - 2);
}
void WindowRadioButtonClicked(WindowBarItem* start, WindowBarItem* end, WindowBarItem* current)
{
    // go back and disable check
    auto p = current;
    while (p >= start)
    {
        if (p->Layout == current->Layout)
        {
            if (p->Type == WindowBarItemType::SingleChoice)
                p->RemoveFlag(WindowBarItemFlags::Checked);
            else
                break;
        }
        p--;
    }
    p = current;
    while (p < end)
    {
        if (p->Layout == current->Layout)
        {
            if (p->Type == WindowBarItemType::SingleChoice)
                p->RemoveFlag(WindowBarItemFlags::Checked);
            else
                break;
        }
        p++;
    }
    current->SetFlag(WindowBarItemFlags::Checked);
}
void MoveWindowPosTo(Window* win, int addX, int addY, bool keepInDesktopBounderies)
{
    auto x      = win->GetX() + addX;
    auto y      = win->GetY() + addY;
    const int w = win->GetWidth();
    const int h = win->GetHeight();
    Size desktopSize;
    if (AppCUI::Application::GetApplicationSize(desktopSize) == false)
        return;
    if (keepInDesktopBounderies)
    {
        x = std::min<>(x, ((int) desktopSize.Width) - w);
        y = std::min<>(y, ((int) desktopSize.Height) - h);
        x = std::max<>(0, x);
        y = std::max<>(0, y);
    }
    else
    {
        x = std::min<>(x, ((int) desktopSize.Width) - 1);
        y = std::min<>(y, ((int) desktopSize.Height) - 1);
        if (x + w < 1)
            x = 1 - w;
        if (y + h < 1)
            y = 1 - h;
    }
    win->MoveTo(x, y);
}
void ResizeWindow(Window* win, int addToWidth, int addToHeight)
{
    const int w = win->GetWidth() + addToWidth;
    const int h = win->GetHeight() + addToHeight;
    win->Resize(w, h);
}
//=========================================================================================================================================================
ItemHandle Controls::WindowControlsBar::AddCommandItem(const ConstString& name, int ID, const ConstString& toolTip)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, InvalidItemHandle);
    CHECK(Members->ControlBar.Count < MAX_WINDOWBAR_ITEMS,
          InvalidItemHandle,
          "Max number of items in a control bar was exceeded !");
    auto* b = &Members->ControlBar.Items[Members->ControlBar.Count];
    CHECK(b->Init(WindowBarItemType::Button, this->Layout, name, toolTip),
          InvalidItemHandle,
          "Fail to initialize item !");
    b->ID = ID;
    Members->ControlBar.Count++;
    UpdateWindowsButtonsPoz(Members);
    return Members->ControlBar.Count - 1;
}
ItemHandle Controls::WindowControlsBar::AddSingleChoiceItem(
      const ConstString& name, int ID, bool checked, const ConstString& toolTip)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, InvalidItemHandle);
    CHECK(Members->ControlBar.Count < MAX_WINDOWBAR_ITEMS,
          InvalidItemHandle,
          "Max number of items in a control bar was exceeded !");
    auto* b = &Members->ControlBar.Items[Members->ControlBar.Count];
    CHECK(b->Init(WindowBarItemType::SingleChoice, this->Layout, name, toolTip),
          InvalidItemHandle,
          "Fail to initialize item !");
    b->ID = ID;
    Members->ControlBar.Count++;
    if (checked)
        WindowRadioButtonClicked(
              Members->ControlBar.Items,
              Members->ControlBar.Items + Members->ControlBar.Count,
              Members->ControlBar.Items + Members->ControlBar.Count - 1);
    UpdateWindowsButtonsPoz(Members);
    return Members->ControlBar.Count - 1;
}
ItemHandle Controls::WindowControlsBar::AddCheckItem(
      const ConstString& name, int ID, bool checked, const ConstString& toolTip)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, InvalidItemHandle);
    CHECK(Members->ControlBar.Count < MAX_WINDOWBAR_ITEMS,
          InvalidItemHandle,
          "Max number of items in a control bar was exceeded !");
    auto* b = &Members->ControlBar.Items[Members->ControlBar.Count];
    CHECK(b->Init(WindowBarItemType::CheckBox, this->Layout, name, toolTip),
          InvalidItemHandle,
          "Fail to initialize item !");
    b->ID = ID;
    Members->ControlBar.Count++;
    if (checked)
        b->SetFlag(WindowBarItemFlags::Checked);
    else
        b->RemoveFlag(WindowBarItemFlags::Checked);
    UpdateWindowsButtonsPoz(Members);
    return Members->ControlBar.Count - 1;
}
ItemHandle Controls::WindowControlsBar::AddTextItem(const ConstString& caption, const ConstString& toolTip)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, InvalidItemHandle);
    CHECK(Members->ControlBar.Count < MAX_WINDOWBAR_ITEMS,
          InvalidItemHandle,
          "Max number of items in a control bar was exceeded !");
    auto* b = &Members->ControlBar.Items[Members->ControlBar.Count];
    CHECK(b->Init(WindowBarItemType::Text, this->Layout, caption, toolTip),
          InvalidItemHandle,
          "Fail to initialize item !");

    Members->ControlBar.Count++;
    UpdateWindowsButtonsPoz(Members);
    return Members->ControlBar.Count - 1;
}
WindowBarItem* GetWindowControlsBarItem(void* Context, ItemHandle itemHandle)
{
    WindowControlContext* Members = (WindowControlContext*) Context;
    CHECK(Members, nullptr, "");
    uint32 id = (uint32) itemHandle;
    CHECK(id < Members->ControlBar.Count, nullptr, "Invalid item index (%d/%d)", id, Members->ControlBar.Count);
    auto* b = Members->ControlBar.Items + id;
    CHECK((b->Type == WindowBarItemType::Button) || (b->Type == WindowBarItemType::CheckBox) ||
                (b->Type == WindowBarItemType::SingleChoice) || (b->Type == WindowBarItemType::Text),
          nullptr,
          "");
    return b;
}
bool Controls::WindowControlsBar::SetItemText(ItemHandle itemHandle, const ConstString& caption)
{
    auto b = GetWindowControlsBarItem(this->Context, itemHandle);
    CHECK(b, false, "");

    CHECK(b->Text.SetWithHotKey(caption, b->HotKeyOffset, b->HotKey, Key::Alt), false, "");
    b->Size = b->Text.Len();
    if (b->Type == WindowBarItemType::CheckBox)
        b->Size += 2;
    UpdateWindowsButtonsPoz((WindowControlContext*) Context);
    return true;
}
bool Controls::WindowControlsBar::SetItemTextWithHotKey(
      ItemHandle itemHandle, const ConstString& caption, uint32 hotKeyTextOffset)
{
    CHECK(SetItemText(itemHandle, caption), false, "");
    auto b = GetWindowControlsBarItem(this->Context, itemHandle);
    CHECK(b, false, "");
    b->HotKeyOffset = CharacterBuffer::INVALID_HOTKEY_OFFSET;
    b->HotKey       = Key::None;

    ConstStringObject txt(caption);
    char16 ch = 0;
    if (hotKeyTextOffset < txt.Length)
    {
        switch (txt.Encoding)
        {
        case StringEncoding::Ascii:
            ch = (((const char*) txt.Data)[hotKeyTextOffset]);
            break;
        case StringEncoding::Unicode16:
            ch = (((const char16*) txt.Data)[hotKeyTextOffset]);
            break;
        case StringEncoding::CharacterBuffer:
            ch = (((const Character*) txt.Data)[hotKeyTextOffset].Code);
            break;
        case StringEncoding::UTF8:
            ch = (((const uint8*) txt.Data)[hotKeyTextOffset]);
            break;
        }
        if (ch != 0)
        {
            b->HotKey = Utils::KeyUtils::CreateHotKey(ch, Key::Alt);
            if (b->HotKey != Key::None)
                b->HotKeyOffset = hotKeyTextOffset;
        }
    }
    return true;
}
bool Controls::WindowControlsBar::SetItemToolTip(ItemHandle itemHandle, const ConstString& caption)
{
    auto b = GetWindowControlsBarItem(this->Context, itemHandle);
    CHECK(b, false, "");
    CHECK(b->ToolTipText.Set(caption), false, "");
    return true;
}
bool Controls::WindowControlsBar::IsItemChecked(ItemHandle itemHandle)
{
    auto b = GetWindowControlsBarItem(this->Context, itemHandle);
    CHECK(b, false, "");
    return b->IsChecked();
}
bool Controls::WindowControlsBar::SetItemCheck(ItemHandle itemHandle, bool value)
{
    auto b = GetWindowControlsBarItem(this->Context, itemHandle);
    CHECK(b, false, "");
    if (b->Type == WindowBarItemType::CheckBox)
    {
        if (value)
            b->SetFlag(WindowBarItemFlags::Checked);
        else
            b->RemoveFlag(WindowBarItemFlags::Checked);
        return true;
    }
    if (b->Type == WindowBarItemType::SingleChoice)
    {
        CHECK(value, false, "For radio buttom only 'true' can be used as a value");
        WindowControlContext* Members = (WindowControlContext*) Context;
        WindowRadioButtonClicked(Members->ControlBar.Items, Members->ControlBar.Items + Members->ControlBar.Count, b);
        return true;
    }
    RETURNERROR(false, "This method can only be applied on Check and Radio items");
}
bool Controls::WindowControlsBar::IsItemVisible(ItemHandle itemHandle)
{
    auto b = GetWindowControlsBarItem(this->Context, itemHandle);
    CHECK(b, false, "");
    return !b->IsHidden();
}
bool Controls::WindowControlsBar::IsItemShown(ItemHandle itemHandle)
{
    auto b = GetWindowControlsBarItem(this->Context, itemHandle);
    CHECK(b, false, "");
    return b->IsVisible() && (!b->IsHidden());
}
bool Controls::WindowControlsBar::SetItemVisible(ItemHandle itemHandle, bool value)
{
    auto b = GetWindowControlsBarItem(this->Context, itemHandle);
    CHECK(b, false, "");
    if ((b->Type == WindowBarItemType::CheckBox) || (b->Type == WindowBarItemType::Button) ||
        (b->Type == WindowBarItemType::SingleChoice) || (b->Type == WindowBarItemType::Text))
    {
        // change visibility
        if (value)
            b->RemoveFlag(WindowBarItemFlags::Hidden);
        else
            b->SetFlag(WindowBarItemFlags::Hidden);
        UpdateWindowsButtonsPoz((WindowControlContext*) this->Context);
        return true;
    }
    RETURNERROR(false, "This method can only be applied on Check and Radio items");
}
//=========================================================================================================================================================
bool WindowBarItem::Init(WindowBarItemType type, WindowControlsBarLayout layout, uint8 size, string_view toolTipText)
{
    this->Type         = type;
    this->Layout       = layout;
    this->Size         = size;
    this->X            = 0;
    this->Y            = 0;
    this->Flags        = WindowBarItemFlags::None;
    this->ID           = -1;
    this->HotKey       = Key::None;
    this->HotKeyOffset = CharacterBuffer::INVALID_HOTKEY_OFFSET;
    if (!toolTipText.empty())
    {
        CHECK(this->ToolTipText.Set(toolTipText), false, "");
    }
    return true;
}
bool WindowBarItem::Init(
      WindowBarItemType type, WindowControlsBarLayout layout, const ConstString& name, const ConstString& toolTip)
{
    this->Type         = type;
    this->Layout       = layout;
    this->Size         = 0;
    this->X            = 0;
    this->Y            = 0;
    this->Flags        = WindowBarItemFlags::None;
    this->ID           = -1;
    this->HotKey       = Key::None;
    this->HotKeyOffset = CharacterBuffer::INVALID_HOTKEY_OFFSET;
    // name
    ConstStringObject objName(name);
    CHECK(objName.Length > 0, false, "Expecting a valid item name (non-empty)");
    CHECK(this->Text.SetWithHotKey(name, this->HotKeyOffset, this->HotKey, Key::Alt), false, "Fail to create name !");
    this->Size = this->Text.Len();
    if (type == WindowBarItemType::CheckBox)
        this->Size += 2; // for the checkmark
    // tool tip
    ConstStringObject objToolTip(toolTip);
    if (objToolTip.Length > 0)
    {
        CHECK(this->ToolTipText.Set(toolTip), false, "");
    }
    // all good
    return true;
}
//=========================================================================================================================================================
Window::~Window()
{
    DELETE_CONTROL_CONTEXT(WindowControlContext);
}
Window::Window(const ConstString& caption, string_view layout, WindowFlags Flags)
    : Control(new WindowControlContext(), caption, layout, false)
{
    auto Members              = reinterpret_cast<WindowControlContext*>(this->Context);
    Members->Layout.MaxHeight = 200000;
    Members->Layout.MaxWidth  = 200000;
    Members->Layout.MinHeight = 3;
    Members->Layout.MinWidth  = 12; // left_corner(1 char), maximize button(3chars),OneSpaceLeftPadding,
                                    // title, OneSpaceRightPadding, close
                                    // button(char),right_corner(1 char) = 10+szTitle (szTitle = min 2 chars)
    ASSERT(SetMargins(1, 1, 1, 1), "Failed to set margins !");
    Members->Flags = GATTR_ENABLE | GATTR_VISIBLE | GATTR_TABSTOP | (uint32) Flags;

    Members->Maximized                       = false;
    Members->ResizeMoveMode                  = false;
    Members->dragStatus                      = WindowDragStatus::None;
    Members->DialogResult                    = Dialogs::Result::None;
    Members->ControlBar.Current              = NO_CONTROLBAR_ITEM;
    Members->ControlBar.IsCurrentItemPressed = false;
    Members->ControlBar.Count                = 0;
    Members->referalItemHandle               = InvalidItemHandle;
    Members->windowItemHandle                = InvalidItemHandle;

    ASSERT(Members->RecomputeLayout(nullptr), "Fail to recompute layout !");
    this->RecomputeLayout();

    // init the buttons
    if ((Flags & WindowFlags::NoCloseButton) == WindowFlags::None)
    {
        Members->ControlBar.Items[Members->ControlBar.Count++].Init(
              WindowBarItemType::CloseButton, WindowControlsBarLayout::TopBarFromRight, 3, "Close window");
    }
    if ((Flags & WindowFlags::Sizeable) != WindowFlags::None)
    {
        Members->ControlBar.Items[Members->ControlBar.Count++].Init(
              WindowBarItemType::MaximizeRestoreButton,
              WindowControlsBarLayout::TopBarFromLeft,
              3,
              "Maximize or restore the size of this window");
        Members->ControlBar.Items[Members->ControlBar.Count++].Init(
              WindowBarItemType::WindowResize,
              WindowControlsBarLayout::BottomBarFromRight,
              1,
              "Click and drag to resize this window");
    }
    // hot key
    Members->ControlBar.Items[Members->ControlBar.Count].Init(
          WindowBarItemType::HotKeY,
          WindowControlsBarLayout::TopBarFromRight,
          3,
          "Press Alt+xx to switch to this window");
    // the button exists but it is hidden
    Members->ControlBar.Items[Members->ControlBar.Count].SetFlag(WindowBarItemFlags::Hidden);
    Members->ControlBar.Count++;

    // TAG
    Members->ControlBar.Items[Members->ControlBar.Count].Init(
          WindowBarItemType::Tag, WindowControlsBarLayout::TopBarFromLeft, 3, "");
    // the button exists but it is hidden
    Members->ControlBar.Items[Members->ControlBar.Count].SetFlag(WindowBarItemFlags::Hidden);
    Members->ControlBar.Count++;

    UpdateWindowsButtonsPoz(Members);

    if ((Flags & WindowFlags::Maximized) == WindowFlags::Maximized)
    {
        ASSERT(Maxim izeRestore(), "Fail to maximize window !");
    }
    if ((Flags & WindowFlags::Menu) == WindowFlags::Menu)
    {
        Members->menu = std::make_unique<Internal::MenuBar>(this, 1, 1);
        Members->Margins.Top += 1;
        Members->menu->SetWidth(Members->Layout.Width - 2);
    }
}

void Window::Paint(Graphics::Renderer& renderer)
{
    // done
}
bool Window::MaximizeRestore()
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, false);
    if (Members->Maximized == false)
    {
        Members->oldPosX = GetX();
        Members->oldPosY = GetY();
        Members->oldW    = GetWidth();
        Members->oldH    = GetHeight();
        Size sz;
        CHECK(Application::GetDesktopSize(sz), false, "Fail to get desktop size");
        this->MoveTo(0, 0);
        if (this->Resize(sz.Width, sz.Height))
            Members->Maximized = true;
    }
    else
    {
        this->MoveTo(Members->oldPosX, Members->oldPosY);
        this->Resize(Members->oldW, Members->oldH);
        Members->Maximized = false;
    }
    UpdateWindowsButtonsPoz(Members);
    AppCUI::Application::GetApplication()->RepaintStatus = REPAINT_STATUS_COMPUTE_POSITION;
    return true;
}
bool Window::CenterScreen()
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, false);
    Size sz;
    CHECK(Application::GetDesktopSize(sz), false, "Fail to get desktop size !");
    MoveTo(((sz.Width - Members->Layout.Width) / 2), ((sz.Height - Members->Layout.Height) / 2));
    UpdateWindowsButtonsPoz(Members);
    return true;
}
void Window::OnMousePressed(int x, int y, Input::MouseButton button)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, );
    Members->dragStatus                      = WindowDragStatus::None;
    Members->ControlBar.IsCurrentItemPressed = false;
    Members->ResizeMoveMode                  = false;

    if (Members->menu)
    {
        if (Members->menu->OnMousePressed(x, y, button))
            return;
    }

    // win buttons
    Members->ControlBar.Current = NO_CONTROLBAR_ITEM;
    for (uint32 tr = 0; tr < Members->ControlBar.Count; tr++)
    {
        if (Members->ControlBar.Items[tr].Contains(x, y))
        {
            Members->ControlBar.Current              = tr; // set current button
            Members->ControlBar.IsCurrentItemPressed = true;
            if (Members->ControlBar.Items[tr].Type == WindowBarItemType::WindowResize)
                Members->dragStatus = WindowDragStatus::Resize;
            return;
        }
    }
    // Hide tool tip
    HideToolTip();

    if ((Members->Flags & WindowFlags::FixedPosition) == WindowFlags::None)
    {
        Members->dragStatus  = WindowDragStatus::Move;
        Members->dragOffsetX = x;
        Members->dragOffsetY = y;
    }
}
bool Window::ProcessControlBarItem(uint32 index)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, false);
    CHECK(index < Members->ControlBar.Count, false, "");
    auto& b = Members->ControlBar.Items[index];
    switch (b.Type)
    {
    case WindowBarItemType::CloseButton:
        RaiseEvent(Event::WindowClose);
        return true;
    case WindowBarItemType::MaximizeRestoreButton:
        MaximizeRestore();
        return true;
    case WindowBarItemType::Button:
        RaiseEvent(Event::Command, b.ID);
        return true;
    case WindowBarItemType::SingleChoice:
        WindowRadioButtonClicked(
              Members->ControlBar.Items,
              Members->ControlBar.Items + Members->ControlBar.Count,
              &Members->ControlBar.Items[index]);
        RaiseEvent(Event::Command, b.ID);
        return true;
    case WindowBarItemType::CheckBox:
        if (b.IsChecked())
            b.RemoveFlag(WindowBarItemFlags::Checked);
        else
            b.SetFlag(WindowBarItemFlags::Checked);
        RaiseEvent(Event::Command, b.ID);
        return true;
    }
    return false;
}
void Window::OnMouseReleased(int, int, Input::MouseButton)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, );
    Members->ControlBar.IsCurrentItemPressed = false;
    Members->ResizeMoveMode                  = false;
    if (Members->dragStatus != WindowDragStatus::None)
    {
        Members->dragStatus = WindowDragStatus::None;
        return;
    }
    if (Members->ControlBar.Current != NO_CONTROLBAR_ITEM)
    {
        if (ProcessControlBarItem(Members->ControlBar.Current))
            return;
    }

    // if (Members->fnMouseReleaseHandler != nullptr)
    //{
    //	// daca vreau sa tratez eu evenimentul
    //	if (Members->fnMouseReleaseHandler(this, x, y, butonState, Members->fnMouseHandlerContext))
    //		return;
    //}
}
bool Window::OnMouseDrag(int x, int y, Input::MouseButton)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, false);
    Members->ResizeMoveMode = false;
    if (Members->dragStatus == WindowDragStatus::Resize)
    {
        bool res = Resize(x + 1, y + 1);
        UpdateWindowsButtonsPoz(Members);
        return res;
    }
    if (Members->dragStatus == WindowDragStatus::Move)
    {
        this->MoveTo(
              x + Members->ScreenClip.ScreenPosition.X - Members->dragOffsetX,
              y + Members->ScreenClip.ScreenPosition.Y - Members->dragOffsetY);
        return true;
    }
    return false;
}
bool Window::OnMouseOver(int x, int y)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, false);
    if (Members->menu)
    {
        bool repaint;
        if (Members->menu->OnMouseMove(x, y, repaint))
        {
            HideToolTip();
            return true;
        }
    }

    // check buttons
    for (uint32 tr = 0; tr < Members->ControlBar.Count; tr++)
    {
        if (Members->ControlBar.Items[tr].Contains(x, y))
        {
            if (!Members->ControlBar.Items[tr].ToolTipText.IsEmpty())
            {
                ShowToolTip(
                      Members->ControlBar.Items[tr].ToolTipText,
                      Members->ControlBar.Items[tr].CenterX(),
                      Members->ControlBar.Items[tr].Y);
            }
            else
            {
                HideToolTip();
            }
            Members->ControlBar.Current = tr; // set current button
            return true;
        }
    }
    // if I reach this point - tool tip should not be shown and there is no win button selected
    HideToolTip();

    if (Members->ControlBar.Current == NO_CONTROLBAR_ITEM)
        return false; // already outside any window button
    Members->ControlBar.Current = NO_CONTROLBAR_ITEM;
    return true;
}
bool Window::OnMouseLeave()
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, false);
    Members->ControlBar.IsCurrentItemPressed = false;
    if (Members->ControlBar.Current == NO_CONTROLBAR_ITEM)
        return false; // already outside any window button
    Members->ControlBar.Current = NO_CONTROLBAR_ITEM;
    return true;
}
bool Window::OnBeforeResize(int newWidth, int newHeight)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, false);
    if ((Members->Flags & WindowFlags::Sizeable) == WindowFlags::None)
        return false;
    return (newWidth >= Members->Layout.MinWidth) && (newWidth <= Members->Layout.MaxWidth) &&
           (newHeight >= Members->Layout.MinHeight) && (newHeight <= Members->Layout.MaxHeight);
}
void Window::OnAfterResize(int, int)
{
    WindowControlContext* Members = (WindowControlContext*) this->Context;
    if (Members)
    {
        UpdateWindowsButtonsPoz(Members);
    }
}
void Window::RemoveMe()
{
    auto app = Application::GetApplication();
    if (!app)
        return;
    // check if I am part of the modal stack
    for (auto i = 0U; i < app->ModalControlsCount; i++)
        if (app->ModalControlsStack[i] == this)
            return;
    if (!app->AppDesktop)
        return;
    // all good -> I am a top level window --> remove me
    app->AppDesktop->RemoveControl(this);
    app->CheckIfAppShouldClose();
}
bool Window::OnEvent(Reference<Control>, Event eventType, int)
{
    if ((eventType == Event::WindowClose) || (eventType == Event::WindowAccept))
    {
        // check if current win is a modal dialog
        auto app = Application::GetApplication();
        if ((app->ModalControlsCount > 0) && (app->ModalControlsStack[app->ModalControlsCount - 1] == this))
        {
            if (eventType == Event::WindowClose)
                return Exit(Dialogs::Result::Cancel);
            else
                return Exit(Dialogs::Result::Ok);
        }
        else
        {
            RemoveMe();
            return true;
        }
    }
    return false;
}
bool Window::OnKeyEvent(Input::Key KeyCode, char16)
{
    Control* tmp;
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, false);

    if (Members->ResizeMoveMode)
    {
        switch (KeyCode)
        {
        case Key::Escape:
        case Key::Enter:
        case Key::Space:
        case Key::Tab:
            Members->ResizeMoveMode = false;
            return true;
        case Key::Up:
            MoveWindowPosTo(this, 0, -1, false);
            return true;
        case Key::Down:
            MoveWindowPosTo(this, 0, 1, false);
            return true;
        case Key::Left:
            MoveWindowPosTo(this, -1, 0, false);
            return true;
        case Key::Right:
            MoveWindowPosTo(this, 1, 0, false);
            return true;
        case Key::Alt | Key::Up:
            MoveWindowPosTo(this, 0, MOVE_TO_LOWER_MARGIN, true);
            return true;
        case Key::Alt | Key::Down:
            MoveWindowPosTo(this, 0, MOVE_TO_UPPER_MARGIN, true);
            return true;
        case Key::Alt | Key::Left:
            MoveWindowPosTo(this, MOVE_TO_LOWER_MARGIN, 0, true);
            return true;
        case Key::Alt | Key::Right:
            MoveWindowPosTo(this, MOVE_TO_UPPER_MARGIN, 0, true);
            return true;
        case Key::C:
            CenterScreen();
            return true;
        case Key::M:
        case Key::R:
            MaximizeRestore();
            return true;
        case Key::Ctrl | Key::Up:
            ResizeWindow(this, 0, -1);
            return true;
        case Key::Ctrl | Key::Down:
            ResizeWindow(this, 0, 1);
            return true;
        case Key::Ctrl | Key::Left:
            ResizeWindow(this, -1, 0);
            return true;
        case Key::Ctrl | Key::Right:
            ResizeWindow(this, 1, 0);
            return true;
        }
    }
    else
    {
        switch (KeyCode)
        {
        case Key::Ctrl | Key::Alt | Key::M:
        case Key::Ctrl | Key::Alt | Key::R:
            Members->ResizeMoveMode = true;
            return true;

        case Key::Tab | Key::Shift:
            tmp = FindNextControl(this, false, true, true, true);
            if (tmp != nullptr)
                tmp->SetFocus();
            return true;
        case Key::Tab:
            tmp = FindNextControl(this, true, true, true, true);
            if (tmp != nullptr)
                tmp->SetFocus();
            return true;
        case Key::Left:
        case Key::Left | Key::Ctrl:
        case Key::Left | Key::Alt:
            tmp = FindClosestControl(this, MoveDirection::ToLeft);
            if (tmp != nullptr)
                tmp->SetFocus();
            return true;
        case Key::Right:
        case Key::Right | Key::Ctrl:
        case Key::Right | Key::Alt:
            tmp = FindClosestControl(this, MoveDirection::ToRight);
            if (tmp != nullptr)
                tmp->SetFocus();
            return true;
        case Key::Up:
        case Key::Up | Key::Ctrl:
        case Key::Up | Key::Alt:
            tmp = FindClosestControl(this, MoveDirection::ToTop);
            if (tmp != nullptr)
                tmp->SetFocus();
            return true;
        case Key::Down:
        case Key::Down | Key::Ctrl:
        case Key::Down | Key::Alt:
            tmp = FindClosestControl(this, MoveDirection::ToBottom);
            if (tmp != nullptr)
                tmp->SetFocus();
            return true;
        case Key::Escape:
            if (!(Members->Flags && WindowFlags::NoCloseButton))
            {
                RaiseEvent(Event::WindowClose);
                return true;
            }
            return false;
        case Key::Enter:
            if (Members->Flags && WindowFlags::ProcessReturn)
            {
                RaiseEvent(Event::WindowAccept);
                return true;
            }
            return false;
        }
        // first we check menu hot keys
        if (Members->menu)
        {
            if (Members->menu->OnKeyEvent(KeyCode))
                return true;
        }
        // check cntrols hot keys
        if ((((uint32) KeyCode) & (uint32) (Key::Shift | Key::Alt | Key::Ctrl)) == ((uint32) Key::Alt))
        {
            if (ProcessHotKey(this, KeyCode))
                return true;
            auto* b = Members->ControlBar.Items;
            auto* e = b + Members->ControlBar.Count;
            while (b < e)
            {
                if (b->HotKey == KeyCode)
                {
                    if (ProcessControlBarItem((uint32) (b - Members->ControlBar.Items)))
                        return true;
                }
                b++;
            }
        }
    }
    // key was not prcessed, pass it to my parent
    return false;
}
void Window::OnHotKeyChanged()
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, );
    // find hotkey win button
    WindowBarItem* btnHotKey = nullptr;
    for (uint32 tr = 0; tr < Members->ControlBar.Count; tr++)
        if (Members->ControlBar.Items[tr].Type == WindowBarItemType::HotKeY)
        {
            btnHotKey = &Members->ControlBar.Items[tr];
            break;
        }
    // sanity check (in reality the pointer should always be valid)
    if (!btnHotKey)
        return;

    if (Members->HotKey == Key::None)
    {
        btnHotKey->SetFlag(WindowBarItemFlags::Hidden);
    }
    else
    {
        btnHotKey->Size = (int) (KeyUtils::GetKeyName(Members->HotKey).size() + 2);
        btnHotKey->ToolTipText.Set("Press Alt+");
        btnHotKey->ToolTipText.Add(KeyUtils::GetKeyName(Members->HotKey));
        btnHotKey->ToolTipText.Add(" to activate this window");
        btnHotKey->RemoveFlag(WindowBarItemFlags::Hidden);
    }
    UpdateWindowsButtonsPoz(Members);
}
void Window::SetTag(const ConstString& name, const ConstString& toolTipText)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, );
    // find tag win button
    WindowBarItem* b = nullptr;
    for (uint32 tr = 0; tr < Members->ControlBar.Count; tr++)
        if (Members->ControlBar.Items[tr].Type == WindowBarItemType::Tag)
        {
            b = &Members->ControlBar.Items[tr];
            break;
        }
    // sanity check (in reality the pointer should always be valid)
    if (!b)
        return;
    if (!b->ToolTipText.Set(toolTipText))
        return;
    if (!b->Text.Set(name))
        return;
    if (b->Text.Len() > MAX_TAG_CHARS)
        if (!b->Text.Delete(MAX_TAG_CHARS, b->Text.Len()))
            return;
    // all good
    b->Size = (int) (b->Text.Len() + 2);
    b->RemoveFlag(WindowBarItemFlags::Hidden);
    UpdateWindowsButtonsPoz(Members);
}
const Graphics::CharacterBuffer& Window::GetTag()
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, tempReferenceChBuf);
    // find tag win button
    for (uint32 tr = 0; tr < Members->ControlBar.Count; tr++)
    {
        if (Members->ControlBar.Items[tr].Type == WindowBarItemType::Tag)
        {
            return Members->ControlBar.Items[tr].Text;
        }
    }

    return tempReferenceChBuf;
}

bool Window::Exit(Dialogs::Result dialogResult)
{
    //CHECK(dialogResult != Dialogs::Result::None, false, "Dialog result code must not be None !");
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, false);
    Members->DialogResult                     = dialogResult;
    Members->ResizeMoveMode                   = false;
    Application::GetApplication()->loopStatus = Internal::LoopStatus::StopCurrent;
    return true;
}
Dialogs::Result Window::Show()
{
    CHECK(GetParent() == nullptr,
          Dialogs::Result::None,
          "Unable to run modal window if it is attached to another control !");
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, Dialogs::Result::None);
    CHECK(Members->RecomputeLayout(nullptr), Dialogs::Result::None, "Fail to recompute layout !");
    this->RecomputeLayout();
    CHECK(Application::GetApplication()->ExecuteEventLoop(this, true), Dialogs::Result::None, "Modal execution failed !");

    return Members->DialogResult;
}
Dialogs::Result Window::GetDialogResult()
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, Dialogs::Result::None);
    return Members->DialogResult;
}
bool Window::IsWindowInResizeMode()
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, false);
    return (Members->dragStatus == WindowDragStatus::Resize);
}
bool Window::EnableResizeMode()
{
    CHECK(this->HasFocus(), false, "To enable resize mode a window must be focused !");
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, false);
    Members->ResizeMoveMode = true;
    return true;
}
Reference<Menu> Window::AddMenu(const ConstString& name)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, nullptr);
    CHECK(Members->menu, nullptr, "Application was not initialized with Menu option set up !");
    ItemHandle itm         = Members->menu->AddMenu(name);
    Controls::Menu* result = Members->menu->GetMenu(itm);
    CHECK(result, nullptr, "Fail to create menu !");
    return Reference<Menu>(result);
}
WindowControlsBar Window::GetControlBar(WindowControlsBarLayout layout)
{
    if ((this->Context) && (layout != WindowControlsBarLayout::None))
        return WindowControlsBar(this->Context, layout);
    else
        return WindowControlsBar(nullptr, WindowControlsBarLayout::None);
}
} // namespace AppCUI


 */
