use super::{coordonate::Coordonate, dimension::Dimension, size::Size, value::Value, Error, NamedParameter, PositionalParameter};
use std::collections::HashMap;

pub(crate) struct NamedParamsMap<'a> {
    pub(super) named: HashMap<u64, u32>,
    pub(super) values: Vec<Value<'a>>,
    pub(super) all_params: HashMap<u64, u32>,
    pub(super) positional_count: usize,
}

impl<'a> NamedParamsMap<'a> {
    pub(crate) fn empty() -> Self {
        NamedParamsMap {
            named: HashMap::new(),
            values: Vec::new(),
            all_params: HashMap::new(),
            positional_count: 0,
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
        //for index in 0..self.positional_count {
        for (index, item) in params.iter().enumerate().take(self.positional_count) {
            let h = crate::utils::compute_hash(item.get_key());
            if self.all_params.contains_key(&h) {
                return Err(Error::new(
                    param_list,
                    format!(
                        "Positional parameter with index {} is duplicated. Check for '`{}`' or one of its aliases in the parameters lists",
                        index + 1,
                        item.get_key()
                    )
                    .as_str(),
                    self.values[index].start,
                    self.values[index].end,
                ));
            }
            // validate
            let v = &mut self.values[index];
            v.validate(param_list, item.get_key(), item.get_param_type())?;
            // all good -> not use somewhere else --> add it to map
            self.named.insert(h, index as u32);
        }
        Ok(())
    }
    pub(crate) fn validate_named_parameters(&mut self, param_list: &str, signature: &[NamedParameter]) -> Result<(), Error> {
        // start validating parameters from signature
        for param_sig in signature {
            let h = crate::utils::compute_hash(param_sig.get_name());
            if let Some(index) = self.named.get(&h) {
                // if parameter with name "..." is present and has an index
                let k = crate::utils::compute_hash(param_sig.get_key());
                if self.all_params.contains_key(&k) {
                    // this means that two aliases were present
                    // two posible errors
                    // 1. a positional parameter and a named one
                    // 2. two named parameters
                    let other_parameter_index = (*(self.all_params.get(&k).unwrap())) as usize;
                    let other_parameter_name = &self.values[other_parameter_index].param_name;
                    let v = &self.values[*index as usize];
                    if !other_parameter_name.is_empty() {
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
    pub(crate) fn check_unkwnon_params(
        &self,
        param_list: &str,
        positional_parameters: &[PositionalParameter],
        named_parameters: &[NamedParameter],
        control_parameters: Option<&[NamedParameter]>,
    ) -> Result<(), Error> {
        // all values must be validated
        for value in &self.values {
            if !value.validated {
                // make a list with all available parameters
                let mut s = String::with_capacity(256);
                let mut m: HashMap<String, bool> = HashMap::with_capacity(32);
                for param in positional_parameters {
                    m.insert(param.get_key().to_string(), true);
                }
                for param in named_parameters {
                    m.insert(param.get_key().to_string(), true);
                }
                if let Some(cp) = control_parameters {
                    for param in cp {
                        m.insert(param.get_key().to_string(), true);
                    }
                }
                for k in m.keys() {
                    if !s.is_empty() {
                        s.push_str(", ");
                    }
                    s.push_str(k.as_str());
                }
                return Err(Error::new(
                    param_list,
                    format!(
                        "Unknwon parameter: '{}' !\nHere is a list of all available parameters: {}",
                        value.param_name, &s
                    )
                    .as_str(),
                    value.start,
                    value.end,
                ));
            }
        }
        Ok(())
    }
    pub(crate) fn get(&self, name: &str) -> Option<&Value<'a>> {
        let k = crate::utils::compute_hash(name);
        if let Some(index) = self.all_params.get(&k) {
            return Some(&self.values[*index as usize]);
        }
        None
    }
    pub(crate) fn contains(&self, name: &str) -> bool {
        let k = crate::utils::compute_hash(name);
        self.all_params.contains_key(&k)
    }
    pub(crate) fn get_mut(&mut self, name: &str) -> Option<&mut Value<'a>> {
        let k = crate::utils::compute_hash(name);
        if let Some(index) = self.all_params.get(&k) {
            return Some(&mut self.values[*index as usize]);
        }
        None
    }
    pub(crate) fn get_bool(&mut self, name: &str) -> Option<bool> {
        self.get_mut(name)?.get_bool()
    }
    pub(crate) fn get_list(&mut self, name: &str) -> Option<&mut Vec<Value<'a>>> {
        self.get_mut(name)?.get_list()
    }
    pub(crate) fn get_size(&mut self, name: &str) -> Option<Size> {
        self.get_mut(name)?.get_size()
    }
    pub(crate) fn get_coordonate(&mut self, name: &str) -> Option<Coordonate> {
        self.get_mut(name)?.get_coordonate()
    }
    pub(crate) fn get_dimension(&mut self, name: &str) -> Option<Dimension> {
        self.get_mut(name)?.get_dimension()
    }
    pub(crate) fn get_parameters_count(&self) -> usize {
        self.all_params.len()
    }
}
