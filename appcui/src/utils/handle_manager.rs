use crate::system::{Handle, HandleSupport};

#[derive(Default)]
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
    // pub(crate) fn new() -> Self {
    //     Self {
    //         objects: Vec::new(),
    //         free: Vec::new(),
    //     }
    // }
    pub(crate) fn add(&mut self, mut object: T) -> Handle<T> {
        if let Some(pos) = self.free.pop() {
            // add at pos
            let h = Handle::new(pos);
            object.set_handle(h);
            self.objects[pos as usize] = Some(object);
            h
        } else {
            // ad at the end
            let h = Handle::new(self.objects.len() as u32);
            object.set_handle(h);
            self.objects.push(Some(object));
            h
        }
    }
    pub(crate) fn remove(&mut self, handle: Handle<T> ) -> bool {
        if handle.is_none() {
            return false;
        }
        let idx = handle.index();
        if idx >= self.objects.len() {
            return false;
        }
        let m = self.objects[idx].as_ref();
        if m.is_none() {
            return false;
        }
        if m.as_ref().unwrap().handle() != handle {
            return false;
        }
        // ok -> we can remove it
        self.objects[idx] = None;
        self.free.push(idx as u32);
        true
    }

    #[inline(always)]
    pub(crate) fn get(&self, handle: Handle<T>) -> Option<&T> {
        let idx = handle.index();
        self.objects.get(idx).and_then(|opt_obj| {
            if let Some(obj) = opt_obj {
                if obj.handle() == handle {
                    return Some(obj);
                }
            }
            None
        })        
    }

    #[inline(always)]
    pub(crate) fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        let idx = handle.index();
        self.objects.get_mut(idx).and_then(|opt_obj| {
            if let Some(obj) = opt_obj {
                if obj.handle() == handle {
                    return Some(obj);
                }
            }
            None
        })
    }
    
    #[inline(always)]
    pub(crate) fn element(&self, index: usize) -> Option<&T> {
        self.objects.get(index).and_then(|x| x.as_ref())
    }
    #[inline(always)]
    pub(crate) fn element_mut(&mut self, index: usize) -> Option<&mut T> {
        self.objects.get_mut(index).and_then(|x| x.as_mut())
    }

    #[inline(always)]
    pub(crate) fn allocated_objects(&self) -> usize {
        self.objects.len()
    }

    #[cfg(test)]
    pub(super) fn free_spaces(&self) -> usize {
        self.free.len()
    }
}
