use crate::parameter_parser::*;

static BAR_DRAW_MODES: &[(&'static str, &'static str)] = &[
    ("Normal", "normal"),
    ("Rectangle", "rectangle"),
    ("Rectangle", "rect"),
    ("SingleLine", "singleline"),
    ("SingleLine", "single-line"),
    ("SingleLine", "single"),
    ("DoubleLine", "doubleline"),
    ("DoubleLine", "double-line"),
    ("DoubleLine", "double"),
];

static VALUE_BAR_POSITIONAL: &[PositionalParameter] = &[PositionalParameter::new("value", ParamType::String)];
static VALUE_BAR_NAMED: &[NamedParameter] = &[
    NamedParameter::new("value", "value", ParamType::String),
    NamedParameter::new("val", "value", ParamType::String),
    NamedParameter::new("v", "value", ParamType::String),
    NamedParameter::new("width", "width", ParamType::Integer),
    NamedParameter::new("w", "width", ParamType::Integer),
    NamedParameter::new("thickness", "width", ParamType::Integer),
    NamedParameter::new("space", "space", ParamType::Integer),
    NamedParameter::new("spacing", "space", ParamType::Integer),
    NamedParameter::new("s", "space", ParamType::Integer),
    NamedParameter::new("attr", "attr", ParamType::String),
    NamedParameter::new("attribute", "attr", ParamType::String),
    NamedParameter::new("charattr", "attr", ParamType::String),
    NamedParameter::new("label", "label", ParamType::String),
    NamedParameter::new("text", "label", ParamType::String),
    NamedParameter::new("caption", "label", ParamType::String),
    NamedParameter::new("draw-mode", "draw-mode", ParamType::String),
    NamedParameter::new("drawmode", "draw-mode", ParamType::String),
    NamedParameter::new("dm", "draw-mode", ParamType::String),
    NamedParameter::new("mode", "draw-mode", ParamType::String),
];

pub(crate) fn validate_bar_number(repr: &str) {
    if repr.parse::<f64>().is_err() {
        panic!("Invalid values format - expecting a valid number but got {repr} !");
    }
}

fn parse_bar_u8(repr: &str, key: &str) -> u8 {
    repr.parse::<u8>().unwrap_or_else(|_| {
        panic!("Invalid values format - expecting a number between 0 and 255 for '{key}' but got {repr} !");
    })
}

fn char_to_rust_literal(ch: char) -> String {
    if ch == '\'' {
        return String::from("'\\''");
    }
    if ch == '\\' {
        return String::from("'\\\\'");
    }
    if ch.is_ascii() && !ch.is_control() {
        return format!("'{ch}'");
    }
    format!("'\\u{{{:x}}}'", ch as u32)
}

fn strip_quotes(repr: &str) -> &str {
    let b = repr.as_bytes();
    if (b.len() >= 2) && ((b[0] == b'\'') || (b[0] == b'"')) && (b[0] == b[b.len() - 1]) {
        &repr[1..repr.len() - 1]
    } else {
        repr
    }
}

fn split_named_param(param: &str) -> Option<(&str, &str)> {
    let param = param.trim();
    let colon = param.find(':')?;
    let key = param[..colon].trim();
    let value = param[colon + 1..].trim();
    if key.is_empty() || value.is_empty() {
        return None;
    }
    Some((key, strip_quotes(value)))
}

fn parse_unicode_code(repr: &str, key: &str) -> u32 {
    let s = repr.trim();
    let parsed = if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16)
    } else if s.chars().all(|c| c.is_ascii_digit()) {
        s.parse::<u32>()
    } else {
        u32::from_str_radix(s, 16)
    };
    parsed.unwrap_or_else(|_| {
        panic!("Invalid {key} - expecting a valid unicode code (decimal or hex) but got {repr} !");
    })
}

fn parse_char_token(repr: &str, key: &str) -> String {
    let mut chars = repr.chars();
    let Some(ch) = chars.next() else {
        panic!("Invalid {key} - expecting a single character but got an empty value !");
    };
    if chars.next().is_none() {
        return char_to_rust_literal(ch);
    }
    if repr.len() == 2 && repr.starts_with('\\') {
        let escaped = match repr.as_bytes()[1] {
            b'n' => '\n',
            b't' => '\t',
            b'r' => '\r',
            b'\\' => '\\',
            b'\'' => '\'',
            b'"' => '"',
            b'0' => '\0',
            _ => panic!("Invalid {key} - unknown escape sequence '{repr}' !"),
        };
        return char_to_rust_literal(escaped);
    }
    panic!("Invalid {key} - expecting a single character but got {repr} !");
}

