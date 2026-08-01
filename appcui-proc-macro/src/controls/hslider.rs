use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ShowValue", "Ticks", "ValueAsMarker"]);
static TYPES: FlagsSignature = FlagsSignature::new(&["Standard", "ProgressBar", "Inline"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[
    PositionalParameter::new("class", ParamType::String),
    PositionalParameter::new("min", ParamType::String),
    PositionalParameter::new("max", ParamType::String),
    PositionalParameter::new("step", ParamType::String),
];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("type", "type", ParamType::String),
    NamedParameter::new("class", "class", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("min", "min", ParamType::String),
    NamedParameter::new("max", "max", ParamType::String),
    NamedParameter::new("step", "step", ParamType::String),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("HSlider", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    
    cb.init_control_with_template("HSlider", "new", "class");

    let type_name = cb.get_value("class").unwrap();
    let accepted = matches!(
        type_name,
        "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "isize" | "f32" | "f64"
    );
    if !accepted {
        panic!("Invalid type for HSlider: '{type_name}' - only the following numeric classes are accepted: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize, f32, f64");
    }

    cb.add_param_value("min");
    cb.add(",");
    cb.add_param_value("max");
    cb.add(",");
    cb.add_param_value("step");
    cb.add_enum_parameter("type", "hslider::Type", &TYPES, Some("Standard"));
    cb.add_layout();
    cb.add_flags_parameter("flags", "hslider::Flags", &FLAGS);
    
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
