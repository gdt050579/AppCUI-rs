use crate::{graphics::*, ui::common::ControlCharAttributesState};

/// Built-in color palettes for a [`Theme`].
pub enum Themes {
    /// The default AppCUI palette.
    Default,
    /// A dark-gray palette.
    DarkGray,
    /// A light palette.
    Light,
}

/// Colors used to fill the desktop background.
#[derive(Default)]
pub struct DesktopTheme {
    /// Character and colors painted across the desktop.
    pub character: Character,
}
/// Text colors for labels, captions, and similar content.
#[derive(Default)]
pub struct TextTheme {
    /// Normal (enabled) text.
    pub normal: CharAttribute,
    /// Hotkey / accelerator character in a caption.
    pub hot_key: CharAttribute,
    /// Disabled / inactive text.
    pub inactive: CharAttribute,
    /// Error or failure text.
    pub error: CharAttribute,
    /// Warning text.
    pub warning: CharAttribute,
    /// Text under the mouse pointer.
    pub hovered: CharAttribute,
    /// Text of the focused control.
    pub focused: CharAttribute,
    /// Highlighted / selected text.
    pub highlighted: CharAttribute,
    /// First emphasis level.
    pub enphasized_1: CharAttribute,
    /// Second emphasis level.
    pub enphasized_2: CharAttribute,
    /// Third emphasis level.
    pub enphasized_3: CharAttribute,
}
/// Colors for tooltip balloons.
#[derive(Default)]
pub struct ToolTipTheme {
    /// Tooltip body text.
    pub text: CharAttribute,
    /// Tooltip arrow / pointer.
    pub arrow: CharAttribute,
}
/// Colors for check marks, arrows, close, maximize, and similar symbols.
#[derive(Default)]
pub struct SymbolTheme {
    /// Disabled symbol.
    pub inactive: CharAttribute,
    /// Symbol under the mouse pointer.
    pub hovered: CharAttribute,
    /// Symbol while pressed.
    pub pressed: CharAttribute,
    /// Checked state (for example `[√]`).
    pub checked: CharAttribute,
    /// Unchecked state (for example `[ ]`).
    pub unchecked: CharAttribute,
    /// Unknown / mixed state.
    pub unknown: CharAttribute,
    /// Navigation arrows.
    pub arrows: CharAttribute,
    /// Window close button.
    pub close: CharAttribute,
    /// Window maximize / restore button.
    pub maximized: CharAttribute,
    /// Window resize grip.
    pub resize: CharAttribute,
}
/// Colors for a dropdown or context menu.
#[derive(Default)]
pub struct MenuTheme {
    /// Item caption.
    pub text: ControlCharAttributesState,
    /// Hotkey character in the caption.
    pub hotkey: ControlCharAttributesState,
    /// Keyboard shortcut shown on the right.
    pub shortcut: ControlCharAttributesState,
    /// Check / radio / submenu marker.
    pub symbol: ControlCharAttributesState,
}

/// Colors for a window title bar.
#[derive(Default)]
pub struct WindowBar {
    /// Title bar when the window is focused.
    pub focus: CharAttribute,
    /// Title bar when the window is not focused.
    pub normal: CharAttribute,
    /// Title bar while the window is being resized.
    pub resizing: CharAttribute, 
    /// Close button.
    pub close_button: CharAttribute,
    /// Maximize / restore button.
    pub maximize_button: CharAttribute,
    /// Optional tag drawn on the title bar.
    pub tag: CharAttribute,
    /// Hotkey character in the title.
    pub hotkey: CharAttribute,  
}

/// Colors for a window frame and title bar.
#[derive(Default)]
pub struct WindowTheme {
    /// Focused window interior.
    pub normal: CharAttribute,
    /// Inactive window interior.
    pub inactive: CharAttribute,
    /// Error-severity window.
    pub error: CharAttribute,
    /// Warning-severity window.
    pub warning: CharAttribute,
    /// Informational / notification window.
    pub info: CharAttribute,
    /// Title-bar colors.
    pub bar: WindowBar,
}
/// Colors for a standard (shadowed) button.
#[derive(Default)]
pub struct RegularButtonTheme {
    /// Button caption.
    pub text: ControlCharAttributesState,
    /// Hotkey character in the caption.
    pub hotkey: ControlCharAttributesState,
    /// Drop shadow.
    pub shadow: CharAttribute,
}
/// Colors for a beveled button.
#[derive(Default)]
pub struct BevelButtonTheme {
    /// Button caption.
    pub text: ControlCharAttributesState,
    /// Hotkey character in the caption.
    pub hotkey: ControlCharAttributesState,
    /// Dark edge of the bevel.
    pub dark_margin: CharAttribute,
    /// Light edge of the bevel.
    pub light_margin: CharAttribute,
}

/// Colors for all button styles.
#[derive(Default)]
pub struct ButtonTheme {
    /// Standard shadowed button.
    pub regular: RegularButtonTheme,
    /// Beveled button.
    pub bevel: BevelButtonTheme,
}
/// Colors for tab headers.
#[derive(Default)]
pub struct TabTheme {
    /// Tab caption.
    pub text: ControlCharAttributesState,
    /// Hotkey character in the caption.
    pub hotkey: ControlCharAttributesState,
}
/// Colors for accordion panel headers.
#[derive(Default)]
pub struct AccordionTheme {
    /// Header caption.
    pub text: ControlCharAttributesState,
    /// Hotkey character in the caption.
    pub hotkey: ControlCharAttributesState,
}
/// Colors for a scroll bar.
#[derive(Default)]
pub struct ScrollBarTheme {
    /// Arrow buttons at each end.
    pub arrow: ControlCharAttributesState,
    /// Track behind the thumb.
    pub bar: ControlCharAttributesState,
    /// Draggable thumb / position marker.
    pub position: ControlCharAttributesState,
}

