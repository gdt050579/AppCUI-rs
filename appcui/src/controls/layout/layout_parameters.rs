use super::Alignament;
use super::Coordonate;
use super::Size;
use super::Parameter;
use crate::utils::KeyValueParser;
use crate::utils::ValueType;
use EnumBitFlags::EnumBitFlags;


#[EnumBitFlags(bits:16)]
pub enum LayoutUsedParams {
    X = 0x0001,
    Y = 0x0002,
    LEFT = 0x0004,
    RIGHT = 0x0008,
    TOP = 0x0010,
    BOTTOM = 0x0020,
    WIDTH = 0x0040,
    HEIGHT = 0x0080,
    ALIGN = 0x0100,
    DOCK = 0x0200,
}
pub(super) struct LayoutParameters {
    pub x: Coordonate,
    pub y: Coordonate,
    pub width: Size,
    pub height: Size,
    pub a_left: Coordonate,
    pub a_right: Coordonate,
    pub a_top: Coordonate,
    pub a_bottom: Coordonate,
    pub used_params: LayoutUsedParams,
    pub align: Alignament,
    pub dock: Alignament,
}
impl Default for LayoutParameters {
    #[inline]
    fn default() -> Self {
        LayoutParameters {
            x: Coordonate::Absolute(0),
            y: Coordonate::Absolute(0),
            width: Size::Absolute(0),
            height: Size::Absolute(0),
            a_left: Coordonate::Absolute(0),
            a_right: Coordonate::Absolute(0),
            a_top: Coordonate::Absolute(0),
            a_bottom: Coordonate::Absolute(0),
            used_params: LayoutUsedParams::None,
            align: Alignament::TopLeft,
            dock: Alignament::TopLeft,
        }    
    }
}
impl LayoutParameters {

    pub (super) fn new(format: &str) -> LayoutParameters {
        let mut inf = LayoutParameters::default();
        let mut parser = KeyValueParser::new(format);

        while let Some(p) = parser.next() {
            if let Some(param) = Parameter::from_hash(p.key_hash)
            {
                match param {
                    Parameter::X => {
                        if p.is_numerical_value() {
                            inf.x = Coordonate::new(&p);
                            inf.used_params |= LayoutUsedParams::X;
                        } else {
                            panic!("Invalid value for X parameter: {} in layout: {} (it should be a numerical or percentage value)",p.value,format);
                        }
                    },
                    Parameter::Y => {
                        if p.is_numerical_value() {
                            inf.y = Coordonate::new(&p);
                            inf.used_params |= LayoutUsedParams::Y;
                        } else {
                            panic!("Invalid value for Y parameter: {} in layout: {} (it should be a numerical or percentage value)",p.value,format);
                        }
                    },
                    Parameter::Left => {
                        if p.is_numerical_value() {
                            inf.a_left = Coordonate::new(&p);
                            inf.used_params |= LayoutUsedParams::LEFT;
                        } else {
                            panic!("Invalid value for LEFT parameter: {} in layout: {} (it should be a numerical or percentage value)",p.value,format);
                        }
                    },
                    Parameter::Right => {
                        if p.is_numerical_value() {
                            inf.a_right = Coordonate::new(&p);
                            inf.used_params |= LayoutUsedParams::RIGHT;
                        } else {
                            panic!("Invalid value for RIGHT parameter: {} in layout: {} (it should be a numerical or percentage value)",p.value,format);
                        }
                    },
                    Parameter::Top => {
                        if p.is_numerical_value() {
                            inf.a_top = Coordonate::new(&p);
                            inf.used_params |= LayoutUsedParams::TOP;
                        } else {
                            panic!("Invalid value for TOP parameter: {} in layout: {} (it should be a numerical or percentage value)",p.value,format);
                        }
                    },
                    Parameter::Bottom => {
                        if p.is_numerical_value() {
                            inf.a_bottom = Coordonate::new(&p);
                            inf.used_params |= LayoutUsedParams::BOTTOM;
                        } else {
                            panic!("Invalid value for BOTTOM parameter: {} in layout: {} (it should be a numerical or percentage value)",p.value,format);
                        }
                    },
                    Parameter::Width => {
                        if p.is_numerical_value() {
                            if p.numerical_value<0 {
                                panic!("The value for WIDTH parameter can not be a negative value: {} in layout: {}",p.value,format);
                            }
                            inf.width = Size::new(&p);
                            inf.used_params |= LayoutUsedParams::WIDTH;
                        } else {
                            panic!("Invalid value for WIDTH parameter: {} in layout: {} (it should be a numerical or percentage positive value)",p.value,format);
                        }
                    },
                    Parameter::Height => {
                        if p.is_numerical_value() {
                            if p.numerical_value<0 {
                                panic!("The value for HEIGHT parameter can not be a negative value: {} in layout: {}",p.value,format);
                            }
                            inf.height = Size::new(&p);
                            inf.used_params |= LayoutUsedParams::HEIGHT;
                        } else {
                            panic!("Invalid value for HEIGHT parameter: {} in layout: {} (it should be a numerical or percentage positive value)",p.value,format);
                        }
                    },
                    Parameter::Align => {
                        if p.value_type != ValueType::String {
                            panic!("Invalid value for dock parameter: {} in layout: {}",p.value,format);
                        }
                        if let Some(d) = Alignament::from_hash(p.value_hash) {
                            inf.dock = d;
                            inf.used_params |= LayoutUsedParams::DOCK;
                        } else {
                            panic!("Invalid value for dock parameter: {} in layout: {}",p.value,format);
                        }
                    },
                    Parameter::Dock => {
                        if p.value_type != ValueType::String {
                            panic!("Invalid value for alignament parameter: {} in layout: {}",p.value,format);
                        }
                        if let Some(a) = Alignament::from_hash(p.value_hash) {
                            inf.align = a;
                            inf.used_params |= LayoutUsedParams::ALIGN;
                        } else {
                            panic!("Invalid value for alignament parameter: {} in layout: {}",p.value,format);
                        }
                    }
                }
            } else {
                panic!("Unknwon layout parameter: {}",p.key);
            }
        }
        return inf;
    }
}
