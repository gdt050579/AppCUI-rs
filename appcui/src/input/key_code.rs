/// Physical or logical key identity, without modifiers.
///
/// Pair with [`KeyModifier`](super::KeyModifier) in a [`Key`](super::Key) to represent shortcuts such as `Ctrl+S`.
/// [`None`](Self::None) means no key.
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub enum KeyCode {
    /// No key.
    None = 0,
    /// Function key `F1`.
    F1 = 1,
    /// Function key `F2`.
    F2,
    /// Function key `F3`.
    F3,
    /// Function key `F4`.
    F4,
    /// Function key `F5`.
    F5,
    /// Function key `F6`.
    F6,
    /// Function key `F7`.
    F7,
    /// Function key `F8`.
    F8,
    /// Function key `F9`.
    F9,
    /// Function key `F10`.
    F10,
    /// Function key `F11`.
    F11,
    /// Function key `F12`.
    F12,
    /// Enter / Return.
    Enter,
    /// Escape.
    Escape,
    /// Insert.
    Insert,
    /// Delete.
    Delete,
    /// Backspace.
    Backspace,
    /// Tab.
    Tab,
    /// Left arrow (`←`).
    Left,
    /// Up arrow (`↑`).
    Up,
    /// Down arrow (`↓`).
    Down,
    /// Right arrow (`→`).
    Right,
    /// Page Up.
    PageUp,
    /// Page Down.
    PageDown,
    /// Home.
    Home,
    /// End.
    End,
    /// Space bar.
    Space,
    /// The `A` key.
    A,
    /// The `B` key.
    B,
    /// The `C` key.
    C,
    /// The `D` key.
    D,
    /// The `E` key.
    E,
    /// The `F` key.
    F,
    /// The `G` key.
    G,
    /// The `H` key.
    H,
    /// The `I` key.
    I,
    /// The `J` key.
    J,
    /// The `K` key.
    K,
    /// The `L` key.
    L,
    /// The `M` key.
    M,
    /// The `N` key.
    N,
    /// The `O` key.
    O,
    /// The `P` key.
    P,
    /// The `Q` key.
    Q,
    /// The `R` key.
    R,
    /// The `S` key.
    S,
    /// The `T` key.
    T,
    /// The `U` key.
    U,
    /// The `V` key.
    V,
    /// The `W` key.
    W,
    /// The `X` key.
    X,
    /// The `Y` key.
    Y,
    /// The `Z` key.
    Z,
    /// Digit `0` (top-row number key).
    N0,
    /// Digit `1` (top-row number key).
    N1,
    /// Digit `2` (top-row number key).
    N2,
    /// Digit `3` (top-row number key).
    N3,
    /// Digit `4` (top-row number key).
    N4,
    /// Digit `5` (top-row number key).
    N5,
    /// Digit `6` (top-row number key).
    N6,
    /// Digit `7` (top-row number key).
    N7,
    /// Digit `8` (top-row number key).
    N8,
    /// Digit `9` (top-row number key).
    N9,
}

const KEY_NAME: [&str; 64] = [
    "",
    "F1",
    "F2",
    "F3",
    "F4",
    "F5",
    "F6",
    "F7",
    "F8",
    "F9",
    "F10",
    "F11",
    "F12",
    "Enter",
    "Escape",
    "Insert",
    "Delete",
    "Backspace",
    "Tab",
    "Left",
    "Up",
    "Down",
    "Right",
    "PageUp",
    "PageDown",
    "Home",
    "End",
    "Space",
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
];


const KEY_NAME_PADDED: [&str; 64] = [
    "",
    " F1 ",
    " F2 ",
    " F3 ",
    " F4 ",
    " F5 ",
    " F6 ",
    " F7 ",
    " F8 ",
    " F9 ",
    " F10 ",
    " F11 ",
    " F12 ",
    " Enter ",
    " Escape ",
    " Insert ",
    " Delete ",
    " Backspace ",
    " Tab ",
    " Left ",
    " Up ",
    " Down ",
    " Right ",
    " PageUp ",
    " PageDown ",
    " Home ",
    " End ",
    " Space ",
    " A ",
    " B ",
    " C ",
    " D ",
    " E ",
    " F ",
    " G ",
    " H ",
    " I ",
    " J ",
    " K ",
    " L ",
    " M ",
    " N ",
    " O ",
    " P ",
    " Q ",
    " R ",
    " S ",
    " T ",
    " U ",
    " V ",
    " W ",
    " X ",
    " Y ",
    " Z ",
    " 0 ",
    " 1 ",
    " 2 ",
    " 3 ",
    " 4 ",
    " 5 ",
    " 6 ",
    " 7 ",
    " 8 ",
    " 9 ",
];

