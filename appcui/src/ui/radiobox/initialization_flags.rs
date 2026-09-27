    #[derive(Copy, Clone, PartialEq, Eq)]
    /// Visual style of a [`super::RadioBox`] selection mark.
    ///
    /// Variants choose the glyphs used for the selected and unselected states, from a
    /// classic `(●)` circle to Unicode bullets, diamonds, or a target.
    pub enum Type {
        /// Parentheses with a bullet: `(●)` when selected, `( )` when not.
        Standard,
        /// Solid/empty circles: `⚫` when selected, `⚪` when not.
        Circle,
        /// Diamonds: `◆` when selected, `◇` when not.
        Diamond,
        /// ASCII parentheses: `(*)` when selected, `( )` when not.
        Ascii,
        /// Circled bullets: `⦿` when selected, `⦾` when not.
        Bullet,
        /// Target rings: `🞉` when selected, `🞅` when not.
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