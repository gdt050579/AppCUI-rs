use crate::prelude::*;
use crate::ui::hyperlink::events::EventData;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct HyperLink {
    link: String,
    desc: String,
    name: String,
}
impl HyperLink {
    /// Creates a hyperlink where the displayed text is the link itself.
    pub fn new(link: &str, layout: Layout) -> Self {
        Self::inner_create(link, "", "", layout, StatusFlags::ThemeType)
    }

    /// Creates a hyperlink with a custom display name.
    pub fn with_name(link: &str, name: &str, layout: Layout) -> Self {
        Self::inner_create(link, name, "", layout, StatusFlags::ThemeType)
    }

    /// Creates a hyperlink with a description (shown as a tooltip).
    pub fn with_desc(link: &str, desc: &str, layout: Layout) -> Self {
        Self::inner_create(link, "", desc, layout, StatusFlags::None)
    }

    /// Creates a hyperlink with both a custom display name and a description.
    pub fn with_name_and_desc(link: &str, name: &str, desc: &str, layout: Layout) -> Self {
        Self::inner_create(link, name, desc, layout, StatusFlags::None)
    }

    fn inner_create(link: &str, name: &str, desc: &str, layout: Layout, status: StatusFlags) -> Self {
        let mut hyper_link = HyperLink {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput | status),
            link: link.to_string(),
            desc: desc.to_string(),
            name: name.to_string(),
        };
        hyper_link.set_size_bounds(1, 1, u16::MAX, 1);
        hyper_link
    }

    fn paint_normal(&self, surface: &mut Surface, theme: &Theme) {
        let col_text = match () {
            _ if !self.is_enabled() => theme.text.inactive,
            _ if self.has_focus() => theme.text.focused,
            _ if self.is_mouse_over() => theme.text.hovered,
            _ => theme.button.regular.text.normal,
        };

        let name = if self.name.trim().is_empty() { &self.link } else { &self.name };
        let attr = if self.is_enabled() && (self.is_mouse_over() || self.has_focus()) {
            CharAttribute::new(col_text.foreground, col_text.background, CharFlags::Underline | col_text.flags)
        } else {
            col_text
        };

        let w = self.size().width;
        let format = TextFormatBuilder::new()
            .position(0, 0)
            .attribute(col_text)
            .align(TextAlignment::Left)
            .chars_count(name.chars().count() as u16)
            .wrap_type(WrapType::SingleLineWrap(w as u16))
            .build();
        surface.write_text(&name, &format);
    }

    /// Sets the URL associated with this hyperlink.
    /// The displayed text is not affected, unless no name was set (in which case the link itself is displayed).
    pub fn set_link(&mut self, link: &str) {
        self.link = link.to_string();
    }

    /// Returns the URL associated with this hyperlink.
    pub fn link(&self) -> &str {
        self.link.as_str()
    }

    /// Sets the description of this hyperlink.
    pub fn set_desc(&mut self, desc: &str) {
        self.desc = desc.to_string();
    }

    /// Returns the description of this hyperlink or an empty string if none was set.
    pub fn desc(&self) -> &str {
        self.desc.as_str()
    }

    /// Sets the text displayed by this hyperlink.
    /// If set to an empty string, the link itself will be displayed instead.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Returns the text explicitly set for this hyperlink or an empty string if none was set.
    /// Note that when no name is set, the control displays the link itself.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl OnDefaultAction for HyperLink {
    fn on_default_action(&mut self) {
        // self.raise_event(ControlEvent {
        //     emitter: self.handle,
        //     receiver: self.event_processor,
        //     data: ControlEventData::HyperLink(EventData),
        // });
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
        self.paint_normal(surface, theme);
    }
}
impl OnMouseEvent for HyperLink {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => {
                if !self.desc().trim().is_empty() {
                    self.show_tooltip(&self.desc);
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
