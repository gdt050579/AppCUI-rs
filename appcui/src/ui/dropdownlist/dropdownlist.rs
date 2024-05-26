use super::events::EventData;
use super::DropDownListType;
use super::Flags;
use crate::prelude::*;
use crate::ui::components::{ComboBoxComponent, ComboBoxComponentDataProvider};

struct DataProvider<T: DropDownListType> {
    items: Vec<T>,
}
impl<T> ComboBoxComponentDataProvider for DataProvider<T> where T: DropDownListType{
    fn count(&self) -> u32 {
        self.items.len() as u32
    }

    fn name(&self, index: u32) -> Option<&str> {
        self.items.get(index as usize).map(|item| DropDownListType::name(item))
    }

    fn description(&self, index: u32) -> Option<&str> {
        self.items.get(index as usize).map(|item| DropDownListType::description(item))
    }
}

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent+OnExpand, internal=true)]
pub struct DropDownList<T>
where
    T: DropDownListType
{
    component: ComboBoxComponent<DataProvider<T>>,
    data: DataProvider<T>,
    flags: Flags,
}
impl<T> DropDownList<T>
where
    T: DropDownListType
{
    pub fn new(value: Option<T>, layout: Layout, flags: Flags) -> Self {
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            component: ComboBoxComponent::new(flags.contains(Flags::AllowNoneSelection), flags.contains(Flags::ShowDescription), 0),
            data: DataProvider { items: Vec::new() },
            flags,
        };
        obj.component.set_none_string("None");
        obj.set_size_bounds(7, 1, u16::MAX, 1);
        obj
    }

    fn emit_on_selection_changed_event(&mut self) {
        // self.raise_event(ControlEvent {
        //     emitter: self.handle,
        //     receiver: self.event_processor,
        //     data: ControlEventData::DropDownList(EventData {
        //         type_id: std::any::TypeId::of::<T>(),
        //     }),
        // });
    }
}
impl<T> OnPaint for DropDownList<T>
where
    T: DropDownListType,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        self.component.on_paint(&self.base, &self.data, surface, theme);
    }
}
impl<T> OnExpand for DropDownList<T>
where
    T: DropDownListType,
{
    fn on_expand(&mut self, direction: ExpandedDirection) {
        self.component.on_expand(&mut self.base, direction);
    }
    fn on_pack(&mut self) {
        self.component.on_pack();
    }
}
impl<T> OnDefaultAction for DropDownList<T>
where
    T: DropDownListType,
{
    fn on_default_action(&mut self) {
        self.component.on_default_action(&mut self.base);
    }
}
impl<T> OnKeyPressed for DropDownList<T>
where
    T: DropDownListType,
{
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        let orig_index = self.component.current_index;
        let result = self.component.on_key_pressed(&mut self.base, &self.data, key, character);
        if orig_index != self.component.current_index {
            self.emit_on_selection_changed_event();
        }
        result
    }
}
impl<T> OnMouseEvent for DropDownList<T>
where
    T: DropDownListType,
{
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        let orig_index = self.component.current_index;
        let result = self.component.on_mouse_event(&mut self.base, &self.data, event);
        if orig_index != self.component.current_index {
            self.emit_on_selection_changed_event();
        }
        result
    }
}
