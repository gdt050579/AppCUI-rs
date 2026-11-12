use crate::{graphics::{Size, Surface}, prelude::Number};

pub(crate) struct Cell<T: Number> {
    value: T,
    label: String,

}
impl<T: Number> Cell<T> {
    pub(super) fn paint(&self, surface: &mut Surface, x: i32, y: i32, size: Size) {

    }
}