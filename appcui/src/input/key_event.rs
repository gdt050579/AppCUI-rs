use super::Key;
use super::KeyCode;
use super::KeyModifier;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct KeyPressed {
    pub key: Key,
    pub character: char,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct KeyModifierChanged {
    pub new_state: KeyModifier,
    pub old_state: KeyModifier,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum KeyEvent {
    KeyPressed(KeyPressed),
    KeyModifierChanged(KeyModifierChanged),
}
impl KeyEvent {
    pub(crate) fn new_key_pressed(
        code: KeyCode,
        modifier: KeyModifier,
        character: char,
    ) -> KeyEvent {
        KeyEvent::KeyPressed(KeyPressed {
            key: Key::new(code, modifier),
            character,
        })
    }
    pub(crate) fn new_modifier_changed(old_state: KeyModifier, new_state: KeyModifier) -> KeyEvent {
        KeyEvent::KeyModifierChanged(KeyModifierChanged {
            new_state,
            old_state,
        })
    }
}
