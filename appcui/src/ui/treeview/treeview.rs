use super::{Flags, Item, TreeDataManager};
use AppCUIProcMacro::*;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct TreeView<T>
where
    T: ListItem + 'static,
{
    flags: Flags,
    manager: TreeDataManager<T>,
    filter: Vec<Handle<Item<T>>>,
    header: ColumnsHeader,
    comp: ListScrollBars,
    top_view: usize,
    pos: usize,
    icon_width: u8,
}
impl<T> TreeView<T>
where
    T: ListItem + 'static,
{
    #[inline(always)]
    fn visible_space(&self) -> Size {
        let mut sz = self.size();
        sz.height = sz.height.saturating_sub(1);
        sz
    }
}
impl<T> OnPaint for TreeView<T> where T: ListItem + 'static {}
impl<T> OnKeyPressed for TreeView<T> where T: ListItem + 'static {}
impl<T> OnMouseEvent for TreeView<T> where T: ListItem + 'static {}
impl<T> OnResize for TreeView<T>
where
    T: ListItem + 'static,
{
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.header.resize(new_size);
        self.comp
            .resize(self.header.width() as u64, self.filter.len() as u64, &self.base, self.visible_space());
    }
}
