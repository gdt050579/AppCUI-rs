#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct ValueMapping {
    lo: f64,
    scale: f64,
    height: u32,
    zero: u32,
}

impl ValueMapping {
    pub(super) fn new(lo: f64, hi: f64, height: u32) -> Self {
        let lo = if lo.is_finite() { lo } else { 0.0 };
        let scale = if height > 0 && hi.is_finite() && hi > lo && (hi - lo).is_finite() {
            height as f64 / (hi - lo)
        } else {
            0.0
        };
        let mut mapping = Self { lo, scale, height, zero: 0 };
        mapping.zero = mapping.position(0.0);
        mapping
    }
    #[inline(always)]
    pub(super) fn height(&self) -> u32 {
        self.height
    }
    #[inline(always)]
    pub(super) fn zero(&self) -> u32 {
        self.zero
    }
    #[inline(always)]
    pub(super) fn above(&self) -> u32 {
        self.height - self.zero
    }
    #[inline(always)]
    pub(super) fn below(&self) -> u32 {
        self.zero
    }
    pub(super) fn position(&self, value: f64) -> u32 {
        let p = (value - self.lo) * self.scale;
        if p.is_nan() {
            return self.zero;
        }
        p.round().clamp(0.0, self.height as f64) as u32
    }
    pub(super) fn offset(&self, value: f64) -> i32 {
        self.position(value) as i32 - self.zero as i32
    }
}
