use std::ops::{Deref, DerefMut};

pub trait ReferenceDrop {
    fn on_reference_drop(&mut self);
}
pub struct Reference<'a, T>
where
    T: ReferenceDrop,
{
    object: &'a mut T,
}
impl<T> Deref for Reference<'_, T>
where
    T: ReferenceDrop,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.object
    }
}
impl<T> DerefMut for Reference<'_, T>
where
    T: ReferenceDrop,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.object
    }
}
impl<T> Drop for Reference<'_, T>
where
    T: ReferenceDrop,
{
    fn drop(&mut self) {
        self.on_reference_drop();
    }
}
