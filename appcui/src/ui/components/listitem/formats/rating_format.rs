#[derive(Copy, Clone, Eq, PartialEq)]
/// How a rating list-item value is rendered.
///
/// The associated `u32` is the maximum score. Variants draw a number, stars,
/// circles, or asterisks.
pub enum RatingFormat {
    Numerical(u32),
    Stars(u32),
    Circles(u32),
    Asterix(u32),
}