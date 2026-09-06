use super::CharAttribute;
use super::Direction;
use crate::ui::selector::EnumSelector;
use appcui_proc_macro::EnumSelector;

pub(super) struct LineTypeChars {
    pub(super) corner_top_left: char,
    pub(super) horizontal_on_top: char,
    pub(super) corner_top_right: char,
    pub(super) vertical_on_right: char,
    pub(super) corner_bottom_right: char,
    pub(super) horizontal_on_bottom: char,
    pub(super) corner_bottom_left: char,
    pub(super) vertical_on_left: char,
    pub(super) horizontal: char,
    pub(super) vertical: char,
}

static LINE_TYPE_CHARS: [LineTypeChars; 8] = [
    /* Single Lines */
    LineTypeChars {
        corner_top_left: '\u{250C}',
        horizontal_on_top: '\u{2500}',
        corner_top_right: '\u{2510}',
        vertical_on_right: '\u{2502}',
        corner_bottom_right: '\u{2518}',
        horizontal_on_bottom: '\u{2500}',
        corner_bottom_left: '\u{2514}',
        vertical_on_left: '\u{2502}',
        horizontal: '\u{2500}',
        vertical: '\u{2502}',
    },
    /* Double Lines */
    LineTypeChars {
        corner_top_left: '\u{2554}',
        horizontal_on_top: '\u{2550}',
        corner_top_right: '\u{2557}',
        vertical_on_right: '\u{2551}',
        corner_bottom_right: '\u{255D}',
        horizontal_on_bottom: '\u{2550}',
        corner_bottom_left: '\u{255A}',
        vertical_on_left: '\u{2551}',
        horizontal: '\u{2550}',
        vertical: '\u{2551}',
    },
    /* Single Thick lines */
    LineTypeChars {
        corner_top_left: '\u{250F}',
        horizontal_on_top: '\u{2501}',
        corner_top_right: '\u{2513}',
        vertical_on_right: '\u{2503}',
        corner_bottom_right: '\u{251B}',
        horizontal_on_bottom: '\u{2501}',
        corner_bottom_left: '\u{2517}',
        vertical_on_left: '\u{2503}',
        horizontal: '\u{2501}',
        vertical: '\u{2503}',
    },
    /* Border */
    LineTypeChars {
        corner_top_left: '\u{2584}',
        horizontal_on_top: '\u{2584}',
        corner_top_right: '\u{2584}',
        vertical_on_right: '\u{2588}',
        corner_bottom_right: '\u{2580}',
        horizontal_on_bottom: '\u{2580}',
        corner_bottom_left: '\u{2580}',
        vertical_on_left: '\u{2588}',
        horizontal: '\u{2588}',
        vertical: '\u{2588}',
    },
    /* Ascii */
    LineTypeChars {
        corner_top_left: '+',
        horizontal_on_top: '-',
        corner_top_right: '+',
        vertical_on_right: '|',
        corner_bottom_right: '+',
        horizontal_on_bottom: '-',
        corner_bottom_left: '+',
        vertical_on_left: '|',
        horizontal: '-',
        vertical: '|',
    },
    /* Ascii Round*/
    LineTypeChars {
        corner_top_left: '/',
        horizontal_on_top: '-',
        corner_top_right: '\\',
        vertical_on_right: '|',
        corner_bottom_right: '/',
        horizontal_on_bottom: '-',
        corner_bottom_left: '\\',
        vertical_on_left: '|',
        horizontal: '-',
        vertical: '|',
    },
    /* SingleRound */
    LineTypeChars {
        corner_top_left: '\u{256D}',
        horizontal_on_top: '\u{2500}',
        corner_top_right: '\u{256E}',
        vertical_on_right: '\u{2502}',
        corner_bottom_right: '\u{256F}',
        horizontal_on_bottom: '\u{2500}',
        corner_bottom_left: '\u{2570}',
        vertical_on_left: '\u{2502}',
        horizontal: '\u{2500}',
        vertical: '\u{2502}',
    },
    /* Braille double line */
    LineTypeChars {
        corner_top_left: '\u{28F6}',
        corner_top_right: '\u{28F6}',
        corner_bottom_right: '\u{283F}',
        corner_bottom_left: '\u{283F}',

        vertical_on_right: '\u{28FF}',
        vertical_on_left: '\u{28FF}',
        vertical: '\u{28FF}',
        horizontal_on_bottom: '\u{2836}',
        horizontal_on_top: '\u{2836}',
        horizontal: '\u{2836}',
    },
];

/// The visual style used when drawing lines, rectangles, and polylines (single, double, thick, ASCII, rounded, or Braille).
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, EnumSelector)]
pub enum LineType {
    #[VariantInfo(name = "Single Lines", description = "Single lines with corners and vertical/horizontal lines")]
    Single,
    #[VariantInfo(name = "Double Lines", description = "Double lines with corners and vertical/horizontal lines")]
    Double,
    #[VariantInfo(
        name = "Single Thick Lines",
        description = "Single thick lines with corners and vertical/horizontal lines"
    )]
    SingleThick,
    #[VariantInfo(name = "Border", description = "A border style with thick lines")]
    Border,
    #[VariantInfo(name = "Ascii", description = "ASCII characters for lines")]
    Ascii,
    #[VariantInfo(name = "Ascii Round", description = "ASCII characters with rounded corners")]
    AsciiRound,
    #[VariantInfo(name = "Single Round", description = "Single lines with rounded corners")]
    SingleRound,
    #[VariantInfo(name = "Braille", description = "Double line drawn with braille characters")]
    Braille,
}

