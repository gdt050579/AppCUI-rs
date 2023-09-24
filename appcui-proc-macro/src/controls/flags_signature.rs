use std::collections::HashMap;

pub(super) struct FlagsSignature {
    flags: &'static [&'static str],
    map: Option<HashMap<u32, &'static str>>,
}
impl FlagsSignature {
    pub(super) const fn new(flags: &'static [&'static str]) -> Self {
        Self { flags, map: None }
    }
    pub(super) fn get(&self, name: &str)->Option<&'static str> {
        None
    }
}
