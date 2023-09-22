use super::named_params_map::NamedParamsMap;
use super::utils;

pub (super) enum ValueType<'a> {
    Undetermined,
    Bool(bool),
    Integer(i32),
    Percentage(f32),
    List(Vec<Value<'a>>),
    Dict(NamedParamsMap<'a>),
}

pub(crate) struct Value<'a> {
    pub(super) raw_data: &'a str,
    pub(super) data_type: ValueType<'a>,
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
    pub(crate) fn get_dict(&mut self) -> Option<&mut NamedParamsMap<'a>> {
        match &mut self.data_type {
            ValueType::Dict(obj) => Some(obj),
            _ => None
        }
    }
    pub(crate) fn get_list(&mut self) -> Option<&mut Vec<Value<'a>>> {
        match &mut self.data_type {
            ValueType::List(obj) => Some(obj),
            _ => None
        }
    }
}
