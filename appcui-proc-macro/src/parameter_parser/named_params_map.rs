use super::{value::Value, Error, ParamSignature};
use std::collections::HashMap;

pub(crate) struct NamedParamsMap<'a> {
    pub(super) named: HashMap<u64, u32>,
    pub(super) positional: Vec<Value<'a>>,
    pub(super) positional_count: usize,
}

impl<'a> NamedParamsMap<'a> {
    fn validate_positional_parameters(&mut self, param_list: &str, signature: &[ParamSignature]) -> Result<(), Error> {
        // first count how many positional parameters are defined in signature
        let len = signature.len();
        let mut pos_count = 0usize;
        while (pos_count < len) && (signature[pos_count].is_mandatory()) {
            pos_count += 1;
        }
        if self.positional_count > pos_count {
            return Err(Error::new(
                param_list,
                format!("Too many positional parameters (max allowd are {})", pos_count).as_str(),
                self.positional[pos_count].start,
                self.positional[self.positional_count - 1].end,
            ));
        }
        // lets validate that they are in the right order
        for index in 0..self.positional_count {
            let h = super::utils::compute_hash(signature[index].get_key());
            if self.named.contains_key(&h) {
                return Err(Error::new(
                    param_list,
                    format!(
                        "Positional parameter with index {} is duplicated. Check for '`{}`' or one of its aliases in the parameters lists",
                        index,
                        signature[index].get_name()
                    )
                    .as_str(),
                    self.positional[index].start,
                    self.positional[index].end,
                ));
            }
            // all good -> not use somewhere else --> add it to map
            self.named.insert(h, index as u32);
        }
        // all good , all positiona parameters computed
        self.positional_count = 0;
        Ok(())
    }
    pub(crate) fn validate_signature(&mut self, param_list: &str, signature: &[ParamSignature]) -> Result<(), Error> {
        if self.positional_count > 0 {
            self.validate_positional_parameters(param_list, signature)?;
        }
        // start validating parameters from signature
        for param_sig in signature {
            let h = super::utils::compute_hash(param_sig.get_key());
            if let Some(index) = self.named.get(&h) {
                let v = &mut self.positional[*index as usize];
                v.validate(param_list, param_sig.get_key(), param_sig.get_param_type())?;
            }
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