/// Colors for a search / filter bar.
#[derive(Default)]
pub struct SearchBarTheme {
    /// Unfocused search field.
    pub normal: CharAttribute,
    /// Focused search field.
    pub focused: CharAttribute,
    /// Match-count text.
    pub count: CharAttribute,
}

/// Colors for the current (highlighted) row in a list or tree.
#[derive(Default)]
pub struct ListCurentItemTheme {
    /// Current item when the list is focused.
    pub focus: CharAttribute,
    /// Pointer over a disabled item.
    pub over_inactive: CharAttribute,
    /// Pointer over a selected item.
    pub over_selection: CharAttribute,
    /// Current item when the list is not focused.
    pub normal: CharAttribute,
    /// Selected (checked) item that is not current.
    pub selected: CharAttribute,
    /// Item icon.
    pub icon: CharAttribute,
}

/// Colors for a list or tree column header.
#[derive(Default)]
pub struct HeaderTheme {
    /// Header caption.
    pub text: ControlCharAttributesState,
    /// Hotkey character in the caption.
    pub hotkey: ControlCharAttributesState,
    /// Sort / resize symbol.
    pub symbol: ControlCharAttributesState,
}

/// Colors for a toggle button.
#[derive(Default)]
pub struct ToggleButtonTheme {
    /// Pressed / selected appearance.
    pub selected: ControlCharAttributesState,
    /// Released / unselected appearance.
    pub unselected: ControlCharAttributesState,
}


/// Colors for markdown rendering.
#[derive(Default)]
pub struct MarkdownTheme {
    /// Body text.
    pub text: CharAttribute,
    /// Bold span.
    pub bold: CharAttribute,
    /// Italic span.
    pub italic: CharAttribute,
    /// Hyperlink.
    pub link: CharAttribute,
    /// Inline code.
    pub code: CharAttribute,
    /// Heading level 1.
    pub h1: CharAttribute,
    /// Heading level 2.
    pub h2: CharAttribute,
    /// Heading level 3.
    pub h3: CharAttribute,
    /// Fenced code block.
    pub code_block: CharAttribute,
    /// Ordered list marker.
    pub ordered_list: CharAttribute,
    /// Unordered list marker.
    pub unordered_list: CharAttribute,
    /// Table cell.
    pub table: CharAttribute,
    /// Table header cell.
    pub table_header: CharAttribute,
}

/// Colors for a progress bar.
#[derive(Default)]
pub struct ProgressBarTheme {
    /// Unfilled track.
    pub background: Color,
    /// Filled (progress) portion.
    pub progress: Color,
    /// Percentage / status text.
    pub text: Color,
}

/// Colors for a horizontal slider.
#[derive(Default)]
pub struct SliderTheme {
    /// Track on the filled side of the marker.
    pub before_line: CharAttribute,
    /// Track on the empty side of the marker.
    pub after_line: CharAttribute,
    /// Draggable marker.
    pub marker: ControlCharAttributesState,
    /// Marker outline.
    pub marker_border: ControlCharAttributesState,
    /// End caps of the track.
    pub cap: CharAttribute,
}

/// Complete color palette used to paint the desktop and all controls.
///
/// Create a built-in palette with [`Theme::new`](Self::new), or mutate the public
/// fields to customize individual controls.
#[derive(Default)]
pub struct Theme {
    /// Accordion panel headers.
    pub accordion: AccordionTheme,
    /// Desktop background.
    pub desktop: DesktopTheme,
    /// Generic text (labels, captions).
    pub text: TextTheme,
    /// Check marks, arrows, and window chrome symbols.
    pub symbol: SymbolTheme,
    /// Tooltip balloons.
    pub tooltip: ToolTipTheme,
    /// Menus owned by the focused control.
    pub menu: MenuTheme,
    /// Parent / ancestor menus.
    pub parent_menu: MenuTheme,
    /// Window frame and title bar.
    pub window: WindowTheme,
    /// Control borders.
    pub border: ControlCharAttributesState,
    /// Decorative lines (HLine, VLine, splitters).
    pub lines: ControlCharAttributesState,
    /// Buttons.
    pub button: ButtonTheme,
    /// Tab headers.
    pub tab: TabTheme,
    /// Scroll bars.
    pub scrollbar: ScrollBarTheme,
    /// Search bars.
    pub searchbar: SearchBarTheme,
    /// Text editors and similar input fields.
    pub editor: ControlCharAttributesState,
    /// Current row in a list or tree.
    pub list_current_item: ListCurentItemTheme,
    /// Column headers.
    pub header: HeaderTheme,
    /// Toggle buttons.
    pub toggle_button: ToggleButtonTheme,
    /// Markdown viewer.
    pub markdown: MarkdownTheme,
    /// Progress bars.
    pub progressbar: ProgressBarTheme,
    /// Hyperlinks.
    pub hyperlink: ControlCharAttributesState,
    /// Horizontal sliders.
    pub hslider: SliderTheme,
}
impl Theme {
    /// Creates a theme from a built-in [`Themes`] variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// let theme = Theme::new(Themes::DarkGray);
    /// ```
    pub fn new(theme: Themes) -> Self {
        match theme {
            Themes::Default => super::default::new(),
            Themes::DarkGray => super::dark_gray::new(),
            Themes::Light => super::light::new(),
        }
    }
}
