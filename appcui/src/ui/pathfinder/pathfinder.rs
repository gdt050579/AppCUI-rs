use navigator::Navigator;
use std::path::PathBuf;
use super::initialization_flags::Flags;
use crate::prelude::*;
use crate::utils::fs::*;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize+OnExpand+OnFocus, internal=true)]
pub struct PathFinder<T>
where
    T: crate::utils::Navigator<Entry, Root, PathBuf>,
{
    flags: Flags,
    navigator: T,
    component: crate::ui::components::NavigatorComponent<T, Entry, Root, PathBuf>,
}

impl<T> PathFinder<T>
where
    T: crate::utils::Navigator<Entry, Root, PathBuf>,
{
    pub(crate) fn with_navigator(file_path: &str, layout: Layout, flags: Flags, nav: T) -> Self {
        let mut c = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            navigator: nav,
            component: crate::ui::components::NavigatorComponent::new(file_path, flags.contains(Flags::ReadOnly)),
        };
        c.set_size_bounds(4, 1, u16::MAX, u16::MAX);
        c
    }

    pub fn new(file_path: &str, layout: Layout, flags: Flags) -> PathFinder<crate::utils::fs::Navigator> {
        PathFinder::<crate::utils::fs::Navigator>::with_navigator(
            file_path, layout, flags, crate::utils::fs::Navigator::new())
    }
}

impl<T> OnPaint for PathFinder<T>
where
    T: crate::utils::Navigator<Entry, Root, PathBuf>,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        self.component.on_paint(&self.base, surface, theme);
    }
}

impl<T> OnKeyPressed for PathFinder<T>
where
    T: crate::utils::Navigator<Entry, Root, PathBuf>,
{
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        self.component.on_key_pressed(&mut self.base, key, character, &self.navigator)
    }
}

impl<T> OnMouseEvent for PathFinder<T> where T: crate::utils::Navigator<Entry, Root, PathBuf> {}

impl<T> OnResize for PathFinder<T>
where
    T: crate::utils::Navigator<Entry, Root, PathBuf>,
{
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {}
}

impl<T> OnExpand for PathFinder<T>
where
    T: crate::utils::Navigator<Entry, Root, PathBuf>,
{
    fn on_expand(&mut self, direction: ExpandedDirection) {
        self.component.on_expand(&self.base, direction);
    }

    fn on_pack(&mut self) {}
}

impl<T> OnFocus for PathFinder<T>
where
    T: crate::utils::Navigator<Entry, Root, PathBuf>,
{
    fn on_focus(&mut self) {
        self.component.on_focus(&mut self.base);
    }

    fn on_lose_focus(&mut self) {}
}
