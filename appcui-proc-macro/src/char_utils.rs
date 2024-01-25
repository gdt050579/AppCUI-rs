use crate::{
    parameter_parser::{self, color::Color, *},
    token_stream_to_string::TokenStreamToString,
};
use proc_macro::*;
use std::str::FromStr;

static mut CHAR_ATTR: FlagsSignature = FlagsSignature::new(&["Bold", "Italic", "Underline"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[
    PositionalParameter::new("value", ParamType::String),
    PositionalParameter::new("fore", ParamType::Color),
    PositionalParameter::new("back", ParamType::Color),
];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("value", "value", ParamType::String),
    NamedParameter::new("char", "value", ParamType::String),
    NamedParameter::new("ch", "value", ParamType::Color),
    NamedParameter::new("fore", "fore", ParamType::Color),
    NamedParameter::new("foreground", "fore", ParamType::Color),
    NamedParameter::new("forecolor", "fore", ParamType::Color),
    NamedParameter::new("color", "fore", ParamType::Color),
    NamedParameter::new("back", "back", ParamType::Color),
    NamedParameter::new("background", "back", ParamType::Color),
    NamedParameter::new("backcolor", "back", ParamType::Color),
    NamedParameter::new("attr", "attr", ParamType::Flags),
    NamedParameter::new("attributes", "attr", ParamType::Flags),
];

fn get_color(param_name: &str, dict: &mut NamedParamsMap) -> Color {
    if !dict.contains(param_name) {
        return Color::Transparent;
    }
    if let Some(color) = dict.get_mut(param_name).unwrap().get_color() {
        return color;
    }
    panic!(
        "Invalid color value {} for parameter '{}'",
        dict.get(param_name).unwrap().get_string(),
        param_name
    );
}
fn create_from_dict(param_list: &str, dict: &mut NamedParamsMap) -> String {
    dict.validate_positional_parameters(param_list, POSILITIONAL_PARAMETERS).unwrap();
    dict.validate_names_parameters(param_list, NAMED_PARAMETERS).unwrap();
    let mut res = String::with_capacity(64);
    res.push_str("Character::new(");
    let val = dict
        .get("value")
        .expect("Missing first positional parameter or the parameter 'value' (the character code)");
    let char_value = val.get_string();
    let count = char_value.chars().count();
    match count {
        0 => res.push_str("0"),
        1 => {
            res.push_str("'");
            res.push_str(char_value);
            res.push_str("'")
        }
        _ => todo!("special chars not implemented"),
    }
    res.push_str(", ");
    let fore = get_color("fore", dict);
    let back = get_color("back", dict);
    res.push_str("Color::");
    res.push_str(fore.get_name());
    res.push_str(", ");
    res.push_str("Color::");
    res.push_str(back.get_name());
    res.push_str(", ");

    if let Some(value) = dict.get_mut("attr") {
        if let Some(list) = value.get_list() {
            if list.len() == 0 {
                res.push_str("CharFlags::None)");
            } else {
                let mut add_or_operator = false;
                for name in list {
                    if let Some(flag) = unsafe { CHAR_ATTR.get(name.get_string()) } {
                        if add_or_operator {
                            res.push_str(" | ")
                        }
                        res.push_str("CharFlags::");
                        res.push_str(flag);
                        add_or_operator = true;
                    } else {
                        Error::new(
                            param_list,
                            format!("Unknwon character attribute: {} !", name.get_string()).as_str(),
                            name.get_start_pos(),
                            name.get_end_pos(),
                        )
                        .panic();
                    }
                }
                res.push_str(")")
            }
        } else {
            panic!("Parameter 'attr' should contain some flags !");
        }
    } else {
        res.push_str("CharFlags::None)");
    }
    res
}
pub(crate) fn create(input: TokenStream) -> TokenStream {
    let s = input.validate_one_string_parameter("char");
    let mut d = parameter_parser::parse(&s).unwrap();
    let res = create_from_dict(&s, &mut d);
    TokenStream::from_str(&res).expect(format!("Fail to convert 'char!' macro content to token stream").as_str())
}
