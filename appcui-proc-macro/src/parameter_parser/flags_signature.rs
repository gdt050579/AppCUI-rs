use std::{collections::HashMap, sync::OnceLock};

use crate::utils;

pub(crate) struct FlagsSignature {
    flags: &'static [&'static str],
    map: OnceLock<HashMap<u64, &'static str>>,
}
impl FlagsSignature {
    pub(crate) fn len(&self) -> usize {
        self.flags.len()
    }
    pub(crate) fn flag_name(&self, index: usize) -> &'static str {
        if index < self.flags.len() {
            self.flags[index]
        } else {
            panic!("Index out of bounds: the length is {} but the index is {}", self.flags.len(), index);
        }
    }
    pub(crate) const fn new(flags: &'static [&'static str]) -> Self {
        Self { flags, map: OnceLock::new() }
    }
    pub(crate) fn get(&self, name: &str)->Option<&'static str> {
        let map = self.map.get_or_init(||{
            let mut m: HashMap<u64, &'static str> = HashMap::with_capacity(8);
            for elem in self.flags {
                m.insert(utils::compute_hash(elem), *elem);
            }
            m
        });
        let hash = utils::compute_hash(name);
        map.get(&hash).copied()
    }
    pub(crate) fn list(&self) -> String {
        let mut l = String::with_capacity(64);
        let mut v: Vec<&'static str> = Vec::with_capacity(self.flags.len());
        for f in self.flags {
            v.push(*f);
        }
        v.sort();
        for item in v {
            if !l.is_empty() {
                l.push_str(", ");
            }
            l.push_str(item);
        }
        l
    }
}