pub(crate) fn parse_bar_draw_mode(repr: &str, key: &str) -> String {
    let repr = repr.trim();
    if let Some(mode) = crate::utils::find_string_in_array(BAR_DRAW_MODES, repr) {
        return format!("vbarchart::BarDrawMode::{mode}");
    }
    if let Some(params) = crate::utils::parse_function_and_parameters(repr, "Char") {
        if params.len() != 1 {
            panic!("Invalid {key} - expecting Char(char) or Char(code: value) !");
        }
        let param = params[0].trim();
        let literal = if let Some((name, value)) = split_named_param(param) {
            if crate::utils::equal_ignore_case(name, "code") {
                let code = parse_unicode_code(value, key);
                let Some(ch) = char::from_u32(code) else {
                    panic!("Invalid {key} - unicode code {code} is not a valid character !");
                };
                char_to_rust_literal(ch)
            } else {
                panic!("Invalid {key} - unknown named parameter '{name}' ! Expected Char(code: value)");
            }
        } else {
            parse_char_token(param, key)
        };
        return format!("vbarchart::BarDrawMode::Char({literal})");
    }
    panic!(
        "Invalid {key}: {repr} - expected one of: {} or Char(char) or Char(code: value)",
        crate::utils::join_strings(BAR_DRAW_MODES)
    );
}

pub(crate) fn parse_bar_attr(repr: &str, key: &str) -> String {
    let repr = repr.trim();
    if repr.is_empty() {
        panic!("Invalid {key} - expecting a character attribute (e.g. 'red', 'red,blue' or 'fore: red, back: blue, flags: Bold') !");
    }
    let mut d = crate::parameter_parser::parse(repr).unwrap_or_else(|e| {
        panic!("Invalid {key}: {repr} !{e:?}");
    });
    crate::chars::builder::create_attr_from_dict(repr, &mut d)
}

pub(crate) fn parse_bar_from_dict(dict: &mut NamedParamsMap, param_list: &str) -> String {
    dict.validate_positional_parameters(param_list, VALUE_BAR_POSITIONAL).unwrap();
    dict.validate_named_parameters(param_list, VALUE_BAR_NAMED).unwrap();
    let value = match dict.get("value") {
        Some(v) => {
            let s = v.get_string();
            validate_bar_number(s);
            s.to_string()
        }
        None => panic!("Invalid values format - missing 'value' ! Expected {{value,width: 10, space: 4, attr: {{...}}}}"),
    };
    let width = dict.get("width").map(|v| parse_bar_u8(v.get_string(), "width"));
    let space = dict.get("space").map(|v| parse_bar_u8(v.get_string(), "space"));
    let label = dict.get("label").map(|v| v.get_string().to_string());
    let draw_mode = dict.get("draw-mode").map(|v| parse_bar_draw_mode(v.get_string(), "draw-mode"));
    let attr = if dict.contains("attr") {
        if let Some(attr_val) = dict.get_mut("attr") {
            if let Some(attr_dict) = attr_val.get_dict() {
                Some(crate::chars::builder::create_attr_from_dict(param_list, attr_dict))
            } else {
                Some(parse_bar_attr(attr_val.get_string(), "attr"))
            }
        } else {
            None
        }
    } else {
        None
    };
    let mut res = format!("vbarchart::BarBuilder::new({value})");
    if let Some(w) = width {
        res.push_str(&format!(".thickness({w})"));
    }
    if let Some(s) = space {
        res.push_str(&format!(".spacing({s})"));
    }
    if let Some(l) = label {
        res.push_str(&format!(".label(\"{l}\")"));
    }
    if let Some(a) = attr {
        res.push_str(".attr(");
        res.push_str(&a);
        res.push(')');
    }
    if let Some(dm) = draw_mode {
        res.push_str(".draw_mode(");
        res.push_str(&dm);
        res.push(')');
    }
    res.push_str(".build()");
    res
}
