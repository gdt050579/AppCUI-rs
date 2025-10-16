use std::cmp::Ordering;

use super::events::*;
use super::{Flags, Group, GroupInformation, Item, ListItem, ViewMode};
use crate::utils;
use appcui_proc_macro::*;
use components::listitem::render_method::RenderData;
use components::{Column, ColumnsHeader, ColumnsHeaderAction, ListScrollBars};

#[derive(Clone, Copy)]
enum CheckMode {
    True,
    False,
    Reverse,
}
#[derive(Clone, Copy, Eq, PartialEq)]
enum Element {
    Item(u32),
    Group(u16),
}

impl Element {
    #[inline(always)]
    fn is_group(&self) -> bool {
        matches!(self, Element::Group(_))
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum HoverStatus {
    None,
    OverItemCheckMark(i32, usize),
    OverGroupCheckMark(i32, usize),
    OverGroupFoldButton(i32, usize),
}

struct TextLine {
    x: i32,
    y: i32,
    width: u32,
}
impl TextLine {
    #[inline(always)]
    fn new(x: i32, y: i32, width: u32) -> Self {
        Self { x, y, width }
    }
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct ListView<T>
where
    T: ListItem + 'static,
{
    flags: Flags,
    data: Vec<Item<T>>,
    filter: Vec<Element>,
    groups: Vec<GroupInformation>,
    header: ColumnsHeader,
    comp: ListScrollBars,
    top_view: usize,
    pos: usize,
    icon_width: u8,
    refilter_enabled: bool,
    view_mode: ViewMode,
    start_mouse_select: usize,
    mouse_check_mode: CheckMode,
    hover_status: HoverStatus,
    selected_items_count: usize,
}

const X_OFFSET_FOR_GROUP_ITEMS: i32 = 2;

impl<T> ListView<T>
where
    T: ListItem + 'static,
{
    /// Creates a new list view with the specified layout and flags
    /// The list view will have a default capacity of 16 items
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// #[derive(ListItem)]
    /// struct Student {
    ///    #[Column(name="Name", width=20)]
    ///    name: &'static str,
    ///    #[Column(name="Grade", width=10)]
    ///    grade: u32
    /// }
    ///
    /// let lv: ListView::<Student> = ListView::new(LayoutBuilder::new().dock(Dock::Fill).build(),
    ///                                             listview::Flags::ScrollBars);
    /// ```
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self::with_capacity(16, layout, flags)
    }

    /// Creates a new list view with the specified layout, flags and capacity
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// #[derive(ListItem)]
    /// struct Student {
    ///   #[Column(name="Name", width=20)]
    ///   name: &'static str,
    ///   #[Column(name="Grade", width=10)]
    ///   grade: u32
    /// }
    ///
    /// let mut lv: ListView::<Student> = ListView::with_capacity(
    ///              100,
    ///              LayoutBuilder::new().dock(Dock::Fill).build(),
    ///              listview::Flags::ScrollBars);
    /// lv.add(Student { name: "John", grade: 10 });
    /// lv.add(Student { name: "Alice", grade: 9 });
    /// lv.add(Student { name: "Bob", grade: 8 });
    /// ```
    pub fn with_capacity(capacity: usize, layout: Layout, flags: Flags) -> Self {
        let mut status_flags = StatusFlags::Enabled | StatusFlags::Visible | StatusFlags::AcceptInput;
        if flags.contains(Flags::ScrollBars) {
            status_flags |= StatusFlags::IncreaseBottomMarginOnFocus;
            status_flags |= StatusFlags::IncreaseRightMarginOnFocus;
        }
        if flags.contains(Flags::SearchBar) {
            status_flags |= StatusFlags::IncreaseBottomMarginOnFocus;
        }
        if flags.contains(Flags::CheckBoxes | Flags::NoSelection) {
            panic!("Invalid flags combination. `CheckBoxes` and `NoSelection` flags cannot be used together !");
        }

        let mut lv = Self {
            base: ControlBase::with_status_flags(layout, status_flags),
            flags,
            top_view: 0,
            pos: 0,
            data: Vec::with_capacity(capacity),
            groups: Vec::new(),
            filter: Vec::with_capacity(capacity),
            header: ColumnsHeader::with_capacity(4),
            comp: ListScrollBars::new(flags.contains(Flags::ScrollBars), flags.contains(Flags::SearchBar)),
            icon_width: if flags.contains(Flags::LargeIcons) {
                3 // includes the extra space
            } else if flags.contains(Flags::SmallIcons) {
                2 // includes the extra space
            } else {
                0 // No extra space
            },
            refilter_enabled: true,
            view_mode: ViewMode::Details,
            start_mouse_select: 0,
            mouse_check_mode: CheckMode::False,
            hover_status: HoverStatus::None,
            selected_items_count: 0,
        };
        // add a default group
        lv.groups.push(GroupInformation::default());
        // add columnes (if described in the type T)
        for i in 0..T::columns_count() {
            lv.header.add(T::column(i));
        }
        lv
    }

    /// Creates a new group with a specified name and returns a gourp identifier. The grpup identifier can be used to add items to the group
    ///
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// #[derive(ListItem)]
    /// struct Student {
    ///     #[Column(name="Name", width=20)]
    ///     name: &'static str,
    ///     #[Column(name="Grade", width=10)]
    ///     grade: u32
    /// }
    ///
    /// let mut lv = listview!("Student, d:f,flags:ShowGroups");
    /// let group = lv.add_group("Group 1");
    /// let students = vec![Student { name: "John", grade: 10 },
    ///                     Student { name: "Alice", grade: 9 }];
    /// lv.add_to_group(students, group);
    /// ```
    pub fn add_group(&mut self, name: &str) -> Group {
        let index = self.groups.len() as u16;
        self.groups.push(GroupInformation::new(name));
        if self.flags.contains(Flags::ShowGroups) {
            // if groups are being shouwn -> we need to refilter intems
            self.refilter();
        }
        Group::new(index)
    }

    /// Adds a new column to the listview. The column will be added after the last existing column.
    /// This method is useful when you manually implement ListItem trait for a type and you want to add columns to the list view. Normally, implementing the ListItem trait (via `#[derive(ListItem)]`) for a type will automatically add columns to the list view.
    ///
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// struct Student {
    ///     name: &'static str,
    ///     grade: u32
    /// }
    ///
    /// impl listview::ListItem for Student {
    ///    fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
    ///       match column_index {
    ///          0 => Some(listview::RenderMethod::Text(self.name)),
    ///          1 => Some(listview::RenderMethod::UInt64(self.grade as u64, listview::NumericFormat::Normal)),  
    ///          _ => None,
    ///       }
    ///    }
    /// }
    ///
    /// let mut lv = listview!("Student, dock:fill");
    /// lv.add_column(Column::new("Name", 10, TextAlignment::Left));
    /// lv.add_column(Column::new("Grade", 6, TextAlignment::Right));
    /// ```
    pub fn add_column(&mut self, column: Column) {
        self.header.add(column);
    }

    /// Adds a new item to the list view.
    ///
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// #[derive(ListItem)]
    /// struct Student {
    ///    #[Column(name="Name", width=20)]
    ///    name: &'static str,
    ///    #[Column(name="Grade", width=10)]
    ///    grade: u32
    /// }
    ///
    /// let mut lv = listview!("type: Student, dock:fill");
    /// lv.add(Student { name: "John", grade: 10 });
    /// lv.add(Student { name: "Alice", grade: 9 });
    /// ```
    #[inline(always)]
    pub fn add(&mut self, item: T) {
        self.add_item(Item::from(item));
    }

    /// Adds a new item to the list view. This method allows one to specify the group, icon, color and selection state for the item upon adding it to the list view.
    ///
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// #[derive(ListItem)]
    /// struct Student {
    ///    #[Column(name="Name", width=20)]
    ///    name: &'static str,
    ///    #[Column(name="Grade", width=10)]
    ///    grade: u32
    /// }
    ///
    /// let mut lv = listview!("type: Student, dock:fill, flags: ShowGroups+LargeIcons");
    /// lv.add_item(listview::Item::new(
    ///                       Student { name: "John", grade: 10 },
    ///                       false,
    ///                       None,
    ///                       ['📁', ' '],
    ///                       listview::Group::None));
    /// lv.add_item(listview::Item::new(
    ///                       Student { name: "Alice", grade: 9 },
    ///                       true,
    ///                       Some(CharAttribute::with_fore_color(Color::White)),
    ///                       ['📁', ' '],
    ///                       listview::Group::None));
    /// ```
    #[inline(always)]
    pub fn add_item(&mut self, mut item: Item<T>) {
        let gid = item.group_id() as usize;
        if gid >= self.groups.len() {
            panic!("Invalid group id `{gid}`. Have you reused a group id from a previous instantiation ?");
        }
        let count = self.groups[gid].items_count();
        self.groups[gid].set_items_count(count + 1);
        // override selection state if the NoSelection flag is set
        if self.flags.contains(Flags::NoSelection) {
            item.set_checked(false);
        }
        self.data.push(item);
        // refilter everything
        self.refilter();
    }

    /// Adds multiple items to the list view. The items will be added after the last existing item.
    ///
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// #[derive(ListItem)]
    /// struct Student {
    ///    #[Column(name="Name", width=20)]
    ///    name: &'static str,
    ///    #[Column(name="Grade", width=10)]
    ///    grade: u32
    /// }
    ///
    /// let mut lv = listview!("type: Student, dock:fill");
    /// let items = vec![
    ///         Student { name: "John", grade: 10 },
    ///         Student { name: "Alice", grade: 9 },
    ///         Student { name: "Bob", grade: 8 },
    ///         Student { name: "Charlie", grade: 7 }
    ///     ];
    /// lv.add_items(items);
    /// ```
    pub fn add_items(&mut self, items: Vec<T>) {
        self.add_multiple_items(items, Group::None, [0u8 as char, 0u8 as char]);
    }

    /// Adds multiple items to the list view and associate them with a specific group.
    ///
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// #[derive(ListItem)]
    /// struct Student {
    ///    #[Column(name="Name", width=20)]
    ///    name: &'static str,
    ///    #[Column(name="Grade", width=10)]
    ///    grade: u32
    /// }
    ///
    /// let mut lv = listview!("type: Student, dock:fill");
    /// let items = vec![
    ///         Student { name: "John", grade: 10 },
    ///         Student { name: "Alice", grade: 9 },
    ///         Student { name: "Bob", grade: 8 },
    ///         Student { name: "Charlie", grade: 7 }
    ///     ];
    /// let group = lv.add_group("Group 1");
    /// lv.add_to_group(items, group);
    /// ```
    pub fn add_to_group(&mut self, items: Vec<T>, group: Group) {
        self.add_multiple_items(items, group, [0u8 as char, 0u8 as char]);
    }
    fn add_multiple_items(&mut self, items: Vec<T>, group: Group, icon: [char; 2]) {
        // disable refiltering while adding all elements
        let old_refilter = self.refilter_enabled;
        self.refilter_enabled = false;
        self.data.reserve(items.len());
        self.filter.reserve(items.len());
        for item in items {
            self.add_item(Item::new(item, false, None, icon, group));
        }
        // restore original refilter state
        self.refilter_enabled = old_refilter;
        self.refilter();
    }

    /// Adds multiple items to the listview. When an item is added to a listview, it is imediatly filtered based on the current search text. If you want to add multiple items (using various methods) and then filter them, you can use the add_batch method.
    ///
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// #[derive(ListItem)]
    /// struct Student {
    ///    #[Column(name="Name", width=20)]
    ///    name: &'static str,
    ///    #[Column(name="Grade", width=10)]
    ///    grade: u32
    /// }
    ///
    /// let mut lv = listview!("type: Student, dock:fill");
    /// lv.add_batch(|lv| {
    ///    lv.add(Student { name: "John", grade: 10 });
    ///    lv.add(Student { name: "Alice", grade: 9 });
    ///    lv.add(Student { name: "Bob", grade: 8 });
    /// });
    /// ```  
    pub fn add_batch<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self),
    {
        let old_refilter = self.refilter_enabled;
        self.refilter_enabled = false;
        f(self);
        // restore original refilter state
        self.refilter_enabled = old_refilter;
        self.refilter();
    }

    /// Clears the content of the listview.
    pub fn clear(&mut self) {
        self.data.clear();
        self.filter.clear();
        // clear counter in groups
        for group in &mut self.groups {
            group.set_items_count(0);
        }
        self.top_view = 0;
        self.pos = 0;
        self.selected_items_count = 0;
        self.update_scrollbars();
        self.update_position(0, false);
    }

    /// Clears the content of the search bar
    pub fn clear_search(&mut self) {
        self.comp.clear_search();
        self.filter_items();
    }

    /// Sets the number of frozen columns. Frozen columns are columns that are always visible, even when the list view is scrolled horizontally. The frozen columns are always the first columns in the list view. Using the value 0 will disable frozen columns.
    pub fn set_frozen_columns(&mut self, count: u16) {
        self.header.set_frozen_columns(count);
        self.update_scrollbars();
    }

    /// Sets the view mode for the list view. The view mode can be one of the following:
    /// - `ViewMode::Details` - displays the list view in details (each column displays a different property of the item)
    /// - `ViewMode::Columns(n)` - the items are displayed in a table with multiple columns. Each column has one item and represents the fist content of the first column in the Details view mode.
    pub fn set_view_mode(&mut self, mode: ViewMode) {
        // safety check
        if mode == ViewMode::Columns(0) {
            panic!("Invalid view mode. Columns count must be greater than 0 !");
        }
        self.view_mode = mode;
        self.update_scrollbars();
        self.update_position(self.pos, false);
    }
    fn compare_items(a: Element, b: Element, column_index: u16, data: &[Item<T>], ascendent: bool) -> Ordering
    where
        T: ListItem,
    {
        match (a, b) {
            (Element::Item(index_a), Element::Item(index_b)) => {
                let rezult = data[index_a as usize].group_id().cmp(&data[index_b as usize].group_id());
                if rezult != Ordering::Equal {
                    rezult
                } else {
                    let item_a = data[index_a as usize].value();
                    let item_b = data[index_b as usize].value();
                    let rezult = ListItem::compare(item_a, item_b, column_index);
                    if ascendent {
                        rezult
                    } else {
                        rezult.reverse()
                    }
                }
            }
            (Element::Group(index_a), Element::Group(index_b)) => index_a.cmp(&index_b),
            (Element::Item(index), Element::Group(group_id)) => match data[index as usize].group_id().cmp(&group_id) {
                Ordering::Equal => Ordering::Greater,
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
            },
            (Element::Group(group_id), Element::Item(index)) => match group_id.cmp(&data[index as usize].group_id()) {
                Ordering::Equal => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
            },
        }
    }

    /// Sorts the items in the list view based on the specified column index. If the column index is invalid, the method will do nothing.
    pub fn sort(&mut self, column_index: u16, ascendent: bool) {
        self.header.set_sort_column(column_index, ascendent, true);
        if self.filter.is_empty() {
            // no need to sort
            return;
        }
        let current_item = if self.pos < self.filter.len() {
            Some(self.filter[self.pos])
        } else {
            None
        };
        // sort elements by column index
        let data = &self.data;
        self.filter.sort_by(|a, b| ListView::compare_items(*a, *b, column_index, data, ascendent));
        // find the new position after sorting
        if let Some(current_item) = current_item {
            // on the same item --> no need to emit an event
            self.goto_element(current_item, false);
        }
    }

    /// Returns a reference to the current item from the list view
    /// if the list view is empty or the current position refers to a group, None is returned
    pub fn current_item(&self) -> Option<&T> {
        if self.pos < self.filter.len() {
            match self.filter[self.pos] {
                Element::Item(index) => Some(self.data[index as usize].value()),
                Element::Group(_) => None,
            }
        } else {
            None
        }
    }

    /// Returns a mutable reference to the current item from the list view
    /// if the list view is empty or the current position refers to a group, None is returned
    pub fn current_item_mut(&mut self) -> Option<&mut T> {
        if self.pos < self.filter.len() {
            match self.filter[self.pos] {
                Element::Item(index) => Some(self.data[index as usize].value_mut()),
                Element::Group(_) => None,
            }
        } else {
            None
        }
    }

    /// Returns the index of the current item from the list view or None if the list view is empty or the current selection is on a group
    pub fn current_item_index(&self) -> Option<usize> {
        if self.pos < self.filter.len() {
            match self.filter[self.pos] {
                Element::Item(index) => {
                    if (index as usize) < self.data.len() {
                        Some(index as usize)
                    } else {
                        None
                    }
                }
                Element::Group(_) => None,
            }
        } else {
            None
        }
    }

    /// Returns the current group (for the current selection for for the item).
    /// If the number of items in the listview is 0 or no group has been associated with the current item, None is returned
    pub fn current_group(&self) -> Option<Group> {
        if self.pos < self.filter.len() {
            match self.filter[self.pos] {
                Element::Item(index) => {
                    let gid = self.data[index as usize].group_id();
                    if gid > 0 {
                        Some(Group::new(gid))
                    } else {
                        None
                    }
                }
                Element::Group(gid) => Some(Group::new(gid)),
            }
        } else {
            None
        }
    }

    /// Returns the name of the group or None if the group object is invalid
    pub fn group_name(&self, group: Group) -> Option<&str> {
        if (group.index() as usize >= self.groups.len()) || (group.index() == 0) {
            None
        } else {
            Some(self.groups[group.index() as usize].name())
        }
    }

    /// Returns the item from the list view at the specified index
    pub fn item(&self, index: usize) -> Option<&T> {
        if index < self.data.len() {
            Some(self.data[index].value())
        } else {
            None
        }
    }

    /// Returns a mutable reference to the item from the list view at the specified index
    /// if the index is out of bounds, None is returned
    pub fn item_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.data.len() {
            Some(self.data[index].value_mut())
        } else {
            None
        }
    }

