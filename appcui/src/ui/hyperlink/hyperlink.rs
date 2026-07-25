use crate::prelude::*;
use crate::ui::hyperlink::events::EventData;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct HyperLink {
    url: String,
    tooltip: String,
    name: String,
}
impl HyperLink {
    /// Creates a hyperlink that displays `name` and points to `url`.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let link = HyperLink::new("AppCUI-rs", "https://github.com/gdt050579/AppCUI-rs", layout!("x:1,y:1,w:10"));
    /// ```
    pub fn new(name: &str, url: &str, layout: Layout) -> Self {
        Self::inner_create(name, url, "", layout, StatusFlags::None)
    }

    /// Creates a hyperlink where the displayed text is the URL itself.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let link = HyperLink::with_url("https://github.com/gdt050579/AppCUI-rs", layout!("x:1,y:1,w:40"));
    /// ```
    pub fn with_url(url: &str, layout: Layout) -> Self {
        Self::inner_create("", url, "", layout, StatusFlags::None)
    }

    /// Creates a hyperlink that displays `name`, points to `url` and shows `tooltip` when the mouse hovers over it.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let link = HyperLink::with_tooltip(
    ///     "AppCUI-rs",
    ///     "https://github.com/gdt050579/AppCUI-rs",
    ///     "A cross-platform TUI framework for Rust",
    ///     layout!("x:1,y:1,w:10"),
    /// );
    /// ```
    pub fn with_tooltip(name: &str, url: &str, tooltip: &str, layout: Layout) -> Self {
        Self::inner_create(name, url, tooltip, layout, StatusFlags::None)
    }

    fn inner_create(name: &str, url: &str, tooltip: &str, layout: Layout, status: StatusFlags) -> Self {
        let mut hyper_link = HyperLink {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput | status),
            url: url.to_string(),
            tooltip: tooltip.to_string(),
            name: name.to_string(),
        };
        hyper_link.set_size_bounds(1, 1, u16::MAX, 1);
        hyper_link
    }

    /// Sets the URL associated with this hyperlink.
    /// The displayed text is not affected, unless no name was set (in which case the URL itself is displayed).
    pub fn set_url(&mut self, url: &str) {
        self.url.clear();
        self.url.push_str(url);
    }

    /// Returns the URL associated with this hyperlink.
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Sets the tooltip shown when the mouse hovers over this hyperlink.
    pub fn set_tooltip(&mut self, tooltip: &str) {
        self.tooltip.clear();
        self.tooltip.push_str(tooltip);
    }

    /// Returns the tooltip of this hyperlink or an empty string if none was set.
    pub fn tooltip(&self) -> &str {
        self.tooltip.as_str()
    }

    /// Sets the text displayed by this hyperlink.
    /// If set to an empty string, the URL itself will be displayed instead.
    pub fn set_name(&mut self, name: &str) {
        self.name.clear();
        self.name.push_str(name);
    }

    /// Returns the text explicitly set for this hyperlink or an empty string if none was set.
    /// Note that when no name is set, the control displays the URL itself.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl OnDefaultAction for HyperLink {
    fn on_default_action(&mut self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::HyperLink(EventData),
        });
    }
}
impl OnKeyPressed for HyperLink {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Enter") => {
                self.on_default_action();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnPaint for HyperLink {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let col_text = match () {
            _ if !self.is_enabled() => theme.text.inactive,
            _ if self.has_focus() => theme.text.focused,
            _ if self.is_mouse_over() => theme.text.hovered,
            _ => theme.text.normal,
        };

        let name = if self.name.trim().is_empty() { &self.url } else { &self.name };
        let attr = if self.is_enabled() && self.is_mouse_over() {
            CharAttribute::new(col_text.foreground, col_text.background, CharFlags::Underline | col_text.flags)
        } else {
            col_text
        };

        let w = self.size().width;
        let format = TextFormatBuilder::new()
            .position(0, 0)
            .attribute(attr)
            .align(TextAlignment::Left)
            .chars_count(name.chars().count() as u16)
            .wrap_type(WrapType::SingleLineWrap(w as u16))
            .build();
        surface.write_text(name, &format);
    }
}
impl OnMouseEvent for HyperLink {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => {
                if !self.tooltip.trim().is_empty() {
                    self.show_tooltip(&self.tooltip);
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Leave => {
                self.hide_tooltip();
                EventProcessStatus::Processed
            }
            MouseEvent::Released(_) => {
                self.on_default_action();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
