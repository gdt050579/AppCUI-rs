use super::named_params_map::NamedParamsMap;
use super::{utils, Error};

pub(super) enum ValueType<'a> {
    Undetermined,
    Bool(bool),
    Integer(i32),
    Percentage(f32),
    List(Vec<Value<'a>>),
    Dict(NamedParamsMap<'a>),
}

pub(crate) struct Value<'a> {
    pub(super) param_name: &'a str,
    pub(super) raw_data: &'a str,
    pub(super) data_type: ValueType<'a>,
    pub(super) validated: bool,
    pub(super) start: usize,
    pub(super) end: usize,
}
impl<'a> Value<'a> {
    pub(crate) fn is_dict(&self) -> bool {
        match self.data_type {
            ValueType::Dict(_) => true,
            _ => false,
        }
    }
    pub(crate) fn is_list(&self) -> bool {
        match self.data_type {
            ValueType::List(_) => true,
            _ => false,
        }
    }
    pub(crate) fn is_value(&self) -> bool {
        match self.data_type {
            ValueType::List(_) | ValueType::Dict(_) => false,
            _ => true,
        }
    }
    pub(crate) fn get_bool(&mut self) -> Option<bool> {
        if !self.is_value() {
            return None;
        }
        if let ValueType::Bool(value) = &self.data_type {
            return Some(*value);
        }
        if utils::equal_ignore_case("true", self.raw_data) {
            self.data_type = ValueType::Bool(true);
            return Some(true);
        }
        if utils::equal_ignore_case("false", self.raw_data) {
            self.data_type = ValueType::Bool(false);
            return Some(false);
        }
        None
    }
    pub(crate) fn get_i32(&mut self) -> Option<i32> {
        if !self.is_value() {
            return None;
        }
        if let ValueType::Integer(value) = &self.data_type {
            return Some(*value);
        }
        if let Some(value) = utils::to_i32(self.raw_data) {
            self.data_type = ValueType::Integer(value);
            return Some(value);
        }
        None
    }
    pub(crate) fn get_percentage(&mut self) -> Option<f32> {
        if !self.is_value() {
            return None;
        }
        if let ValueType::Percentage(value) = &self.data_type {
            return Some(*value);
        }
        if let Some(value) = utils::to_percentage(self.raw_data) {
            self.data_type = ValueType::Percentage(value);
            return Some(value);
        }
        None
    }
    #[inline(always)]
    pub(crate) fn get_string(&self) -> &str {
        self.raw_data
    }
    pub(crate) fn get_dict(&mut self) -> Option<&mut NamedParamsMap<'a>> {
        match &mut self.data_type {
            ValueType::Dict(obj) => Some(obj),
            _ => None,
        }
    }
    pub(crate) fn get_list(&mut self) -> Option<&mut Vec<Value<'a>>> {
        match &mut self.data_type {
            ValueType::List(obj) => Some(obj),
            _ => None,
        }
    }
    fn validate_bool(&mut self, display_param_name: &str, param_list: &str) -> Result<(), Error> {
        if let Some(_) = self.get_bool() {
            return Ok(());
        }
        return Err(Error::new(
            param_list,
            format!(
                "Expecting a bool value (true or false) for parameter '{}' but found '{}'",
                display_param_name, self.raw_data
            )
            .as_str(),
            self.start,
            self.end,
        ));
    }
    pub(crate) fn validate(&mut self, key_name: &str, param_list: &str, expected_type: super::signature::ParamType) -> Result<(), Error> {
        let display_param_name = if self.param_name.len() > 0 { self.param_name } else { key_name };
        match expected_type {
            super::ParamType::String => { /* always possible */ }
            super::ParamType::Bool => self.validate_bool(display_param_name, param_list)?,
            super::ParamType::Flags => todo!(),
            super::ParamType::Alignament => todo!(),
            super::ParamType::Layout => todo!(),
        }
        // all good
        self.validated = true;
        Ok(())
    }
}
