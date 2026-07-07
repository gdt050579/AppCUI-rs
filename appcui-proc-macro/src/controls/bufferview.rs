use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&[
    "ScrollBars",
    "SearchBar",
    "HideHeader",
    "ShowAddress",
    "ShowIntervalNames",
    "NoPanelDimming",
    "ShowAsciiStrings",
    "ShowUtf16AsciiStrings",
    "DecodeUTF8Characters",
    "ReadOnly",
]);

static OFFSET_FORMAT: FlagsSignature = FlagsSignature::new(&["Hex", "Dec"]);
static ENDIAN: FlagsSignature = FlagsSignature::new(&["Little", "Big"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[
    PositionalParameter::new("type", ParamType::String),
    PositionalParameter::new("buffer", ParamType::String),
];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("type", "type", ParamType::String),
    NamedParameter::new("class", "type", ParamType::String),
    NamedParameter::new("buffer", "buffer", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("left-scroll-margin", "lsm", ParamType::Integer),
    NamedParameter::new("lsm", "lsm", ParamType::Integer),
    NamedParameter::new("top-scroll-margin", "tsm", ParamType::Integer),
    NamedParameter::new("tsm", "tsm", ParamType::Integer),
    NamedParameter::new("offset-format", "offset", ParamType::String),
    NamedParameter::new("offset", "offset", ParamType::String),
    NamedParameter::new("address-format", "offset", ParamType::String),
    NamedParameter::new("endian", "endian", ParamType::String),
    NamedParameter::new("format", "format", ParamType::String),
    NamedParameter::new("data-format", "format", ParamType::String),
    NamedParameter::new("representation", "format", ParamType::String),
    NamedParameter::new("columns", "columns", ParamType::String),
    NamedParameter::new("columns-count", "columns", ParamType::String),
    NamedParameter::new("codepage", "codepage", ParamType::String),
    NamedParameter::new("cp", "codepage", ParamType::String),
    NamedParameter::new("address-width", "address-width", ParamType::Integer),
    NamedParameter::new("aw", "address-width", ParamType::Integer),
    NamedParameter::new("address-name", "address-name", ParamType::String),
    NamedParameter::new("address", "address-name", ParamType::String),
    NamedParameter::new("interval-name-width", "interval-name-width", ParamType::Integer),
    NamedParameter::new("inw", "interval-name-width", ParamType::Integer),
    NamedParameter::new("interval-name-title", "interval-name-title", ParamType::String),
    NamedParameter::new("interval-title", "interval-name-title", ParamType::String),
    NamedParameter::new("intervals", "intervals", ParamType::String),
    NamedParameter::new("selection-start", "selection-start", ParamType::String),
    NamedParameter::new("selection-end", "selection-end", ParamType::String),
];

fn data_representation_format_code(value: &str) -> String {
    if value.contains("::") {
        return value.to_string();
    }
    match value {
        "Oct" => "bufferview::DataRepresentationFormat::Oct".to_string(),
        "Bin" => "bufferview::DataRepresentationFormat::Bin".to_string(),
        "Char" => "bufferview::DataRepresentationFormat::Char".to_string(),
        "Hex" | "Hex(Byte)" => {
            "bufferview::DataRepresentationFormat::Hex(bufferview::HexFormat::Byte)".to_string()
        }
        "Hex(Word)" => "bufferview::DataRepresentationFormat::Hex(bufferview::HexFormat::Word)".to_string(),
        "Hex(DWord)" => "bufferview::DataRepresentationFormat::Hex(bufferview::HexFormat::DWord)".to_string(),
        "Hex(QWord)" => "bufferview::DataRepresentationFormat::Hex(bufferview::HexFormat::QWord)".to_string(),
        "UInt(U8)" | "UInt" => {
            "bufferview::DataRepresentationFormat::UInt(bufferview::UIntFormat::U8)".to_string()
        }
        "UInt(U16)" => "bufferview::DataRepresentationFormat::UInt(bufferview::UIntFormat::U16)".to_string(),
        "UInt(U32)" => "bufferview::DataRepresentationFormat::UInt(bufferview::UIntFormat::U32)".to_string(),
        "UInt(U64)" => "bufferview::DataRepresentationFormat::UInt(bufferview::UIntFormat::U64)".to_string(),
        "Int(I8)" | "Int" => "bufferview::DataRepresentationFormat::Int(bufferview::IntFormat::I8)".to_string(),
        "Int(I16)" => "bufferview::DataRepresentationFormat::Int(bufferview::IntFormat::I16)".to_string(),
        "Int(I32)" => "bufferview::DataRepresentationFormat::Int(bufferview::IntFormat::I32)".to_string(),
        "Int(I64)" => "bufferview::DataRepresentationFormat::Int(bufferview::IntFormat::I64)".to_string(),
        "Float(Scientific32)" | "Float" => {
            "bufferview::DataRepresentationFormat::Float(bufferview::FloatFormat::Scientific32)".to_string()
        }
        "Float(Scientific64)" => {
            "bufferview::DataRepresentationFormat::Float(bufferview::FloatFormat::Scientific64)".to_string()
        }
        "Float(E4M3)" => "bufferview::DataRepresentationFormat::Float(bufferview::FloatFormat::E4M3)".to_string(),
        "Float(E5M2)" => "bufferview::DataRepresentationFormat::Float(bufferview::FloatFormat::E5M2)".to_string(),
        other => panic!(
            "Unknown data format: '{other}' !\nSupported shorthand values are: Hex, Hex(Word), Oct, Bin, Char, UInt(U8), Int(I8), Float(Scientific32), ..."
        ),
    }
}

fn columns_count_code(value: &str) -> String {
    if value.contains("::") {
        return value.to_string();
    }
    if value == "Auto" {
        return "bufferview::ColumnsCount::Auto".to_string();
    }
    if let Some(n) = value.strip_prefix("Fixed(").and_then(|s| s.strip_suffix(')')) {
        return format!("bufferview::ColumnsCount::Fixed({n})");
    }
    if let Ok(n) = value.parse::<u8>() {
        return format!("bufferview::ColumnsCount::Fixed({n})");
    }
    panic!("Unknown columns value: '{value}' ! Use Auto, Fixed(N), or an integer from 1 to 255.");
}

fn codepage_code(value: &str) -> String {
    if value.contains("::") {
        return value.to_string();
    }
    match value {
        "Default" => r#"bufferview::Codepage::new("Default")"#.to_string(),
        "ASCII" => "bufferview::Codepage::ASCII".to_string(),
        "CP437" => "bufferview::Codepage::CP437".to_string(),
        "WINDOWS_1252" => "bufferview::Codepage::WINDOWS_1252".to_string(),
        other => format!(r#"bufferview::Codepage::new("{other}")"#),
    }
}

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("bufferview", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control_with_template("BufferView", "new", "type");
    cb.add_param_value("buffer");
    cb.add_layout();
    cb.add_flags_parameter("flags", "bufferview::Flags", &FLAGS);
    cb.finish_control_initialization();

    if cb.has_parameter("offset") {
        cb.add("control.set_offset_format(");
        cb.add_enum_parameter("offset", "bufferview::OffsetFormat", &OFFSET_FORMAT, None);
        cb.add_line(");");
    }

    if cb.has_parameter("endian") {
        cb.add("control.set_endian(");
        cb.add_enum_parameter("endian", "bufferview::Endian", &ENDIAN, None);
        cb.add_line(");");
    }

    if cb.has_parameter("format") {
        let format = cb.get_value("format").unwrap().to_string();
        cb.add_line(
            format!(
                "control.set_data_representation_format({});",
                data_representation_format_code(&format)
            )
            .as_str(),
        );
    }

    if cb.has_parameter("columns") {
        if let Some(columns) = cb.get_i32("columns") {
            if !(1..=255).contains(&columns) {
                panic!("Parameter `columns` must be between 1 and 255, got {columns}");
            }
            cb.add_line(format!("control.set_columns_count(bufferview::ColumnsCount::Fixed({columns}));").as_str());
        } else {
            let columns = cb.get_value("columns").unwrap().to_string();
            cb.add_line(format!("control.set_columns_count({});", columns_count_code(&columns)).as_str());
        }
    }

    if cb.has_parameter("codepage") {
        let codepage = cb.get_value("codepage").unwrap().to_string();
        cb.add_line(format!("control.set_codepage({});", codepage_code(&codepage)).as_str());
    }

    if let Some(width) = cb.get_i32("address-width") {
        if width < 0 {
            panic!("Parameter `address-width` can not be negative");
        }
        cb.add_line(format!("control.set_address_width({width} as u32);").as_str());
    }

    if cb.has_parameter("address-name") {
        cb.add("control.set_address_name(");
        cb.add_string_parameter("address-name", None);
        cb.add_line(");");
    }

    if let Some(width) = cb.get_i32("interval-name-width") {
        if width < 0 {
            panic!("Parameter `interval-name-width` can not be negative");
        }
        cb.add_line(format!("control.set_interval_name_width({width} as u32);").as_str());
    }

    if cb.has_parameter("interval-name-title") {
        cb.add("control.set_interval_name_title(");
        cb.add_string_parameter("interval-name-title", None);
        cb.add_line(");");
    }

    if cb.has_parameter("intervals") {
        let intervals = cb.get_value("intervals").unwrap().to_string();
        cb.add_line(format!("control.set_intervals({intervals});").as_str());
    }

    if cb.has_parameter("selection-start") || cb.has_parameter("selection-end") {
        if !(cb.has_parameter("selection-start") && cb.has_parameter("selection-end")) {
            panic!("Both `selection-start` and `selection-end` must be provided together");
        }
        let start = cb.get_value("selection-start").unwrap().to_string();
        let end = cb.get_value("selection-end").unwrap().to_string();
        cb.add_line(format!("control.set_selection({start}, {end});").as_str());
    }

    cb.add_scroll_margin_setup("lsm", "tsm");
    cb.add_basecontrol_operations();
    cb.into()
}
