pub (super) enum Coordonate {
    Absolute(i32),
    Percentage(i32)
}
impl Coordonate {
    pub (super) fn to_absolute_coordonate(&self, parent_size: u32)->i32 {
        match self
        {
            Coordonate::Absolute(v) => { return *v; },
            Coordonate::Percentage(v) => { return v * (parent_size as i32) / 10000; }
        }
    }
}