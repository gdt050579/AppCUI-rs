#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum OrthogonalDirection {
    HorizontalFirst,
    VerticalFirst,
    #[default]
    Auto,
}
