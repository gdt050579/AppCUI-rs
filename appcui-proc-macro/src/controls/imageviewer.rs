use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ScrollBars"]);
static CHAR_SET: FlagsSignature = FlagsSignature::new(&["SmallBlocks", "LargeBlocks", "DitheredShades", "Braille", "AsciiArt"]);
static COLOR_SCHEMAS: FlagsSignature = FlagsSignature::new(&["Auto", "Color16", "TrueColors", "GrayScale4", "GrayScaleTrueColors", "BlackAndWhite"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("image", "image", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("background", "back", ParamType::Dict),
    NamedParameter::new("back", "back", ParamType::Dict),
    NamedParameter::new("left-scroll-margin", "lsm", ParamType::Integer),
    NamedParameter::new("lsm", "lsm", ParamType::Integer),
    NamedParameter::new("top-scroll-margin", "tsm", ParamType::Integer),
    NamedParameter::new("tsm", "tsm", ParamType::Integer),
    NamedParameter::new("scale", "scale", ParamType::Percentage),
    NamedParameter::new("charset", "charset", ParamType::String),
    NamedParameter::new("char_set", "charset", ParamType::String),
    NamedParameter::new("color_schema", "color_schema", ParamType::String),
    NamedParameter::new("colorschema", "color_schema", ParamType::String),
    NamedParameter::new("cs", "color_schema", ParamType::String),
    NamedParameter::new("luminance_threshold", "luminance_threshold", ParamType::Percentage),
    NamedParameter::new("lt", "luminance_threshold", ParamType::Percentage),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("imageviewer", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("ImageViewer::new");
    // Image argument
    if cb.has_parameter("image") {
        cb.add("Image::from_str(");
        cb.add_string_parameter("image", None);
        cb.add(").unwrap()");
    } else {
        cb.add("Image::new(1,1).unwrap()");
    }
    // Layout argument
    cb.add_layout();
    // RenderOptions argument (builder pattern)
    cb.add(", image::RenderOptionsBuilder::new()");
    // Character set (enum)
    if cb.has_parameter("charset") {
        cb.add(".character_set(");
        cb.add_enum_parameter("charset", "image::CharacterSet", &CHAR_SET, None);
        cb.add(")");
    }
    // Scale
    let scale_value = if cb.has_parameter("scale") { cb.get_percentage("scale") } else { None };
    if let Some(rap) = scale_value {
        let rap = rap as i32;
        cb.add(".scale(image::Scale::");
        cb.add(match rap {
            100 => "NoScale",
            50 => "Scale50",
            33 => "Scale33",
            25 => "Scale25",
            20 => "Scale20",
            10 => "Scale10",
            5 => "Scale5",
            _ => {
                panic!("ImageViewer only supports the following scales: 100%, 50%, 33%, 25%, 20%, 10% and 5%");
            }
        });
        cb.add(")");
    }
    // Color schema (enum)
    if cb.has_parameter("color_schema") {
        cb.add_enum_parameter("color_schema", ".color_schema(image::ColorSchema", &COLOR_SCHEMAS, None);
        cb.add(")");
    }
    // Luminance threshold
    if cb.has_parameter("luminance_threshold") {
        if let Some(perc) = cb.get_percentage("luminance_threshold") {
            let perc = perc.round();
            if !(0.0..=100.0).contains(&perc) {
                panic!("luminance_threshold must be a percentage between 0 and 100, got {}", perc);
            }
            let value = perc / 100.0;
            cb.add(".luminance_threshold(");
            cb.add(&value.to_string());
            cb.add(")");
        }
    }
    cb.add(".build()");
    // Flags argument
    cb.add_flags_parameter("flags", "imageviewer::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();

    let has_back_param = cb.has_parameter("back");
    if has_back_param {
        let str_repr = String::from(cb.get_string_representation());
        if let Some(d) = cb.get_dict("back") {
            let s = crate::chars::builder::create_from_dict(&str_repr, d);
            cb.add_line(format!("control.set_backgound({s});").as_str());
        }
    }
    cb.add_scroll_margin_setup("lsm","tsm");
    cb.into()
}
