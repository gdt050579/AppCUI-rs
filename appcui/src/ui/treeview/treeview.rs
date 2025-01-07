use super::{Flags, TreeDataManager};
use AppCUIProcMacro::*;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct TreeView<T>
where
    T: ListItem + 'static,
{
    flags: Flags,
    manager: TreeDataManager<T>,
}
impl<T> TreeView<T> where T: ListItem + 'static {}
impl<T> OnPaint for TreeView<T> where T: ListItem + 'static {}
impl<T> OnKeyPressed for TreeView<T> where T: ListItem + 'static {}
impl<T> OnMouseEvent for TreeView<T> where T: ListItem + 'static {}
impl<T> OnResize for TreeView<T> where T: ListItem + 'static {}
