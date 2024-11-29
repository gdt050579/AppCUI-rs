use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ReadOnly = 0x01,   
    OnlyFolders = 0x02,
    ShowSuggestionWithFullPath = 0x04
}