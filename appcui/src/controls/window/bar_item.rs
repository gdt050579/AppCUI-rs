use EnumBitFlags::EnumBitFlags;

use crate::{
    graphics::{CharAttribute, Character, SpecialChar, Surface},
    system::Theme,
    utils::Caption,
};

pub(super) struct BarItemPaintData {
    pub(super) focused: bool,
    pub(super) current: bool,
    pub(super) maximized: bool,
    pub(super) is_current_item_pressed: bool,
    pub(super) sep_attr: CharAttribute,
}

#[repr(u8)]
pub(super) enum BarItemType {
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

enum SymbolAttrState {
    Hovered,
    Normal,
    Pressed,
    Inactive,
}
impl SymbolAttrState {
    fn new(paint_data: &BarItemPaintData) -> Self {
        if paint_data.current {
            if paint_data.is_current_item_pressed {
                SymbolAttrState::Pressed
            } else {
                // showChecked = ((Members->Focused) && (btn->IsChecked()));
                SymbolAttrState::Hovered
            }
        } else {
            if paint_data.focused {
                // showChecked = btn->IsChecked();
                SymbolAttrState::Normal
            } else {
                SymbolAttrState::Inactive
            }
        }
    }
    #[inline(always)]
    fn get_attr(&self, theme: &Theme, default_attr: CharAttribute) -> CharAttribute {
        match self {
            SymbolAttrState::Hovered => theme.symbol.hovered,
            SymbolAttrState::Normal => default_attr,
            SymbolAttrState::Pressed => theme.symbol.pressed,
            SymbolAttrState::Inactive => theme.symbol.inactive,
        }
    }
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
    pub(super) fn is_visible(&self) -> bool {
        self.status.contains(StatusFlags::Visible)
    }
    #[inline(always)]
    pub(super) fn is_hidden(&self) -> bool {
        self.status.contains(StatusFlags::Hidden)
    }
    #[inline(always)]
    pub(super) fn is_checked(&self) -> bool {
        self.status.contains(StatusFlags::Checked)
    }
    #[inline(always)]
    pub(super) fn center_x(&self) -> i32 {
        self.x + ((self.width / 2) as i32)
    }
    #[inline(always)]
    pub(super) fn contains(&self, x: i32, y: i32) -> bool {
        (y == self.y)
            && (x >= self.x)
            && (x < (self.x + (self.width as i32)))
            && ((self.status & (StatusFlags::Visible | StatusFlags::Hidden))
                == StatusFlags::Visible)
    }

