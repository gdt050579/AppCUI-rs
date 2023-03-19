use EnumBitFlags::EnumBitFlags;

use crate::{utils::Caption, graphics::Surface, system::Theme};

#[repr(u8)]
pub(super) enum BarItemType {
    None,
    HotKeY,
    CloseButton,
    MaximizeRestoreButton,
    WindowResize,
    Tag,
    Button,
    SingleChoice,
    CheckBox,
    Text,
}

#[repr(u8)]
enum BarItemLayout {
    None,
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

#[EnumBitFlags(bits = 8)]
enum StatusFlags {
    Visible = 0x01,
    Hidden = 0x02,
    Checked = 0x04,
    LeftGroupMarker = 0x08,
    RightGroupMarker = 0x10,
}
pub(super) struct BarItem {
    tooltip: String,
    text: Caption,
    item_type: BarItemType,
    status: StatusFlags,
    x: i32,
    y: i32,
    width: u16,
    id: u32,
    layout: BarItemLayout,
}

impl BarItem {
    #[inline(always)]
    pub (super) fn is_visible(&self) -> bool {
        self.status.contains(StatusFlags::Visible)
    }
    #[inline(always)]
    pub (super) fn is_hidden(&self) -> bool {
        self.status.contains(StatusFlags::Hidden)
    }
    #[inline(always)]
    pub (super) fn is_checked(&self) -> bool {
        self.status.contains(StatusFlags::Checked)
    }
    #[inline(always)]
    pub (super) fn center_x(&self) -> i32 {
        self.x + ((self.width / 2) as i32)
    }
    #[inline(always)]
    pub (super) fn contains(&self, x: i32, y: i32) -> bool {
        (y == self.y)
            && (x >= self.x)
            && (x < (self.x + (self.width as i32)))
            && ((self.status & (StatusFlags::Visible | StatusFlags::Hidden))
                == StatusFlags::Visible)
    }
    pub (super) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        if (self.is_visible()==false) || (self.is_hidden()) {
            return;
        }
        let from_left = match self.layout {
            BarItemLayout::TopLeft | BarItemLayout::BottomLeft => true,
            _ => false 
        };
/*  
        auto* btn = Members->ControlBar.Items;

       {
           if ((!btn->IsVisible()) || (btn->IsHidden()))
               continue;
           bool fromLeft = (btn->Layout == WindowControlsBarLayout::TopBarFromLeft) ||
                           (btn->Layout == WindowControlsBarLayout::BottomBarFromLeft);
           bool showChecked        = false;
           colorStartEndSeparators = colorBorder;
           auto state              = ControlState::Normal;

           if (Members->ControlBar.Current == tr)
           {
               // hover or pressed
               if (Members->ControlBar.IsCurrentItemPressed)
                   state = ControlState::PressedOrSelected;
               else
               {
                   showChecked = ((Members->Focused) && (btn->IsChecked()));
                   state       = ControlState::Hovered;
               }
           }
           else
           {
               if (Members->Focused)
               {
                   showChecked = btn->IsChecked();
                   state       = ControlState::Focused;
               }
               else
                   state = ControlState::Inactive;
           }
           // bool hoverOrPressed = (state == ControlState::Hovered) || (state == ControlState::PressedOrSelected);
           bool drawSeparators = false;
           switch (btn->Type)
           {
           case WindowBarItemType::CloseButton:
               renderer.WriteSingleLineText(
                     btn->X, btn->Y, "[ ]", Members->GetSymbolColor(state, colorStartEndSeparators));
               renderer.WriteCharacter(
                     btn->X + 1, btn->Y, 'x', Members->GetSymbolColor(state, Members->Cfg->Symbol.Close));
               break;
           case WindowBarItemType::MaximizeRestoreButton:
               renderer.WriteSingleLineText(
                     btn->X, btn->Y, "[ ]", Members->GetSymbolColor(state, colorStartEndSeparators));
               renderer.WriteSpecialCharacter(
                     btn->X + 1,
                     btn->Y,
                     Members->Maximized ? SpecialChars::ArrowUpDown : SpecialChars::ArrowUp,
                     Members->GetSymbolColor(state, Members->Cfg->Symbol.Maximized));
               break;
           case WindowBarItemType::WindowResize:
               if (Members->Focused)
                   renderer.WriteSpecialCharacter(
                         btn->X,
                         btn->Y,
                         SpecialChars::BoxBottomRightCornerSingleLine,
                         Members->GetSymbolColor(state, Members->Cfg->Symbol.Resize));
               break;
           case WindowBarItemType::HotKeY:
               renderer.WriteCharacter(btn->X, btn->Y, '[', colorStartEndSeparators);
               tmpCol = Members->Focused ? Members->Cfg->Text.Normal : Members->Cfg->Text.Inactive;
               renderer.WriteSingleLineText(btn->X + 1, btn->Y, KeyUtils::GetKeyName(Members->HotKey), tmpCol);
               renderer.WriteCharacter(btn->X + btn->Size - 1, btn->Y, ']', colorStartEndSeparators);
               break;
           case WindowBarItemType::Tag:
               renderer.WriteCharacter(btn->X, btn->Y, '[', colorStartEndSeparators);
               tmpCol = Members->Focused ? Members->Cfg->Text.Emphasized2 : Members->Cfg->Text.Inactive;
               renderer.WriteSingleLineText(btn->X + 1, btn->Y, btn->Text, tmpCol);
               renderer.WriteCharacter(btn->X + btn->Size - 1, btn->Y, ']', colorStartEndSeparators);
               break;

           case WindowBarItemType::Button:
           case WindowBarItemType::SingleChoice:
               switch (state)
               {
               case ControlState::Hovered:
                   tmpCol = Members->Cfg->Button.Text.Hovered;
                   tmpHK  = Members->Cfg->Button.Text.Hovered;
                   break;
               case ControlState::Normal:
                   tmpCol = Members->Cfg->Text.Normal;
                   tmpHK  = Members->Cfg->Text.HotKey;
                   break;
               case ControlState::Focused:
                   tmpCol = Members->Cfg->Text.Normal;
                   tmpHK  = Members->Cfg->Text.HotKey;
                   break;
               case ControlState::PressedOrSelected:
                   tmpCol = Members->Cfg->Button.Text.PressedOrSelected;
                   tmpHK  = Members->Cfg->Button.Text.PressedOrSelected;
                   break;
               default:
                   tmpHK = tmpCol = Members->Cfg->Text.Inactive;
                   break;
               }
               if (showChecked)
                   renderer.WriteSingleLineText(
                         btn->X,
                         btn->Y,
                         btn->Text,
                         Members->Cfg->Button.Text.PressedOrSelected,
                         Members->Cfg->Button.HotKey.PressedOrSelected,
                         btn->HotKeyOffset);
               else
                   renderer.WriteSingleLineText(btn->X, btn->Y, btn->Text, tmpCol, tmpHK, btn->HotKeyOffset);
               drawSeparators = true;
               break;
           case WindowBarItemType::CheckBox:
               switch (state)
               {
               case ControlState::Hovered:
                   tmpCol = Members->Cfg->Button.Text.Hovered;
                   tmpHK  = Members->Cfg->Button.Text.Hovered;
                   break;
               case ControlState::Normal:
                   tmpCol = Members->Cfg->Text.Normal;
                   tmpHK  = Members->Cfg->Text.HotKey;
                   break;
               case ControlState::Focused:
                   tmpCol = Members->Cfg->Text.Normal;
                   tmpHK  = Members->Cfg->Text.HotKey;
                   break;
               case ControlState::PressedOrSelected:
                   tmpCol = Members->Cfg->Button.Text.PressedOrSelected;
                   tmpHK  = Members->Cfg->Button.Text.PressedOrSelected;
                   break;
               default:
                   tmpHK = tmpCol = Members->Cfg->Text.Inactive;
                   break;
               }
               renderer.FillHorizontalLine(btn->X, btn->Y, btn->X + 1, ' ', tmpCol);
               renderer.WriteSingleLineText(btn->X + 2, btn->Y, btn->Text, tmpCol, tmpHK, btn->HotKeyOffset);
               if (btn->IsChecked())
               {
                   // tmpCol = (Members->Focused && (!hoverOrPressed)) ? wcfg->ControlBar.CheckMark : c_i->Text;
                   tmpCol = Members->GetSymbolColor(state, Members->Cfg->Symbol.Checked);
                   renderer.WriteSpecialCharacter(btn->X, btn->Y, SpecialChars::CheckMark, tmpCol);
               }
               drawSeparators = true;
               break;
           case WindowBarItemType::Text:
               tmpCol = Members->Focused ? Members->Cfg->Text.Normal : Members->Cfg->Text.Inactive;
               renderer.WriteSingleLineText(btn->X, btn->Y, btn->Text, tmpCol);
               drawSeparators = true;
               break;
           }
           // separators
           if (drawSeparators)
           {
               if ((uint8) btn->Flags & (uint8) WindowBarItemFlags::LeftGroupMarker)
                   renderer.WriteCharacter(btn->X - 1, btn->Y, '[', colorStartEndSeparators);
               else if (fromLeft)
                   renderer.WriteCharacter(btn->X - 1, btn->Y, '|', sepColor);
               if ((uint8) btn->Flags & (uint8) WindowBarItemFlags::RightGroupMarker)
                   renderer.WriteCharacter(btn->X + btn->Size, btn->Y, ']', colorStartEndSeparators);
               else if (!fromLeft)
                   renderer.WriteCharacter(btn->X + btn->Size, btn->Y, '|', sepColor);
           }
       }


*/
    }
}
// inline void SetFlag(WindowBarItemFlags flg)
// {
//     Flags = static_cast<WindowBarItemFlags>(((unsigned char) Flags) | ((unsigned char) flg));
// }
// inline void RemoveFlag(WindowBarItemFlags flg)
// {
//     Flags = static_cast<WindowBarItemFlags>(((unsigned char) Flags) & (~((unsigned char) flg)));
// }
