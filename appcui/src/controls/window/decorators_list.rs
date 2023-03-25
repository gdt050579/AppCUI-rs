use super::{DecoratorPaintData, Decorator};
use crate::{utils::VectorIndex, graphics::Surface, system::Theme};

pub(super) struct DecoratorsList {
    items: Vec<Decorator>,
    current: VectorIndex,
    pressed: bool,
}

impl DecoratorsList {
    pub(super) fn new() -> Self {
        Self {
            items: Vec::with_capacity(4),
            current: VectorIndex::invalid(),
            pressed: false,
        }
    }
    pub(super) fn add(&mut self,decorator: Decorator) {
        self.items.push(decorator);
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, focused: bool, maximized: bool) {
        let mut paint_data = DecoratorPaintData {
            focused,
            current: false,
            maximized,
            is_current_item_pressed: self.pressed,
            sep_attr: if focused { theme.lines.normal } else { theme.lines.inactive },
        };
        let current_bar_index = self.current.index();
        // paint bar items
        for (index, item) in self.items.iter().enumerate() {
            paint_data.current = index == current_bar_index;
            item.paint(surface, theme, &paint_data);
        }
    }
}

/*
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


 */