#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Type {
    Standard,
    Circle,
    Diamond,
    Ascii,
    Bullet,
}
// ⚪⚫ 
// ◉  ○ ⦾⦿ⵔ⭘ ◯○  ⮿
impl Type {
    pub(super) fn selected_symbol(&self) -> &str {
        match self {
            Type::Standard => "(●)",
            Type::Ascii => "(*)",
            Type::Circle => "⚫ ",
            Type::Diamond => "◆",
            Type::Bullet => "⦿",
        }
    }

    pub(super) fn unselected_symbol(&self) -> &str {
        match self {
            Type::Standard => "( )",
            Type::Ascii => "( )",
            Type::Circle => "⚪ ",
            Type::Diamond => "◇",
            Type::Bullet => "⦾",
        }
    }
} 