use std::marker::PhantomData;

use super::events::EventData;
use super::EnumSelector;
use super::Flags;
use crate::prelude::*;
use crate::ui::components::{ComboBoxComponent, ComboBoxComponentDataProvider};

struct DataProvider<T>
where
    T: EnumSelector + Copy + Eq + 'static,
{
    _phanton: PhantomData<T>,
}
impl<T> ComboBoxComponentDataProvider for DataProvider<T>
where
    T: EnumSelector + Copy + Eq + 'static,
{
    fn count(&self) -> u32 {
        T::COUNT
    }

    fn name(&self, index: u32) -> Option<&str> {
        T::from_index(index).map(|p| p.name())
    }

    fn description(&self, index: u32) -> &str {
        T::from_index(index).map(|p| p.description()).unwrap_or("")
    }
}

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent+OnExpand, internal=true)]
pub struct Selector<T>
where
    T: EnumSelector + Copy + Eq + 'static,
{
    component: ComboBoxComponent<DataProvider<T>>,
    flags: Flags,
}
impl<T> Selector<T>
where
    T: EnumSelector + Copy + Eq + 'static,
{
    pub fn new(value: Option<T>, layout: Layout, flags: Flags) -> Self {
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            component: ComboBoxComponent::new(flags.contains(Flags::AllowNoneVariant), T::COUNT),
            flags,
        };
        if let Some(val) = value {
            let count = T::COUNT;
            for i in 0..count {
                if T::from_index(i) == Some(val) {
                    obj.component.current_index = i;
                    break;
                }
            }
        } else {
            // value is None
            if !obj.flags.contains(Flags::AllowNoneVariant) {
                panic!(
                    "You can not instantiate a selector with `None` value without setting the flags `AllowNoneVariant`. Have you forgot to do this ?"
                );
            }
            obj.component.current_index = T::COUNT;
        }
        if T::COUNT == 0 {
            panic!("You should have at least one variant in the enum associated with the seclector control !");
        }
        obj.set_size_bounds(7, 1, u16::MAX, 1);
        obj
    }
    #[inline(always)]
    pub fn value(&self) -> T {
        EnumSelector::from_index(self.component.current_index).unwrap()
    }
    #[inline(always)]
    pub fn try_value(&self) -> Option<T> {
        EnumSelector::from_index(self.component.current_index)
    }
    fn emit_on_selection_changed_event(&mut self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::Selector(EventData {
                type_id: std::any::TypeId::of::<T>(),
            }),
        });
    }
}
impl<T> OnPaint for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let data: DataProvider<T> = DataProvider { _phanton: PhantomData };
        self.component.on_paint(&self.base, &data, surface, theme);
    }
}
impl<T> OnExpand for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_expand(&mut self, direction: ExpandedDirection) {
        self.component.on_expand(&mut self.base, direction);
    }
    fn on_pack(&mut self) {
        self.component.on_pack();
    }
}
impl<T> OnDefaultAction for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_default_action(&mut self) {
        self.component.on_default_action(&mut self.base);
    }
}
impl<T> OnKeyPressed for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        let orig_index = self.component.current_index;
        let result = self.component.on_key_pressed(&mut self.base, key, _character);
        if orig_index != self.component.current_index {
            self.emit_on_selection_changed_event();
        }
        result
    }
}
impl<T> OnMouseEvent for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        let data: DataProvider<T> = DataProvider { _phanton: PhantomData };
        let orig_index = self.component.current_index;
        let result = self.component.on_mouse_event(&mut self.base, &data, event);
        if orig_index != self.component.current_index {
            self.emit_on_selection_changed_event();
        }
        result
    }
}
