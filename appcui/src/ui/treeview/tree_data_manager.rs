use super::Item;
use super::ListItem;
use crate::system::Handle;

pub(super) struct TreeDataManager<T>
where
    T: ListItem + 'static,
{
    data: Vec<Option<Item<T>>>,
    free: Vec<u32>,
}

impl<T> TreeDataManager<T> where T: ListItem + 'static {
    pub(super) fn with_capacity(capacity: u32) -> Self {
        Self {
            data: Vec::with_capacity(capacity as usize),
            free: Vec::with_capacity(16),
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
        let prev_child = if let Some(idx) = self.handle_to_index(parent) {
            // I kwno that the parent is not None
            let parent = self.data[idx].as_mut().unwrap();
            let h = parent.child;
            parent.child = item.handle;
            h
        } else {
            Handle::None
        };
        if let Some(idx) = self.handle_to_index(prev_child) {
            let prev_child = self.data[idx].as_mut().unwrap();
            prev_child.prev_sibling = item.handle;
            item.next_sibling = prev_child.handle;
        }
        // move to vector
        let idx = item.handle.index() as usize;
        let h = item.handle;
        if idx<self.data.len() {
            self.data[idx] = Some(item);
        } else {
            self.data.push(Some(item));
        }
        h
    }
}
