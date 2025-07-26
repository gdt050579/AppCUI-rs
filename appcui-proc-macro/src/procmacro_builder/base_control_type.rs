#[derive(Copy,Clone,Eq,PartialEq,)]
pub(crate) enum BaseControlType {
    Window,
    Desktop,
    ModalWindow,
    CustomControl,
    CustomContainer,
}
impl BaseControlType {
    pub (crate) fn as_string(&self) -> String {
        String::from(match self {
            BaseControlType::Window => "Window",
            BaseControlType::Desktop => "Desktop",
            BaseControlType::ModalWindow => "ModalWindow<()>",
            BaseControlType::CustomControl => "ControlBase",
            BaseControlType::CustomContainer => "ContainerBase",
        })
    }
}
