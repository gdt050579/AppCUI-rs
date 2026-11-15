use crate::prelude::*;
use super::stackbar::StackWithLayout;
use super::{BarsStack, Flags, StackBarDrawMode};

#[derive(Copy, Clone)]
struct Defaults {
    thickness: u8,
    draw_mode: StackBarDrawMode,
}

#[CustomControl(internal = true)]
pub struct VStackedBarChart<T>
where
    T: Number + 'static,
{
    flags: Flags,
    stacks: Vec<StackWithLayout<T>>,
    defaults: Defaults,
}

impl<T> VStackedBarChart<T>
where
    T: Number + 'static,
{
    pub fn new(layout: Layout, flags: Flags) -> Self {
        let extra = if flags.contains(Flags::ScrollBars) {
            StatusFlags::IncreaseBottomMarginOnFocus
        } else {
            StatusFlags::None
        };
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput | extra),
            flags,
            stacks: Vec::new(),
            defaults: Defaults {
                thickness: 1,
                draw_mode: StackBarDrawMode::default(),
            },
        }
    }
    pub fn add_stack<S>(&mut self, stack: S)
    where
        S: Into<BarsStack<T>>,
    {
        let entry = self.new_entry(stack.into());
        self.stacks.push(entry);
    }
    pub fn add_stacks<S>(&mut self, stacks: impl IntoIterator<Item = S>)
    where
        S: Into<BarsStack<T>>,
    {
        for stack in stacks {
            let entry = self.new_entry(stack.into());
            self.stacks.push(entry);
        }
    }
    #[inline(always)]
    pub fn stacks_count(&self) -> usize {
        self.stacks.len()
    }
    #[inline(always)]
    pub fn get_stack(&self, index: usize) -> Option<&BarsStack<T>> {
        self.stacks.get(index).map(|entry| &entry.stack)
    }
    pub fn update_stack<F>(&mut self, index: usize, f: F)
    where
        F: FnOnce(&mut BarsStack<T>),
    {
        let defaults = self.defaults;
        if let Some(entry) = self.stacks.get_mut(index) {
            f(&mut entry.stack);
            entry.stack.update_actual_thickness(defaults.thickness, defaults.draw_mode);
        }
    }
    pub fn clear(&mut self) {
        self.stacks.clear();
    }
    fn new_entry(&self, mut stack: BarsStack<T>) -> StackWithLayout<T> {
        stack.update_actual_thickness(self.defaults.thickness, self.defaults.draw_mode);
        StackWithLayout::new(stack)
    }
}
