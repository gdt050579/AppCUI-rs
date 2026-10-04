use crate::{
    parameter_parser::{self, *},
    token_stream_to_string::TokenStreamToString,
};
use proc_macro::*;
use std::str::FromStr;

static BASES: &[(&str, &str)] = &[
    ("2", "2"),
    ("2", "bin"),
    ("2", "binary"),
    ("8", "8"),
    ("8", "oct"),
    ("8", "octal"),
    ("10", "10"),
    ("10", "dec"),
    ("10", "decimal"),
    ("16", "16"),
    ("16", "hex"),
    ("16", "hexadecimal"),
];

static POSITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("base", ParamType::String)];

static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("base", "base", ParamType::String),
    NamedParameter::new("radix", "base", ParamType::String),
    NamedParameter::new("group", "group", ParamType::Integer),
    NamedParameter::new("groups", "group", ParamType::Integer),
    NamedParameter::new("group-size", "group", ParamType::Integer),
    NamedParameter::new("groupsize", "group", ParamType::Integer),
    NamedParameter::new("gs", "group", ParamType::Integer),
    NamedParameter::new("sep", "sep", ParamType::String),
    NamedParameter::new("separator", "sep", ParamType::String),
    NamedParameter::new("group-char", "sep", ParamType::String),
    NamedParameter::new("groupchar", "sep", ParamType::String),
    NamedParameter::new("digits", "digits", ParamType::Integer),
    NamedParameter::new("representation", "digits", ParamType::Integer),
    NamedParameter::new("representation-digits", "digits", ParamType::Integer),
    NamedParameter::new("repr", "digits", ParamType::Integer),
    NamedParameter::new("decimals", "decimals", ParamType::Integer),
    NamedParameter::new("precision", "decimals", ParamType::Integer),
    NamedParameter::new("prefix", "prefix", ParamType::String),
    NamedParameter::new("pre", "prefix", ParamType::String),
    NamedParameter::new("suffix", "suffix", ParamType::String),
    NamedParameter::new("suf", "suffix", ParamType::String),
    NamedParameter::new("fill", "fill", ParamType::Integer),
    NamedParameter::new("width", "fill", ParamType::Integer),
    NamedParameter::new("w", "fill", ParamType::Integer),
    NamedParameter::new("fillchar", "fillchar", ParamType::String),
    NamedParameter::new("fill-char", "fillchar", ParamType::String),
    NamedParameter::new("padchar", "fillchar", ParamType::String),
    NamedParameter::new("pad-char", "fillchar", ParamType::String),
];

fn parse_base(text: &str) -> u8 {
    if let Some(base) = crate::utils::find_string_in_array(BASES, text) {
        return base.parse::<u8>().expect("internal base value");
    }
    panic!("Invalid base '{text}'. Expected 2 (bin, binary), 8 (oct, octal), 10 (dec, decimal) or 16 (hex, hexadecimal).");
}

fn optional_u8(dict: &NamedParamsMap, key: &str) -> Option<u8> {
    let value = dict.get(key)?;
    let text = value.get_string();
    let n = text.parse::<i32>().unwrap_or_else(|_| {
        panic!("Invalid value '{text}' for parameter '{key}'. Expected an integer.");
    });
    if !(0..=255).contains(&n) {
        panic!("Invalid value '{n}' for parameter '{key}'. Expected a number between 0 and 255.");
    }
    Some(n as u8)
}

fn optional_ascii_char(dict: &NamedParamsMap, key: &str) -> Option<u8> {
    let value = dict.get(key)?;
    let text = value.get_string();
    let mut chars = text.chars();
    let Some(ch) = chars.next() else {
        panic!("Invalid value for parameter '{key}'. Expected a single printable ASCII character.");
    };
    if chars.next().is_some() || !ch.is_ascii() || ch.is_ascii_control() {
        panic!("Invalid value '{text}' for parameter '{key}'. Expected a single printable ASCII character (codes 32 to 126). Quote characters such as comma or space: '{key}: \",\"'.");
    }
    Some(ch as u8)
}

fn push_byte_literal(output: &mut String, ch: u8) {
    if ch == 0 {
        output.push('0');
        return;
    }
    output.push_str("b'");
    match ch {
        b'\\' => output.push_str("\\\\"),
        b'\'' => output.push_str("\\'"),
        _ => output.push(ch as char),
    }
    output.push('\'');
}

