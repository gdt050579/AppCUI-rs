use super::FoldStatus;
use super::Item;
use super::ItemVisibility;
use super::ListItem;
use crate::prelude::ColumnsHeader;
use crate::prelude::*;
use crate::system::Handle;
use std::cmp::Ordering;

macro_rules! new_mutable_ref {
    ($current_ref:expr) => {
        unsafe {
            let px = $current_ref as *mut Vec<Handle<Item<T>>>;
            &mut *px
        }
    };
}

pub(super) struct TreeDataManager<T>
where
    T: ListItem + 'static,
{
    data: Vec<Option<Item<T>>>,
    free: Vec<u32>,
    roots: Vec<Handle<Item<T>>>,
    selected_count: usize,
    count: usize,
}

impl<T> TreeDataManager<T>
where
    T: ListItem + 'static,
{
    pub(super) fn with_capacity(capacity: u32) -> Self {
        Self {
            data: Vec::with_capacity(capacity as usize),
            free: Vec::with_capacity(16),
            roots: Vec::with_capacity(16),
            selected_count: 0,
            count: 0,
        }
    }
    fn handle_to_index(&self, handle: Handle<Item<T>>) -> Option<usize> {
        if handle.is_none() {
            return None;
        }
        let idx = handle.index();
        if idx >= self.data.len() {
            None
        } else if let Some(item) = &self.data[idx] {
            if item.handle == handle {
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn inner_add(&mut self, mut item: Item<T>, parent_handle: Handle<Item<T>>) -> Handle<Item<T>> {
        // find the position and set my own handle
        if let Some(index) = self.free.pop() {
            item.handle = Handle::new(index);
        } else {
            item.handle = Handle::new(self.data.len() as u32);
        }
        // add to parent
        if let Some(idx) = self.handle_to_index(parent_handle) {
            item.parent = parent_handle;
            // I kwno that the parent is not None
            let parent = self.data[idx].as_mut().unwrap();
            item.depth = parent.depth + 1;
            parent.children.push(item.handle);
            if parent.fold_status == FoldStatus::NonExpandable {
                parent.fold_status = FoldStatus::Expanded;
                log!("INFO", "Setting parent to expanded for {:?}", parent_handle);
            }
        } else {
            item.parent = Handle::None;
            item.depth = 0;
            self.roots.push(item.handle);
        };
        // update selection
        if item.is_selected() {
            self.selected_count += 1;
        }
        // move to vector
        let idx = item.handle.index();
        let h = item.handle;
        if idx < self.data.len() {
            self.data[idx] = Some(item);
        } else {
            self.data.push(Some(item));
        }
        self.count += 1;
        h
    }
    pub(super) fn add(&mut self, item: Item<T>, parent: Handle<Item<T>>) -> Handle<Item<T>> {
        self.inner_add(item, parent)
    }
    pub(super) fn delete_children(&mut self, parent: Handle<Item<T>>) {
        if let Some(idx) = self.handle_to_index(parent) {
            let parent_ref = self.data[idx].as_mut().unwrap();
            let children = new_mutable_ref!(&mut parent_ref.children);
            for handle in children.iter() {
                self.delete_children(*handle);
                if let Some(idx) = self.handle_to_index(*handle) {
                    if self.data[idx].as_ref().unwrap().is_selected() {
                        self.selected_count = self.selected_count.saturating_sub(1);
                    }
                    self.free.push(idx as u32);
                    self.data[idx] = None;
                    self.count = self.count.saturating_sub(1);
                }
            }
            self.data[idx].as_mut().unwrap().children.clear();
        }
    }
    pub(super) fn delete(&mut self, handle: Handle<Item<T>>) {
        if let Some(idx) = self.handle_to_index(handle) {
            self.delete_children(handle);
            // search position in parent child list
            let parent_handle = self.data[idx].as_mut().unwrap().parent;
            if let Some(parent_idx) = self.handle_to_index(parent_handle) {
                let parent = self.data[parent_idx].as_mut().unwrap();
                parent.children.retain(|h| *h != handle);
            } else {
                self.roots.retain(|h| *h != handle);
            }
            if self.data[idx].as_ref().unwrap().is_selected() {
                self.selected_count = self.selected_count.saturating_sub(1);
            }
            self.free.push(idx as u32);
            self.data[idx] = None;
            self.count = self.count.saturating_sub(1);
        }
    }
    pub(super) fn clear(&mut self) {
        self.data.clear();
        self.free.clear();
        self.roots.clear();
        self.selected_count = 0;
    }
    #[inline(always)]
    pub(super) fn roots(&self) -> &[Handle<Item<T>>] {
        &self.roots
    }
    #[inline(always)]
    pub(super) fn get(&self, handle: Handle<Item<T>>) -> Option<&Item<T>> {
        if let Some(idx) = self.handle_to_index(handle) {
            self.data[idx].as_ref()
        } else {
            None
        }
    }
    #[inline(always)]
    pub(super) fn get_mut(&mut self, handle: Handle<Item<T>>) -> Option<&mut Item<T>> {
        if let Some(idx) = self.handle_to_index(handle) {
            self.data[idx].as_mut()
        } else {
            None
        }
    }
    #[inline(always)]
    pub(super) fn len(&self) -> usize {
        self.data.len()
    }

    fn pupulate_children(&mut self, handle_list: &[Handle<Item<T>>], output: &mut Vec<Handle<Item<T>>>, last_mask: u32, depth: u16) {
        if handle_list.is_empty() {
            return;
        }
        let mut last_mask = last_mask;
        // find the last position that will be added
        let mut last_index = None;
        for (index, h) in handle_list.iter().rev().enumerate() {
            if let Some(item) = self.get_mut(*h) {
                if item.is_visible() {
                    last_index = Some(handle_list.len() - 1 - index);
                    break;
                }
            }
        }
        if let Some(idx) = last_index {
            if idx > 0 {
                // process all until the last one
                for h in handle_list[..idx].iter() {
                    if let Some(item) = self.get_mut(*h) {
                        if item.is_visible() {
                            last_mask = item.set_line_mask(last_mask, depth, false);
                            output.push(*h);
                            if item.fold_status == FoldStatus::Expanded {
                                let list = new_mutable_ref!(&mut item.children);
                                self.pupulate_children(list, output, last_mask, depth + 1);
                            }
                        }
                    }
                }
            }
            // process the last one (we know it should be added)
            let h = handle_list[idx];
            if let Some(item) = self.get_mut(h) {
                last_mask = item.set_line_mask(last_mask, depth, true);
                output.push(h);
                if item.fold_status == FoldStatus::Expanded {
                    let list = new_mutable_ref!(&mut item.children);
                    self.pupulate_children(list, output, last_mask, depth + 1);
                }
            }
        }
    }
    pub(super) fn populate(&mut self, output: &mut Vec<Handle<Item<T>>>) {
        let l = new_mutable_ref!(&mut self.roots);
        let mut last_mask = 0;
        for h in l.iter() {
            if let Some(item) = self.get_mut(*h) {
                if item.is_visible() {
                    last_mask = item.set_line_mask(last_mask, 0, false);
                    output.push(*h);
                    if item.fold_status == FoldStatus::Expanded {
                        let list = new_mutable_ref!(&mut item.children);
                        self.pupulate_children(list, output, last_mask, 1);
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn compare(&self, h1: Handle<Item<T>>, h2: Handle<Item<T>>, column_index: u16, ascendent: bool) -> Ordering {
        let i1 = self.get(h1);
        let i2 = self.get(h2);
        match (i1, i2) {
            (Some(item1), Some(item2)) => {
                let result = item1.value().compare(item2.value(), column_index);
                if !ascendent {
                    result.reverse()
                } else {
                    result
                }
            }
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        }
    }

    fn sort_by(data: &mut [Handle<Item<T>>], manager: &mut TreeDataManager<T>, column_index: u16, ascendent: bool) {
        data.sort_by(|h1, h2| manager.compare(*h1, *h2, column_index, ascendent));
        for h in data.iter() {
            if let Some(item) = manager.get_mut(*h) {
                if !item.children.is_empty() {
                    let p = new_mutable_ref!(&mut item.children);
                    TreeDataManager::sort_by(p, manager, column_index, ascendent);
                }
            }
        }
    }

    pub(super) fn sort(&mut self, column_index: u16, ascendent: bool) {
        let p = new_mutable_ref!(&mut self.roots);
        TreeDataManager::sort_by(p, self, column_index, ascendent);
    }

    fn filter(&mut self, handle: Handle<Item<T>>, search_text: &str, header: Option<&ColumnsHeader>) -> bool {
        let visibility = if let Some(item) = self.get_mut(handle) {
            let match_search_criteria = item.matches(search_text, header);
            let p = new_mutable_ref!(&mut item.children);
            let mut has_visible_children = false;
            for h in p.iter() {
                has_visible_children |= self.filter(*h, search_text, header);
            }
            match (match_search_criteria, has_visible_children) {
                (true, _) => Some(ItemVisibility::Visible),
                (false, true) => Some(ItemVisibility::VisibleBecauseOfChildren),
                (false, false) => Some(ItemVisibility::Hidden),
            }
        } else {
            None
        };
        if let Some(vis) = visibility {
            if let Some(item) = self.get_mut(handle) {
                item.visibility = vis;
                if matches!(vis, ItemVisibility::Visible | ItemVisibility::VisibleBecauseOfChildren) {
                    // expand its parent
                    let parent = item.parent;
                    if let Some(parent) = self.get_mut(parent) {
                        parent.fold_status = FoldStatus::Expanded;
                    }
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }
    pub(super) fn refilter(&mut self, search_text: &str, header: Option<&ColumnsHeader>) {
        //println!("Refiltering for [{search_text}]");
        let list = new_mutable_ref!(&mut self.roots);
        for h in list.iter() {
            self.filter(*h, search_text, header);
        }
    }

    pub(super) fn set_fold_status(&mut self, parent: Handle<Item<T>>, fold_status: FoldStatus) -> bool {
        if let Some(item) = self.get_mut(parent) {
            if item.fold_status != FoldStatus::NonExpandable {
                let mut change = item.fold_status != fold_status;
                item.fold_status = fold_status;
                let p = new_mutable_ref!(&mut item.children);
                for h in p.iter() {
                    change |= self.set_fold_status(*h, fold_status);
                }
                change
            } else {
                false
            }
        } else {
            false
        }
    }

    pub(super) fn collapse_all(&mut self) -> bool {
        let mut change = false;
        let list = new_mutable_ref!(&mut self.roots);
        for h in list.iter() {
            change |= self.set_fold_status(*h, FoldStatus::Collapsed);
        }
        change
    }

    pub(super) fn expand_all(&mut self) -> bool {
        let mut change = false;
        let list = new_mutable_ref!(&mut self.roots);
        for h in list.iter() {
            change |= self.set_fold_status(*h, FoldStatus::Expanded);
        }
        change
    }

    #[inline(always)]
    pub(super) fn update_selected_count(&mut self, increase: bool) {
        if increase {
            self.selected_count += 1;
        } else {
            self.selected_count = self.selected_count.saturating_sub(1);
        }
    }

    #[inline(always)]
    pub(super) fn selected_count(&self) -> usize {
        self.selected_count
    }

    #[inline(always)]
    pub(super) fn count(&self) -> usize {
        self.count
    }

    #[cfg(test)]
    pub(super) fn free_list(&self) -> &Vec<u32> {
        &self.free
    }
}
