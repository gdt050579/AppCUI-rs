use super::initialization_flags::Flags;
use crate::prelude::*;
use navigator::Navigator;
use pathfinder::inner_pathfinder::InnerPathFinder;
use std::path::Path;

macro_rules! IMPLEMENT_METHODS  {
    ($data_type:ty) => {
        impl OnPaint for $data_type {
            fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
                self.inner.on_paint(surface, theme, &self.base);
            }
        }
        impl OnKeyPressed for $data_type {
            fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
                self.inner.on_key_pressed(key, character, &mut self.base)
            }
        }
        impl OnExpand for $data_type {
            fn on_expand(&mut self, direction: ExpandedDirection) {
                self.inner.on_expand(direction, &self.base);
            }
        }
        impl OnFocus for $data_type {
            fn on_focus(&mut self) {
                self.inner.on_focus(&mut self.base);
            }
        }
        impl $data_type {
            pub fn path(&self) -> &Path {
                self.inner.path()
            }
        }
    };
}
#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnExpand+OnFocus, internal=true)]
pub struct PathFinder {
    inner: InnerPathFinder<fs::Navigator>,
}

impl PathFinder {
    pub(crate) fn new(file_path: &str, layout: Layout, flags: Flags) -> Self {
        let mut c = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            inner: InnerPathFinder::new(file_path, fs::Navigator::new(), flags),
        };
        c.set_size_bounds(4, 1, u16::MAX, u16::MAX);
        c
    }
}

IMPLEMENT_METHODS!(PathFinder);
#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnExpand+OnFocus, internal=true)]
pub(crate) struct PathFinderSimulator {
    inner: InnerPathFinder<fs::NavSimulator>,
}

impl PathFinderSimulator {
    pub(crate) fn new(file_path: &str, layout: Layout, flags: Flags, ns: fs::NavSimulator) -> Self {
        let mut c = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            inner: InnerPathFinder::new(file_path, ns, flags),
        };
        c.set_size_bounds(4, 1, u16::MAX, u16::MAX);
        c
    }
}

IMPLEMENT_METHODS!(PathFinderSimulator);
