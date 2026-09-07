    #[derive(Copy, Clone, PartialEq, Eq)]
    /// Visual style of a [`super::RadioBox`] selection mark.
    ///
    /// Variants choose the glyphs used for the selected and unselected states, from a
    /// classic `(●)` circle to Unicode bullets, diamonds, or a target.
    pub enum Type {
        Standard,
        Circle,
        Diamond,
        Ascii,
        Bullet,
        Target,
    }
    impl Type {
        pub(super) fn selected_symbol(&self) -> &str {
            match self {
                Type::Standard => "(●)",
                Type::Ascii => "(*)",
                Type::Circle => "⚫ ",
                Type::Diamond => "◆",
                Type::Bullet => "⦿",
                Type::Target => "🞉",
            }
        }

        pub(super) fn unselected_symbol(&self) -> &str {
            match self {
                Type::Standard => "( )",
                Type::Ascii => "( )",
                Type::Circle => "⚪ ",
                Type::Diamond => "◇",
                Type::Bullet => "⦾",
                Type::Target => "🞅",
            }
        }
    } 