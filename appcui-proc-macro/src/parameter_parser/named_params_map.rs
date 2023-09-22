use std::collections::HashMap;

use super::value::Value;


pub(crate) struct NamedParamsMap<'a> {
    pub (super) named: HashMap<u64, Value<'a>>,
    pub (super) ordered: Vec<Value<'a>>,
}