use super::{value::Value, Error, NamedParameter, PositionalParameter, color::Color};
use std::collections::HashMap;

pub(crate) struct NamedParamsMap<'a> {
    pub(super) named: HashMap<u64, u32>,
    pub(super) values: Vec<Value<'a>>,
    pub(super) all_params: HashMap<u64, u32>,
    pub(super) positional_count: usize,
}

impl<'a> NamedParamsMap<'a> {
    pub(crate) fn empty()->Self {
        NamedParamsMap {
            named: HashMap::new(),
            values: Vec::new(),
            all_params: HashMap::new(),
            positional_count: 0
        }
    }
    pub(crate) fn validate_positional_parameters(&mut self, param_list: &str, params: &[PositionalParameter]) -> Result<(), Error> {
        if self.positional_count > params.len() {
            return Err(Error::new(
                param_list,
                format!("Too many positional parameters (max allowd are {})", params.len()).as_str(),
                self.values[params.len()].start,
                self.values[self.positional_count - 1].end,
            ));
        }
        // lets validate that they are in the right order
        for index in 0..self.positional_count {
            let h = super::utils::compute_hash(params[index].get_key());
            if self.all_params.contains_key(&h) {
                return Err(Error::new(
                    param_list,
                    format!(
                        "Positional parameter with index {} is duplicated. Check for '`{}`' or one of its aliases in the parameters lists",
                        index + 1,
                        params[index].get_key()
                    )
                    .as_str(),
                    self.values[index].start,
                    self.values[index].end,
                ));
            }
            // validate
            let v = &mut self.values[index];
            v.validate(param_list, params[index].get_key(), params[index].get_param_type())?;
            // all good -> not use somewhere else --> add it to map
            self.named.insert(h, index as u32);
        }
        Ok(())
    }
    pub(crate) fn validate_names_parameters(&mut self, param_list: &str, signature: &[NamedParameter]) -> Result<(), Error> {
        // start validating parameters from signature
        for param_sig in signature {
            let h = super::utils::compute_hash(param_sig.get_name());
            if let Some(index) = self.named.get(&h) {
                // if parameter with name "..." is present and has an index
                let k = super::utils::compute_hash(param_sig.get_key());
                if self.all_params.contains_key(&k) {
                    // this means that two aliases were present
                    // two posible errors
                    // 1. a positional parameter and a named one
                    // 2. two named parameters
                    let other_parameter_index = (*(self.all_params.get(&k).unwrap())) as usize;
                    let other_parameter_name = &self.values[other_parameter_index].param_name;
                    let v = &self.values[*index as usize];
                    if other_parameter_name.len() > 0 {
                        return Err(Error::new(
                            param_list,
                            format!(
                                "Parameters '{}' and '{}' are aliases and can not be used at the same time. Keep only one of them !",
                                v.param_name, other_parameter_name
                            )
                            .as_str(),
                            self.values[other_parameter_index].start,
                            self.values[other_parameter_index].end,
                        ));
                    } else {
                        return Err(Error::new(
                            param_list,
                            format!(
                                "Parameter '{}' its the same with by the parameter with index '{}`. Keep only one of them !",
                                v.param_name, self.all_params[&k]
                            )
                            .as_str(),
                            v.start,
                            v.end,
                        ));
                    }
                }
                let v = &mut self.values[*index as usize];
                v.validate(param_list, param_sig.get_key(), param_sig.get_param_type())?;
                // since it was already validated --> add the key to map
                self.all_params.insert(k, *index);
            }
        }
        Ok(())
    }
    pub(crate) fn check_unkwnon_params(&self, param_list: &str) -> Result<(), Error> {
        // all values must be validated
        for value in &self.values {
            if value.validated == false {
                return Err(Error::new(
                    param_list,
                    format!("Unknwon parameter: '{}' !", value.param_name).as_str(),
                    value.start,
                    value.end,
                ));
            }
        }
        Ok(())
    }
    pub(crate) fn get(&self, name: &str) -> Option<&Value<'a>> {
        let k = super::utils::compute_hash(name);
        if let Some(index) = self.all_params.get(&k) {
            return Some(&self.values[*index as usize]);
        }
        None
    }
    pub(crate) fn contains(&self, name: &str) -> bool {
        let k = super::utils::compute_hash(name);
        self.all_params.contains_key(&k)
    }
    pub(crate) fn get_mut(&mut self, name: &str) -> Option<&mut Value<'a>> {
        let k = super::utils::compute_hash(name);
        if let Some(index) = self.all_params.get(&k) {
            return Some(&mut self.values[*index as usize]);
        }
        None
    }
    pub(crate) fn get_bool(&mut self, name: &str) -> Option<bool> {
        self.get_mut(name)?.get_bool()
    }
}
