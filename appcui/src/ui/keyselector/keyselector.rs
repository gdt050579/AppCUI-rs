use crate::prelude::*;
use crate::ui::keyselector::{events::EventData, Flags};

#[CustomControl(overwrite=OnPaint+OnKeyPressed, internal=true)]
pub struct KeySelector {
    flags: Flags,
    key: Key
}
impl KeySelector {
    fn new()->Self {
        Self {
            base: todo!(),
            flags: todo!(),
            key: todo!(),
        }
    }
}
impl OnPaint for KeySelector {

}
impl OnKeyPressed for KeySelector {

}