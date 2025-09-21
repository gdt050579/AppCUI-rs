/// Specifies how a appbar item will be position relative to the appbar:
/// - `Side::Left` - the appbar item (a button, label, separator, etc) will be position from the left side of the appbar
/// - `Side::Right` - the appbar item (a button, label, separator, etc) will be position from the right side of the appbar
#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub enum Side {
    Left,
    Right
}