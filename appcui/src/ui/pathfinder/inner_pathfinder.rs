use super::initialization_flags::Flags;
use crate::prelude::*;
use crate::utils::fs::*;
use std::path::{Path, PathBuf};

pub(super) struct InnerPathFinder<T>
where
    T: crate::utils::Navigator<Entry, Root, PathBuf>,
{
    flags: Flags,
    navigator: T,
    component: crate::ui::components::NavigatorComponent<T, Entry, Root>,
}

impl<T> InnerPathFinder<T>
where
    T: crate::utils::Navigator<Entry, Root, PathBuf>,
{
    pub(super) fn new(file_path: &str, navigator: T, flags: Flags) -> Self {
        Self {
            flags,
            navigator,
            component: crate::ui::components::NavigatorComponent::new(file_path, flags.contains(Flags::ReadOnly), flags.contains(Flags::CaseSensitive)),
        }
    }

    pub(super) fn on_paint(&self, surface: &mut Surface, theme: &Theme, control: &ControlBase) {
        self.component.on_paint(control, surface, theme);
    }

    pub(super) fn on_key_pressed(&mut self, key: Key, character: char, control: &mut ControlBase) -> EventProcessStatus {
        self.component.on_key_pressed(control, key, character, &self.navigator)
    }

    pub(super) fn on_expand(&mut self, direction: ExpandedDirection, control: &ControlBase) {
        self.component.on_expand(control, direction);
    }

    pub(super) fn on_resize(&mut self, control: &ControlBase, old_size: Size, new_size: Size) {
        self.component.on_resize(control, old_size, new_size);
    }

    pub(super) fn on_focus(&mut self, control: &mut ControlBase) {
        self.component.on_focus(control);
    }

    pub(super) fn on_lose_focus(&mut self, control: &mut ControlBase) {
        self.component.on_lose_focus(control);
    }

    pub(super) fn path(&self) -> &Path {
        Path::new(self.component.path())
    }
    pub(super) fn set_path(&mut self, path: &str, control: &ControlBase) {
        self.component.set_input_path(path,true, control);
    }
}
