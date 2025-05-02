#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Type {
    Standard,
    Circle,
    Diamond,
    Square,
    Star,
    Dot,
}

impl Type {
    pub(super) fn selected_symbol(&self) -> &str {
        match self {
            Type::Standard => "●",
            Type::Circle => "◉",
            Type::Diamond => "◆",
            Type::Square => "■",
            Type::Star => "★",
            Type::Dot => "•",
        }
    }

    pub(super) fn unselected_symbol(&self) -> &str {
        match self {
            Type::Standard => "○",
            Type::Circle => "○",
            Type::Diamond => "◇",
            Type::Square => "□",
            Type::Star => "☆",
            Type::Dot => "○",
        }
    }
} 