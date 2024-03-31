use crate::prelude::*;
use crate::ui::tab::{Flags, Type};

#[CustomControl(overwrite=OnPaint, internal=true)]
pub struct Tab {
    tab_type: Type,
    flags: Flags,
    tab_width: u8,
}

impl Tab {
    fn update_margins(&mut self) {
        match self.tab_type {
            Type::Hidden => self.base.set_margins(0, 0, 0, 0),
            Type::OnTop => self.base.set_margins(0, 1, 0, 0),
            Type::OnBottom => self.base.set_margins(0, 0, 0, 1),
            Type::OnLeft => self.base.set_margins(self.tab_width, 0, 0, 0),
            Type::List => {
                let idx = self.base.focused_child_index.index();
                let cnt = self.base.children.len();
                if idx < cnt {
                    self.base.set_margins(0, 1 + idx as u8, 0, (cnt - (idx + 1)) as u8);
                } else {
                    self.base.set_margins(0, 0, 0, 0);
                }
            }
        }
    }
    fn mouse_position_to_index(&self, x: i32, y: i32) -> Option<usize> {
        let count = self.base.children.len();
        if count == 0 {
            return None;
        }
        match self.tab_type {
            Type::Hidden => None,
            Type::OnTop => {
                if (y != 0) || (x < 1) {
                    return None;
                }
                let idx = (x as usize - 1) / ((self.tab_width as usize) + 1usize);
                if idx >= count {
                    return None;
                }
                Some(idx)
            }
            Type::OnBottom => {
                if (y != self.size().height as i32 - 1) || (x < 1) {
                    return None;
                }
                let idx = (x as usize - 1) / ((self.tab_width as usize) + 1usize);
                if idx >= count {
                    return None;
                }
                Some(idx)
            }
            Type::OnLeft => {
                if (x < 0) || (x > self.tab_width as i32) || (y < 1) {
                    return None;
                }
                let idx = y as usize - 1;
                if idx >= count {
                    return None;
                }
                Some(idx)
            }
            Type::List => {
                if y < 0 {
                    return None;
                }
                let fc = self.base.focused_child_index.index();
                // check top allignament
                if y as usize <= fc {
                    return Some(y as usize);
                }
                if fc >= count {
                    return None;
                }
                // check bottom allignament
                let bottom_index = (count - fc) as i32;
                let h = self.size().height as i32;
                if h < bottom_index {
                    return None;
                }
                if y >= (h - bottom_index) && (y < h) {
                    Some(fc + 1 + ((h - bottom_index) as usize))
                } else {
                    None
                }
            }
        }
    }
    #[inline(always)]
    fn get_backattr(&self, theme: &Theme) -> CharAttribute {
        match () {
            _ if !self.is_enabled() => theme.tab.text.inactive,
            _ if self.has_focus() => theme.tab.text.focused,
            _ => theme.tab.text.normal,
        }
    }
    #[inline(always)]
    fn get_tabattr(&self, theme: &Theme) -> CharAttribute {
        match () {
            _ if !self.is_enabled() => theme.tab.text.inactive,
            _ if self.has_focus() => theme.tab.text.hovered,
            _ => theme.tab.text.normal,
        }
    }
    fn paint_horizontal_tab(&self, surface: &mut Surface, theme: &Theme, y: i32) {
        let mut format = TextFormat {
            x: 1,
            y,
            width: Some(self.tab_width as u16 - 2),
            align: TextAlignament::Center,
            text_wrap: TextWrap::None,
            multi_line: false,
            ..Default::default()
        };

        let sz = self.size();
        if !self.flags.contains(Flags::TransparentBackground) {
            let fill_char = Character::with_attributes(' ', self.get_backattr(theme));
            if y == 0 {
                surface.fill_rect(Rect::new(0, 1, sz.width as i32, sz.height as i32), fill_char);
            } else {
                surface.fill_rect(Rect::new(0, 0, sz.width as i32, sz.height as i32 - 2), fill_char);
            }
        }

        if self.flags.contains(Flags::TabsBar) {
            surface.fill_horizontal_line_with_size(0, y, sz.width, Character::with_attributes(' ', self.get_tabattr(theme)));
        }

        //     WriteTextParams params(
        //         WriteTextFlags::OverwriteColors | WriteTextFlags::SingleLine | WriteTextFlags::HighlightHotKey |
        //         WriteTextFlags::ClipToWidth | WriteTextFlags::FitTextToWidth);

        //   params.Width = this->TabTitleSize - 2;
        //   params.Align = TextAlignament::Center;
        //   params.Y     = onTop ? 0 : this->Layout.Height - 1;
        //   int poz      = 1;

        //   if ((this->Flags & TabFlags::TransparentBackground) != TabFlags::TransparentBackground)
        //       renderer.FillRectSize(0, onTop ? 1 : 0, this->Layout.Width, this->Layout.Height - 1, ' ', this->GetPageColor());

        //   if ((this->Flags & TabFlags::TabsBar) == TabFlags::TabsBar)
        //       renderer.FillHorizontalLineSize(0, params.Y, this->Layout.Width, ' ', this->GetTabBarColor());

        //   for (uint32 tr = 0; tr < this->ControlsCount; tr++, poz += (this->TabTitleSize + 1))
        //   {
        //       if (this->Controls[tr] == nullptr)
        //           continue;
        //       ControlContext* cc = (ControlContext*) this->Controls[tr]->Context;
        //       if (cc == nullptr)
        //           continue;

        //       const auto state = this->GetComponentState(
        //             ControlStateFlags::All,
        //             tr == static_cast<uint32>(this->HoveredTabIndex),
        //             tr == this->CurrentControlIndex);
        //       params.Color       = this->Cfg->Tab.Text.GetColor(state);
        //       params.HotKeyColor = this->Cfg->Tab.HotKey.GetColor(state);

        //       renderer.FillHorizontalLineSize(poz, params.Y, this->TabTitleSize, ' ', params.Color);
        //       params.HotKeyPosition = cc->HotKeyOffset;
        //       params.X              = poz + 1;
        //       renderer.WriteText(cc->Text, params);
        //   }
    }
}

impl OnPaint for Tab {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        match self.tab_type {
            Type::Hidden => todo!(),
            Type::OnTop => self.paint_horizontal_tab(surface, theme, 0),
            Type::OnBottom => todo!(),
            Type::OnLeft => todo!(),
            Type::List => todo!(),
        }
    }
}
