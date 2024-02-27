use crate::system::{Handle, HandleSupport};

pub(crate) struct HandleManager<T> {
    objects: Vec<Option<T>>,
    free: Vec<u32>,
}
impl<T> HandleManager<T>
where
    T: HandleSupport<T>,
{
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            objects: Vec::with_capacity(capacity),
            free: Vec::with_capacity(8),
        }
    }
    pub(crate) fn new() -> Self {
        Self {
            objects: Vec::new(),
            free: Vec::new(),
        }
    }
    pub(crate) fn add(&mut self, mut object: T) -> Handle<T> {
        if let Some(pos) = self.free.pop() {
            // add at pos
            let h = Handle::new(pos);
            object.set_handle(h);
            self.objects[pos as usize] = Some(object);
            return h;
        } else {
            // ad at the end
            let h = Handle::new(self.objects.len() as u32);
            object.set_handle(h);
            self.objects.push(Some(object));
            return h;
        }
    }
    pub(crate) fn remove(&mut self, handle: Handle<T> ) -> bool {
        if handle.is_none() {
            return false;
        }
        let idx = handle.get_index();
        if idx >= self.objects.len() {
            return false;
        }
        let m = self.objects[idx].as_ref();
        if m.is_none() {
            return false;
        }
        if m.as_ref().unwrap().get_handle() != handle {
            return false;
        }
        // ok -> we can remove it
        self.objects[idx] = None;
        self.free.push(idx as u32);
        return true;
    }
    pub(crate) fn get(&self, handle: Handle<T>) -> Option<&T> {
        if handle.is_none() {
            return None;
        }
        let idx = handle.get_index();
        if idx < self.objects.len() {
            let m = self.objects[idx].as_ref();
            if m.is_some() {
                if m.as_ref().unwrap().get_handle() == handle {
                    return m;
                }
            }
        }
        None
    }
    pub(crate) fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        if handle.is_none() {
            return None;
        }
        let idx = handle.get_index();
        if idx < self.objects.len() {
            let m = self.objects[idx].as_mut();
            if m.is_some() {
                if m.as_ref().unwrap().get_handle() == handle {
                    return m;
                }
            }
        }
        None
    }
    pub(crate) fn get_element(&self, index: usize) -> Option<&T> {
        if index < self.objects.len() {
            return self.objects[index].as_ref();
        }
        None
    }
    pub(crate) fn get_element_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.objects.len() {
            return self.objects[index].as_mut();
        }
        None
    }

    pub(crate) fn allocated_objects(&self) -> usize {
        self.objects.len()
    }

    pub fn free_spaces(&self) -> usize {
        self.free.len()
    }
}
