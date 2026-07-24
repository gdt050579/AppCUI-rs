use crate::prelude::*;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct HyperLink {
    link: String,
    desc: String,
    name: String,
}
impl HyperLink {
    pub fn new(link: &str, name: &str, desc: &str, layout: Layout) -> Self {
        Self::inner_create(link, name, desc, layout, StatusFlags::ThemeType)
    }

    fn inner_create(link: &str, name: &str, desc: &str, layout: Layout, status: StatusFlags) -> Self {
        let hyper_link = HyperLink {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput | status),
            link: link.to_string(),
            desc: desc.to_string(),
            name: name.to_string(),
        };
        hyper_link
    }

    fn paint_normal(&self, surface: &mut Surface, theme: &Theme) {
        let col_text = match () {
            _ if !self.is_enabled() => theme.button.regular.text.inactive,
            _ if self.has_focus() => theme.button.regular.text.focused,
            _ if self.is_mouse_over() => theme.button.regular.text.hovered,
            _ => theme.button.regular.text.normal,
        };


        let w = self.size().width;
        let format = TextFormatBuilder::new()
            .position(0, 0)
            .attribute(col_text)
            .align(TextAlignment::Left)
            .chars_count(self.name.chars().count() as u16)
            .wrap_type(WrapType::SingleLineWrap(w as u16))
            .build();
        surface.write_text(&self.name, &format);
    }
}

impl OnDefaultAction for HyperLink {
    fn on_default_action(&mut self) {
        // self.raise_event(ControlEvent {
        //     emitter: self.handle,
        //     receiver: self.event_processor,
        //     data: ControlEventData::HyperLink(EventData {}),
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
            MouseEvent::Enter | MouseEvent::Leave => EventProcessStatus::Processed,
            MouseEvent::Released(_) => {
                self.on_default_action();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
