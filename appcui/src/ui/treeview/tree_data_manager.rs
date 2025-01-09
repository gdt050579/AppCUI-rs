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
    pub(super) fn len(&self) -> usize {
        self.data.len()
    }
    #[cfg(test)]
    pub(super) fn free_list(&self) -> &Vec<u32> {
        &self.free
    }
}
