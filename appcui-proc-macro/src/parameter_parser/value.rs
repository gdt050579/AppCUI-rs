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
    fn validate_layout(&mut self, display_param_name: &str, param_list: &str) -> Result<(), Error> {
        if let Some(value) = self.get_i32() {
            if (value < -30000) || (value > 30000) {
                return Err(Error::new(
                    param_list,
                    format!(
                        "The value for parameter '{}' should be between -30000 and 30000. Current value is {} !",
                        display_param_name, value
                    )
                    .as_str(),
                    self.start,
                    self.end,
                ));
            }
            return Ok(());
        }
        if let Some(value) = self.get_percentage() {
            if (value < -300.0f32) || (value > 300.0f32) {
                return Err(Error::new(
                    param_list,
                    format!(
                        "The value for parameter '{}' should be between -300% and 300%. Current value is {}% !",
                        display_param_name, value
                    )
                    .as_str(),
                    self.start,
                    self.end,
                ));
            }
            return Ok(());
        }
        return Err(Error::new(
            param_list,
            format!(
                "Expecting an integer value or a percentage for parameter '{}' but found '{}'",
                display_param_name, self.raw_data
            )
            .as_str(),
            self.start,
            self.end,
        ));
    }
    fn validate_flags(&mut self, display_param_name: &str, param_list: &str) -> Result<(), Error> {
        if self.is_list() {
            return Ok(());
        }
        if self.is_dict() {
            return Err(Error::new(
                param_list,
                format!(
                    "Expecting a flag value for parameter '{}' (a string, a word or a list [...]), but got parameter list '{{...}}'",
                    display_param_name
                )
                .as_str(),
                self.start,
                self.end,
            ));
        }
        // parse th flag list
        let buf = self.raw_data.as_bytes();
        let mut v: Vec<Value> = Vec::with_capacity(8);
        let mut pos = 0;
        let len = buf.len();
        while pos < len {
            pos = utils::skip_spaces(buf, pos);
            if !utils::is_word_character(buf[pos]) {
                return Err(Error::new(
                    param_list,
                    format!(
                        "Invalid flag format for parameter '{}'. Expecting a flags name formed out of th following characters: 0-9, a-z, A-Z and underline.",
                        display_param_name
                    )
                    .as_str(),
                    self.start+pos,
                    self.start+pos+1,
                ));
            }
            let next = utils::skip_words(buf, pos);
            v.push(Value {
                param_name: "",
                raw_data: &self.raw_data[pos..next],
                data_type: ValueType::Undetermined,
                validated: false,
                start: self.start + pos,
                end: self.start + next,
            });
            pos = utils::skip_spaces(buf, next);
            // skip any separator if any
            if (pos < len) && ((buf[pos] == b'+') || (buf[pos] == b',') || (buf[pos] == b'|') || (buf[pos] == b';')) {
                pos += 1;
            }
        }
        // all good --> lets set up the vector
        self.data_type = ValueType::List(v);
        return Ok(());
    }
    pub(crate) fn validate(&mut self, key_name: &str, param_list: &str, expected_type: super::signature::ParamType) -> Result<(), Error> {
        let display_param_name = if self.param_name.len() > 0 { self.param_name } else { key_name };
        match expected_type {
            super::ParamType::String => { /* always possible */ }
            super::ParamType::Bool => self.validate_bool(display_param_name, param_list)?,
            super::ParamType::Flags => self.validate_flags(display_param_name, param_list)?,
            super::ParamType::Alignament => todo!("validate alignament"),
            super::ParamType::Layout => self.validate_layout(display_param_name, param_list)?,
        }
        // all good
        self.validated = true;
        Ok(())
    }
}
