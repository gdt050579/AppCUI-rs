use super::alignament::Alignament;
use super::color::Color;
use super::named_params_map::NamedParamsMap;
use super::size::Size;
use super::Error;
use crate::utils;

pub(super) enum ValueType<'a> {
    Undetermined,
    Bool(bool),
    Integer(i32),
    Percentage(f32),
    Size(Size),
    Alignament(Alignament),
    Color(Color),
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
    #[inline(always)]
    pub(crate) fn get_start_pos(&self) -> usize {
        self.start
    }
    #[inline(always)]
    pub(crate) fn get_end_pos(&self) -> usize {
        self.end
    }
    pub(crate) fn is_dict(&self) -> bool {
        matches!(self.data_type, ValueType::Dict(_))
    }
    pub(crate) fn is_list(&self) -> bool {
        matches!(self.data_type, ValueType::List(_))
    }
    pub(crate) fn is_value(&self) -> bool {
        !matches!(self.data_type, ValueType::List(_) | ValueType::Dict(_))
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
    pub(crate) fn to_align(&self) -> Alignament {
        if let ValueType::Alignament(value) = &self.data_type {
            return *value;
        }
        panic!("Invalid alignament value: {}", self.raw_data);
    }
    pub(crate) fn get_alignament(&mut self) -> Option<Alignament> {
        if !self.is_value() {
            return None;
        }
        if let ValueType::Alignament(value) = &self.data_type {
            return Some(*value);
        }
        if let Some(value) = Alignament::from_hash(utils::compute_hash(self.raw_data)) {
            self.data_type = ValueType::Alignament(value);
            return Some(value);
        }
        None
    }
    pub(crate) fn get_color(&mut self) -> Option<Color> {
        if !self.is_value() {
            return None;
        }
        if let ValueType::Color(value) = &self.data_type {
            return Some(*value);
        }
        if let Some(value) = Color::from_hash(utils::compute_hash(self.raw_data)) {
            self.data_type = ValueType::Color(value);
            return Some(value);
        }
        None
    }
    pub(crate) fn get_size(&mut self) -> Option<Size> {
        if !self.is_value() {
            return None;
        }
        if let ValueType::Size(value) = &self.data_type {
            return Some(*value);
        }
        if let Some(value) = Size::from_str(self.raw_data) {
            self.data_type = ValueType::Size(value);
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
        if self.get_bool().is_some() {
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
            if !(-30000..=30000).contains(&value) {
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
            if !(-300.0f32..=300.0f32).contains(&value) {
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
        Ok(())
    }
    fn validate_alignament(&mut self, display_param_name: &str, param_list: &str) -> Result<(), Error> {
        if self.get_alignament().is_some() {
            return Ok(());
        }
        return Err(Error::new(
            param_list,
            format!(
                "Expecting an alignament constant (left,topleft,top,topright,right,bottomright,bottom,bottomleft,center) for parameter '{}' but found '{}'",
                display_param_name, self.raw_data
            )
            .as_str(),
            self.start,
            self.end,
        ));
    }
    fn validate_color(&mut self, display_param_name: &str, param_list: &str) -> Result<(), Error> {
        if self.get_color().is_some() {
            return Ok(());
        }
        return Err(Error::new(
            param_list,
            format!(
                "Expecting an color name (Black,DarkBlue,DarkGreen,Teal,DarkRed,Magenta,Olive,Silver,Gray,Blue,Green,Aqua,Red,Pink,Yellow,White,Transparent) for parameter '{}' but found '{}'",
                display_param_name, self.raw_data
            )
            .as_str(),
            self.start,
            self.end,
        ));
    }
    fn validate_size(&mut self, display_param_name: &str, param_list: &str) -> Result<(), Error> {
        if self.get_size().is_some() {
            return Ok(());
        }
        return Err(Error::new(
            param_list,
            format!(
                "Expecting a size (width x height) for parameter '{}' but found '{}'",
                display_param_name, self.raw_data
            )
            .as_str(),
            self.start,
            self.end,
        ));
    }
    fn validate_dict(&mut self, display_param_name: &str, param_list: &str) -> Result<(), Error> {
        if self.get_dict().is_some() {
            return Ok(());
        }
        return Err(Error::new(
            param_list,
            format!(
                "Expecting a dictionary with values `{{..}}` for parameter '{}' but found '{}'",
                display_param_name, self.raw_data
            )
            .as_str(),
            self.start,
            self.end,
        ));
    }
    fn validate_list(&mut self, display_param_name: &str, param_list: &str) -> Result<(), Error> {
        if self.get_list().is_some() {
            return Ok(());
        }
        return Err(Error::new(
            param_list,
            format!(
                "Expecting a list with values `[]..]` for parameter '{}' but found '{}'",
                display_param_name, self.raw_data
            )
            .as_str(),
            self.start,
            self.end,
        ));
    }
    fn validate_i32(&mut self, display_param_name: &str, param_list: &str) -> Result<(), Error> {
        if self.get_i32().is_some() {
            return Ok(());
        }
        return Err(Error::new(
            param_list,
            format!(
                "Expecting a numerical value (integer) for parameter '{}' but found '{}'",
                display_param_name, self.raw_data
            )
            .as_str(),
            self.start,
            self.end,
        ));
    }
    pub(crate) fn validate(&mut self, param_list: &str, key_name: &str, expected_type: super::signature::ParamType) -> Result<(), Error> {
        let display_param_name = if !self.param_name.is_empty() { self.param_name } else { key_name };
        match expected_type {
            super::ParamType::String => { /* always possible */ }
            super::ParamType::Bool => self.validate_bool(display_param_name, param_list)?,
            super::ParamType::Flags => self.validate_flags(display_param_name, param_list)?,
            super::ParamType::Alignament => self.validate_alignament(display_param_name, param_list)?,
            super::ParamType::Color => self.validate_color(display_param_name, param_list)?,
            super::ParamType::Layout => self.validate_layout(display_param_name, param_list)?,
            super::ParamType::Size => self.validate_size(display_param_name, param_list)?,
            super::ParamType::Dict => self.validate_dict(display_param_name, param_list)?,
            super::ParamType::List => self.validate_list(display_param_name, param_list)?,
            super::ParamType::Integer => self.validate_i32(display_param_name, param_list)?,
            
        }
        // all good
        self.validated = true;
        Ok(())
    }
}
