use crate::prelude::*;
use crate::ui::panel::Type;

#[CustomControl(overwrite=OnPaint, internal=true)]
pub struct Panel {
    caption: Caption,
    panel_type: Type,
}
impl Panel {
    /// Creates a new Panel control with the specified caption and layout.
    /// The type of the panel will be obtained from the current theme.
    /// 
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let mut panel = Panel::new("Panel",
    ///                            layout!("x:1,y:1,w:20,h:10"));
    /// ```
    pub fn new(caption: &str, layout: Layout) -> Self {
        Self::inner_create(caption, layout, Type::Border, StatusFlags::ThemeType)
    }
    /// Creates a new Panel control with the specified caption, layout and type.
    /// The panel type is one of the following values:
    /// * `Type::Border` - a panel with a border around it
    /// * `Type::Window` - a panel with a border around it and a title bar
    /// * `Type::Page` - a panel without a border, used to group controls
    /// * `Type::TopBar` - a panel without a border, used to group controls and to display a title bar
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut panel = Panel::with_type("Panel",
    ///                                  layout!("x:1,y:1,w:20,h:10"),
    ///                                  panel::Type::Border);
    /// ```
    pub fn with_type(caption: &str, layout: Layout, panel_type: Type) -> Self {
        Self::inner_create(caption, layout, panel_type, StatusFlags::None)
    }

    pub fn inner_create(caption: &str, layout: Layout, panel_type: Type, status: StatusFlags) -> Self {
        let mut panel = Panel {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | status),
            caption: Caption::new(caption, ExtractHotKeyMethod::NoHotKey),
            panel_type,
        };
        panel.update_margins();
        panel
    }

    fn update_margins(&mut self) {
        match self.panel_type {
            Type::Border => self.base.set_margins(1, 1, 1, 1),
            Type::Window => self.base.set_margins(1, 1, 1, 1),
            Type::Page => self.base.set_margins(0, 0, 0, 0),
            Type::TopBar => self.base.set_margins(0, 1, 0, 0),
        }
    }

    /// Sets the title of the panel. The title is displayed only if the panel type is `Type::Window` , `Type::TopBar` or
    pub fn set_title(&mut self, text: &str) {
        self.caption.set_text(text, ExtractHotKeyMethod::NoHotKey);
    }

    /// Returns the title of the panel
    #[inline(always)]
    pub fn title(&self) -> &str {
        self.caption.text()
    }

    /// Returns the type of the panel
    #[inline(always)]
    pub fn panel_type(&self) -> Type {
        self.panel_type
    }

    /// Adds a new control to the panel. The control will be added as a child of the panel and will be automatically removed when the panel is destroyed.
    /// Returns a handle to the newly created control.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut panel = Panel::new("Panel", layout!("x:1,y:1,w:10,h:10"), panel::Type::Border);
    /// let handle_button = panel.add(Button::new("Button", layout!("x:1,y:1,w:8")));
    /// ```
    #[inline(always)]
    pub fn add<T>(&mut self, control: T) -> Handle<T>
    where
        T: Control + NotWindow + NotDesktop + 'static,
    {
        self.add_child(control)
    }
    #[inline(always)]
    fn paint_border(&self, surface: &mut Surface, theme: &Theme) {
        let sz = self.size();
        let border_color = if self.is_enabled() { theme.border.normal } else { theme.border.inactive };
        surface.clear(Character::with_char(' '));
        surface.draw_rect(Rect::with_point_and_size(Point::ORIGIN, sz), LineType::Single, border_color);
        let chars_count = self.caption.chars_count();
        if (chars_count > 0) && (sz.width > 7) {
            let format = TextFormatBuilder::new()
                .position(3, 0)
                .attribute(if self.is_enabled() { theme.text.normal } else { theme.text.inactive })
                .align(TextAlignment::Left)
                .wrap_type(WrapType::SingleLineWrap((sz.width - 6) as u16))
                .chars_count(chars_count as u16)
                .build();
            surface.write_text(self.caption.text(), &format);
            surface.write_char(2, 0, Character::with_char(' '));
            if chars_count > (sz.width - 6) as usize {
                surface.write_char((sz.width - 3) as i32, 0, Character::with_char(' '));
                surface.write_char((sz.width - 4) as i32, 0, Character::with_char(SpecialChar::ThreePointsHorizontal));
            } else {
                surface.write_char(3 + chars_count as i32, 0, Character::with_char(' '));
            }
        }
    }
    #[inline(always)]
    fn paint_window(&self, surface: &mut Surface, theme: &Theme) {
        let sz = self.size();
        let border_color = if self.is_enabled() { theme.border.normal } else { theme.border.inactive };
        surface.clear(Character::with_char(' '));
        surface.draw_rect(Rect::with_point_and_size(Point::ORIGIN, sz), LineType::Single, border_color);
        let chars_count = self.caption.chars_count();
        if (chars_count > 0) && (sz.width > 7) {
            let mut format = TextFormatBuilder::new()
                .position(3, 0)
                .attribute(if self.is_enabled() { theme.text.normal } else { theme.text.inactive })
                .align(TextAlignment::Left)
                .wrap_type(WrapType::SingleLineWrap((sz.width - 6) as u16))
                .chars_count(chars_count as u16)
                .build();

            if chars_count > (sz.width - 6) as usize {
                surface.write_text(self.caption.text(), &format);
                surface.write_char(2, 0, Character::with_char(' '));
                surface.write_char((sz.width - 3) as i32, 0, Character::with_char(' '));
                surface.write_char((sz.width - 4) as i32, 0, Character::with_char(SpecialChar::ThreePointsHorizontal));
            } else {
                let x = ((sz.width / 2) as i32) - ((chars_count + 2) as i32) / 2;
                format.x = x + 1;
                surface.write_text(self.caption.text(), &format);
                surface.write_char(x, 0, Character::with_char(' '));
                surface.write_char(x + 1 + chars_count as i32, 0, Character::with_char(' '));
            }
        }
    }
    #[inline(always)]
    fn paint_page(&self, surface: &mut Surface, theme: &Theme) {
        // title si ignored
        if self.is_enabled() {
            surface.clear(Character::with_attributes(' ', theme.tab.text.pressed_or_selected));
        } else {
            surface.clear(Character::with_char(' '));
        }
    }
    #[inline(always)]
    fn paint_topbar(&self, surface: &mut Surface, theme: &Theme) {
        let sz = self.size();
        if self.is_enabled() {
            surface.clear(Character::with_attributes(' ', theme.tab.text.pressed_or_selected));
            surface.fill_horizontal_line(0, 0, sz.width as i32, Character::with_attributes(' ', theme.tab.text.normal))
        } else {
            surface.clear(Character::with_char(' '));
        }
        let chars_count = self.caption.chars_count();
        if (chars_count > 0) && (sz.width > 7) {
            let mut format = TextFormatBuilder::new()
                .position(3, 0)
                .attribute(if self.is_enabled() { theme.tab.text.normal } else { theme.text.inactive })
                .align(TextAlignment::Left)
                .wrap_type(WrapType::SingleLineWrap((sz.width - 6) as u16))
                .chars_count(chars_count as u16)
                .build();
            if chars_count > (sz.width - 6) as usize {
                surface.write_text(self.caption.text(), &format);
                surface.write_char(2, 0, Character::with_char(' '));
                surface.write_char((sz.width - 3) as i32, 0, Character::with_char(' '));
                surface.write_char((sz.width - 4) as i32, 0, Character::with_char(SpecialChar::ThreePointsHorizontal));
            } else {
                let x = ((sz.width / 2) as i32) - ((chars_count + 2) as i32) / 2;
                format.x = x + 1;
                surface.write_text(self.caption.text(), &format);
                surface.write_char(x, 0, Character::with_char(' '));
                surface.write_char(x + 1 + chars_count as i32, 0, Character::with_char(' '));
            }
        }
    }
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