    fn paint_hotkey(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        paint_data: &BarItemPaintData,
    ) -> bool {
        surface.write_char(
            self.x,
            self.y,
            Character::with_attributes('[', paint_data.sep_attr),
        );
        let attr = match paint_data.focused {
            true => theme.text.normal,
            false => theme.text.inactive,
        };
        surface.write_string(self.x + 1, self.y, self.text.get_text(), attr, false);
        surface.write_char(
            self.x + (self.width as i32) - 1,
            self.y,
            Character::with_attributes(']', paint_data.sep_attr),
        );
        return false;
    }
    fn paint_tag(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        paint_data: &BarItemPaintData,
    ) -> bool {
        surface.write_char(
            self.x,
            self.y,
            Character::with_attributes('[', paint_data.sep_attr),
        );
        let attr = match paint_data.focused {
            true => theme.text.enphasized_2,
            false => theme.text.inactive,
        };
        surface.write_string(self.x + 1, self.y, self.text.get_text(), attr, false);
        surface.write_char(
            self.x + (self.width as i32) - 1,
            self.y,
            Character::with_attributes(']', paint_data.sep_attr),
        );
        return false;
    }
    fn paint_text(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        paint_data: &BarItemPaintData,
    ) -> bool {
        let attr = match paint_data.focused {
            true => theme.text.normal,
            false => theme.text.inactive,
        };
        surface.write_string(self.x + 1, self.y, self.text.get_text(), attr, false);
        return true;
    }
    fn paint_close_button(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        paint_data: &BarItemPaintData,
    ) -> bool {
        let st = SymbolAttrState::new(paint_data);
        surface.write_string(
            self.x,
            self.y,
            "[ ]",
            st.get_attr(theme, paint_data.sep_attr),
            false,
        );
        surface.write_char(
            self.x + 1,
            self.y,
            Character::with_attributes('x', st.get_attr(theme, theme.symbol.close)),
        );
        return false;
    }
    fn paint_max_button(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        paint_data: &BarItemPaintData,
    ) -> bool {
        let st = SymbolAttrState::new(paint_data);
        surface.write_string(
            self.x,
            self.y,
            "[ ]",
            st.get_attr(theme, paint_data.sep_attr),
            false,
        );
        let ch = match paint_data.maximized {
            true => SpecialChar::ArrowUpDown,
            false => SpecialChar::ArrowUp,
        };
        surface.write_char(
            self.x + 1,
            self.y,
            Character::with_attributes(ch, st.get_attr(theme, theme.symbol.maximized)),
        );
        return false;
    }
    fn paint_resize_button(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        paint_data: &BarItemPaintData,
    ) -> bool {
        if paint_data.focused {
            let st = SymbolAttrState::new(paint_data);
            surface.write_char(
                self.x,
                self.y,
                Character::with_attributes(
                    SpecialChar::BoxBottomRightCornerSingleLine,
                    st.get_attr(theme, theme.symbol.resize),
                ),
            );
        }
        return false;
    }
    pub(super) fn paint(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        paint_data: &BarItemPaintData,
    ) {
        if (self.is_visible() == false) || (self.is_hidden()) {
            return;
        }

        let from_left = match self.layout {
            BarItemLayout::TopLeft | BarItemLayout::BottomLeft => true,
            _ => false,
        };
        let draw_separators = match self.item_type {
            BarItemType::HotKeY => self.paint_hotkey(surface, theme, paint_data),
            BarItemType::CloseButton => self.paint_close_button(surface, theme, paint_data),
            BarItemType::MaximizeRestoreButton => self.paint_max_button(surface, theme, paint_data),
            BarItemType::WindowResize => self.paint_resize_button(surface, theme, paint_data),
            BarItemType::Tag => self.paint_tag(surface, theme, paint_data),
            BarItemType::Button => todo!(),
            BarItemType::SingleChoice => todo!(),
            BarItemType::CheckBox => todo!(),
            BarItemType::Text => self.paint_text(surface, theme, paint_data),
        };
        // separators
        if draw_separators {
            if self.status.contains(StatusFlags::LeftGroupMarker) {
                surface.write_char(
                    self.x - 1,
                    self.y,
                    Character::with_attributes('[', paint_data.sep_attr),
                );
            } else if from_left {
                surface.write_char(
                    self.x - 1,
                    self.y,
                    Character::with_attributes('|', paint_data.sep_attr),
                );
            }
            if self.status.contains(StatusFlags::RightGroupMarker) {
                surface.write_char(
                    self.x + (self.width as i32),
                    self.y,
                    Character::with_attributes(']', paint_data.sep_attr),
                );
            } else if !from_left {
                surface.write_char(
                    self.x + (self.width as i32),
                    self.y,
                    Character::with_attributes('|', paint_data.sep_attr),
                );
            }
        }
        /*
                auto* btn = Members->ControlBar.Items;

               {
                   bool showChecked        = false;
                   colorStartEndSeparators = colorBorder;
                   auto state              = ControlState::Normal;


                   // bool hoverOrPressed = (state == ControlState::Hovered) || (state == ControlState::PressedOrSelected);

                   switch (btn->Type)
                   {
                   case WindowBarItemType::CloseButton:
                        // done
                       break;
                   case WindowBarItemType::MaximizeRestoreButton:
                       // done
                       break;
                   case WindowBarItemType::WindowResize:
                        // done
                       break;
                   case WindowBarItemType::HotKeY:
                            // done
                       break;
                   case WindowBarItemType::Tag:
                            // done
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
                       // done
                       break;
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
