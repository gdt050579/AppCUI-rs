use std::collections::HashMap;
use super::{value::Value, ParamSignature};

pub(crate) struct NamedParamsMap<'a> {
    pub (super) named: HashMap<u64, u32>,
    pub (super) ordered: Vec<Value<'a>>,
    pub (super) non_named_count: u32,
}

impl<'a> NamedParamsMap<'a> {
    pub (crate) fn validate_signature(&mut self, signature: &[ParamSignature]) {

    }
    pub (crate) fn check_unkwnon_params(&self) {

    }
}