impl KeyCode {
    /// Returns the name of the key.
    /// 
    /// # Returns
    /// The name of the key.
    /// 
    /// # Example
    /// ```rust 
    /// use appcui::prelude::*;
    /// 
    /// let key = KeyCode::A;
    /// let name = key.name();
    /// ```
    pub fn name(&self) -> &'static str {
        let index = ((*self) as u8) as usize;
        if index<64 {
            return KEY_NAME[index];
        }
        ""
    }
    
    /// Returns the name of the key with padding (spaces added to the left and right).  
    /// 
    /// # Returns
    /// The name of the key with padding.
    /// 
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    /// 
    /// let key = KeyCode::A;
    /// let name = key.name_padded();
    /// ```
    pub fn name_padded(&self) -> &'static str {
        let index = ((*self) as u8) as usize;
        if index<64 {
            return KEY_NAME_PADDED[index];
        }
        ""
    }
}

impl From<u8> for KeyCode {
    /// Creates a new key code from a u8.
    /// 
    /// # Arguments
    /// * `value` - The u8 to create the key code from.
    /// 
    /// # Returns
    /// A new key code created from the u8. If the u8 is not a valid key code, the function will return `KeyCode::None`.
    /// 
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    /// 
    /// let key = KeyCode::from(0x05);
    /// ``` 
    fn from(value: u8) -> Self {
        match value {
            0 => KeyCode::None,
            1 => KeyCode::F1,
            2 => KeyCode::F2,
            3 => KeyCode::F3,
            4 => KeyCode::F4,
            5 => KeyCode::F5,
            6 => KeyCode::F6,
            7 => KeyCode::F7,
            8 => KeyCode::F8,
            9 => KeyCode::F9,
            10 => KeyCode::F10,
            11 => KeyCode::F11,
            12 => KeyCode::F12,
            13 => KeyCode::Enter,
            14 => KeyCode::Escape,
            15 => KeyCode::Insert,
            16 => KeyCode::Delete,
            17 => KeyCode::Backspace,
            18 => KeyCode::Tab,
            19 => KeyCode::Left,
            20 => KeyCode::Up,
            21 => KeyCode::Down,
            22 => KeyCode::Right,
            23 => KeyCode::PageUp,
            24 => KeyCode::PageDown,
            25 => KeyCode::Home,
            26 => KeyCode::End,
            27 => KeyCode::Space,
            28 => KeyCode::A,
            29 => KeyCode::B,
            30 => KeyCode::C,
            31 => KeyCode::D,
            32 => KeyCode::E,
            33 => KeyCode::F,
            34 => KeyCode::G,
            35 => KeyCode::H,
            36 => KeyCode::I,
            37 => KeyCode::J,
            38 => KeyCode::K,
            39 => KeyCode::L,
            40 => KeyCode::M,
            41 => KeyCode::N,
            42 => KeyCode::O,
            43 => KeyCode::P,
            44 => KeyCode::Q,
            45 => KeyCode::R,
            46 => KeyCode::S,
            47 => KeyCode::T,
            48 => KeyCode::U,
            49 => KeyCode::V,
            50 => KeyCode::W,
            51 => KeyCode::X,
            52 => KeyCode::Y,
            53 => KeyCode::Z,
            54 => KeyCode::N0,
            55 => KeyCode::N1,
            56 => KeyCode::N2,
            57 => KeyCode::N3,
            58 => KeyCode::N4,
            59 => KeyCode::N5,
            60 => KeyCode::N6,
            61 => KeyCode::N7,
            62 => KeyCode::N8,
            63 => KeyCode::N9,
            _ => KeyCode::None,
        }
    }
}