    /// Returns the number of items in the list view
    pub fn items_count(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the item at the specified index is checked, `false` otherwise
    pub fn is_item_selected(&self, index: usize) -> bool {
        if index < self.data.len() {
            self.data[index].is_checked()
        } else {
            false
        }
    }

    /// Change the selection state of the item at the specified index
    pub fn select_item(&mut self, index: usize, selected: bool) {
        if self.select_item_and_update_count(index, selected) {
            self.update_check_count_for_groups();
        }
    }

    /// Returns the number of selected (checked) items
    pub fn selected_items_count(&self) -> usize {
        self.selected_items_count
    }

    /// Returns a reference to a column at the specified index or None if the index is out of bounds
    pub fn column(&self, index: usize) -> Option<&Column> {
        self.header.columns().get(index)
    }

    /// Returns a mutable reference to a column at the specified index or None if the index is out of bounds
    pub fn column_mut(&mut self, index: usize) -> Option<&mut Column> {
        self.header.columns_mut().get_mut(index)
    }

    fn goto_element(&mut self, element: Element, emit_event: bool) -> bool {
        for (index, item) in self.filter.iter().enumerate() {
            if *item == element {
                self.update_position(index, emit_event);
                return true;
            }
        }
        false
    }
    fn is_item_filtered_out(&self, item: &Item<T>) -> bool {
        if self.flags.contains(Flags::CustomFilter) {
            let search_text = self.comp.search_text();
            if search_text.is_empty() {
                false
            } else {
                !item.value().matches(self.comp.search_text())
            }
        } else {
            // check if content is filtered out
            let value = item.value();
            let search_text = self.comp.search_text();
            if search_text.is_empty() {
                return false;
            }
            let mut output: [u8; 256] = [0; 256];
            let columns_count = if self.view_mode == ViewMode::Details {
                self.header.columns().len()
            } else {
                1
            };
            for column_index in 0..columns_count {
                if let Some(rm) = value.render_method(column_index as u16) {
                    if let Some(item_text) = rm.string_representation(&mut output) {
                        if item_text.index_ignoring_case(search_text).is_some() {
                            return false;
                        }
                    }
                }
            }
            true
        }
    }
    fn refilter(&mut self) {
        if !self.refilter_enabled {
            return;
        }
        // refilter elements
        self.filter.clear();
        // reserve space for the entire list + groups
        self.filter.reserve(self.data.len() + self.groups.len());
        // add items
        if self.flags.contains(Flags::ShowGroups) {
            // clear counter in groups
            for group in &mut self.groups {
                group.set_items_count(0);
            }
            // add items
            for (index, item) in self.data.iter().enumerate() {
                if self.is_item_filtered_out(item) {
                    continue;
                }
                let group = &mut self.groups[item.group_id() as usize];
                if group.items_count() == 0 {
                    // first encounter of a grooup
                    self.filter.push(Element::Group(item.group_id()));
                }
                group.increment_items_count();
                if group.is_collapsed() {
                    continue;
                }
                self.filter.push(Element::Item(index as u32));
            }
            // add empty groups at the end
            // if self.flags.contains(Flags::ShowEmptyGroups) {
            //     for (index, group) in self.groups.iter().enumerate() {
            //         if group.is_empty() {
            //             self.filter.push(Element::Group(index as u16));
            //         }
            //     }
            // }
        } else {
            for (index, item) in self.data.iter().enumerate() {
                if self.is_item_filtered_out(item) {
                    continue;
                }
                self.filter.push(Element::Item(index as u32));
            }
        }
        if let Some(column_index) = self.header.sort_column() {
            self.sort(column_index, self.header.should_sort_ascendent());
        } else {
            self.sort(u16::MAX, true);
        }
    }
    fn filter_items(&mut self) {
        if self.data.is_empty() {
            return;
        }
        let (current_element, is_group) = if self.pos < self.filter.len() {
            let el = self.filter[self.pos];
            (Some(el), el.is_group())
        } else {
            (None, false)
        };
        self.refilter();
        let found = if let Some(current_element) = current_element {
            self.goto_element(current_element, true)
        } else {
            false
        };
        if (!found) || (is_group) {
            // move to the first non_group element
            for (index, element) in self.filter.iter().enumerate() {
                if let Element::Item(_) = element {
                    self.update_position(index, true);
                    break;
                }
            }
        }
    }
    fn update_check_count_for_groups(&mut self) {
        // if ShowGroups is not present, we do not need to update the check count for groups
        if !self.flags.contains(Flags::ShowGroups) {
            return;
        }
        // clear all check counts
        for group in &mut self.groups {
            group.set_items_checked_count(0);
        }
        // iterate over each item from the filtered list and update the check count for group it belongs to
        for item in &self.filter {
            match item {
                Element::Item(index) => {
                    let group_id = self.data[*index as usize].group_id();
                    let group = &mut self.groups[group_id as usize];
                    if self.data[*index as usize].is_checked() {
                        group.set_items_checked_count(group.items_checked_count() + 1);
                    }
                }
                Element::Group(_) => {}
            }
        }
    }
    fn autoresize_column(&mut self, column_index: u16) {
        let mut new_width = 0u32;
        let mut found = false;
        for item in self.filter.iter() {
            match item {
                Element::Item(index) => {
                    let item = &self.data[*index as usize];
                    if let Some(rm) = item.value().render_method(column_index) {
                        new_width = new_width.max(listview::RenderMethod::min_width(&rm));
                        found = true;
                    }
                }
                Element::Group(_) => {}
            }
        }
        if found {
            if column_index == 0 {
                // add extra spaces required
                if self.flags.contains(Flags::ShowGroups) {
                    new_width += 2
                };
                if self.flags.contains(Flags::CheckBoxes) {
                    new_width += 2
                };
                new_width += self.icon_width as u32;
            }
            self.header.set_column_width(column_index, new_width.min(u8::MAX as u32) as u8);
        }
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        self.header.scroll_to(self.comp.horizontal_index() as u32);
        self.top_view = (self.comp.vertical_index() as usize).min(self.filter.len());
    }
    fn update_scrollbars(&mut self) {
        match self.view_mode {
            ViewMode::Details => {
                self.comp
                    .resize(self.header.width() as u64, self.filter.len() as u64, &self.base, self.visible_space());
                self.comp.set_indexes(self.header.scroll_pos() as u64, self.top_view as u64);
            }
            ViewMode::Columns(_) => {
                self.comp.resize(0, self.filter.len() as u64, &self.base, self.visible_space());
                self.comp.set_indexes(0, self.top_view as u64);
            }
        }
    }
    fn select_until_position(&mut self, new_pos: usize) {
        let start = self.pos;
        let mode = self.toggle_current_item_selection();
        self.update_position(new_pos, true);
        self.check_items(start, self.pos, mode, true);
    }
    fn toggle_group_collapse_status(&mut self, gid: u16, emit_event: bool) {
        if gid as usize >= self.groups.len() {
            return;
        }
        let group = &mut self.groups[gid as usize];
        let pos = self.pos;
        group.set_collapsed(!group.is_collapsed());
        let is_collapsed = group.is_collapsed();
        self.refilter();
        self.update_position(pos, true);
        if emit_event {
            self.raise_event(ControlEvent {
                emitter: self.handle,
                receiver: self.event_processor,
                data: ControlEventData::ListView(EventData {
                    event_type: listview::events::ListViewEventTypes::GroupFoldedOrUnfolded(Group::new(gid), is_collapsed),
                    type_id: std::any::TypeId::of::<T>(),
                }),
            });
        }
    }
    fn execute_column_header_action(&mut self, action: ColumnsHeaderAction) -> bool {
        match action {
            ColumnsHeaderAction::Sort((index, ascendent)) => {
                self.sort(index, ascendent);
                self.update_scrollbars();
                true
            }
            ColumnsHeaderAction::AutoResize(index) => {
                self.autoresize_column(index);
                self.update_scrollbars();
                true
            }
            ColumnsHeaderAction::ResizeColumn => {
                self.update_scrollbars();
                true
            }
            ColumnsHeaderAction::UpdateScroll => {
                self.update_scrollbars();
                true
            }
            ColumnsHeaderAction::Processed => true,
            ColumnsHeaderAction::None => false,
            ColumnsHeaderAction::Repaint => false,
        }
    }
    #[inline(always)]
    fn visible_items(&self) -> usize {
        match self.view_mode {
            ViewMode::Details => self.size().height.saturating_sub(1) as usize,
            ViewMode::Columns(count) => self.size().height as usize * (count as usize),
        }
    }
    #[inline(always)]
    fn item_width(&self) -> u32 {
        match self.view_mode {
            ViewMode::Details => self.size().width,
            ViewMode::Columns(count) => {
                if count == 0 {
                    0
                } else {
                    (self.size().width.saturating_sub(count as u32 - 1) / (count as u32)).max(1)
                }
            }
        }
    }
    #[inline(always)]
    fn visible_space(&self) -> Size {
        match self.view_mode {
            ViewMode::Details => Size::new(self.size().width, self.size().height.saturating_sub(1)),
            ViewMode::Columns(count) => Size::new(self.item_width(), self.size().height * count as u32),
        }
    }
    #[inline(always)]
    fn toggle_current_item_selection(&self) -> CheckMode {
        if self.pos < self.filter.len() {
            match self.filter[self.pos] {
                Element::Item(index) => {
                    if self.data[index as usize].is_checked() {
                        CheckMode::False
                    } else {
                        CheckMode::True
                    }
                }
                Element::Group(_) => CheckMode::False,
            }
        } else {
            CheckMode::False
        }
    }
    #[inline(always)]
    fn is_entire_list_selected(&self) -> bool {
        for item in &self.filter {
            if let Element::Item(idx) = item {
                if !self.data[*idx as usize].is_checked() {
                    return false;
                }
            }
        }
        true
    }
    fn process_key_pressed(&mut self, key: Key) -> bool {
        // process key for items
        match key.value() {
            // movements
            key!("Up") => {
                self.update_position(self.pos.saturating_sub(1), true);
                true
            }
            key!("Down") => {
                self.update_position(self.pos.saturating_add(1), true);
                true
            }
            key!("Ctrl+Alt+Up") => {
                self.move_scroll_to(self.top_view.saturating_sub(1));
                true
            }
            key!("Ctrl+Alt+Down") => {
                self.move_scroll_to(self.top_view.saturating_add(1));
                true
            }
            key!("Home") => {
                self.update_position(0, true);
                true
            }
            key!("End") => {
                self.update_position(self.filter.len(), true);
                true
            }
            key!("PageUp") => {
                self.update_position(self.pos.saturating_sub(self.visible_items()), true);
                true
            }
            key!("PageDown") => {
                self.update_position(self.pos.saturating_add(self.visible_items()), true);
                true
            }
            key!("Left") => {
                self.update_position(self.pos.saturating_sub(self.size().height as usize), true);
                true
            }
            key!("Right") => {
                self.update_position(self.pos.saturating_add(self.size().height as usize), true);
                true
            }

            // Selection
            key!("Space") => {
                if self.flags.contains(Flags::CheckBoxes) {
                    self.check_item(self.pos, CheckMode::Reverse, true, true);
                    true
                } else if let Some(Element::Group(gid)) = self.filter.get(self.pos) {
                    self.toggle_group_collapse_status(*gid, true);
                    true
                } else {
                    false
                }
            }
            key!("Insert") | key!("Shift+Down") => {
                self.check_item(self.pos, CheckMode::Reverse, true, true);
                self.update_position(self.pos.saturating_add(1), true);
                true
            }
            key!("Shift+Up") => {
                self.check_item(self.pos, CheckMode::Reverse, true, true);
                self.update_position(self.pos.saturating_sub(1), true);
                true
            }
            key!("Shift+Home") => {
                self.select_until_position(0);
                true
            }
            key!("Shift+End") => {
                self.select_until_position(self.filter.len());
                true
            }
            key!("Shift+PageUp") => {
                self.select_until_position(self.pos.saturating_sub(self.visible_items()));
                true
            }
            key!("Shift+PageDown") => {
                self.select_until_position(self.pos.saturating_add(self.visible_items()));
                true
            }
            key!("Shift+Left") => {
                self.select_until_position(self.pos.saturating_sub(self.size().height as usize));
                true
            }
            key!("Shift+Right") => {
                self.select_until_position(self.pos.saturating_add(self.size().height as usize));
                true
            }

            key!("Ctrl+A") => {
                if self.is_entire_list_selected() {
                    self.check_items(0, self.filter.len(), CheckMode::False, true);
                } else {
                    self.check_items(0, self.filter.len(), CheckMode::True, true);
                }
                true
            }

            // Action
            key!("Enter") => {
                match self.filter.get(self.pos) {
                    Some(Element::Item(index)) => self.emit_item_action_event(*index as usize),
                    Some(Element::Group(gid)) => self.toggle_group_collapse_status(*gid, true),
                    _ => {}
                }
                true
            }
            _ => false,
        }
    }
    fn paint_group(&self, gi: &GroupInformation, tl: TextLine, surface: &mut Surface, theme: &Theme, attr: Option<CharAttribute>) {
        let w = tl.width;
        surface.draw_horizontal_line_with_size(tl.x, tl.y, w, LineType::Single, attr.unwrap_or(theme.lines.focused));
        let mut left = tl.x + 1;
        if gi.is_collapsed() {
            surface.write_char(
                left,
                tl.y,
                Character::with_attributes(SpecialChar::TriangleRight, attr.unwrap_or(theme.symbol.arrows)),
            );
        } else {
            surface.write_char(
                left,
                tl.y,
                Character::with_attributes(SpecialChar::TriangleDown, attr.unwrap_or(theme.symbol.arrows)),
            );
        }
        left += 2;
        if self.flags.contains(Flags::CheckBoxes) && left + 4 < tl.x + w as i32 {
            surface.write_string(left, tl.y, "[ ]", attr.unwrap_or(theme.text.focused), false);
            let count = gi.items_count();
            let checked = gi.items_checked_count();
            if (count == checked) && (count > 0) {
                surface.write_char(
                    left + 1,
                    tl.y,
                    Character::with_attributes(SpecialChar::CheckMark, attr.unwrap_or(theme.symbol.checked)),
                );
            } else if checked == 0 {
                surface.write_char(left + 1, tl.y, Character::with_attributes('x', attr.unwrap_or(theme.symbol.unchecked)));
            } else {
                surface.write_char(left + 1, tl.y, Character::with_attributes('?', attr.unwrap_or(theme.symbol.unknown)));
            }
            left += 4;
        }
        let items_in_group = gi.items_count();
        let digits = utils::FormatNumber::number_of_digits(items_in_group as u64) as i32;
        let right = if (tl.x + w as i32) - digits - 8 >= left {
            (tl.x + w as i32) - digits - 3
        } else {
            tl.x + w as i32
        };
        if left + 3 < right {
            let txwidth = gi.name_chars_count() as i32;
            let space_width = if left + 3 + txwidth <= right { txwidth } else { right - left - 3 };
            let format = TextFormatBuilder::new()
                .position(left + 1, tl.y)
                .align(TextAlignment::Left)
                .attribute(attr.unwrap_or(theme.text.hovered))
                .wrap_type(WrapType::SingleLineWrap(space_width as u16))
                .build();
            surface.write_text(gi.name(), &format);
            surface.write_char(left, tl.y, Character::with_attributes(' ', attr.unwrap_or(theme.text.focused)));
            surface.write_char(
                left + space_width + 1,
                tl.y,
                Character::with_attributes(' ', attr.unwrap_or(theme.text.focused)),
            );
            if left + txwidth + 3 > right {
                surface.write_char(left + space_width, tl.y, Character::with_char(SpecialChar::ThreePointsHorizontal));
            }
        }
        if right + digits + 3 <= tl.x + w as i32 {
            surface.write_char(right, tl.y, Character::with_attributes('[', attr.unwrap_or(theme.text.focused)));
            surface.write_char(
                right + digits + 1,
                tl.y,
                Character::with_attributes(']', attr.unwrap_or(theme.text.focused)),
            );
            let mut temp_buf: [u8; 40] = [0; 40];
            let to_print_buf = utils::FormatNumber::write_to_buffer(items_in_group as u64, &mut temp_buf);
            surface.write_ascii(right + 1, tl.y, to_print_buf, attr.unwrap_or(theme.text.normal), false);
        }
    }
    fn paint_groups(&self, surface: &mut Surface, theme: &Theme) {
        let has_focus = self.base.has_focus();
        let is_enabled = self.is_enabled();
        let attr = if !is_enabled {
            Some(theme.text.inactive)
        } else if !has_focus {
            Some(theme.text.normal)
        } else {
            None
        };
        let start_y_poz = if self.view_mode == ViewMode::Details { 1 } else { 0 };
        let mut y = start_y_poz;
        let mut x = 0;
        let max_y = self.size().height as i32;
        let item_size = self.item_width();
        let mut idx = self.top_view;
        let max_idx = self.filter.len();
        let visible_items = self.visible_items();
        let mut item_count = 0;
        let (hover_left, hover_right, hover_pos) = match self.hover_status {
            HoverStatus::OverGroupCheckMark(x, pos) => (x, x + 2, pos),
            HoverStatus::OverGroupFoldButton(x, pos) => (x, x, pos),
            _ => (0, 0, usize::MAX),
        };
        surface.reset_clip();
        surface.reset_origin();
        while (item_count < visible_items) && (idx < max_idx) {
            match self.filter[idx] {
                Element::Group(group_id) => {
                    self.paint_group(&self.groups[group_id as usize], TextLine::new(x, y, item_size), surface, theme, attr);
                    // paint group
                    if is_enabled {
                        if (has_focus) && (idx == self.pos) {
                            surface.fill_horizontal_line_with_size(x, y, item_size, Character::with_attributes(0, theme.list_current_item.focus));
                        }
                        if idx == hover_pos {
                            surface.fill_horizontal_line(hover_left, y, hover_right, Character::with_attributes(0, theme.button.regular.text.hovered));
                        }
                    }
                }
                Element::Item(_) => {}
            }
            y += 1;
            idx += 1;
            item_count += 1;
            if y >= max_y {
                y = start_y_poz;
                x += item_size as i32 + 1;
            }
        }
        surface.reset_clip();
        surface.reset_origin();
    }
    #[inline(always)]
    fn paint_icon(&self, x: i32, item: &Item<T>, attr: Option<CharAttribute>, surface: &mut Surface, theme: &Theme) {
        let attr = attr.unwrap_or(theme.list_current_item.icon);
        match self.icon_width {
            3 => {
                surface.write_char(x, 0, Character::with_attributes(item.icon_first_character(), attr));
                surface.write_char(x + 1, 0, Character::with_attributes(item.icon_second_character(), attr));
            }
            2 => {
                surface.write_char(x, 0, Character::with_attributes(item.icon_first_character(), attr));
            }
            _ => {}
        }
    }
    fn paint_item(&self, item: &Item<T>, y: i32, surface: &mut Surface, theme: &Theme, attr: Option<CharAttribute>) {
        let width = self.header.width() as i32;
        let frozen_columns = self.header.frozen_columns();
        let columns = self.header.columns();
        if columns.is_empty() {
            return;
        }
        let min_left = if frozen_columns == 0 {
            columns[0].x
        } else {
            let c = &columns[frozen_columns as usize - 1];
            c.x + c.width as i32 + 1
        };
        // first column
        let c = &columns[0];
        let l = c.x
            + if self.flags.contains(Flags::ShowGroups) {
                X_OFFSET_FOR_GROUP_ITEMS
            } else {
                0
            };
        let r = c.x + c.width as i32;
        let mut extra = 0;
        let mut rd = RenderData {
            theme,
            alignment: TextAlignment::Left,
            width: 0,
            attr: None,
        };
        if (r >= 0) && (l < width) && (c.width != 0)
        /*&& (r >= min_left)*/
        {
            if frozen_columns == 0 {
                surface.set_relative_clip(l.max(min_left), y, r.max(min_left), y);
                surface.set_origin(l, y);
            } else {
                surface.set_relative_clip(l, y, r, y);
                surface.set_origin(l, y);
            }
            if self.flags.contains(Flags::CheckBoxes) {
                if item.is_checked() {
                    surface.write_char(
                        0,
                        0,
                        Character::with_attributes(SpecialChar::CheckMark, attr.unwrap_or(theme.symbol.checked)),
                    );
                } else {
                    surface.write_char(0, 0, Character::with_attributes('x', attr.unwrap_or(theme.symbol.unchecked)));
                }
                extra = 2;
            }
            // icon
            if self.icon_width > 0 {
                self.paint_icon(extra, item, attr, surface, theme);
                extra += self.icon_width as i32;
            }
            if extra > 0 {
                if frozen_columns == 0 {
                    surface.set_relative_clip((l + extra).max(min_left), y, r.max(min_left), y);
                    surface.set_origin(l + extra, y);
                } else {
                    surface.set_relative_clip(l + extra, y, r, y);
                    surface.set_origin(l + extra, y);
                }
            }
            if let Some(render_method) = ListItem::render_method(item.value(), 0) {
                rd.width = c.width as u16;
                rd.alignment = c.alignment;
                rd.attr = if attr.is_none() { item.render_attr() } else { attr };
                if !render_method.paint(surface, &rd) {
                    // custom paint required
                    ListItem::paint(item.value(), 0, c.width.saturating_sub(extra as u8) as u16, surface, theme, rd.attr)
                }
            }
        }
        rd.attr = if attr.is_none() { item.render_attr() } else { attr };
        for (index, c) in columns.iter().enumerate().skip(1) {
            let r = c.x + c.width as i32;
            if (r < 0) /*|| (r < min_left)*/ || (c.x >= width) || (c.width == 0) {
                continue;
            }
            if index < frozen_columns as usize {
                surface.set_relative_clip(c.x, y, r, y);
            } else {
                surface.set_relative_clip(c.x.max(min_left), y, r.max(min_left), y);
            }
            surface.set_origin(c.x, y);
            if let Some(render_method) = ListItem::render_method(item.value(), index as u16) {
                rd.width = c.width as u16;
                rd.alignment = c.alignment;

                if !render_method.paint(surface, &rd) {
                    // custom paint required
                    ListItem::paint(item.value(), index as u32, c.width as u16, surface, theme, rd.attr)
                }
            }
        }
    }
    fn paint_item_for_fist_column(&self, item: &Item<T>, tl: TextLine, surface: &mut Surface, theme: &Theme, attr: Option<CharAttribute>) {
        // assume that x and y are valid and possitve (this will be ensured by the caller)
        let c = &self.header.columns()[0];
        let l = tl.x + if self.flags.contains(Flags::ShowGroups) { 2 } else { 0 };
        let r = tl.x + tl.width.saturating_sub(1) as i32;
        let mut extra = 0;

        surface.set_relative_clip(l, tl.y, r, tl.y);
        surface.set_origin(l, tl.y);
        if self.flags.contains(Flags::CheckBoxes) {
            if item.is_checked() {
                surface.write_char(
                    0,
                    0,
                    Character::with_attributes(SpecialChar::CheckMark, attr.unwrap_or(theme.symbol.checked)),
                );
            } else {
                surface.write_char(0, 0, Character::with_attributes('x', attr.unwrap_or(theme.symbol.unchecked)));
            }
            extra = 2;
        }
        // icon
        if self.icon_width > 0 {
            self.paint_icon(extra, item, attr, surface, theme);
            extra += self.icon_width as i32;
        }
        if l + extra < r {
            if extra > 0 {
                surface.set_relative_clip(l + extra, tl.y, r, tl.y);
                surface.set_origin(l + extra, tl.y);
            }
            let item_render_width = (r + 1 - l - extra) as u16;
            if let Some(render_method) = ListItem::render_method(item.value(), 0) {
                let rd = RenderData {
                    theme,
                    alignment: c.alignment,
                    width: item_render_width,
                    attr: if attr.is_none() { item.render_attr() } else { attr },
                };
                if !render_method.paint(surface, &rd) {
                    // custom paint required
                    ListItem::paint(item.value(), 0, item_render_width, surface, theme, rd.attr);
                }
            }
        }
    }
    fn paint_column_lines(&self, surface: &mut Surface, theme: &Theme) {
        if let ViewMode::Columns(count) = self.view_mode {
            if count < 2 {
                return;
            }
            let attr = match () {
                _ if !self.is_enabled() => theme.lines.inactive,
                _ if self.has_focus() => theme.lines.focused,
                _ => theme.lines.normal,
            };
            let count = count - 1;
            let mut x = 0;
            let item_size = self.item_width() as i32;
            let h = self.size().height;
            for _ in 0..count {
                x += item_size;
                surface.draw_vertical_line_with_size(x, 0, h, LineType::Single, attr);
                x += 1;
            }
        }
    }
    fn paint_items(&self, surface: &mut Surface, theme: &Theme) -> bool {
        let has_focus = self.base.has_focus();
        let is_enabled = self.is_enabled();
        let attr = if !is_enabled {
            Some(theme.text.inactive)
        } else if !has_focus {
            Some(theme.text.normal)
        } else {
            None
        };
        let mut found_groups = false;
        let start_y_poz = if self.view_mode == ViewMode::Details { 1 } else { 0 };
        let mut y = start_y_poz;
        let mut x = 0;
        let item_size = self.item_width();
        let max_y = self.size().height as i32;
        let mut idx = self.top_view;
        let max_idx = self.filter.len();
        let visible_items = self.visible_items();
        let mut item_count = 0;
        let (hover_checkmark_x, hover_pos) = match self.hover_status {
            HoverStatus::OverItemCheckMark(x, pos) => (x, pos),
            _ => (0, usize::MAX),
        };
        // very simply code
        while (item_count < visible_items) && (idx < max_idx) {
            match self.filter[idx] {
                Element::Group(_) => {
                    found_groups = true;
                }
                Element::Item(index) => {
                    let item = &self.data[index as usize];
                    match self.view_mode {
                        ViewMode::Details => self.paint_item(item, y, surface, theme, attr),
                        ViewMode::Columns(_) => self.paint_item_for_fist_column(item, TextLine::new(x, y, item_size), surface, theme, attr),
                    };
                    if (item.is_checked()) && (has_focus) && (!self.flags.contains(Flags::CheckBoxes)) {
                        surface.reset_clip();
                        surface.reset_origin();
                        surface.fill_horizontal_line_with_size(x, y, item_size, Character::with_attributes(0, theme.list_current_item.selected));
                    }
                    if is_enabled {
                        if idx == self.pos {
                            surface.reset_clip();
                            surface.reset_origin();
                            if has_focus {
                                let current_item_attr = match () {
                                    _ if self.flags.contains(Flags::CheckBoxes) => theme.list_current_item.focus,
                                    _ if item.is_checked() => theme.list_current_item.over_selection,
                                    _ => theme.list_current_item.focus,
                                };
                                surface.fill_horizontal_line_with_size(x, y, item_size, Character::with_attributes(0, current_item_attr));
                            }
                        }
                        if idx == hover_pos {
                            surface.reset_clip();
                            surface.reset_origin();
                            surface.write_char(hover_checkmark_x, y, Character::with_attributes(0, theme.button.regular.text.hovered));
                        }
                    }
                }
            }
            y += 1;
            idx += 1;
            item_count += 1;
            if y >= max_y {
                y = start_y_poz;
                x += item_size as i32 + 1;
            }
        }
        surface.reset_clip();
        surface.reset_origin();
        found_groups
    }
    fn update_position(&mut self, new_pos: usize, emit_event: bool) {
        let len = self.filter.len();
        if len == 0 {
            return;
        }
        let new_pos = new_pos.min(len - 1);
        let h = self.visible_items();
        if h == 0 {
            return;
        }

        // check the top view
        if self.top_view + h >= len {
            self.top_view = len.saturating_sub(h);
        }
        if new_pos < self.top_view {
            self.top_view = new_pos;
        } else {
            let diff = new_pos - self.top_view;
            if (diff >= h) && (h > 0) {
                self.top_view = new_pos - h + 1;
            }
        }
        // update scrollbars
        self.update_scrollbars();
        let should_emit = (self.pos != new_pos) && emit_event;
        self.pos = new_pos;
        if should_emit {
            self.raise_event(ControlEvent {
                emitter: self.handle,
                receiver: self.event_processor,
                data: ControlEventData::ListView(EventData {
                    event_type: listview::events::ListViewEventTypes::CurrentItemChanged,
                    type_id: std::any::TypeId::of::<T>(),
                }),
            });
        }
    }
    fn move_scroll_to(&mut self, new_poz: usize) {
        if new_poz == self.top_view {
            return;
        }
        let visible_items = self.visible_items();
        let max_value = self.filter.len().saturating_sub(visible_items);
        self.top_view = new_poz.min(max_value);
        self.update_scrollbars();
    }
    fn emit_selection_update_event(&self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::ListView(EventData {
                event_type: listview::events::ListViewEventTypes::SelectionChanged,
                type_id: std::any::TypeId::of::<T>(),
            }),
        });
    }
    fn emit_item_action_event(&self, index: usize) {
        if index < self.data.len() {
            self.raise_event(ControlEvent {
                emitter: self.handle,
                receiver: self.event_processor,
                data: ControlEventData::ListView(EventData {
                    event_type: listview::events::ListViewEventTypes::ItemAction(index),
                    type_id: std::any::TypeId::of::<T>(),
                }),
            });
        }
    }

    #[inline(always)]
    fn select_item_and_update_count(&mut self, data_index: usize, value: bool) -> bool {
        if self.flags.contains(Flags::NoSelection) {
            return false;
        }
        let item = &mut self.data[data_index];
        let status = item.is_checked();
        item.set_checked(value);
        if status != value {
            if value {
                self.selected_items_count += 1;
            } else {
                self.selected_items_count -= 1;
            }
            true
        } else {
            false
        }
    }
    /// Returns true if the selection has been changed, false otherwise
    fn check_item(&mut self, pos: usize, mode: CheckMode, update_group_check_count: bool, emit_event: bool) -> bool {
        if self.flags.contains(Flags::NoSelection) {
            return false;
        }
        if pos >= self.filter.len() {
            return false;
        }
        let mut selection_has_changed = false;
        match self.filter[pos] {
            Element::Item(index) => {
                let item = &mut self.data[index as usize];
                let status = item.is_checked();
                match mode {
                    CheckMode::True => item.set_checked(true),
                    CheckMode::False => item.set_checked(false),
                    CheckMode::Reverse => item.set_checked(!status),
                }
                selection_has_changed = item.is_checked() != status;
                if selection_has_changed {
                    if item.is_checked() {
                        self.selected_items_count += 1;
                    } else {
                        self.selected_items_count -= 1;
                    }
                }
                if update_group_check_count {
                    self.update_check_count_for_groups();
                }
            }
            Element::Group(gid) => {
                let group = &mut self.groups[gid as usize];
                let checked = group.items_checked_count();
                let count = group.items_count();
                let new_status = checked < count;
                if group.is_collapsed() {
                    // iterate through all items that and check if they are filtered or not and check them
                    let len = self.data.len();
                    for idx in 0..len {
                        let item = &self.data[idx];
                        if item.group_id() != gid {
                            continue;
                        }
                        if self.is_item_filtered_out(item) {
                            continue;
                        }
                        selection_has_changed |= self.select_item_and_update_count(idx, new_status);
                    }
                } else {
                    let len = self.filter.len();
                    for idx in pos + 1..len {
                        match self.filter[idx] {
                            Element::Item(index) => {
                                let item = &self.data[index as usize];
                                if item.group_id() != gid {
                                    break;
                                }
                                selection_has_changed |= self.select_item_and_update_count(index as usize, new_status);
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                }
                let group = &mut self.groups[gid as usize];
                group.set_items_checked_count(if checked < count { count } else { 0 });
            }
        }
        if (emit_event) && (selection_has_changed) {
            self.emit_selection_update_event();
        }
        selection_has_changed
    }
    fn check_items(&mut self, start: usize, end: usize, mode: CheckMode, emit_event: bool) {
        if self.flags.contains(Flags::NoSelection) {
            return;
        }
        let len = self.filter.len();
        if len == 0 {
            return;
        }
        let p_start = start.min(end).min(len - 1);
        let p_end = end.max(start).min(len - 1);
        let mut selection_has_changed = false;
        for pos in p_start..=p_end {
            selection_has_changed |= self.check_item(pos, mode, false, false);
        }
        self.update_check_count_for_groups();
        if (emit_event) && (selection_has_changed) {
            self.emit_selection_update_event();
        }
    }
    fn mouse_pos_to_index(&self, x: i32, y: i32) -> Option<usize> {
        match self.view_mode {
            ViewMode::Details => {
                let sz = self.size();
                if (y >= 1) && (x >= 0) && (x < sz.width as i32) && (y < sz.height as i32) {
                    let new_pos = self.top_view + (y - 1) as usize;
                    if new_pos < self.filter.len() {
                        Some(new_pos)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            ViewMode::Columns(_) => {
                let sz = self.size();
                if (y >= 0) && (x >= 0) && (x < sz.width as i32) && (y < sz.height as i32) {
                    let item_width = (self.item_width() + 1) as i32;
                    let column = x / item_width;
                    let index = self.top_view as i32 + column * (self.size().height as i32) + y;
                    if (index >= 0) && ((index as usize) < self.filter.len()) {
                        Some(index as usize)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }
    fn hover_status_for_mouse_pos(&self, pos: usize, x: i32) -> HoverStatus {
        if (pos >= self.filter.len()) || (self.header.columns().is_empty()) {
            return HoverStatus::None;
        }
        let left_pos = match self.view_mode {
            ViewMode::Details => self.header.columns()[0].x,
            ViewMode::Columns(_) => {
                let item_width = (self.item_width() + 1) as i32;
                (x / item_width) * item_width
            }
        };
        match self.filter[pos] {
            Element::Item(_) => {
                if self.flags.contains(Flags::CheckBoxes) {
                    let mut left = left_pos;
                    left += if self.flags.contains(Flags::ShowGroups) {
                        X_OFFSET_FOR_GROUP_ITEMS
                    } else {
                        0
                    };
                    if x == left {
                        HoverStatus::OverItemCheckMark(left, pos)
                    } else {
                        HoverStatus::None
                    }
                } else {
                    HoverStatus::None
                }
            }
            Element::Group(_) => {
                let l = if self.view_mode == ViewMode::Details { 0 } else { left_pos };
                if x == l + 1 {
                    HoverStatus::OverGroupFoldButton(l + 1, pos)
                } else if x >= l + 3 && x <= l + 5 && self.flags.contains(Flags::CheckBoxes) {
                    HoverStatus::OverGroupCheckMark(l + 3, pos)
                } else {
                    HoverStatus::None
                }
            }
        }
    }
    fn process_mouse_event(&mut self, event: &MouseEvent) -> bool {
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                if self.hover_status != HoverStatus::None {
                    self.hover_status = HoverStatus::None;
                    true
                } else {
                    false
                }
            }
            MouseEvent::Over(point) => {
                let new_hover_status = if let Some(pos) = self.mouse_pos_to_index(point.x, point.y) {
                    self.hover_status_for_mouse_pos(pos, point.x)
                } else {
                    HoverStatus::None
                };
                if new_hover_status != self.hover_status {
                    self.hover_status = new_hover_status;
                    true
                } else {
                    false
                }
            }
            MouseEvent::Pressed(ev) => {
                if let Some(pos) = self.mouse_pos_to_index(ev.x, ev.y) {
                    if pos != self.pos {
                        self.update_position(pos, true);
                    }
                    let left_pos = match self.view_mode {
                        ViewMode::Details => self.header.columns()[0].x,
                        ViewMode::Columns(_) => {
                            let item_width = (self.item_width() + 1) as i32;
                            (ev.x / item_width) * item_width
                        }
                    };
                    match self.filter[self.pos] {
                        Element::Item(_) => {
                            if self.flags.contains(Flags::CheckBoxes) {
                                let l = if self.flags.contains(Flags::ShowGroups) {
                                    X_OFFSET_FOR_GROUP_ITEMS
                                } else {
                                    0
                                };
                                if ev.x == l + left_pos {
                                    self.check_item(self.pos, CheckMode::Reverse, true, true);
                                }
                            }
                        }
                        Element::Group(gid) => {
                            let l = if self.view_mode == ViewMode::Details { 0 } else { left_pos };
                            if ev.x == l + 1 {
                                self.toggle_group_collapse_status(gid, true);
                            }
                            if self.flags.contains(Flags::CheckBoxes) && ev.x >= l + 3 && ev.x <= l + 5 {
                                self.check_item(self.pos, CheckMode::Reverse, true, true);
                            }
                        }
                    }
                    self.start_mouse_select = self.pos;
                    self.mouse_check_mode = self.toggle_current_item_selection();
                } else {
                    self.start_mouse_select = usize::MAX;
                }
                true
            }
            MouseEvent::Released(_) => true,
            MouseEvent::DoubleClick(ev) => {
                if let Some(pos) = self.mouse_pos_to_index(ev.x, ev.y) {
                    if pos != self.pos {
                        self.update_position(pos, true);
                    }
                    match self.filter[self.pos] {
                        Element::Item(index) => {
                            self.emit_item_action_event(index as usize);
                        }
                        Element::Group(gid) => {
                            self.toggle_group_collapse_status(gid, true);
                        }
                    }
                }
                true
            }
            MouseEvent::Drag(ev) => {
                if self.start_mouse_select != usize::MAX {
                    if let Some(pos) = self.mouse_pos_to_index(ev.x, ev.y) {
                        if pos != self.pos {
                            self.update_position(pos, true);
                            self.check_items(self.start_mouse_select, pos, self.mouse_check_mode, true);
                        }
                    }
                }
                true
            }
            MouseEvent::Wheel(dir) => {
                match dir {
                    MouseWheelDirection::Up => self.move_scroll_to(self.top_view.saturating_sub(1)),
                    MouseWheelDirection::Down => self.move_scroll_to(self.top_view.saturating_add(1)),
                    MouseWheelDirection::Left => {
                        OnKeyPressed::on_key_pressed(self, Key::new(KeyCode::Left, KeyModifier::None), 0 as char);
                    }
                    MouseWheelDirection::Right => {
                        OnKeyPressed::on_key_pressed(self, Key::new(KeyCode::Right, KeyModifier::None), 0 as char);
                    }
                }
                true
            }
        }
    }
}

impl<T> OnPaint for ListView<T>
where
    T: ListItem + 'static,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        match self.view_mode {
            ViewMode::Details => {
                // paint columns
                self.header.paint(surface, theme, &self.base);
                // paint items
                let has_groups = self.paint_items(surface, theme);
                // paint separation lines (columns)
                self.header.paint_columns(surface, theme, &self.base);
                // paint groups if visible
                if has_groups {
                    self.paint_groups(surface, theme);
                }
            }
            ViewMode::Columns(_) => {
                // paint items & groups
                if self.paint_items(surface, theme) {
                    self.paint_groups(surface, theme);
                }
                // paint a header and columns
                self.paint_column_lines(surface, theme);
            }
        }

        // paint scroll bars and searh bars
        self.comp.paint(surface, theme, &self.base);
    }
}

impl<T> OnKeyPressed for ListView<T>
where
    T: ListItem + 'static,
{
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        let action = if self.view_mode == ViewMode::Details {
            self.header.process_key_pressed(key)
        } else {
            ColumnsHeaderAction::None
        };
        if self.execute_column_header_action(action) {
            return EventProcessStatus::Processed;
        }
        if self.comp.process_key_pressed(key, character) {
            self.filter_items();
            return EventProcessStatus::Processed;
        }
        if self.process_key_pressed(key) {
            self.comp.exit_edit_mode();
            return EventProcessStatus::Processed;
        }
        if (action.should_repaint()) || (self.comp.should_repaint()) {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

impl<T> OnMouseEvent for ListView<T>
where
    T: ListItem + 'static,
{
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.comp.process_mouse_event(event) {
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
        }
        let action = if self.view_mode == ViewMode::Details {
            self.header.process_mouse_event(event)
        } else {
            ColumnsHeaderAction::None
        };
        if self.execute_column_header_action(action) {
            return EventProcessStatus::Processed;
        }
        // process mouse event for items
        if self.process_mouse_event(event) {
            return EventProcessStatus::Processed;
        }
        if (action.should_repaint()) || (self.comp.should_repaint()) {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}
impl<T> OnResize for ListView<T>
where
    T: ListItem + 'static,
{
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.header.resize(new_size);
        match self.view_mode {
            ViewMode::Details => {
                self.comp
                    .resize(self.header.width() as u64, self.filter.len() as u64, &self.base, self.visible_space());
            }
            ViewMode::Columns(_) => {
                self.comp.resize(0, self.filter.len() as u64, &self.base, self.visible_space());
            }
        }
    }
}
