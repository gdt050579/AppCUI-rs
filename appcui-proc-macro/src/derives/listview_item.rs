use super::structure::Structure;
use super::structure::StructureField;
use core::panic;
use proc_macro::*;
use std::str::FromStr;
use std::u32;

static TEMPLATE: &str = r#"
impl listview::ListItem for $(STRUCT_NAME) {
    const COLUMNS_COUNT: u16 = $(COLUMNS_COUNT);
    fn column(index: u16) -> components::Column{ 
        match index {
            $(COLUMNS)
            _ => components::Column::new("", 10, TextAlignament::Left)
        }
    }
    fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
        match column_index {
            $(RENDER_METHODS)
            _ => None
        }
    }
    fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
        match column_index {
            $(COMPARE_METHODS)
            _ => std::cmp::Ordering::Equal
        }    
    }
}
"#;



#[derive(Debug, Copy, Clone)]
enum RenderMethod {
    Text,
    Ascii,
    Int64(&'static str),    
    UInt64(&'static str),   
    Float(&'static str), 
    Bool(&'static str),
    Size(&'static str),
    Area(&'static str),
    Distance(&'static str),
    Volume(&'static str),
    Weight(&'static str),
    Speed(&'static str),
    Percentage(&'static str),
    Temperature(&'static str),
    Currency(&'static str),
    Rating(&'static str, u32),
    Status(&'static str),
    DateTime(&'static str),
    Date(&'static str),
    Time(&'static str),
    Duration(&'static str),
}

impl RenderMethod {
    const NAME_TO_RENDER_METHOD: [(&'static str, Self); 26] = [
        ("Text", Self::Text),
        ("Ascii", Self::Ascii),
        ("Int64", Self::Int64("Normal")),
        ("Int", Self::Int64("Normal")),
        ("UInt64", Self::UInt64("Normal")),
        ("UInt", Self::UInt64("Normal")),
        ("Float", Self::Float("Normal")),
        ("Bool", Self::Bool("CheckmarkMinus")),
        ("Boolean", Self::Bool("CheckmarkMinus")),
        ("Checkmark", Self::Bool("CheckmarkMinus")),
        ("Size", Self::Size("Auto")),
        ("Area", Self::Area("SquaredMeters")),
        ("Distance", Self::Distance("Meters")),
        ("Volume", Self::Volume("CubicMeters")),
        ("Weight", Self::Weight("Kilograms")),
        ("Speed", Self::Speed("KilometersPerHour")),
        ("Percentage", Self::Percentage("Normal")),
        ("Temperature", Self::Temperature("Celsius")),
        ("Currency", Self::Currency("USD")),
        ("Rating", Self::Rating("Stars", 5)),
        ("Status", Self::Status("Graphical")),
        ("DateTime", Self::DateTime("Normal")),
        ("Date", Self::Date("Full")),
        ("Time", Self::Time("Normal")),
        ("Duration", Self::Duration("Auto")),
        ("TimeDelta", Self::Duration("Auto")),
    ];

    const NUMERIC_FORMATS: [&'static str; 6] = ["Normal", "Separator", "Hex", "Hex16", "Hex32", "Hex64"];
    const FLOAT_FORMATS: [&'static str; 4] = ["Normal","TwoDigits", "ThreeDigits", "FourDigits",];
    const BOOL_FORMATS: [&'static str; 4] = ["TrueFalse", "YesNo", "XMinus", "CheckmarkMinus"];
    const SIZE_FORMATS: [&'static str; 11] = ["Bytes","KiloBytes","MegaBytes","GigaBytes","TeraBytes","KiloBytesWithDecimals","MegaBytesWithDecimals","GigaBytesWithDecimals","TeraBytesWithDecimals","Auto","AutoWithDecimals"];
    const AREA_FORMATS: [&'static str; 10] = ["SquaredMillimeters","SquaredCentimeters","SquaredMeters","SquaredKilometers","Hectares","Ares","SquareFeet","SquareInches","SquareYards","SquareMiles"];
    const DISTANCE_FORMATS: [&'static str; 8] = ["Kilometers","Meters","Centimeters","Millimeters","Inches","Feet","Yards","Miles",];
    const VOLUME_FORMATS: [&'static str; 11] = ["CubicMilimeters","CubicCentimeters","CubicMeters","CubicKilometers","Liters","Milliliters","Gallons","CubicFeet","CubicInches","CubicYards","CubicMiles",];
    const WEIGHT_FORMATS: [&'static str; 5] = ["Grans","Milligrams","Kilograms","Pounds","Tons",];
    const SPEED_FORMATS: [&'static str; 9] = ["KilometersPerHour","MetersPerHour","KilometersPerSecond","MetersPerSecond","MilesPerHour","MilesPerSecond","Knots","FeetPerSecond","Mach",  ];
    const PERCENTAGE_FORMATS: [&'static str; 2] = ["Normal","Decimals"];
    const TEMPERATURE_FORMATS: [&'static str; 3] = ["Celsius","Fahrenheit","Kelvin"];
    const CURRENCY_FORMATS: [&'static str; 11] = ["USD","USDSymbol","EUR","EURSymbol","GBP","GBPSymbol","YEN","YENSymbol","Bitcoin","BitcoinSymbol","RON"];
    const RATNG_FORMATS: [&'static str; 4] = ["Numerical","Stars","Circles","Asterix"];
    const STATUS_FORMATS: [&'static str; 3] = ["Hashtag","Graphical","Arrow"];
    const DATETIME_FORMATS: [&'static str; 3] = ["Full","Normal","Short"];
    const TIME_FORMATS: [&'static str; 3] = ["Short","AMPM","Normal"];
    const DATE_FORMATS: [&'static str; 3] = ["Full","YearMonthDay","DayMonthYear"];
    const DURATION_FORMATS: [&'static str; 3] = ["Auto","Seconds","Details"];

    fn validate_format(self, fmt: &str, available: &[&'static str]) -> &'static str {
        for f in available {
            if crate::utils::equal_ignore_case(fmt, f) {
                return f;
            }
        }
        panic!(
            "Invalid format value: '{}' for render method '{}'. Available values are: {}",
            fmt,
            self.name(),
            available.join(", ")
        );
    }

    fn validate_rating(self, fmt: &str) -> (&'static str, u32) {
        let r_type = {
            let mut result = "";
            for f in RenderMethod::RATNG_FORMATS {
                if f.len() == fmt.len() && crate::utils::equal_ignore_case(f, fmt) {
                    // default case
                    return (f, 5);
                } else if f.len()<fmt.len() && crate::utils::equal_ignore_case(&fmt[..f.len()], f) {
                    result = f;
                    break;
                }
            }
            if result.is_empty() {
                panic!(
                    "Invalid rating format value: '{}' for render method '{}'. Available values are: {}",
                    fmt,
                    self.name(),
                    RenderMethod::RATNG_FORMATS.join(", ")
                );
            }
            result
        };
        let extra_part = fmt[r_type.len()..].trim();
        if !extra_part.starts_with('(') {
            panic!("Invalid rating format value: '{}' for render method '{}'. Expected format is: '{}(number)'", fmt, self.name(), r_type);
        }
        if !extra_part.ends_with(')') {
            panic!("Invalid rating format value: '{}' for render method '{}'. Expected format is: '{}(number)'", fmt, self.name(), r_type);
        }
        let number_representation = &extra_part[1..extra_part.len() - 1].trim();
        match number_representation.parse::<u32>() {
            Ok(v) => {
                if v==0 {
                    panic!("Invalid rating format value: '{}' for render method '{}'. The number of ratings must be greater than 0", fmt, self.name());
                }
                (r_type, v)
            },
            Err(_) => panic!(
                "Invalid numeric format value: '{}' for render method '{}'. Expected format is: '{}(number)'",
                fmt, self.name(), r_type
            ),
        }
    }

    fn from_type(vartype: &str)->Option<Self> {
        match vartype {
            "String" | "&str" => Some(Self::Text),
            "i8" | "i16" | "i32" | "i64" => Some(Self::Int64("Normal")),
            "u8" | "u16" | "u32" | "u64" => Some(Self::UInt64("Normal")),
            "f32" | "f64" => Some(Self::Float("Normal")),
            "bool" => Some(Self::Bool("CheckmarkMinus")),
            "Status" | "listview::Status" => Some(Self::Status("Graphical")),
            "chrono::NaiveDateTime" | "NaiveDateTime" => Some(Self::DateTime("Normal")),
            "chrono::NaiveDate" | "NaiveDate" => Some(Self::Date("Full")),
            "chrono::NaiveTime" | "NaiveTime" => Some(Self::Time("Normal")),
            "chrono::Duration" | "Duration" => Some(Self::Duration("Auto")),
            _ => None,
        }
    }
    fn from_str(value: &str) -> Option<Self> {
        for (text, variant) in &Self::NAME_TO_RENDER_METHOD {
            if crate::utils::equal_ignore_case(value, text) {
                return Some(*variant);
            }
        }
        None
    }
    fn name(&self) -> &'static str {
        match self {
            Self::Text => "Text",
            Self::Ascii => "Ascii",
            Self::Int64(_) => "Int64",
            Self::UInt64(_) => "UInt64",
            Self::Float(_) => "Float",
            Self::Bool(_) => "Bool",
            Self::Size(_) => "Size",
            Self::Area(_) => "Area",
            Self::Distance(_) => "Distance",
            Self::Volume(_) => "Volume",
            Self::Weight(_) => "Weight",
            Self::Speed(_) => "Speed",
            Self::Percentage(_) => "Percentage",
            Self::Temperature(_) => "Temperature",
            Self::Currency(_) => "Currency",
            Self::Rating(_,_) => "Rating",
            Self::Status(_) => "Status",
            Self::DateTime(_) => "DateTime",
            Self::Date(_) => "Date",
            Self::Time(_) => "Time",
            Self::Duration(_) => "Duration",
        }
    }
    fn update_format(&self, fmt: &str) -> Self {
        match self {
            Self::Text | Self::Ascii => *self,
            Self::Int64(_) => Self::Int64(self.validate_format(fmt, RenderMethod::NUMERIC_FORMATS.as_slice())),
            Self::UInt64(_) => Self::UInt64(self.validate_format(fmt, RenderMethod::NUMERIC_FORMATS.as_slice())),
            Self::Float(_) => Self::Float(self.validate_format(fmt, RenderMethod::FLOAT_FORMATS.as_slice())),
            Self::Bool(_) => Self::Bool(self.validate_format(fmt, RenderMethod::BOOL_FORMATS.as_slice())),
            Self::Size(_) => Self::Size(self.validate_format(fmt, RenderMethod::SIZE_FORMATS.as_slice())),
            Self::Area(_) => Self::Area(self.validate_format(fmt, RenderMethod::AREA_FORMATS.as_slice())),
            Self::Distance(_) => Self::Distance(self.validate_format(fmt, RenderMethod::DISTANCE_FORMATS.as_slice())),
            Self::Volume(_) => Self::Volume(self.validate_format(fmt, RenderMethod::VOLUME_FORMATS.as_slice())),
            Self::Weight(_) => Self::Weight(self.validate_format(fmt, RenderMethod::WEIGHT_FORMATS.as_slice())),
            Self::Speed(_) => Self::Speed(self.validate_format(fmt, RenderMethod::SPEED_FORMATS.as_slice())),
            Self::Percentage(_) => Self::Percentage(self.validate_format(fmt, RenderMethod::PERCENTAGE_FORMATS.as_slice())),
            Self::Temperature(_) => Self::Temperature(self.validate_format(fmt, RenderMethod::TEMPERATURE_FORMATS.as_slice())),
            Self::Currency(_) => Self::Currency(self.validate_format(fmt, RenderMethod::CURRENCY_FORMATS.as_slice())),
            Self::Rating(_,_) => {
                let (r_type, number) = self.validate_rating(fmt);
                Self::Rating(r_type, number)
            }
            Self::Status(_) => Self::Status(self.validate_format(fmt, RenderMethod::STATUS_FORMATS.as_slice())),
            Self::DateTime(_) => Self::DateTime(self.validate_format(fmt, RenderMethod::DATETIME_FORMATS.as_slice())),
            Self::Date(_) => Self::Date(self.validate_format(fmt, RenderMethod::DATE_FORMATS.as_slice())),
            Self::Time(_) => Self::Time(self.validate_format(fmt, RenderMethod::TIME_FORMATS.as_slice())),
            Self::Duration(_) => Self::Duration(self.validate_format(fmt, RenderMethod::DURATION_FORMATS.as_slice())),
        }
    }
    fn renderer_code(&self, index: usize, varname: &str, vartype: &str) -> String {
        match self {           
            Self::Text | Self::Ascii => {
                match vartype {
                    "String" | "&str" => format!("{} => Some(listview::RenderMethod::{}(self.{})),\n", index,self.name(),varname),
                    _ => panic!("Unsupported rendering method '{}' for type '{}', for field '{}'. Implement ListItem manually to provide explicit implementation for this type !", self.name(),vartype, varname),
                }
            }
            Self::Int64(fmt) => {
                match vartype {
                    "u8" | "u16" | "u32" | "i8" | "i16" | "i32" | "i64"  => format!("{} => Some(listview::RenderMethod::{}(self.{} as i64, listview::NumericFormat::{})),\n", index,self.name(),varname, *fmt),
                    _ => panic!("Unsupported rendering method '{}' for type '{}', for field '{}'. Implement ListItem manually to provide explicit implementation for this type !", self.name(),vartype, varname),
                }
            }
            Self::UInt64(fmt) => {
                match vartype {
                    "u8" | "u16" | "u32" | "u64"  => format!("{} => Some(listview::RenderMethod::{}(self.{} as u64, listview::NumericFormat::{})),\n", index,self.name(),varname, *fmt),
                    _ => panic!("Unsupported rendering method '{}' for type '{}', for field '{}'. Implement ListItem manually to provide explicit implementation for this type !", self.name(),vartype, varname),
                }
            }
            Self::Float(fmt) => {
                match vartype {
                    "f32" | "f64" => format!("{} => Some(listview::RenderMethod::{}(self.{} as f64, listview::FloatFormat::{})),\n", index,self.name(),varname, *fmt),
                    "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => format!("{} => Some(listview::RenderMethod::{}(self.{} as f64, listview::FloatFormat::{})),\n", index,self.name(),varname, *fmt),
                    _ => panic!("Unsupported rendering method '{}' for type '{}', for field '{}'. Implement ListItem manually to provide explicit implementation for this type !", self.name(),vartype, varname),
                }
            }
            Self::Bool(fmt) => {
                match vartype {
                    "bool" => format!("{} => Some(listview::RenderMethod::{}(self.{}, listview::BoolFormat::{})),\n", index,self.name(),varname, *fmt),
                    "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => format!("{} => Some(listview::RenderMethod::{}(self.{} != 0, listview::BoolFormat::{})),\n", index,self.name(),varname, *fmt), 
                    "usize" | "u128" | "i128" | "isize" => format!("{} => Some(listview::RenderMethod::{}(self.{} != 0, listview::BoolFormat::{})),\n", index,self.name(),varname, *fmt),   
                    _ => panic!("Unsupported rendering method '{}' for type '{}', for field '{}'. Implement ListItem manually to provide explicit implementation for this type !", self.name(),vartype, varname),
                }
            }
            Self::Area(fmt)|Self::Size(fmt)|Self::Distance(fmt)|Self::Volume(fmt)|Self::Weight(fmt)|Self::Speed(fmt) => {
                match vartype {
                    "u8" | "u16" | "u32" | "u64"  => format!("{} => Some(listview::RenderMethod::{}(self.{} as u64, listview::{}Format::{})),\n", index,self.name(),varname,self.name(), *fmt),
                    _ => panic!("Unsupported rendering method '{}' for type '{}', for field '{}'. Implement ListItem manually to provide explicit implementation for this type !", self.name(),vartype, varname),
                }
            }
            Self::Percentage(fmt) => {
                match vartype {
                    "f32" | "f64" => format!("{} => Some(listview::RenderMethod::{}(self.{} as f64, listview::PercentageFormat::{})),\n", index,self.name(),varname, *fmt),
                    "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => format!("{} => Some(listview::RenderMethod::{}((self.{} as f64)/100.0, listview::PercentageFormat::{})),\n", index,self.name(),varname, *fmt),
                    _ => panic!("Unsupported rendering method '{}' for type '{}', for field '{}'. Implement ListItem manually to provide explicit implementation for this type !", self.name(),vartype, varname),
                }
            }
            Self::Temperature(fmt)|Self::Currency(fmt) => {
                match vartype {
                    "f32" | "f64" | "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => format!("{} => Some(listview::RenderMethod::{}(self.{} as f64, listview::{}Format::{})),\n", index,self.name(),varname,self.name(), *fmt),
                    _ => panic!("Unsupported rendering method '{}' for type '{}', for field '{}'. Implement ListItem manually to provide explicit implementation for this type !", self.name(),vartype, varname),
                }
            }
            Self::Rating(fmt, number) => {
                match vartype {
                    "u8" | "u16" | "u32"   => format!("{} => Some(listview::RenderMethod::{}(self.{} as u32, listview::RatingFormat::{}({}))),\n", index,self.name(),varname, *fmt, number),
                    _ => panic!("Unsupported rendering method '{}' for type '{}', for field '{}'. Implement ListItem manually to provide explicit implementation for this type !", self.name(),vartype, varname),
                }
            }
            Self::Status(fmt) => {
                match vartype {
                    "Status" | "listview::Status" => format!("{} => Some(listview::RenderMethod::{}(self.{}, listview::StatusFormat::{})),\n", index,self.name(),varname, *fmt),
                    _ => panic!("Unsupported rendering method '{}' for type '{}', for field '{}'. Implement ListItem manually to provide explicit implementation for this type !", self.name(),vartype, varname),
                }
            }
            Self::DateTime(fmt) => {
                match vartype {
                    "chrono::NaiveDateTime" | "NaiveDateTime"  => format!("{} => Some(listview::RenderMethod::{}(self.{}, listview::DateTimeFormat::{})),\n", index,self.name(),varname, *fmt),
                    _ => panic!("Unsupported rendering method '{}' for type '{}', for field '{}'. Implement ListItem manually to provide explicit implementation for this type !", self.name(),vartype, varname),
                }
            }
            Self::Date(fmt) => {
                match vartype {
                    "chrono::NaiveDate" | "NaiveDate"  => format!("{} => Some(listview::RenderMethod::{}(self.{}, listview::DateFormat::{})),\n", index,self.name(),varname, *fmt),
                    _ => panic!("Unsupported rendering method '{}' for type '{}', for field '{}'. Implement ListItem manually to provide explicit implementation for this type !", self.name(),vartype, varname),
                }
            }
            Self::Time(fmt) => {
                match vartype {
                    "chrono::NaiveTime" | "NaiveTime"  => format!("{} => Some(listview::RenderMethod::{}(self.{}, listview::TimeFormat::{})),\n", index,self.name(),varname, *fmt),
                    _ => panic!("Unsupported rendering method '{}' for type '{}', for field '{}'. Implement ListItem manually to provide explicit implementation for this type !", self.name(),vartype, varname),
                }
            }
            Self::Duration(fmt) => {
                match vartype {
                    "chrono::Duration" | "Duration"  => format!("{} => Some(listview::RenderMethod::{}(self.{}, listview::DurationFormat::{})),\n", index,self.name(),varname, *fmt),
                    "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" => format!("{} => Some(listview::RenderMethod::{}(chrono::Duration::seconds(self.{} as i64) , listview::DurationFormat::{})),\n", index,self.name(),varname, *fmt),
                    _ => panic!("Unsupported rendering method '{}' for type '{}', for field '{}'. Implement ListItem manually to provide explicit implementation for this type !", self.name(),vartype, varname),
                }
            }
            
        }
    }
}

struct Column {
    name: String,
    width: u16,
    index: u32,
    align: &'static str,
    varname: String,
    vartype: String,
    render: RenderMethod,
}
impl Column {
    fn align(value: &str) -> Option<&'static str> {
        if crate::utils::equal_ignore_case(value, "left") {
            Some("Left")
        } else if crate::utils::equal_ignore_case(value, "l") {
            return Some("Left");
        } else if crate::utils::equal_ignore_case(value, "right") {
            return Some("Right");
        } else if crate::utils::equal_ignore_case(value, "r") {
            return Some("Right");
        } else if crate::utils::equal_ignore_case(value, "center") {
            return Some("Center");
        } else if crate::utils::equal_ignore_case(value, "c") {
            return Some("Center");
        } else {
            None
        }
    }
    fn try_from(field: &StructureField) -> Option<Self> {
        if field.attributes.is_empty() {
            return None;
        }
        let mut name = None;
        let mut align = None;
        let mut width = None;
        let mut idx = u32::MAX;
        let mut render = None;
        let mut format = None;
        for (attr_name, value) in field.attributes.iter() {
            if !attr_name.starts_with("Column.") {
                continue;
            }
            match attr_name[7..].trim() {
                "align" | "a" | "alignament" => {
                    align = Column::align(value);
                    if align.is_none() {
                        panic!(
                            "Unknown alignament value: '{}' for field '{}'. Allowed values are 'left', 'right' or 'center' !",
                            value, field.name
                        );
                    }
                }
                "name" | "text" | "caption" => {
                    name = Some(value.to_string());
                }
                "width" | "w" | "size" => {
                    if let Ok(v) = value.parse::<u16>() {
                        if v > 200 {
                            panic!(
                                "Invalid width value: '{}' for field '{}'. Maximum allowed value is 200 !",
                                value, field.name
                            );
                        }
                        width = Some(v);
                    } else {
                        panic!("Invalid numerical value for width: '{}' for field '{}'", value, field.name);
                    }
                }
                "index" | "idx" => {
                    if let Ok(idx_value) = value.parse::<u32>() {
                        idx = idx_value;
                    } else {
                        panic!("Invalid numerical value for column index: '{}' for field '{}'", value, field.name);
                    }
                }
                "render" | "r" => {
                    if let Some(render_method) = RenderMethod::from_str(value) {
                        render = Some(render_method);
                    } else {
                        panic!(
                            "Unknown render method: '{}' for field '{}'. Available render methods are: 'text', 'ascii', 'int64', 'int' !",
                            value, field.name
                        );
                    }
                }
                "format" | "fmt" | "f" => {
                    format = Some(value.as_str());
                }
                _ => {
                    panic!(
                        "Unknown attribute: '{}' for field '{}'. Available attributes are: 'name', 'align', 'width', 'index', 'render' and 'format' !",
                        &attr_name[7..],
                        field.name
                    );
                }
            }
        }
        if name.is_none() {
            panic!("Missing 'name' attribute for field '{}'", field.name);
        }   
        if render.is_none() {
            if format.is_some() {
                panic!(
                    "Missing 'render' attribute for field '{}', but 'format' attribute is specified. You need to specify the rendering method !",
                    field.name
                );
            }
            render = RenderMethod::from_type(&field.ty);
            if render.is_none() {
                panic!(
                    "There is no default rendering method for field '{}' of type '{}'. You need to manually implement ListItem and specify the rendering method !",
                    field.name, field.ty
                );
            }
        }
        let r = if let Some(fmt) = format {
            render.unwrap().update_format(fmt)
        } else {
            render.unwrap()
        };
        Some(Self {
            name: name.unwrap(),
            index: idx,
            width: width.unwrap_or(10),
            align: align.unwrap_or("Left"),
            varname: field.name.clone(),
            vartype: field.ty.clone(),
            render: r,
        })
    }
    fn to_column_code(&self, index: usize) -> String {
        format!(
            "{} => components::Column::new(\"{}\", {}, TextAlignament::{}),\n",
            index, self.name, self.width, self.align
        )
    }
    fn to_compare_code(&self, index: usize) -> String {
        match self.vartype.as_str() {
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => {
                format!("{} => self.{}.cmp(&other.{}),\n", index, self.varname, self.varname)
            }
            "f32" | "f64" => {
                format!(
                    "{} => self.{}.partial_cmp(&other.{}).unwrap_or(std::cmp::Ordering::Equal),\n",
                    index, self.varname, self.varname
                )
            }
            "bool" => {
                format!("{} => self.{}.cmp(&other.{}),\n", index, self.varname, self.varname)
            }
            "char" => {
                format!("{} => self.{}.cmp(&other.{}),\n", index, self.varname, self.varname)
            }
            "String" => {
                format!("{} => self.{}.cmp(&other.{}),\n", index, self.varname, self.varname)
            }
            "&str" => {
                format!("{} => self.{}.cmp(other.{}),\n", index, self.varname, self.varname)
            }
            "Status" | "listview::Status" => {
                format!("{} => self.{}.cmp(&other.{}),\n", index, self.varname, self.varname)
            }
            "chrono::NaiveDateTime" | "NaiveDateTime" => {
                format!("{} => self.{}.cmp(&other.{}),\n", index, self.varname, self.varname)
            }
            "chrono::NaiveDate" | "NaiveDate" => {
                format!("{} => self.{}.cmp(&other.{}),\n", index, self.varname, self.varname)
            }
            "chrono::NaiveTime" | "NaiveTime" => {
                format!("{} => self.{}.cmp(&other.{}),\n", index, self.varname, self.varname)
            }
            "chrono::Duration" | "Duration" => {
                format!("{} => self.{}.cmp(&other.{}),\n", index, self.varname, self.varname)
            }
            _ => panic!(
                "Unsupported type '{}' for field '{}'. Implement ListItem manually to provide explicit implementation for this type !",
                self.vartype, self.varname
            ),
        }
    }
    fn to_render_code(&self, index: usize) -> String {
        self.render.renderer_code(index, &self.varname, &self.vartype)
    }
}

fn build_derive_code(s: &Structure) -> TokenStream {
    let mut columns = Vec::with_capacity(s.fields.len());
    let mut last_index = u32::MAX;
    let mut first_index = u32::MAX;
    for field in &s.fields {
        if let Some(mut column) = Column::try_from(field) {
            if column.index == u32::MAX {
                if last_index == u32::MAX {
                    column.index = 0;
                } else {
                    column.index = last_index + 1;
                }
            }
            if first_index == u32::MAX {
                first_index = column.index;
            }
            last_index = column.index;
            columns.push(column);
        }
    }
    if columns.is_empty() {
        panic!(
            "No columns defined for structure '{}' (use #[Column(...)] before structure fields to specify column information)",
            s.name
        );
    }
    if first_index > 1 {
        panic!("Invalid column index. The first column index must be 0 or 1, but it is {}", first_index);
    }
    if last_index + 1 - first_index != columns.len() as u32 {
        panic!("Invalid column indexes. Make sure that all column indexes are unique and sequential starting from 0 or 1");
    }
    columns.sort_by(|a, b| a.index.cmp(&b.index));
    let mut columns_code = String::new();
    let mut compare_code = String::new();
    let mut render_code = String::new();
    for (index, column) in columns.iter().enumerate() {
        columns_code.push_str(&column.to_column_code(index));
        compare_code.push_str(&column.to_compare_code(index));
        render_code.push_str(&column.to_render_code(index));
    }

    let output = TEMPLATE
        .replace("$(STRUCT_NAME)", &s.name)
        .replace("$(COLUMNS_COUNT)", &columns.len().to_string())
        .replace("$(COLUMNS)", &columns_code)
        .replace("$(COMPARE_METHODS)", &compare_code)
        .replace("$(RENDER_METHODS)", &render_code);
    TokenStream::from_str(&output).expect("Fail to convert structure derived definition to token stream")
}

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    match Structure::from(input) {
        Ok(s) => build_derive_code(&s),
        Err(desc) => panic!("Fail to derive ListItem for structure: {}", desc),
    }
}
