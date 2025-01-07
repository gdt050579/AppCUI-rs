use std::marker::PhantomData;

use AppCUIProcMacro::*;
use super::Flags;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct TreeView<T>
where
    T: ListItem + 'static,
{
    flags: Flags,
    data: PhantomData<T>
}
impl<T> OnPaint for TreeView<T> where T: ListItem + 'static {}
impl<T> OnKeyPressed for TreeView<T> where T: ListItem + 'static {}
impl<T> OnMouseEvent for TreeView<T> where T: ListItem + 'static {}
impl<T> OnResize for TreeView<T> where T: ListItem + 'static {}