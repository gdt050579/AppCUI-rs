use super::events::EventData;
use super::Flags;
use super::Item;
use crate::prelude::*;
use crate::ui::components::{ComboBoxComponent, ComboBoxComponentDataProvider};


struct DataProvider {
    items: Vec<Item>
}
impl ComboBoxComponentDataProvider for DataProvider
{
    fn count(&self) -> u32 {
        self.items.len() as u32
    }

    fn name(&self, index: u32) -> Option<&str> {
        self.items.get(index as usize).map(|item| item.value.as_str())
    }

    fn description(&self, index: u32) -> Option<&str> {
        self.items.get(index as usize).map(|item| item.description.as_str())
    }
}

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent+OnExpand, internal=true)]
pub struct ComboBox
{
    component: ComboBoxComponent<DataProvider>,
    data: DataProvider,
    flags: Flags,
}
impl ComboBox
{
    pub fn new(layout: Layout, flags: Flags) -> Self {
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            component: ComboBoxComponent::new(false, 0),
            data: DataProvider { items: Vec::new() },
            flags,
        };

        obj.set_size_bounds(7, 1, u16::MAX, 1);
        obj
    }
    #[inline(always)]
    pub fn value(&self) -> &str {
        self.data.name(self.component.current_index).unwrap()
    }
    #[inline(always)]
    pub fn try_value(&self) -> Option<&str> {
        self.data.name(self.component.current_index)
    }

    pub fn add(&mut self, value: &str) {
        self.add_item(Item::new(value, ""));
    }
    pub fn add_item(&mut self, item: combobox::Item) {
        self.data.items.push(item);
        self.component.update_count(self.data.items.len() as u32);
    }
    pub fn selected_item(&self)->Option<&Item> {
        let idx = self.component.current_index;
        if idx>=self.data.count() {
            None 
        } else {
            Some(&self.data.items[idx as usize])
        }
    }
    pub fn selected_item_mut(&mut self)->Option<&mut Item> {
        let idx = self.component.current_index;
        if idx>=self.data.count() {
            None 
        } else {
            Some(&mut self.data.items[idx as usize])
        }
    }
    pub fn index(&self)->Option<u32> {
        let idx = self.component.current_index;
        if idx>=self.data.count() {
            None 
        } else {
            Some(idx)
        }
    }
    pub fn set_index(&mut self, index: u32) {
        if index<self.data.count() {
            self.component.update_current_index(index);
        }
    }
    pub fn clear(&mut self) {
        self.data.items.clear();
        self.component.clear();
    }
    #[inline(always)]
    pub fn has_selection(&self)->bool {
        self.component.current_index < self.data.count()
    }
    fn emit_on_selection_changed_event(&mut self) {
        // self.raise_event(ControlEvent {
        //     emitter: self.handle,
        //     receiver: self.event_processor,
        //     data: ControlEventData::Selector(EventData {
        //         type_id: std::any::TypeId::of::<T>(),
        //     }),
        // });
    }
}
impl OnPaint for ComboBox
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        self.component.on_paint(&self.base, &self.data, surface, theme);
    }
}
impl OnExpand for ComboBox
{
    fn on_expand(&mut self, direction: ExpandedDirection) {
        self.component.on_expand(&mut self.base, direction);
    }
    fn on_pack(&mut self) {
        self.component.on_pack();
    }
}
impl OnDefaultAction for ComboBox
{
    fn on_default_action(&mut self) {
        self.component.on_default_action(&mut self.base);
    }
}
impl OnKeyPressed for ComboBox
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
impl OnMouseEvent for ComboBox
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
