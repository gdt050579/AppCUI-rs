use crate::parameter_parser;
use crate::parameter_parser::size::Size;
use crate::parameter_parser::*;
use crate::token_stream_to_string::TokenStreamToString;
use proc_macro::*;
use std::str::FromStr;

use super::layout;

static CONTROL_NAMED_PARAMATERS: &[NamedParameter] = &[
    // generic characteristics
    NamedParameter::new("visible", "visible", ParamType::Bool),
    NamedParameter::new("enabled", "enabled", ParamType::Bool),
    NamedParameter::new("enable", "enabled", ParamType::Bool),
    // layout
    NamedParameter::new("x", "x", ParamType::Layout),
    NamedParameter::new("y", "y", ParamType::Layout),
    NamedParameter::new("left", "left", ParamType::Layout),
    NamedParameter::new("l", "left", ParamType::Layout),
    NamedParameter::new("right", "right", ParamType::Layout),
    NamedParameter::new("r", "right", ParamType::Layout),
    NamedParameter::new("top", "top", ParamType::Layout),
    NamedParameter::new("t", "top", ParamType::Layout),
    NamedParameter::new("bottom", "bottom", ParamType::Layout),
    NamedParameter::new("b", "bottom", ParamType::Layout),
    NamedParameter::new("width", "width", ParamType::Layout),
    NamedParameter::new("w", "width", ParamType::Layout),
    NamedParameter::new("height", "height", ParamType::Layout),
    NamedParameter::new("h", "height", ParamType::Layout),
    NamedParameter::new("align", "align", ParamType::Alignament),
    NamedParameter::new("a", "align", ParamType::Alignament),
    NamedParameter::new("alignament", "align", ParamType::Alignament),
    NamedParameter::new("dock", "dock", ParamType::Alignament),
    NamedParameter::new("d", "dock", ParamType::Alignament),
];

pub(super) struct ControlBuilder<'a> {
    name: &'static str,
    content: String,
    ref_str: &'a str,
    string_representation: String, //Box<String>,
    parser: NamedParamsMap<'a>,
}

