use crate::prelude::*;

#[ModalWindow(internal: true, response: T, events: ButtonEvents+TextFieldEvents)]
pub(super) struct StringImputDialog<T>
where
    T: for<'a> From<&'a str> + Sized + std::fmt::Display + 'static,
{
    validation: Option<fn(T) -> Result<T, String>>,
    txt: Handle<TextField>,
    btn_ok: Handle<Button>,
}

impl<T> StringImputDialog<T>
where
    T: for<'a> From<&'a str> + Sized + std::fmt::Display + 'static,
{
    pub(super) fn new(title: &str, text: &str, value: Option<T>, validation: Option<fn(T) -> Result<T, String>>) -> Self {
        let chars_count = text.chars().count();
        let height = ((chars_count / 36) + 1).clamp(1, 6);
        let format_str = format!("d:c,w:40,h:{}", height + 8);
        let layout = Layout::new(format_str.as_str());
        let mut me = Self {
            base: ModalWindow::new(title, layout, window::Flags::NoCloseButton),
            txt: Handle::None,
            btn_ok: Handle::None,
            validation,
        };
        me.add(Label::new(text, Layout::new("l:1,t:1,r:1,b:6")));
        me.btn_ok = me.add(Button::new("&Ok", Layout::new("x:12,a:b,w:12"), button::Type::Normal));
        me.add(Button::new("&Cancel", Layout::new("x:28,a:b,w:12"), button::Type::Normal));
        let content = if let Some(value) = value {
            format! {"{}",value}
        } else {
            String::new()
        };
        me.txt = me.add(TextField::new(&content, Layout::new("l:1,r:1,b:4,h:1"), textfield::Flags::ProcessEnter));
        me
    }
    fn validate(&self) {}
}

impl<T> ButtonEvents for StringImputDialog<T>
where
    T: for<'a> From<&'a str> + Sized + std::fmt::Display + 'static,
{
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.btn_ok {
            self.validate();
        } else {
            self.close();
        }
        EventProcessStatus::Processed
    }
}

impl<T> TextFieldEvents for StringImputDialog<T>
where
    T: for<'a> From<&'a str> + Sized + std::fmt::Display + 'static,
{
    fn on_validate(&mut self, _: Handle<TextField>, _: &str) -> EventProcessStatus {
        self.validate();
        EventProcessStatus::Processed
    }
}
