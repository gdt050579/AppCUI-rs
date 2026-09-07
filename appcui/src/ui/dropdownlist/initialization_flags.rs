use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=8)]
/// Initialization flags for a [`struct@super::DropDownList`].
///
/// Combine values with `|`. `Flags::None` always has a selected item and shows
/// only its name in the closed control.
pub enum Flags {
    /// Allow clearing the selection so that no item is chosen.
    AllowNoneSelection = 0x01,
    /// Show each item's description next to its name.
    ShowDescription = 0x02,
}

pub trait DropDownListType {
    fn name(&self) -> &str;
    fn description(&self) -> &str {
        ""
    }
    fn symbol(&self) -> &str {
        ""
    }
}
