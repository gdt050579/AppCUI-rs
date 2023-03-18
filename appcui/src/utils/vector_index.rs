pub enum Strategy {
    Clamp,
    Rotate,
}

#[derive(Copy, Clone, PartialEq)]
pub(crate) struct VectorIndex {
    value: usize,
}

impl VectorIndex {
    const INVALID_INDEX: usize = usize::MAX;
    #[inline(always)]
    pub fn first() -> Self {
        Self { value: 0 }
    }
    #[inline(always)]
    pub fn last(count: usize) -> Self {
        Self {
            value: if count > 0 {
                count - 1
            } else {
                Self::INVALID_INDEX
            },
        }
    }
    #[inline(always)]
    pub fn with_value(value: usize) -> Self {
        Self { value }
    }
    #[inline(always)]
    pub fn invalid() -> Self {
        Self {
            value: Self::INVALID_INDEX,
        }
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
        (self.value < count) && (count != Self::INVALID_INDEX)
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
            self.value = if value >= count {
                Self::INVALID_INDEX
            } else {
                value
            };
        }
    }
    #[inline(always)]
    pub fn add(&mut self, value: usize, count: usize, strategy: Strategy) {
        if self.value == Self::INVALID_INDEX {
            return;
        }
        if count == 0 {
            self.value = Self::INVALID_INDEX;
            return;
        }
        match strategy {
            Strategy::Clamp => {
                self.value += value;
                if self.value >= count {
                    self.value = count - 1;
                }
            }
            Strategy::Rotate => {
                self.value = (self.value + value) % count;
            }
        }
    }
    #[inline(always)]
    pub fn sub(&mut self, value: usize, count: usize, strategy: Strategy) {
        if self.value == Self::INVALID_INDEX {
            return;
        }
        if count == 0 {
            self.value = Self::INVALID_INDEX;
            return;
        }
        match strategy {
            Strategy::Clamp => {
                if self.value >= value {
                    self.value -= value;
                } else {
                    self.value = 0;
                }
            }
            Strategy::Rotate => {
                let value = value % count;
                self.value = (self.value + count - value) % count;
            }
        }
    }
}
impl Default for VectorIndex {
    fn default() -> Self {
        Self {
            value: Self::INVALID_INDEX,
        }
    }
}