impl<'a> ControlBuilder<'a> {
    fn token_stream_to_string(name: &str, input: TokenStream) -> String {
        input.validate_one_string_parameter(name)
    }
    pub(super) fn new(
        name: &'static str,
        input: TokenStream,
        positional_parameters: &[PositionalParameter],
        named_parameters: &[NamedParameter],
        add_common_parameters: bool,
    ) -> Self {
        let string_repr = ControlBuilder::token_stream_to_string(name, input);
        let mut builder = Self {
            name,
            string_representation: string_repr, //Box::new(string_repr),
            content: String::with_capacity(512),
            parser: NamedParamsMap::empty(),
            ref_str: "",
        };
        unsafe {
            let ref_str: &str = std::mem::transmute(builder.string_representation.as_str());
            builder.parser = parameter_parser::parse(ref_str).unwrap();
            builder.parser.validate_positional_parameters(ref_str, positional_parameters).unwrap();
            builder.parser.validate_named_parameters(ref_str, named_parameters).unwrap();
            if add_common_parameters {
                builder.parser.validate_named_parameters(ref_str, CONTROL_NAMED_PARAMATERS).unwrap();
            }
            builder.parser.check_unkwnon_params(ref_str).unwrap();
            builder.ref_str = ref_str;
        }
        builder.content.push_str("{\n\tlet mut control = ");

        builder
    }
    fn add_comma(&mut self) {
        if !self.content.ends_with('(') {
            self.content.push_str(", ");
        }
    }
    fn add_text(&mut self, text: &str) {
        self.content.push('"');
        self.content.push_str(text);
        self.content.push('"');
    }
    fn add_bool(&mut self, value: bool) {
        if value {
            self.content.push_str("true");
        } else {
            self.content.push_str("false");
        }
    }
    fn add_size(&mut self, value: Size) {
        self.content.push_str(format!("Size::new({},{})", value.width, value.height).as_str());
    }
    pub(super) fn init_control(&mut self, method: &str) {
        self.content.push_str(method);
        self.content.push('(');
    }
    pub(super) fn finish_control_initialization(&mut self) {
        self.content.push_str(");\n\t");
    }
    pub(super) fn add_string_parameter(&mut self, param_name: &str, default: Option<&str>) {
        self.add_comma();
        let value = self.parser.get(param_name);
        if let Some(str_value) = value {
            unsafe {
                let x = std::mem::transmute(str_value.get_string());
                self.add_text(x);
            }
        } else if let Some(default_value) = default {
            self.add_text(default_value);
        } else {
            panic!(
                "Parameter {} is mandatory ! (you need to provided it as part of macro initialization)",
                param_name
            );
        }
    }
    pub(super) fn add_bool_parameter(&mut self, param_name: &str, default: Option<bool>) {
        self.add_comma();
        let value = self.parser.get_bool(param_name);
        if let Some(bool_value) = value {
            self.add_bool(bool_value);
        } else if let Some(default_value) = default {
            self.add_bool(default_value);
        } else {
            panic!(
                "Parameter {} is mandatory ! (you need to provided it as part of macro initialization)",
                param_name
            );
        }
    }
    pub(super) fn add_size_parameter(&mut self, param_name: &str, default: Option<Size>) {
        self.add_comma();
        let value = self.parser.get_size(param_name);
        if let Some(size_value) = value {
            self.add_size(size_value);
        } else if let Some(default_value) = default {
            self.add_size(default_value);
        } else {
            panic!(
                "Parameter {} is mandatory ! (you need to provided it as part of macro initialization)",
                param_name
            );
        }
    }
    pub(super) fn add_layout(&mut self) {
        self.add_comma();
        layout::add_layout(&mut self.content, &self.parser);
    }
    pub(super) fn add_toolbaritem_operations(&mut self) {
        if let Some(tooltip_value) = self.parser.get("tooltip") {
            let txt = tooltip_value.get_string();
            if !txt.is_empty() {
                self.content.push_str("control.set_tooltip(");
                unsafe {
                    let x = std::mem::transmute(txt);
                    self.add_text(x);
                }
                self.content.push_str(");\n\t");
            }
        }
        if let Some(is_visible) = self.parser.get_bool("visible") {
            if !is_visible {
                self.content.push_str("control.set_visible(false);\n\t");
            }
        }
    }
    pub(super) fn add_basecontrol_operations(&mut self) {
        if let Some(is_enabled) = self.parser.get_bool("enabled") {
            if !is_enabled {
                self.content.push_str("control.set_enabled(false);\n\t");
            }
        }
        if let Some(is_visible) = self.parser.get_bool("visible") {
            if !is_visible {
                self.content.push_str("control.set_visible(false);\n\t");
            }
        }
    }
    pub(super) fn get_enum_value(&mut self, param_name: &str, available_variants: &mut FlagsSignature) -> Option<&str> {
        if let Some(value) = self.parser.get(param_name) {
            let variant = value.get_string();
            if let Some(variant_name) = available_variants.get(variant) {
                Some(variant_name)
            } else {
                None
            }
        } else {
            None
        }
    }
    pub(super) fn add_enum_parameter(&mut self, param_name: &str, enum_name: &str, available_variants: &mut FlagsSignature, default: Option<&str>) {
        self.add_comma();
        if let Some(value) = self.parser.get(param_name) {
            let variant = value.get_string();
            if let Some(variant_name) = available_variants.get(variant) {
                self.content.push_str(enum_name);
                self.content.push_str("::");
                self.content.push_str(variant_name);
            } else {
                Error::new(
                    self.ref_str,
                    format!("Unknwon enum variant: {} !", variant).as_str(),
                    value.get_start_pos(),
                    value.get_end_pos(),
                )
                .panic();
            }
        } else if let Some(default_value) = default {
            self.content.push_str(enum_name);
            self.content.push_str("::");
            self.content.push_str(default_value);
        } else {
            panic!(
                "Parameter {} is mandatory ! (you need to provided it as part of macro initialization)",
                param_name
            );
        }
    }
    pub(super) fn add_flags_parameter(&mut self, param_name: &str, flag_name: &str, available_flags: &mut FlagsSignature) {
        self.add_comma();
        if let Some(value) = self.parser.get_mut(param_name) {
            if let Some(list) = value.get_list() {
                if list.is_empty() {
                    self.content.push_str(flag_name);
                    self.content.push_str("::None");
                } else {
                    let mut add_or_operator = false;
                    for name in list {
                        if let Some(flag) = available_flags.get(name.get_string()) {
                            if add_or_operator {
                                self.content.push_str(" | ")
                            }
                            self.content.push_str(flag_name);
                            self.content.push_str("::");
                            self.content.push_str(flag);
                            add_or_operator = true;
                        } else {
                            Error::new(
                                self.ref_str,
                                format!("Unknwon flag: {} !", name.get_string()).as_str(),
                                name.get_start_pos(),
                                name.get_end_pos(),
                            )
                            .panic();
                        }
                    }
                }
            } else {
                panic!("Parameter '{}' should contain some flags !", param_name);
            }
        } else {
            self.content.push_str(flag_name);
            self.content.push_str("::None");
        }
    }
    pub(super) fn add_command(&mut self, content: &str) {
        self.content.push_str(content);
    }
    pub(super) fn add_line(&mut self, content: &str) {
        if !self.content.ends_with('\n') {
            self.content.push('\n');
        }
        self.content.push('\t');
        self.content.push_str(content);
        self.content.push('\n');
    }    
    #[inline(always)]
    pub(super) fn get_dict(&mut self, name: &str) -> Option<&mut NamedParamsMap<'a>> {
        self.parser.get_mut(name)?.get_dict()
    }
    #[inline(always)]
    pub(super) fn get_list(&mut self, name: &str) -> Option<&mut Vec<Value<'a>>> {
        self.parser.get_mut(name)?.get_list()
    }
    #[inline(always)]
    pub(super) fn get_i32(&mut self, name: &str) -> Option<i32> {
        self.parser.get_mut(name)?.get_i32()
    }
    #[inline(always)]
    pub(super) fn get_string_representation(&self) -> &str {
        &self.string_representation
    }
    #[inline(always)]
    pub(super) fn has_parameter(&self, name: &str) -> bool {
        self.parser.contains(name)
    }
}
impl From<ControlBuilder<'_>> for TokenStream {
    fn from(mut val: ControlBuilder<'_>) -> Self {
        val.content.push_str("\n\tcontrol\n}");
        TokenStream::from_str(val.content.as_str()).unwrap_or_else(|_| panic!("Fail to convert '{}!' macro content to token stream", val.name))
    }
}
