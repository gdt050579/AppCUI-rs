#[derive(Copy, Clone)]
pub enum Strategy {
    Clamp,
    Rotate,
    RotateWithInvalidState,
    RotateFromInvalidState,
}

#[derive(Copy, Clone, PartialEq)]
pub(crate) struct VectorIndex {
    value: usize,
}

impl VectorIndex {
    const INVALID_INDEX: usize = usize::MAX;
    #[allow(non_upper_case_globals)]
    pub(crate) const Invalid: VectorIndex = VectorIndex { value: usize::MAX };
    #[allow(non_upper_case_globals)]
    pub(crate) const First: VectorIndex = VectorIndex { value: 0 };

    #[inline(always)]
    pub fn last(count: usize) -> Self {
        Self {
            value: if count > 0 { count - 1 } else { Self::INVALID_INDEX },
        }
    }
    #[inline(always)]
    pub fn with_value(value: usize) -> Self {
        Self { value }
    }
    #[inline(always)]
    pub fn index(&self) -> usize {
        self.value
    }
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        self.value != Self::INVALID_INDEX
    }
    #[inline(always)]
    pub fn in_range(&self, count: usize) -> bool {
        self.value < count
    }
    #[inline(always)]
    pub fn set(&mut self, value: usize, count: usize, clamp: bool) {
        if count == 0 {
            self.value = Self::INVALID_INDEX;
            return;
        }
        if clamp {
            self.value = value.min(count - 1);
        } else {
            self.value = if value >= count { Self::INVALID_INDEX } else { value };
        }
    }
    #[inline(always)]
    pub fn add(&mut self, value: usize, count: usize, strategy: Strategy) {
        if count == 0 {
            self.value = Self::INVALID_INDEX;
            return;
        }
        match strategy {
            Strategy::Clamp => {
                if self.value != Self::INVALID_INDEX {
                    self.value += value;
                    if self.value >= count {
                        self.value = count - 1;
                    }
                }
            }
            Strategy::Rotate => {
                if self.value != Self::INVALID_INDEX {
                    self.value = (self.value + value) % count;
                }
            }
            Strategy::RotateWithInvalidState => {
                if self.value != Self::INVALID_INDEX {
                    self.value += value;
                    if self.value >= count {
                        self.value = Self::INVALID_INDEX
                    }
                } else {
                    self.value = 0; // first
                }
            }
            Strategy::RotateFromInvalidState => {
                if self.value != Self::INVALID_INDEX {
                    self.value = (self.value + value) % count;
                } else {
                    self.value = 0; // first
                }
            }
        }
    }
    #[inline(always)]
    pub fn sub(&mut self, value: usize, count: usize, strategy: Strategy) {
        if count == 0 {
            self.value = Self::INVALID_INDEX;
            return;
        }
        match strategy {
            Strategy::Clamp => {
                if self.value != Self::INVALID_INDEX {
                    if self.value >= value {
                        self.value -= value;
                    } else {
                        self.value = 0;
                    }
                }
            }
            Strategy::Rotate => {
                if self.value != Self::INVALID_INDEX {
                    let value = value % count;
                    self.value = (self.value + count - value) % count;
                }
            }
            Strategy::RotateWithInvalidState => {
                if self.value != Self::INVALID_INDEX {
                    if self.value >= value {
                        self.value -= value;
                    } else {
                        self.value = Self::INVALID_INDEX;
                    }
                } else {
                    self.value = count - 1;
                }
            }
            Strategy::RotateFromInvalidState => {
                if self.value != Self::INVALID_INDEX {
                    let value = value % count;
                    self.value = (self.value + count - value) % count;
                } else {
                    self.value = count - 1;
                }
            }
        }
    }
}
impl Default for VectorIndex {
    fn default() -> Self {
        Self { value: Self::INVALID_INDEX }
    }
}
impl From<usize> for VectorIndex {
    fn from(value: usize) -> Self {
        VectorIndex { value }
    }
}
impl From<u32> for VectorIndex {
    fn from(value: u32) -> Self {
        VectorIndex { value: value as usize }
    }
}
