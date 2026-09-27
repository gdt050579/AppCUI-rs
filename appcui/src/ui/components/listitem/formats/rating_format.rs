#[derive(Copy, Clone, Eq, PartialEq)]
/// How a rating list-item value is rendered.
///
/// The associated `u32` is the maximum score. Variants draw a number, stars,
/// circles, or asterisks.
pub enum RatingFormat {
    /// A numeric score, for example `3/5`.
    Numerical(u32),
    /// Stars, for example `★★★☆☆`.
    Stars(u32),
    /// Circles, for example `●●●○○`.
    Circles(u32),
    /// Asterisks, for example `***--`.
    Asterix(u32),
}