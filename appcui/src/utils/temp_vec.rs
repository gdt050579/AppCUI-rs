struct StackVec<T: Copy, const N: usize> {
    buffer: [T; N],
    len: u16,
}
enum InnerTempVec<T: Copy, const N: usize> {
    StackVec(StackVec<T, N>),
    HeapVec(Vec<T>),
}
pub(crate) struct TempVec<T: Copy, const N: usize> {
    inner: InnerTempVec<T, N>,
}
impl<T: Copy + Default, const N: usize> TempVec<T, N> {
    pub(crate) fn new() -> Self {
        if N == 0 {
            panic!("N must be greater than 0");
        }
        if N > 65534 {
            panic!("N must be less than 65535");
        }
        Self { inner: InnerTempVec::StackVec(StackVec { buffer: [T::default(); N], len: 0 }) }
    }
}
impl<T: Copy, const N: usize> TempVec<T, N> {
    pub(crate) fn push(&mut self, value: T) {
        match &mut self.inner {
            InnerTempVec::StackVec(vec) => {
                if (vec.len as usize) < N {
                    vec.buffer[vec.len as usize] = value;
                    vec.len += 1;
                } else {
                    let mut heap_vec = Vec::with_capacity(N);
                    heap_vec.extend_from_slice(&vec.buffer[..vec.len as usize]);
                    heap_vec.push(value);
                    self.inner = InnerTempVec::HeapVec(heap_vec);
                }
            }
            InnerTempVec::HeapVec(vec) => {
                vec.push(value);
            }
        }
    }
    #[inline(always)]
    #[cfg(test)]
    pub(crate) fn clear(&mut self) {
        match &mut self.inner {
            InnerTempVec::StackVec(vec) => {
                vec.len = 0;
            }
            InnerTempVec::HeapVec(vec) => {
                vec.clear();
            }
        }
    }
    #[inline(always)]
    pub(crate) fn as_slice(&self) -> &[T] {
        match &self.inner {
            InnerTempVec::StackVec(vec) => &vec.buffer[..vec.len as usize],
            InnerTempVec::HeapVec(vec) => vec.as_slice(),
        }
    }
    #[inline(always)]
    #[cfg(test)]
    pub(crate) fn get(&self, index: usize) -> Option<&T> {
        match &self.inner {
            InnerTempVec::StackVec(vec) => {
                if index < (vec.len as usize) {
                    Some(&vec.buffer[index])
                } else {
                    None
                }
            }
            InnerTempVec::HeapVec(vec) => {
                vec.get(index)
            }
        }
    }
    #[inline(always)]
    #[cfg(test)]
    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match &mut self.inner {
            InnerTempVec::StackVec(vec) => {
                if index < (vec.len as usize) {
                    Some(&mut vec.buffer[index])
                } else {
                    None
                }
            }
            InnerTempVec::HeapVec(vec) => {
                vec.get_mut(index)
            }
        }
    }
    #[inline(always)]
    #[cfg(test)]
    pub(crate) fn is_on_heap(&self) -> bool {
        matches!(&self.inner, InnerTempVec::HeapVec(_))
    }
    #[inline(always)]
    pub(crate) fn is_empty(&self) -> bool {
        match &self.inner {
            InnerTempVec::StackVec(vec) => vec.len == 0,
            InnerTempVec::HeapVec(vec) => vec.is_empty(),
        }
    }
    #[inline(always)]
    pub(crate) fn len(&self) -> usize {
        match &self.inner {
            InnerTempVec::StackVec(vec) => vec.len as usize,
            InnerTempVec::HeapVec(vec) => vec.len(),
        }
    }
}