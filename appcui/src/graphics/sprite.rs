use super::{Character, Size, Surface};

/// A rectangular block of [`Character`]s that can be drawn onto a [`Surface`].
///
/// A sprite keeps its own characters and has no origin or clip area.
/// Paint it with [`Surface::draw_sprite`], or draw into it with [`Sprite::edit`].
/// The width and height are always clamped between `1` and `10000`.
pub struct Sprite {
    chars: Vec<Character>,
    size: Size
}
impl Sprite {
    /// Creates a sprite with the specified width and height.
    /// Every cell is filled with [`Character::default`] (a space with a white foreground and a black background).
    /// The width and height are clamped between `1` and `10000`.
    ///
    /// # Parameters
    /// - `width`: The number of columns.
    /// - `height`: The number of rows.
    ///
    /// # Example
    /// ```rust
    /// use appcui::graphics::Sprite;
    ///
    /// let sprite = Sprite::new(8, 4);
    /// assert_eq!(sprite.size().width, 8);
    /// assert_eq!(sprite.size().height, 4);
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        let w = width.clamp(1, super::surface::MAX_SURFACE_WIDTH);
        let h = height.clamp(1, super::surface::MAX_SURFACE_HEIGHT);
        let count = (w as usize) * (h as usize);
        Self { 
            chars: vec![Character::default(); count],
            size: Size::new(w, h)
        }
    }
    /// Returns the size of the sprite (width and height).
    ///
    /// # Example
    /// ```rust
    /// use appcui::graphics::Sprite;
    ///
    /// let sprite = Sprite::new(8, 4);
    /// let size = sprite.size();
    /// assert_eq!((size.width, size.height), (8, 4));
    /// ```
    #[inline(always)]
    pub fn size(&self) -> Size {
        self.size
    }
    /// Returns the character at `(x, y)`.
    ///
    /// Returns `None` when `x` or `y` is outside the sprite.
    ///
    /// # Parameters
    /// - `x`: The column, starting at 0.
    /// - `y`: The row, starting at 0.
    ///
    /// # Example
    /// ```rust
    /// use appcui::graphics::{Character, CharFlags, Color, Sprite};
    ///
    /// let mut sprite = Sprite::new(2, 2);
    /// let ch = Character::new('A', Color::Red, Color::Black, CharFlags::None);
    /// sprite.set(0, 0, ch);
    /// assert_eq!(sprite.get(0, 0), Some(ch));
    /// assert!(sprite.get(2, 0).is_none());
    /// ```
    #[inline(always)]
    pub fn get(&self, x: u32, y: u32) -> Option<Character> {
        if x >= self.size.width || y >= self.size.height {
            return None;
        }
        Some(self.chars[y as usize * self.size.width as usize + x as usize])
    }
    /// Writes a character at `(x, y)`.
    ///
    /// Coordinates outside the sprite are ignored.
    ///
    /// # Parameters
    /// - `x`: The column, starting at 0.
    /// - `y`: The row, starting at 0.
    /// - `ch`: The character to store.
    ///
    /// # Example
    /// ```rust
    /// use appcui::graphics::{Character, CharFlags, Color, Sprite};
    ///
    /// let mut sprite = Sprite::new(2, 2);
    /// sprite.set(1, 0, Character::new('B', Color::Yellow, Color::Black, CharFlags::None));
    /// assert_eq!(sprite.get(1, 0).unwrap().code, 'B');
    /// ```
    #[inline(always)]
    pub fn set(&mut self, x: u32, y: u32, ch: Character) {
        if x >= self.size.width || y >= self.size.height {
            return;
        }   
        self.chars[y as usize * self.size.width as usize + x as usize] = ch;
    }
    /// Fills every cell with the specified character.
    ///
    /// # Parameters
    /// - `ch`: The character written into every cell.
    ///
    /// # Example
    /// ```rust
    /// use appcui::graphics::{Character, CharFlags, Color, Sprite};
    ///
    /// let mut sprite = Sprite::new(4, 2);
    /// let fill = Character::new('.', Color::Gray, Color::Black, CharFlags::None);
    /// sprite.clear(fill);
    /// assert_eq!(sprite.get(3, 1), Some(fill));
    /// ```
    #[inline(always)]
    pub fn clear(&mut self, ch: Character) {
        self.chars.fill(ch);
    }
    /// Changes the sprite size and fills every cell with `ch`.
    ///
    /// Cells that were already present are replaced as well.
    /// The width and height are clamped between `1` and `10000`.
    ///
    /// # Parameters
    /// - `width`: The new number of columns.
    /// - `height`: The new number of rows.
    /// - `ch`: The character written into every cell of the new size.
    ///
    /// # Example
    /// ```rust
    /// use appcui::graphics::{Character, CharFlags, Color, Sprite};
    ///
    /// let mut sprite = Sprite::new(4, 2);
    /// sprite.set(0, 0, Character::new('A', Color::White, Color::Black, CharFlags::None));
    /// let fill = Character::new('#', Color::Aqua, Color::Black, CharFlags::None);
    /// sprite.resize(2, 1, fill);
    /// assert_eq!(sprite.size().width, 2);
    /// assert_eq!(sprite.size().height, 1);
    /// assert_eq!(sprite.get(0, 0), Some(fill));
    /// ```
    #[inline(always)]
    pub fn resize(&mut self, width: u32, height: u32, ch: Character) {
        let w = width.clamp(1, super::surface::MAX_SURFACE_WIDTH);
        let h = height.clamp(1, super::surface::MAX_SURFACE_HEIGHT);
        let count = (w as usize) * (h as usize);
        self.chars.resize(count, ch);
        self.chars.fill(ch);
        self.size = Size::new(w, h);
    }
    /// Draws into this sprite through a temporary [`Surface`].
    ///
    /// The callback receives a surface with this sprite's size and characters.
    /// Drawing methods such as [`Surface::write_char`] and [`Surface::write_string`]
    /// update the sprite. When the callback returns, the sprite adopts the surface size.
    ///
    /// # Parameters
    /// - `f`: A function that receives the temporary surface.
    ///
    /// # Example
    /// ```rust
    /// use appcui::graphics::{Character, CharFlags, Color, Sprite};
    ///
    /// let mut sprite = Sprite::new(5, 3);
    /// sprite.edit(|surface| {
    ///     surface.write_char(1, 1, Character::new('X', Color::Yellow, Color::Black, CharFlags::None));
    /// });
    /// assert_eq!(sprite.get(1, 1).unwrap().code, 'X');
    /// ```
    #[inline(always)]
    pub fn edit(&mut self, f: impl Fn(&mut Surface)) {
        let chars = std::mem::take(&mut self.chars);
        let mut surface = Surface::from_parts(chars, self.size);
        f(&mut surface);
        self.size = surface.size();
        std::mem::swap(&mut self.chars, &mut surface.chars);
    }
    #[inline(always)]
    pub(super) fn chars(&self) -> &[Character] {
        &self.chars
    }
}
