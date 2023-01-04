pub (super) enum Size {
    Absolute(u32),
    Percentage(u32)
}
impl Size {
    pub (super) fn to_absolute_coordonate(&self, parent_size: u32)->u32 {
        match self
        {
            Size::Absolute(v) => { return *v; },
            Size::Percentage(v) => { return v * parent_size  / 10000u32; }
        }
    }
}