impl LineType {
    pub(super) fn charset(&self) -> &'static LineTypeChars {
        &LINE_TYPE_CHARS[(*self as u8) as usize]
    }
}

#[derive(Copy, Clone)]
pub(super) struct LineCapChars {
    pub(super) up: char,
    pub(super) down: char,
    pub(super) left: char,
    pub(super) right: char,
}

static LINE_CAP_CHARS_ARROWS: LineCapChars = LineCapChars {
    up: '\u{2191}',
    down: '\u{2193}',
    left: '\u{2190}',
    right: '\u{2192}',
};
static LINE_CAP_CHARS_TRIANGLES: LineCapChars = LineCapChars {
    up: '\u{25B2}',
    down: '\u{25BC}',
    left: '\u{25C0}',
    right: '\u{25B6}',
};

/// The glyph drawn at the start or end of a polyline.
///
/// Caps are applied after the segments are drawn. For
/// [`Arrow`](Self::Arrow) and [`Triangle`](Self::Triangle) the glyph is chosen
/// from the direction of the first or last segment. [`Auto`](Self::Auto) does
/// not pick a directional glyph; it merges the endpoint with existing
/// box-drawing characters (for example when a connector meets a rectangle).
///
/// Caps are ignored when the polyline is closed (the first and last points
/// are the same).
#[derive(Copy, Clone)]
pub enum LineCap {
    /// Merge the endpoint with neighboring box-drawing characters.
    ///
    /// This is intended for connectors that attach to an existing rectangle
    /// or line. The endpoint is rewritten as a T-junction or similar glyph
    /// based on the characters already on the surface.
    Auto,
    /// A directional arrow (`↑`, `↓`, `←`, `→`) inferred from the terminal segment.
    Arrow,
    /// A directional triangle (`▲`, `▼`, `◀`, `▶`) inferred from the terminal segment.
    Triangle,
    /// The same custom character at the endpoint, regardless of direction.
    Char(char),
}
impl LineCap {
    pub(super) fn char(&self, direction: Direction) -> char {
        match self {
            LineCap::Auto => 0 as char,
            LineCap::Arrow => {
                match direction {
                    Direction::Up => LINE_CAP_CHARS_ARROWS.up,
                    Direction::Down => LINE_CAP_CHARS_ARROWS.down,
                    Direction::Left => LINE_CAP_CHARS_ARROWS.left,
                    Direction::Right => LINE_CAP_CHARS_ARROWS.right,
                }
            }
            LineCap::Triangle => {
                match direction {
                    Direction::Up => LINE_CAP_CHARS_TRIANGLES.up,
                    Direction::Down => LINE_CAP_CHARS_TRIANGLES.down,
                    Direction::Left => LINE_CAP_CHARS_TRIANGLES.left,
                    Direction::Right => LINE_CAP_CHARS_TRIANGLES.right,
                }
            }
            LineCap::Char(c) => *c,
        }
    }
}

/// How a polyline should be rendered: line style, colors, caps, and joints.
///
/// Create a format with [`PolyLineFormatBuilder`] and pass it to
/// [`Surface::draw_polyline`](crate::graphics::Surface::draw_polyline).
///
/// By default the format has no start cap, no end cap, and no custom joint
/// character. Orthogonal corners are then chosen automatically from the
/// incoming and outgoing segment directions. Cap and joint attributes that
/// are left unset inherit the line [`CharAttribute`].
///
/// # Example
/// ```rust
/// use appcui::prelude::*;
///
/// let format = PolyLineFormatBuilder::new(LineType::Single, charattr!("white,black"))
///     .start_cap(LineCap::Arrow)
///     .end_cap(LineCap::Triangle)
///     .build();
/// ```
#[derive(Copy, Clone)]
pub struct PolyLineFormat {
    pub(crate) line_type: LineType,
    pub(crate) attr: CharAttribute,

    pub(crate) start_cap: Option<LineCap>,
    pub(crate) start_attr: Option<CharAttribute>, // None => inherit `attr`
    pub(crate) end_cap: Option<LineCap>,
    pub(crate) end_attr: Option<CharAttribute>, // None => inherit `attr`

