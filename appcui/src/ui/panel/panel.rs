use crate::prelude::*;
use crate::ui::panel::Type;

#[CustomControl(overwrite=OnPaint, internal=true)]
pub struct Panel {
    caption: Caption,
    panel_type: Type,
}
impl Panel {
    pub fn new(caption: &str, layout: Layout, panel_type: Type) -> Self {
        let mut panel = Panel {
            base: ControlBase::new(layout, StatusFlags::Visible | StatusFlags::Enabled),
            caption: Caption::new(caption, false),
            panel_type,
        };
        match panel_type {
            Type::Border => panel.base.set_margins(1, 1, 1, 1),
            Type::Window => panel.base.set_margins(1, 1, 1, 1),
            Type::Page => {}
            Type::TopBar => panel.base.set_margins(0, 1, 0, 0),
        }
        panel
    }
    pub fn set_caption(&mut self, text: &str) {
        self.caption.set_text(text, false);
    }
    pub fn get_caption(&self) -> &str {
        self.caption.get_text()
    }
    #[inline]
    fn paint_border(&self, surface: &mut Surface, theme: &Theme) {
        let sz = self.get_size();
        let border_color = if self.is_enabled() { theme.border.normal } else { theme.border.inactive };
        surface.clear(Character::with_char(' '));
        surface.draw_rect(Rect::with_point_and_size(Point::ORIGIN, sz), LineType::Single, border_color);
        if (self.caption.get_chars_count() > 0) && (sz.width > 7) {
            let text_color = if self.is_enabled() { theme.text.normal } else { theme.text.inactive };
            let mut format = TextFormat::new(3, 0, text_color, TextAlignament::Left, false);
            format.width = Some((sz.width - 6) as u16);
            let chars_count = self.caption.get_chars_count();
            surface.write_text(self.caption.get_text(), &format);
            surface.write_char(2, 0, Character::with_char(' '));
            if chars_count > (sz.width - 6) as usize {
                surface.write_char((sz.width - 3) as i32, 0, Character::with_char(' '));
                surface.write_char((sz.width - 4) as i32, 0, Character::with_char(SpecialChar::ThreePointsHorizontal));
            } else {
                surface.write_char(3 + chars_count as i32, 0, Character::with_char(' '));
            }
        }
    }
    #[inline]
    fn paint_window(&self, surface: &mut Surface, theme: &Theme) {}
    #[inline]
    fn paint_page(&self, surface: &mut Surface, theme: &Theme) {
        // title si ignored
        if self.is_enabled()
        {
            surface.clear(Character::with_attributes(' ',theme.tab.text.pressed_or_selectd));
        } else {
            surface.clear(Character::with_char(' '));
        }
    }
    #[inline]
    fn paint_topbar(&self, surface: &mut Surface, theme: &Theme) {}
}
impl OnPaint for Panel {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        match self.panel_type {
            Type::Border => self.paint_border(surface, theme),
            Type::Window => self.paint_window(surface, theme),
            Type::Page => self.paint_page(surface, theme),
            Type::TopBar => self.paint_topbar(surface, theme),
        }
    }
}
