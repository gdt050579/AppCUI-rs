use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ScrollBars"]);
static RENDER_METHOD: FlagsSignature = FlagsSignature::new(&["SmallBlocks", "LargeBlocks64Colors", "GrayScale", "AsciiArt"]);

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
    NamedParameter::new("render", "render", ParamType::String),
    NamedParameter::new("rendermethod", "render", ParamType::String),
    NamedParameter::new("rm", "render", ParamType::String),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("imageviewer", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);

    cb.init_control("ImageViewer::new");
    if cb.has_parameter("image") {
        cb.add("Image::with_str(");
        cb.add_string_parameter("image", None);
        cb.add(").unwrap()");
    } else {
        cb.add("Image::new(1,1).unwrap()");
    }
    cb.add_layout();
    cb.add_enum_parameter("render", "image::RendererType", &RENDER_METHOD, Some("SmallBlocks"));
    if cb.has_parameter("scale") {
        let rap = cb.get_percentage("scale").unwrap() as i32;
        cb.add(", image::Scale::");
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
    } else {
        cb.add(", image::Scale::NoScale");
    }

    cb.add_flags_parameter("flags", "imageviewer::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();

    let has_back_param = cb.has_parameter("back");
    if has_back_param {
        let str_repr = String::from(cb.get_string_representation());
        if let Some(d) = cb.get_dict("back") {
            let s = crate::chars::builder::create_from_dict(&str_repr, d);
            cb.add_line(format!("control.set_backgound({});", s).as_str());
        }
    }
    cb.add_scroll_margin_setup("lsm", "tsm");
    cb.into()
}
