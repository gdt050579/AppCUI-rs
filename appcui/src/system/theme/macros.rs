#[macro_export]
macro_rules! controlattr {
    ($normal: literal, $focused: literal, $hovered: literal, $inactive: literal, $pressed: literal) => {
        ControlCharAttributesState {
            normal: charattr!($normal),
            focused: charattr!($focused),
            hovered: charattr!($hovered),
            inactive: charattr!($inactive),
            pressed_or_selectd: charattr!($pressed),
        }       
    };
}