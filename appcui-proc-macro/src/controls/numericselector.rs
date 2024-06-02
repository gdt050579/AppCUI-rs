use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["HideButtons", "ReadOnly"]);

static NUMERIC_FORMAT: FlagsSignature = FlagsSignature::new(&["Decimal", "Percentage", "DigitGrouping", "Hex", "Size"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[
    PositionalParameter::new("type", ParamType::String),
    PositionalParameter::new("value", ParamType::String),
    PositionalParameter::new("min", ParamType::String),
    PositionalParameter::new("max", ParamType::String),
    PositionalParameter::new("step", ParamType::String),
];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("type", "type", ParamType::String),
    NamedParameter::new("class", "type", ParamType::String),
    NamedParameter::new("value", "value", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("min", "min", ParamType::String),
    NamedParameter::new("max", "max", ParamType::String),
    NamedParameter::new("step", "step", ParamType::String),
    NamedParameter::new("format", "format", ParamType::String),
    NamedParameter::new("numericformat", "format", ParamType::String),
    NamedParameter::new("nf", "format", ParamType::String),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("NumericSelector", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control_with_template("NumericSelector", "with_format", "type");

    // check for a number format
    let type_name = cb.get_value("type").unwrap(); // we know it exists atthis point
    let accepted = matches!(
        type_name,
        "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "isize" | "f32" | "f64"
    );
    let is_float = matches!(type_name, "f32" | "f64");
    if !accepted {
        panic!("Invalid type for NumericSelector: '{}' - only the following numeric types are accepted: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize, f32, f64", type_name);
    }

    // all good --> lets add parameters
    cb.add_param_value("value");

    // min
    cb.add(",");
    if cb.has_parameter("min") {
        cb.add_param_value("min");
    } else {
        cb.add_param_value("type");
        cb.add("::MIN");
    }

    // max
    cb.add(",");
    if cb.has_parameter("max") {
        cb.add_param_value("max");
    } else {
        cb.add_param_value("type");
        cb.add("::MAX");
    }

    // step
    cb.add(",");
    if cb.has_parameter("step") {
        cb.add_param_value("step");
    } else if is_float {
        cb.add("1.0");
    } else {
        cb.add("1");
    }

    cb.add_layout();
    cb.add_flags_parameter("flags", "numericselector::Flags", &FLAGS);
    cb.add_enum_parameter("format", "numericselector::Format", &NUMERIC_FORMAT, Some("Decimal"));
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
