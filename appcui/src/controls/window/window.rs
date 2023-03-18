use AppCUIProcMacro::AppCUIControl;

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
    resize_move_mode: bool,
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
            drag_status: DragStatus::None,
            title_max_width: 0,
            title_left_margin: 0,
        }
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
        let color_sep: CharAttribute;
        let line_type: LineType;

        // initialization
        if self.has_focus() {
            color_title = theme.text.focused;
            color_sep = theme.lines.normal;
            color_border = match self.drag_status {
                DragStatus::None => theme.border.focused,
                _ => theme.border.pressed_or_selectd
            };
            line_type = match self.drag_status {
                DragStatus::None => LineType::Double,
                _ => LineType::Single,
            };
        } else {
            color_title = theme.text.normal;
            color_sep = theme.lines.inactive;
            color_border = theme.border.normal;
            line_type = LineType::Single;
        }
    
        surface.clear(Character::with_attributes(' ', color_window));
        surface.draw_rect(Rect::with_size(0, 0, self.get_width(), self.get_height()), line_type, color_border);

        // paint bar items
        // to be added

        // paint title
        if self.title_max_width >= 2 {
            let mut format = TextFormat::single_line(self.title_left_margin, 0, color_title, TextAlignament::Center);
            format.width = Some(self.title_max_width);
            surface.write_text(self.title.as_str(), &format);
        }
        // paint the menu
        if self.menu.is_some() {
            self.menu.as_ref().unwrap().paint(surface, theme);
        }
    }
    /*
       CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, );
       ColorPair colorTitle, colorWindow, colorBorder, colorStartEndSeparators, tmpCol, tmpHK;
       LineType lineType;


       auto* btn = Members->ControlBar.Items;
       for (uint32 tr = 0; tr < Members->ControlBar.Count; tr++, btn++)
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
