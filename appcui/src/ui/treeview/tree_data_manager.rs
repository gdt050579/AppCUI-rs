use std::cmp::Ordering;

use super::Item;
use super::ListItem;
use crate::system::Handle;

pub(super) struct TreeDataManager<T>
where
    T: ListItem + 'static,
{
    data: Vec<Option<Item<T>>>,
    free: Vec<u32>,
    roots: Vec<Handle<Item<T>>>,
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
        }
    }
    fn handle_to_index(&self, handle: Handle<Item<T>>) -> Option<usize> {
        if handle.is_none() {
            return None;
        }
        let idx = handle.index();
        if idx >= self.data.len() {
            None
        } else {
            if let Some(item) = &self.data[idx as usize] {
                if item.handle == handle {
                    Some(idx as usize)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    fn inner_add(&mut self, mut item: Item<T>, parent: Handle<Item<T>>) -> Handle<Item<T>> {
        // find the position and set my own handle
        if let Some(index) = self.free.pop() {
            item.handle = Handle::new(index);
        } else {
            item.handle = Handle::new(self.data.len() as u32);
        }
        // add to parent
        if let Some(idx) = self.handle_to_index(parent) {
            item.parent = parent;
            // I kwno that the parent is not None
            let parent = self.data[idx].as_mut().unwrap();
            item.depth = parent.depth + 1;
            parent.children.push(item.handle);
        } else {
            item.parent = Handle::None;
            item.depth = 0;
            self.roots.push(item.handle);
        };
        // move to vector
        let idx = item.handle.index() as usize;
        let h = item.handle;
        if idx < self.data.len() {
            self.data[idx] = Some(item);
        } else {
            self.data.push(Some(item));
        }
        h
    }
    pub(super) fn add(&mut self, item: Item<T>, parent: Handle<Item<T>>) -> Handle<Item<T>> {
        self.inner_add(item, parent)
    }
    pub(super) fn delete_children(&mut self, parent: Handle<Item<T>>) {
        if let Some(idx) = self.handle_to_index(parent) {
            let parent_ref = self.data[idx].as_mut().unwrap();
            let parent = unsafe {
                let p = parent_ref as *mut Item<T>;
                &mut *p
            };
            for handle in parent.children.iter() {
                self.delete_children(*handle);
                if let Some(idx) = self.handle_to_index(*handle) {
                    self.free.push(idx as u32);
                    self.data[idx] = None;
                }
            }
            parent.children.clear();
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
            self.free.push(idx as u32);
            self.data[idx] = None;
        }
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

    fn pupulate_children(&self, handle: Handle<Item<T>>, output: &mut Vec<Handle<Item<T>>>) {
        if let Some(idx) = self.handle_to_index(handle) {
            let item = self.data[idx].as_ref().unwrap();
            for h in item.children.iter() {
                output.push(*h);
                self.pupulate_children(*h, output);
            }
        }
    }
    pub(super) fn populate(&self, output: &mut Vec<Handle<Item<T>>>) {
        for h in self.roots.iter() {
            output.push(*h);
            self.pupulate_children(*h, output);
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

    fn sort_by(data: &mut Vec<Handle<Item<T>>>, manager: &mut TreeDataManager<T>, column_index: u16, ascendent: bool)
    {
        data.sort_by(|h1,h2| manager.compare(*h1, *h2, column_index, ascendent));
        for h in data.iter() {
            if let Some(item) = manager.get_mut(*h) {
                if !item.children.is_empty() {
                    let p = unsafe {
                        let px = &mut item.children as *mut Vec<Handle<Item<T>>>;
                        &mut *px
                    };
                    TreeDataManager::sort_by(p, manager, column_index, ascendent);
                }
            }
        }
    }

    pub(super) fn sort(&mut self, column_index: u16, ascendent: bool) {
        let p = unsafe {
            let px = &mut self.roots as *mut Vec<Handle<Item<T>>>;
            &mut *px
        };
        TreeDataManager::sort_by(p, self, column_index, ascendent);
    }

    #[cfg(test)]
    pub(super) fn free_list(&self) -> &Vec<u32> {
        &self.free
    }
}
