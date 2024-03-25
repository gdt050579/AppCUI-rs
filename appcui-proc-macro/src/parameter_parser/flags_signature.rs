use std::{collections::HashMap, sync::Mutex};

use crate::utils;

pub(crate) struct FlagsSignature {
    flags: &'static [&'static str],
    map: Mutex<Option<HashMap<u64, &'static str>>>,
}
impl FlagsSignature {
    pub(crate) const fn new(flags: &'static [&'static str]) -> Self {
        Self { flags, map: Mutex::new(None) }
    }
    pub(crate) fn get(&mut self, name: &str)->Option<&'static str> {
        let hash = utils::compute_hash(name);
        let map = self.map.get_mut().unwrap();
        if map.is_none() {
            // build the actual map
            let mut m: HashMap<u64, &'static str> = HashMap::with_capacity(8);
            for elem in self.flags {
                m.insert(utils::compute_hash(elem), *elem);
            }
            *map = Some(m);
        }
        if let Some(m) = map {
            return m.get(&hash).copied();
        }
        None
    }
}
