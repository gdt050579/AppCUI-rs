use crate::prelude::*;
use crate::ui::togglebutton::{events::EventData, Type};
use flat_string::FlatString;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent+OnSiblingSelected, internal=true)]
pub struct ToggleButton {
    caption: FlatString<22>,
    tooltip: String,
    state: bool,
    button_type: Type,
    single_selection: bool,
}
impl ToggleButton {
    /// Creates a new toggle button with the specified caption, tooltip, layout, selected state and button type.    
    /// The button type can be `Normal` or `Underlined`.
    /// 
    /// # Examples
    /// ```rust,no_run
    /// use appcui::prelude::*;
    /// let mut button = ToggleButton::new("🐼", 
    ///                                    "Enable Panda Mode", 
    ///                                    Layout::new("x:1,y:1,w:2"), 
    ///                                    false, 
    ///                                    togglebutton::Type::Normal);
    /// ```
    pub fn new(caption: &str, tooltip: &str, layout: Layout, selected: bool, button_type: Type) -> Self {
        Self::inner_new(caption, tooltip, layout, selected, button_type, false)
    }

    /// Creates a new toggle button with the specified caption, tooltip, layout, selected state and button type.
    /// The button type can be `Normal` or `Underlined`.
    /// This type of button is considered to be part of a group of buttons, and only one button can be selected at a time.
    /// 
    /// Example:
    /// ```rust,no_run
    /// use appcui::prelude::*;
    /// let panda = ToggleButton::with_single_selection("🐼",
    ///                                                 "Enable Panda Mode",
    ///                                                 Layout::new("x:1,y:1,w:2"),
    ///                                                 false,
    ///                                                 togglebutton::Type::Normal);
    /// let dog = ToggleButton::with_single_selection("🐶",
    ///                                               "Enable Dog Mode",
    ///                                               Layout::new("x:3,y:1,w:2"),
    ///                                               true,
    ///                                               togglebutton::Type::Normal);
    /// ```
    pub fn with_single_selection(caption: &str, tooltip: &str, layout: Layout, selected: bool, button_type: Type) -> Self {
        Self::inner_new(caption, tooltip, layout, selected, button_type, true)
    }
    fn inner_new(caption: &str, tooltip: &str, layout: Layout, selected: bool, button_type: Type, single_selection: bool) -> Self {
        let mut but = ToggleButton {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            caption: FlatString::from_str(caption),
            tooltip: tooltip.to_string(),
            state: selected,
            button_type,
            single_selection,
        };

        match button_type {
            Type::Normal => but.set_size_bounds(1, 1, u16::MAX, 1),
            Type::Underlined => but.set_size_bounds(1, 2, u16::MAX, 2),
        }
        but
    }
    /// Sets the caption of a toggle button.
    pub fn set_caption(&mut self, caption: &str) {
        self.caption.set(caption);
    }
    /// Returns the toggle button caption.
    pub fn caption(&self) -> &str {
        self.caption.as_str()
    }
    /// Returns the state of the toggle button (pressed or not)
    pub fn is_selected(&self) -> bool {
        self.state
    }
    /// Sets the state of the toggle button (pressed or not).
    /// 
    /// If the button is part of a group of buttons, only one button can be selected at a time. As such, calling this function in this case with a `false` value will have no effect.
    pub fn set_selected(&mut self, selected: bool) {
        if self.single_selection {
            if (!selected) || self.handle.is_none() {
                return;
            }
            self.state = true;
            if let Some(parent) = RuntimeManager::get().get_controls_mut().get(self.parent) {
                parent.base().notify_children_of_selection(self.handle);
            }
        } else {
            self.state = selected;
        }
    }
}
impl OnDefaultAction for ToggleButton {
    fn on_default_action(&mut self) {
        if self.single_selection {
            self.set_selected(true);
        } else {
            self.set_selected(!self.state);
        }
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::ToggleButton(EventData { status: self.state }),
        });
    }
}
impl OnKeyPressed for ToggleButton {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Space") | key!("Enter") => {
                self.on_default_action();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnPaint for ToggleButton {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let state_cols = if self.state {
            &theme.toggle_button.selected
        } else {
            &theme.toggle_button.unselected
        };
        let col_text = match () {
            _ if !self.is_enabled() => state_cols.inactive,
            _ if self.has_focus() => state_cols.focused,
            _ if self.is_mouse_over() => state_cols.hovered,
            _ => state_cols.normal,
        };
        let w = self.size().width;
        let format = TextFormatBuilder::new()
            .position((w / 2) as i32, 0)
            .attribute(col_text)
            .align(TextAlignament::Center)
            .chars_count(self.caption.chars_count() as u16)
            .wrap_type(WrapType::SingleLineWrap(w as u16))
            .build();

        surface.fill_horizontal_line_with_size(0, 0, w, Character::with_attributes(' ', col_text));
        surface.write_text(self.caption.as_str(), &format);
        if self.button_type == Type::Underlined {
            if self.state {
                surface.fill_horizontal_line(
                    0,
                    1,
                    self.size().width as i32,
                    Character::with_attributes(SpecialChar::LineOnTop, state_cols.normal),
                );
            } else {
                surface.fill_horizontal_line(0, 1, self.size().width as i32, Character::with_attributes(' ', state_cols.normal));
            }
        }
    }
}
impl OnMouseEvent for ToggleButton {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => {
                self.show_tooltip(&self.tooltip);
                EventProcessStatus::Processed
            }
            MouseEvent::Leave => EventProcessStatus::Processed,
            MouseEvent::Pressed(_) => {
                self.on_default_action();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
impl OnSiblingSelected for ToggleButton {
    #[allow(private_interfaces)]
    fn on_sibling_selected(&mut self, handle: Handle<()>) {
        if self.single_selection {
            self.state = self.handle == handle;
        }
    }
}
