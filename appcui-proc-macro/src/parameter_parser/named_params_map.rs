use super::{value::Value, Error, ParamSignature};
use std::collections::HashMap;

pub(crate) struct NamedParamsMap<'a> {
    pub(super) named: HashMap<u64, u32>,
    pub(super) ordered: Vec<Value<'a>>,
    pub(super) non_named_count: u32,
}

impl<'a> NamedParamsMap<'a> {
    pub(crate) fn validate_signature(&mut self, param_list: &str, signature: &[ParamSignature]) -> Result<(), Error> {
        Ok(())
    }
    pub(crate) fn check_unkwnon_params(&self, param_list: &str) -> Result<(), Error> {
        // all values must be validated
        for value in &self.ordered {
            if value.validated == false {
                return Err(Error::new(
                    param_list,
                    format!("Unknwon key: '{}' !", value.raw_data).as_str(),
                    value.start,
                    value.end,
                ));
            }
        }
        Ok(())
    }
}
