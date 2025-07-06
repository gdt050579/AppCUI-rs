use crate::prelude::*;

#[ModalWindow(internal: true, response: T)]
pub(super) struct StringImputDialog<T>
where
    T: for<'a> From<&'a str> + Sized + 'static,
{
    validation: Option<fn(T) -> Result<T, String>>,
    txt: Handle<TextField>,
    btn_ok: Handle<Button>,
}

impl<T> StringImputDialog<T>
where
    T: for<'a> From<&'a str> + Sized + 'static,
{
    pub(super) fn new(title: &str, text: &str, value: Option<T>, validation: Option<fn(T) -> Result<T, String>>) -> Self {
        todo!()
    }
}
