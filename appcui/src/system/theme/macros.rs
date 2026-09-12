/// Builds a `ControlCharAttributesState` from five `charattr!` strings.
///
/// Used when defining a [`crate::system::Theme`]: each argument is a `charattr!`
/// literal (foreground, optional background, optional flags). The five values
/// map to the control states, in order:
///
/// 1. **normal** — enabled, not focused
/// 2. **focused** — the control has keyboard focus
/// 3. **hovered** — the mouse is over the control
/// 4. **inactive** — disabled
/// 5. **pressed_or_selected** — pressed, checked, or otherwise selected
///
/// `ControlCharAttributesState` and `charattr!` must be in scope at the call site
/// (theme modules import both).
///
/// # Examples
///
/// ```rust, ignore
/// use appcui::prelude::*;
/// use appcui::controlattr;
///
/// //                  normal         focused         hovered          inactive   pressed
/// let border = controlattr!("white,black", "black,white", "black,silver", "gray,?", "yellow,blue");
/// ```
#[macro_export]
macro_rules! controlattr {
    ($normal: literal, $focused: literal, $hovered: literal, $inactive: literal, $pressed: literal) => {
        ControlCharAttributesState {
            normal: charattr!($normal),
            focused: charattr!($focused),
            hovered: charattr!($hovered),
            inactive: charattr!($inactive),
            pressed_or_selected: charattr!($pressed),
        }       
    };
}