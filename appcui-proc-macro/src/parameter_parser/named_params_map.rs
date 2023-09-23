use super::{value::Value, Error, ParamSignature};
use std::collections::HashMap;

pub(crate) struct NamedParamsMap<'a> {
    pub(super) named: HashMap<u64, u32>,
    pub(super) positional: Vec<Value<'a>>,
    pub(super) positional_count: u32,
}

impl<'a> NamedParamsMap<'a> {
    fn validate_positional_parameters(&mut self, param_list: &str, signature: &[ParamSignature]) -> Result<(), Error> {
        
        Ok(())
    }
    pub(crate) fn validate_signature(&mut self, param_list: &str, signature: &[ParamSignature]) -> Result<(), Error> {
        if self.positional_count > 0 {
            self.validate_positional_parameters(param_list, signature)?;
        }
        Ok(())
    }
    pub(crate) fn check_unkwnon_params(&self, param_list: &str) -> Result<(), Error> {
        // all values must be validated
        for value in &self.positional {
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
