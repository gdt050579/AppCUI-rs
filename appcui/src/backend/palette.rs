//! Palettes for the 16 named terminal colors.
//!
//! [`Palette`] chooses which RGB table a backend uses for named [`Color`](crate::graphics::Color)
//! values. [`ColorPalette`] is that table: one red, green, and blue triple for each color from
//! [`Black`](crate::graphics::Color::Black) through [`White`](crate::graphics::Color::White).
//! [`Transparent`](crate::graphics::Color::Transparent) is not included.
//!
//! Pass a palette to any application builder. If you never do, the application uses
//! [`Palette::Terminal`], which keeps the backend's default colors.
//! `palette_as_rgb(true)` cannot be combined with [`Palette::Terminal`]: [`run`](crate::system::MultiWindowAppBuilder::run)
//! returns [`ErrorKind::InvalidParameter`](crate::system::ErrorKind::InvalidParameter).
//! Use [`Palette::BuiltIn`] or [`Palette::Custom`] when RGB output is enabled.
//!
//! | Variant | What the backend uses |
//! |---------|------------------------|
//! | [`Palette::Terminal`] | The backend's default colors. The terminal palette is left unchanged. |
//! | [`Palette::BuiltIn`] | The standard AppCUI colors from [`ColorPalette::built_in`]. Silver is `(192, 192, 192)`. |
//! | [`Palette::Custom`] | RGB values you supply through [`ColorPalette::from_pairs`]. |
//!
//! Windows Console writes `BuiltIn` and `Custom` into the console color table. Windows VT uses
//! the same values for the colors it paints. Other backends may ignore the palette.
//!
//! # Examples
//!
//! Standard AppCUI colors:
//!
//! ```rust, no_run
//! use appcui::prelude::*;
//!
//! App::new().palette(appcui::backend::Palette::BuiltIn);
//! ```
//!
//! A custom table. Colors you leave out stay black `(0, 0, 0)`:
//!
//! ```rust, no_run
//! use appcui::backend::{ColorPalette, Palette};
//! use appcui::prelude::*;
//!
//! let palette = ColorPalette::from_pairs(&[
//!     (Color::Red, 255, 85, 85),
//!     (Color::Blue, 100, 140, 255),
//! ]);
//! App::new().palette(Palette::Custom(palette));
//! ```

use crate::graphics::Color;

/// The 16 named colors a backend uses for [`Color`](crate::graphics::Color).
///
/// Pass one to [`MultiWindowAppBuilder::palette`](crate::system::MultiWindowAppBuilder::palette)
/// (the same method is available on every application builder). If you never call it, the
/// application uses [`Palette::Terminal`].
///
/// [`Palette::Terminal`] cannot be used with `palette_as_rgb(true)`. That combination makes
/// `run` return [`ErrorKind::InvalidParameter`](crate::system::ErrorKind::InvalidParameter).
/// [`Palette::BuiltIn`] and [`Palette::Custom`] are valid with RGB mode.
///
/// A palette holds one RGB triple for each named color from
/// [`Black`](crate::graphics::Color::Black) through [`White`](crate::graphics::Color::White).
/// [`Transparent`](crate::graphics::Color::Transparent) has no slot.
///
/// # Examples
///
/// ```rust, no_run
/// use appcui::prelude::*;
///
/// App::new().palette(appcui::backend::Palette::BuiltIn);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Palette {
    /// Keep the backend's default colors and leave the terminal palette unchanged.
    ///
    /// This is the default. It cannot be combined with `palette_as_rgb(true)`.
    Terminal,

    /// The standard AppCUI colors returned by [`ColorPalette::built_in`].
    ///
    /// Silver is `(192, 192, 192)`.
    BuiltIn,

    /// RGB values supplied by the application.
    ///
    /// Build the table with [`ColorPalette::from_pairs`].
    Custom(ColorPalette),
}

/// One RGB triple for each named color from [`Black`](crate::graphics::Color::Black)
/// through [`White`](crate::graphics::Color::White).
///
/// Index `0` is black and index `15` is white, matching [`Color::from_value`](crate::graphics::Color::from_value).
#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    pub(crate) colors: [(u8, u8, u8); 16],
}

impl ColorPalette {
    /// The standard AppCUI palette.
    ///
    /// | Color | RGB |
    /// |-------|-----|
    /// | Black | 0, 0, 0 |
    /// | DarkBlue | 0, 0, 128 |
    /// | DarkGreen | 0, 128, 0 |
    /// | Teal | 0, 128, 128 |
    /// | DarkRed | 128, 0, 0 |
    /// | Magenta | 128, 0, 128 |
    /// | Olive | 128, 128, 0 |
    /// | Silver | 192, 192, 192 |
    /// | Gray | 128, 128, 128 |
    /// | Blue | 0, 0, 255 |
    /// | Green | 0, 255, 0 |
    /// | Aqua | 0, 255, 255 |
    /// | Red | 255, 0, 0 |
    /// | Pink | 255, 0, 255 |
    /// | Yellow | 255, 255, 0 |
    /// | White | 255, 255, 255 |
    ///
    /// # Example
    ///
    /// ```rust
    /// use appcui::backend::ColorPalette;
    ///
    /// let _palette = ColorPalette::built_in();
    /// ```
    pub const fn built_in() -> Self {
        Self {
            colors: [
                (0, 0, 0),
                (0, 0, 128),
                (0, 128, 0),
                (0, 128, 128),
                (128, 0, 0),
                (128, 0, 128),
                (128, 128, 0),
                (192, 192, 192),
                (128, 128, 128),
                (0, 0, 255),
                (0, 255, 0),
                (0, 255, 255),
                (255, 0, 0),
                (255, 0, 255),
                (255, 255, 0),
                (255, 255, 255),
            ],
        }
    }

    /// Builds a palette from `(color, red, green, blue)` entries.
    ///
    /// Every slot starts at black `(0, 0, 0)`. Each pair replaces the slot of that named
    /// color, and a later pair for the same color replaces the earlier one. Colors you
    /// leave out stay black.
    ///
    /// `color` must be one of the 16 named colors (`Black` through `White`).
    /// [`Transparent`](crate::graphics::Color::Transparent) has no slot.
    /// With the `TRUE_COLORS` feature, a [`Color::RGB`](crate::graphics::Color) value is
    /// folded into one of those 16 slots and is not stored as its own entry.
    ///
    /// # Example
    ///
    /// ```rust
    /// use appcui::backend::{ColorPalette, Palette};
    /// use appcui::prelude::*;
    ///
    /// let palette = ColorPalette::from_pairs(&[
    ///     (Color::Red, 255, 85, 85),
    ///     (Color::Blue, 100, 140, 255),
    /// ]);
    /// let _custom = Palette::Custom(palette);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `color` is [`Transparent`](crate::graphics::Color::Transparent).
    pub fn from_pairs(pairs: &[(Color, u8, u8, u8)]) -> Self {
        let mut colors = [(0, 0, 0); 16];
        for (color, r, g, b) in pairs {
            let index = color.as_color_index();
            assert!(index < 16, "You can only use the first 16 colors in a palette (Black, DarkBlue, DarkGreen, Teal, DarkRed, Magenta, Olive, Silver, Gray, Blue, Green, Aqua, Red, Pink, Yellow, White) !");
            colors[index as usize] = (*r, *g, *b);
        }
        Self { colors }
    }
}
