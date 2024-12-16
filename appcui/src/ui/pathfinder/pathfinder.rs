use super::initialization_flags::Flags;
use crate::prelude::*;
use navigator::Navigator;
use pathfinder::inner_pathfinder::InnerPathFinder;
use std::path::{Path, PathBuf};

macro_rules! IMPLEMENT_METHODS  {
    ($data_type:ident, $navigator:ty, $($g:ident)?, $($constraints:tt)*) => {
        impl<$($g)?> OnPaint for $data_type<$($g)?> where $($constraints)* {
            fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
                self.inner.on_paint(surface, theme, &self.base);
            }
        }
        impl<$($g)?> OnKeyPressed for $data_type<$($g)?> where $($constraints)* {
            fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
                self.inner.on_key_pressed(key, character, &mut self.base)
            }
        }
        impl<$($g)?> OnExpand for $data_type<$($g)?> where $($constraints)* {
            fn on_expand(&mut self, direction: ExpandedDirection) {
                self.inner.on_expand(direction, &self.base);
            }
        }
        impl<$($g)?> OnResize for $data_type<$($g)?> where $($constraints)* {
            fn on_resize(&mut self, old_size: Size, new_size: Size) {
                self.inner.on_resize(&self.base, old_size, new_size);
            }
        }
        impl<$($g)?> OnFocus for $data_type<$($g)?> where $($constraints)* {
            fn on_focus(&mut self) {
                self.inner.on_focus(&mut self.base);
            }
            fn on_lose_focus(&mut self) {
                self.inner.on_lose_focus(&mut self.base);
            }
        }
        impl<$($g)?> $data_type<$($g)?> where $($constraints)* {
            pub(crate) fn with_navigator(file_path: &str, layout: Layout, flags: Flags, ns: $navigator) -> Self {
                let mut c = Self {
                    base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
                    inner: InnerPathFinder::new(file_path, ns, flags),
                };
                c.set_size_bounds(4, 1, u16::MAX, u16::MAX);
                c
            }
            pub fn path(&self) -> &Path {
                self.inner.path()
            }
        }
    };
}
#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnExpand+OnFocus+OnResize, internal=true)]
pub struct PathFinder {
    inner: InnerPathFinder<fs::Navigator>,
}

impl PathFinder {
    pub fn new(file_path: &str, layout: Layout, flags: Flags) -> Self {
        Self::with_navigator(file_path, layout, flags, fs::Navigator::new())
    }
}
IMPLEMENT_METHODS!(PathFinder, fs::Navigator,,);

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnExpand+OnFocus+OnResize, internal=true)]
pub(crate) struct GenericPathFinder<T>
where T: Navigator<fs::Entry, fs::Root, PathBuf> {
    inner: InnerPathFinder<T>,
}

IMPLEMENT_METHODS!(GenericPathFinder, T, T, T: Navigator<fs::Entry, fs::Root, PathBuf>);