    pub(crate) joint: Option<char>,               // None => auto-resolve corner glyph from directions
    pub(crate) joint_attr: Option<CharAttribute>, // None => inherit `attr`
}
impl PolyLineFormat {
    #[inline(always)]
    pub fn attr(&self) -> CharAttribute {
        self.attr
    }
    #[inline(always)]
    pub fn set_attr(&mut self, attr: CharAttribute) {
        self.attr = attr;
    }
    #[inline(always)]
    pub fn line_type(&self) -> LineType {
        self.line_type
    }
    #[inline(always)]
    pub fn set_line_type(&mut self, line_type: LineType) {
        self.line_type = line_type;
    }
    #[inline(always)]
    pub fn start_cap(&self) -> Option<LineCap> {
        self.start_cap
    }
    #[inline(always)]
    pub fn set_start_cap(&mut self, start_cap: Option<LineCap>) {
        self.start_cap = start_cap;
    }
    #[inline(always)]
    pub fn start_attr(&self) -> Option<CharAttribute> {
        self.start_attr
    }
    #[inline(always)]
    pub fn set_start_attr(&mut self, start_attr: Option<CharAttribute>) {
        self.start_attr = start_attr;
    }
    #[inline(always)]
    pub fn end_cap(&self) -> Option<LineCap> {
        self.end_cap
    }
    #[inline(always)]
    pub fn set_end_cap(&mut self, end_cap: Option<LineCap>) {
        self.end_cap = end_cap;
    }
    #[inline(always)]
    pub fn end_attr(&self) -> Option<CharAttribute> {
        self.end_attr
    }
    #[inline(always)]
    pub fn set_end_attr(&mut self, end_attr: Option<CharAttribute>) {
        self.end_attr = end_attr;
    }
    #[inline(always)]
    pub fn joint(&self) -> Option<char> {
        self.joint
    }
    #[inline(always)]
    pub fn set_joint(&mut self, joint: Option<char>) {
        self.joint = joint;
    }
    #[inline(always)]
    pub fn joint_attr(&self) -> Option<CharAttribute> {
        self.joint_attr
    }
    #[inline(always)]
    pub fn set_joint_attr(&mut self, joint_attr: Option<CharAttribute>) {
        self.joint_attr = joint_attr;
    }
}

/// A builder for [`PolyLineFormat`].
///
/// Start with [`PolyLineFormatBuilder::new`], optionally set caps, colors, and
/// a custom joint, then call [`build`](Self::build).
///
/// # Example
/// ```rust
/// use appcui::prelude::*;
///
/// let format = PolyLineFormatBuilder::new(LineType::Double, charattr!("aqua,black"))
///     .start_cap(LineCap::Auto)
///     .end_cap(LineCap::Auto)
///     .joint_attr(charattr!("yellow,black"))
///     .build();
/// ```
pub struct PolyLineFormatBuilder {
    format: PolyLineFormat,
}

impl PolyLineFormatBuilder {
    /// Creates a builder with the given line style and attributes.
    ///
    /// Caps and a custom joint are unset. Orthogonal corners will be
    /// resolved automatically when the polyline is drawn.
    pub fn new(line_type: LineType, attr: CharAttribute) -> Self {
        Self {
            format: PolyLineFormat {
                line_type,
                attr,
                start_cap: None,
                start_attr: None,
                end_cap: None,
                end_attr: None,
                joint: None,
                joint_attr: None,
            },
        }
    }
}
impl PolyLineFormatBuilder {
    /// Sets the line style used for every segment.
    pub fn line_type(mut self, line_type: LineType) -> Self {
        self.format.line_type = line_type;
        self
    }
    /// Sets the character attributes used for the line segments.
    ///
    /// Cap and joint attributes inherit this value unless they are set
    /// separately.
    pub fn attr(mut self, attr: CharAttribute) -> Self {
        self.format.attr = attr;
        self
    }
    /// Sets the glyph drawn at the first point of the polyline.
    ///
    /// Ignored when the polyline is closed (first and last points are equal).
    pub fn start_cap(mut self, start_cap: LineCap) -> Self {
        self.format.start_cap = Some(start_cap);
        self
    }
    /// Sets the attributes of the start cap. If omitted, the line attributes
    /// are used.
    pub fn start_attr(mut self, start_attr: CharAttribute) -> Self {
        self.format.start_attr = Some(start_attr);
        self
    }
    /// Sets the glyph drawn at the last point of the polyline.
    ///
    /// Ignored when the polyline is closed (first and last points are equal).
    pub fn end_cap(mut self, end_cap: LineCap) -> Self {
        self.format.end_cap = Some(end_cap);
        self
    }
    /// Sets the attributes of the end cap. If omitted, the line attributes
    /// are used.
    pub fn end_attr(mut self, end_attr: CharAttribute) -> Self {
        self.format.end_attr = Some(end_attr);
        self
    }
    /// Uses a custom character at every inner vertex instead of an
    /// automatically chosen corner glyph.
    pub fn joint(mut self, joint: char) -> Self {
        self.format.joint = Some(joint);
        self
    }
    /// Sets the attributes of the joints (automatic corners or a custom
    /// joint character). If omitted, the line attributes are used.
    pub fn joint_attr(mut self, joint_attr: CharAttribute) -> Self {
        self.format.joint_attr = Some(joint_attr);
        self
    }
    /// Builds the [`PolyLineFormat`] value.
    pub fn build(self) -> PolyLineFormat {
        self.format
    }
}
