enum Flags {
    Enabled,
    Visible,
    AcceptInput,
    AllowHovering,
    OnLeft
}

pub(super) struct ItemBase {
    flags: Flags,
    tooltip: String,
    x: i32,
    width: u8,
}