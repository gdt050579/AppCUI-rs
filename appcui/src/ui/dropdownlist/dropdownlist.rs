use super::events::EventData;
use super::DropDownListType;
use super::Flags;
use crate::prelude::*;
use crate::ui::components::{ComboBoxComponent, ComboBoxComponentDataProvider};

struct DataProvider<T: DropDownListType> {
    items: Vec<T>,
}
impl<T> ComboBoxComponentDataProvider for DataProvider<T>
where
    T: DropDownListType,
{
    fn count(&self) -> u32 {
        self.items.len() as u32
    }

    fn name(&self, index: u32) -> Option<&str> {
        self.items.get(index as usize).map(|item| DropDownListType::name(item))
    }

    fn description(&self, index: u32) -> Option<&str> {
        self.items.get(index as usize).map(|item| DropDownListType::description(item))
    }

    fn symbol(&self, index: u32) -> Option<&str> {
        self.items.get(index as usize).map(|item| DropDownListType::symbol(item))
    }
}

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent+OnExpand, internal=true)]
pub struct DropDownList<T>
where
    T: DropDownListType,
{
    component: ComboBoxComponent<DataProvider<T>>,
    data: DataProvider<T>,
    flags: Flags,
}
impl<T> DropDownList<T>
where
    T: DropDownListType,
{
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self::with_symbol(0, layout, flags)
    }

    pub fn with_symbol(symbol_size: u8, layout: Layout, flags: Flags) -> Self {
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            component: ComboBoxComponent::new(
                flags.contains(Flags::AllowNoneSelection),
                flags.contains(Flags::ShowDescription),
                0,
                symbol_size,
            ),
            data: DataProvider { items: Vec::new() },
            flags,
        };
        if flags.contains(Flags::AllowNoneSelection) {
            obj.component.set_none_string("None");
        }
        obj.set_size_bounds(7, 1, u16::MAX, 1);
        obj
    }

    pub fn add(&mut self, value: T) {
        self.data.items.push(value);
        self.component.update_count(&mut self.base, self.data.items.len() as u32);
    }

    /// Returns the selected item from the ComboBox control. If no item is selected, the code will return None
    pub fn selected_item(&self) -> Option<&T> {
        let idx = self.component.current_index;
        if idx >= self.data.count() {
            None
        } else {
            Some(&self.data.items[idx as usize])
        }
    }

    /// Returns the selected item from the ComboBox control. If no item is selected, the code will return None
    pub fn selected_item_mut(&mut self) -> Option<&mut T> {
        let idx = self.component.current_index;
        if idx >= self.data.count() {
            None
        } else {
            Some(&mut self.data.items[idx as usize])
        }
    }

    /// Returns the index of the selected item. If no item is selected, the code will return None
    pub fn index(&self) -> Option<u32> {
        let idx = self.component.current_index;
        if idx >= self.data.count() {
            None
        } else {
            Some(idx)
        }
    }

    /// Sets the selected item based on the provided index. If the index is invalid, the index will be ignored
    pub fn set_index(&mut self, index: u32) {
        if index < self.data.count() {
            self.component.update_current_index(index);
        }
    }

    /// Clears all items from the ComboBox control
    pub fn clear(&mut self) {
        self.data.items.clear();
        self.component.clear();
    }

    /// Returns true if the ComboBox control has a selected item
    #[inline(always)]
    pub fn has_selection(&self) -> bool {
        self.component.current_index < self.data.count()
    }

    /// Returns the number of items in the ComboBox control
    #[inline(always)]
    pub fn count(&self) -> u32 {
        self.data.count()
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
