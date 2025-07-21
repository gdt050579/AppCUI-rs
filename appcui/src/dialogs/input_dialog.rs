use super::InputCallback;
use crate::prelude::*;
use std::str::FromStr;

#[ModalWindow(internal: true, response: T, events: ButtonEvents+TextFieldEvents)]
pub(super) struct StringImputDialog<T>
where
    T: FromStr + Sized + std::fmt::Display + 'static,
{
    validation: Option<InputCallback<T>>,
    txt: Handle<TextField>,
    btn_ok: Handle<Button>,
}

impl<T> StringImputDialog<T>
where
    T: FromStr + Sized + std::fmt::Display + 'static,
{
    pub(super) fn new(title: &str, text: &str, value: Option<T>, validation: Option<InputCallback<T>>) -> Self {
        let chars_count = text.chars().count();
        let height = ((chars_count / 36) + 1).clamp(1, 6) as u32;
        let layout = LayoutBuilder::new().alignment(Alignment::Center).width(40).height(height + 8).build();
        let mut me = Self {
            base: ModalWindow::new(title, layout, window::Flags::NoCloseButton),
            txt: Handle::None,
            btn_ok: Handle::None,
            validation,
        };
        me.add(Label::new(text, layout!("l:1,t:1,r:1,b:5")));
        me.btn_ok = me.add(Button::new("&Ok", layout!("l:5,b:0,w:13"), button::Type::Normal));
        me.add(Button::new("&Cancel", layout!("l:21,b:0,w:13"), button::Type::Normal));
        let content = if let Some(value) = value {
            format! {"{value}"}
        } else {
            String::new()
        };
        me.txt = me.add(TextField::new(&content, layout!("l:1,r:1,b:3,h:1"), textfield::Flags::ProcessEnter));
        me
    }
    fn validate(&mut self) {
        if let Some(tf) = self.control(self.txt) {
            let text = tf.text();
            let result = if let Ok(value) = T::from_str(text) {
                if let Some(validation) = self.validation {
                    validation(&value).map(|()| value)
                } else {
                    // all good
                    Ok(value)
                }
            } else {
                let msg = format!("Invalid value: '{text}'");
                Err(msg)
            };
            match result {
                Ok(value) => self.exit_with(value),
                Err(err) => dialogs::error("Error", err.as_str()),
            }
        }
    }
}

impl<T> ButtonEvents for StringImputDialog<T>
where
    T: FromStr + Sized + std::fmt::Display + 'static,
{
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.btn_ok {
            self.validate();
        } else {
            self.exit();
        }
        EventProcessStatus::Processed
    }
}

impl<T> TextFieldEvents for StringImputDialog<T>
where
    T: FromStr + Sized + std::fmt::Display + 'static,
{
    fn on_validate(&mut self, _: Handle<TextField>, _: &str) -> EventProcessStatus {
        self.validate();
        EventProcessStatus::Processed
    }
}
