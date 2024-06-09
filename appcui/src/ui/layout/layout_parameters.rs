use super::anchors::Anchors;
use super::Alignament;
use super::Coordonate16;
use super::Parameter;
use super::Dimension16;
use crate::utils::KeyValueParser;
use crate::utils::ValueType;

pub(super) struct LayoutParameters {
    pub x: Option<Coordonate16>,
    pub y: Option<Coordonate16>,
    pub width: Option<Dimension16>,
    pub height: Option<Dimension16>,
    pub a_left: Option<Coordonate16>,
    pub a_right: Option<Coordonate16>,
    pub a_top: Option<Coordonate16>,
    pub a_bottom: Option<Coordonate16>,
    pub align: Option<Alignament>,
    pub dock: Option<Alignament>,
}
impl Default for LayoutParameters {
    #[inline]
    fn default() -> Self {
        LayoutParameters {
            x: None,
            y: None,
            width: None,
            height: None,
            a_left: None,
            a_right: None,
            a_top: None,
            a_bottom: None,
            align: None,
            dock: None,
        }
    }
}
impl LayoutParameters {
    pub(super) fn new(format: &str) -> LayoutParameters {
        let mut inf = LayoutParameters::default();
        let mut parser = KeyValueParser::new(format);

        while let Some(p) = parser.next() {
            if let Some(param) = Parameter::from_hash(p.key_hash) {
                match param {
                    Parameter::X => {
                        if p.is_numerical_value() {
                            inf.x = Coordonate16::new(p);
                        } else {
                            panic!("Invalid value for X parameter: {} in layout: {} (it should be a numerical or percentage value)",p.value,format);
                        }
                    }
                    Parameter::Y => {
                        if p.is_numerical_value() {
                            inf.y = Coordonate16::new(p);
                        } else {
                            panic!("Invalid value for Y parameter: {} in layout: {} (it should be a numerical or percentage value)",p.value,format);
                        }
                    }
                    Parameter::Left => {
                        if p.is_numerical_value() {
                            inf.a_left = Coordonate16::new(p);
                        } else {
                            panic!("Invalid value for LEFT parameter: {} in layout: {} (it should be a numerical or percentage value)",p.value,format);
                        }
                    }
                    Parameter::Right => {
                        if p.is_numerical_value() {
                            inf.a_right = Coordonate16::new(p);
                        } else {
                            panic!("Invalid value for RIGHT parameter: {} in layout: {} (it should be a numerical or percentage value)",p.value,format);
                        }
                    }
                    Parameter::Top => {
                        if p.is_numerical_value() {
                            inf.a_top = Coordonate16::new(p);
                        } else {
                            panic!("Invalid value for TOP parameter: {} in layout: {} (it should be a numerical or percentage value)",p.value,format);
                        }
                    }
                    Parameter::Bottom => {
                        if p.is_numerical_value() {
                            inf.a_bottom = Coordonate16::new(p);
                        } else {
                            panic!("Invalid value for BOTTOM parameter: {} in layout: {} (it should be a numerical or percentage value)",p.value,format);
                        }
                    }
                    Parameter::Width => {
                        if p.is_numerical_value() {
                            if p.numerical_value < 0 {
                                panic!("The value for WIDTH parameter can not be a negative value: {} in layout: {}",p.value,format);
                            }
                            inf.width = Dimension16::new(p);
                        } else {
                            panic!("Invalid value for WIDTH parameter: {} in layout: {} (it should be a numerical or percentage positive value)",p.value,format);
                        }
                    }
                    Parameter::Height => {
                        if p.is_numerical_value() {
                            if p.numerical_value < 0 {
                                panic!("The value for HEIGHT parameter can not be a negative value: {} in layout: {}",p.value,format);
                            }
                            inf.height = Dimension16::new(p);
                        } else {
                            panic!("Invalid value for HEIGHT parameter: {} in layout: {} (it should be a numerical or percentage positive value)",p.value,format);
                        }
                    }
                    Parameter::Align => {
                        if p.value_type != ValueType::String {
                            panic!(
                                "Invalid value for 'align' parameter: {} in layout: {}",
                                p.value, format
                            );
                        }
                        if let Some(a) = Alignament::from_hash(p.value_hash) {
                            inf.align = Some(a);
                        } else {
                            panic!(
                                "Invalid value for 'align' parameter: {} in layout: {}",
                                p.value, format
                            );
                        }
                    }
                    Parameter::Dock => {
                        if p.value_type != ValueType::String {
                            panic!(
                                "Invalid value for 'dock' parameter: {} in layout: {}",
                                p.value, format
                            );
                        }
                        if let Some(d) = Alignament::from_hash(p.value_hash) {
                            inf.dock = Some(d);
                        } else {
                            panic!(
                                "Invalid value for 'dock' parameter: {} in layout: {}",
                                p.value, format
                            );
                        }
                    }
                }
            } else {
                panic!("Unknwon layout parameter: {}", p.key);
            }
        }
        inf
    }

    pub(super) fn get_anchors(&self) -> Anchors {
        Anchors::new(
            self.a_left.is_some(),
            self.a_top.is_some(),
            self.a_right.is_some(),
            self.a_bottom.is_some(),
        )
    }
}
