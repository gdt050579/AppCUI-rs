use crate::prelude::*;
use crate::ui::textfield::events::EventData;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct TextField {

}
impl OnPaint for TextField {}
impl OnKeyPressed for TextField {}
impl OnMouseEvent for TextField {}