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
        impl<$($g)?> OnThemeChanged for $data_type<$($g)?> where $($constraints)* {
            fn on_theme_changed(&mut self, theme: &Theme) {
                self.inner.on_theme_changed(theme);
            }
        }
        impl<$($g)?> OnMouseEvent for $data_type<$($g)?> where $($constraints)* {
            fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
                self.inner.on_mouse_event(&self.base, event)
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

            ///
            /// Returns the current path selected in the path editor control.
            ///
            /// # Example
            /// ```rust, no_run
            /// use appcui::prelude::*;
            /// let mut control = PathFinder::new("C:\\Program Files", Layout::new("x:1 , y:1 , width:40"), pathfinder::Flags::CaseSensitive);
            /// let path = control.path();
            /// ```
            pub fn path(&self) -> &Path {
                self.inner.path()
            }

            ///
            /// Sets the path editor control current path to the value of the given parameter.
            ///
            /// # Example
            /// ```rust, no_run
            /// use appcui::prelude::*;
            /// let mut control = PathFinder::new("C:\\Program Files", Layout::new("x:1 , y:1 , width:40"), pathfinder::Flags::CaseSensitive);
            /// let path = control.set_path(Path::new("C:\\Windows"));
            /// ```
            pub fn set_path(&mut self, path: &Path) {
                self.inner.set_path(path.to_str().unwrap_or_default(), &self.base);
            }
        }
    };
}
#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnExpand+OnFocus+OnResize+OnThemeChanged+OnMouseEvent, internal=true)]
pub struct PathFinder {
    inner: InnerPathFinder<fs::Navigator>,
}

impl PathFinder {
    ///
    /// Creates a new PathFinder control with the specified layout and flags and sets the starting path.
    ///
    /// The flags can be a combination of the following values:
    /// * `Flags::ReadOnly` - if set, only the file path passed as a parameter will be displayed and any other changes can not be made.
    /// * `Flags::CaseSensitive` - if set, the path matching is case sensitive when navigating
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// let mut control = PathFinder::new("C:\\Program Files", Layout::new("x:1 , y:1 , width:40"), pathfinder::Flags::CaseSensitive);
    /// ```
    pub fn new(file_path: &str, layout: Layout, flags: Flags) -> Self {
        Self::with_navigator(file_path, layout, flags, fs::Navigator::new())
    }
}
IMPLEMENT_METHODS!(PathFinder, fs::Navigator,,);

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnExpand+OnFocus+OnResize+OnThemeChanged+OnMouseEvent, internal=true)]
pub(crate) struct GenericPathFinder<T>
where T: Navigator<fs::Entry, fs::Root, PathBuf> {
    inner: InnerPathFinder<T>,
}

IMPLEMENT_METHODS!(GenericPathFinder, T, T, T: Navigator<fs::Entry, fs::Root, PathBuf>);
