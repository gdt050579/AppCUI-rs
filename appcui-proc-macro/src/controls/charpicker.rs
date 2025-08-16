use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static SETS: FlagsSignature = FlagsSignature::new(&[
    "Ascii",
    "Arrows",
    "Animals",
    "Braille",
    "Blocks",
    "BoxDrawing",
    "Currency",
    "Emoticons",
    "Shapes",
    "Latin",
    "Punctuation",
]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("char", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("char", "char", ParamType::String),
    NamedParameter::new("ch", "char", ParamType::String),
    NamedParameter::new("code", "code", ParamType::Integer),
    NamedParameter::new("sets", "sets", ParamType::List),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("charpicker", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("CharPicker::new");
    cb.add("None");
    cb.add_layout();
    cb.finish_control_initialization();
    let s = if cb.has_parameter("sets") {
        let mut cmd = String::with_capacity(1024);
        if let Some(list) = cb.get_list("sets") {
            for item in list {
                if let Some(name) = SETS.get(item.get_string()) {
                    cmd.push_str("control.add_set(charpicker::Set::from_unicode_symbols(\"");
                    cmd.push_str(name);
                    cmd.push_str("\", charpicker::UnicodeSymbols::");
                    cmd.push_str(name);
                    cmd.push_str("));\n");
                } else {
                    panic!("Unknwon set `{}`. Available sets are: {}", item.get_string(), SETS.list())
                }
            }
            cmd
        } else {
            panic!(
                "Parameter 'sets' should be a list of available character sets (e.g. sets = [Arrows, Ascii]. Available sets are: {}",
                SETS.list()
            );
        }
    } else {
        String::new()
    };
    if !s.is_empty() {
        cb.add("control.clear_sets();");
        cb.add(&s);
    }
    // code part
    let s = if cb.has_parameter("code") {
        if let Some(code_id) = cb.get_i32("code") {
            if code_id > 0 {
                format!("if let Some(ch) = char::from_u32({code_id}) {{ control.select_char(ch); }}")
            } else {
                panic!("Character code should be a positive, non-null number (used value was : '{code_id}')");
            }
        } else {
            panic!("You need to provide a numerical (positive, non-null) value for the `code` parameter !");
        }
    } else {
        String::new()
    };
    if !s.is_empty() {
        cb.add(&s);
    }
    // char
    // code part
    let s = if let Some(val) = cb.get_value("char") {
        if !val.is_empty() {
            let ch = val.chars().next().unwrap() as u32;
            format!("control.select_char(char::from_u32({ch}).unwrap());")
        } else {
            panic!("You need to provide a character for the `char` parameter !")
        }
    } else {
        String::new()
    };
    if !s.is_empty() {
        cb.add(&s);
    }    
    cb.add_basecontrol_operations();
    cb.into()
}
