#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RatingFormat {
    Numerical(u32),
    Stars(u32),
    Circles(u32),
    Asterix(u32),
}