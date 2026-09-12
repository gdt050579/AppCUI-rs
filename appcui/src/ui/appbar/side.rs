/// Which edge of the app bar an item is laid out from.
///
/// Combined with the item's `order` (lower first). Left items grow rightward;
/// right items grow leftward.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Side {
    /// Pack from the left edge of the app bar (menus, File/Edit, and similar).
    Left,
    /// Pack from the right edge of the app bar (Help, status, and similar).
    Right,
}