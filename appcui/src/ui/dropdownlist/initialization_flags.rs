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

/// An item that can appear in a [`struct@super::DropDownList`].
///
/// Implement this for your item type. [`name`](Self::name) is required;
/// [`description`](Self::description) and [`symbol`](Self::symbol) are optional extras
/// shown when the corresponding flags are set.
pub trait DropDownListType {
    /// Primary caption shown for this item.
    fn name(&self) -> &str;
    /// Optional secondary text; defaults to an empty string.
    fn description(&self) -> &str {
        ""
    }
    /// Optional symbol drawn before the name; defaults to an empty string.
    fn symbol(&self) -> &str {
        ""
    }
}