fn push_str_literal(output: &mut String, text: &str) {
    output.push('"');
    for ch in text.chars() {
        match ch {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            c if (c as u32) < 32 || c == '\u{7f}' => {
                output.push_str(&format!("\\u{{{:x}}}", c as u32));
            }
            c => output.push(c),
        }
    }
    output.push('"');
}

fn validate_digits(base: u8, digits: u8) {
    if digits == 0 {
        panic!("Invalid number of representation digits. Expected a number greater than 0.");
    }
    let max = match base {
        2 => 128u8,
        8 => 43,
        10 => 39,
        16 => 32,
        _ => 0,
    };
    if digits > max {
        panic!("Invalid number of representation digits ({digits}). Maximum for base {base} is {max}.");
    }
}

pub(crate) fn create_from_dict(param_list: &str, dict: &mut NamedParamsMap) -> String {
    dict.validate_positional_parameters(param_list, POSITIONAL_PARAMETERS).unwrap();
    dict.validate_named_parameters(param_list, NAMED_PARAMETERS).unwrap();
    dict.check_unkwnon_params(param_list, POSITIONAL_PARAMETERS, NAMED_PARAMETERS, None).unwrap();

    let base_text = dict
        .get("base")
        .unwrap_or_else(|| panic!("Missing base. Use a positional value (for example `dec` or `16`) or `base: hex`."))
        .get_string();
    let base = parse_base(base_text);

    let group = optional_u8(dict, "group");
    let separator = optional_ascii_char(dict, "sep");
    let digits = optional_u8(dict, "digits");
    let decimals = optional_u8(dict, "decimals");
    let fill = optional_u8(dict, "fill");
    let fill_char = optional_ascii_char(dict, "fillchar");
    let prefix = dict.get("prefix").map(|v| v.get_string());
    let suffix = dict.get("suffix").map(|v| v.get_string());

    if let Some(size) = group {
        match size {
            0 | 3 | 4 => (),
            _ => panic!("Invalid group size {size}. Expected 0, 3 or 4."),
        }
        if size == 0 && separator.is_some() {
            panic!("Separator can not be set when group size is 0.");
        }
    } else if separator.is_some() {
        panic!("Parameter 'sep' requires 'group' (3 or 4) to be set.");
    }

    if let Some(count) = digits {
        validate_digits(base, count);
    }
    if let Some(count) = decimals {
        if count > 8 {
            panic!("Invalid number of decimals ({count}). Maximum number of decimals is 8.");
        }
    }
    if let Some(width) = fill {
        if width == 0 && fill_char.is_some() {
            panic!("Fill character can not be set when fill width is 0.");
        }
    } else if fill_char.is_some() {
        panic!("Parameter 'fillchar' requires 'fill' (the width) to be set.");
    }

    let mut res = String::with_capacity(96);
    res.push_str("FormatNumber::new(");
    res.push_str(&base.to_string());
    res.push(')');
    if let Some(size) = group {
        let sep = if size == 0 { 0 } else { separator.unwrap_or(b',') };
        res.push_str(".group(");
        res.push_str(&size.to_string());
        res.push_str(", ");
        push_byte_literal(&mut res, sep);
        res.push(')');
    }
    if let Some(count) = digits {
        res.push_str(".representation_digits(");
        res.push_str(&count.to_string());
        res.push(')');
    }
    if let Some(count) = decimals {
        res.push_str(".decimals(");
        res.push_str(&count.to_string());
        res.push(')');
    }
    if let Some(text) = prefix {
        res.push_str(".prefix(");
        push_str_literal(&mut res, text);
        res.push(')');
    }
    if let Some(text) = suffix {
        res.push_str(".suffix(");
        push_str_literal(&mut res, text);
        res.push(')');
    }
    if let Some(width) = fill {
        let ch = if width == 0 { 0 } else { fill_char.unwrap_or(b' ') };
        res.push_str(".fill(");
        res.push_str(&width.to_string());
        res.push_str(", ");
        push_byte_literal(&mut res, ch);
        res.push(')');
    }
    res
}

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let s = input.validate_one_string_parameter("numericformat");
    let mut d = parameter_parser::parse(&s).unwrap();
    let res = create_from_dict(&s, &mut d);
    TokenStream::from_str(&res).expect("Fail to convert 'numericformat!' macro content to token stream")
}
