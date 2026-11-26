# Changelog

All notable changes to AppCUI-rs are listed here, newest first.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/). Each version links to its GitHub release, which has
screenshots and the full notes.

## [0.5.11] - 2026-11-26
 
### Added
- **Terminal restore on panic**: a panic hook restores the terminal (screen, cursor, mouse reporting,
  palette) before the panic message is printed. On a normal exit the same restore path runs.
  Each backend provides its own restorer: Windows Console, Windows VT, Termios, NCurses and
  CrossTerm. New example: `panic` (closes #300).
- **Palettes** for the 16 named colors (closes #306):
  - `Palette` (`Terminal`, `BuiltIn`, `Custom`) and `ColorPalette` (`built_in()`, `from_pairs(...)`)
    in `appcui::backend`;
  - the `palette(...)` and `palette_as_rgb(...)` application builder methods;
  - supported by Windows Console, Windows VT, Termios and CrossTerm (NCurses and the Web Terminal
    ignore the palette). Termios and CrossTerm reset the palette with OSC 104 on exit;
  - new examples: `palette` and `palette_gray`.
- **MainWindow**: a borderless window with no title and no close button that fills the desktop and
  is used with `App::main_window(...)`. Adds the `MainWindowControl` marker trait, the
  `#[MainWindow]` proc-macro attribute and the event-trait wiring for all controls (#313).
- `window::Type::Borderless`: a title bar without a surrounding frame and without inner margins.
  The title is left-aligned and the toolbar items (tag, hotkey, buttons) are placed on the right
  (closes #311, #312).
- `window::Flags::NoTitle`: hides the title text while keeping the frame, close button, tag and
  hotkey. `window!` accepts `Borderless` and `NoTitle`.
- `Themes::HighContrast`, a high-contrast predefined theme (black surfaces, white text, yellow and
  aqua accents) by @emreocell in #303 (#309).
- documentation: a `MainWindow` chapter, palette sections in the application and backend chapters,
  a refreshed event-loop `Window` chapter and a `HighContrast` entry in the predefined themes.
### Changed
- **Breaking:** `App::single_window` is renamed `App::main_window`, and `SingleWindowAppBuilder`
  is renamed `MainWindowAppBuilder`. The window must implement `MainWindowControl`
  (use `MainWindow` or `#[MainWindow]`).
- **Breaking:** `color_schema(bool)` is removed from the application builders; use `palette(...)`
  and `palette_as_rgb(...)`. `palette_as_rgb(true)` with `Palette::Terminal` makes `run()` return
  `ErrorKind::InvalidParameter`.
- Window tab, hotkey and title colors are now computed per window type (including `Borderless`).
- Internal: Windows console code split into `console` and `console_restorer`;
  `Backend::on_close` replaced by `take_restorer`.
- `appcui_proc_macro` bumped to 0.3.11.
### Fixed
- The Windows console palette is restored on exit (only the palette, not the whole console state).
- Several Termios and CrossTerm fixes found while adding palette support.
- Various compiler and Clippy warnings.

## [0.5.10] - 2026-11-20

### Added
- **VStackedBarChart** control: vertical stacked bar chart where each column (a stack) is made of
  several bars (segments) stacked on top of each other, with positive values growing upward and
  negative values downward from a zero line. Generic over any numeric type. Stacks are built with
  `BarsStackBuilder` (or from plain values / arrays) and each bar has a value, an optional label,
  an optional color and its own draw mode (`Fill`, `Rectangle`, `FilledRectangle`).
  Supports:
  - scale modes (`FromZero`, `FitData`, `Fixed`, `FromZeroMinRange`, `Percentage`);
  - X axis labels (none, stack labels, index, or custom spans) and a Y axis with grid and step;
  - horizontal scrolling by keyboard (`Left`/`Right`, `Home`/`End`, `Ctrl+Left`/`Ctrl+Right`),
    scrollbars and the mouse wheel, with the same behavior as `VBarChart`;
  - selection of a bar or of a whole stack by keyboard, mouse click (bar) and double click (stack),
    kept visible without focus, with a `theme.chart.selection_border` frame and the
    `DimBarsOnSelection` flag;
  - hover tooltips, number formatting and `ShowZeroLineOnYAxis`;
  - the `on_stack_selected` / `on_cell_selected` / `on_clear_selection` events
    (`VStackedBarChartEvents`).
  Created with `VStackedBarChart::new` or `vstackedbarchart!`.
- `vstackedbarchart!` proc-macro (`appcui-proc-macro` 0.3.10), together with the
  `VStackedBarChartEvents` event trait wiring for `#[Window(events = ...)]` and modal windows.
- Public types `BarsStack`, `BarsStackBuilder`, `StackBar`, `StackBarBuilder`, `StackBarDrawMode`
  and `StackBarScale`.
- **Sprite**: a rectangular block of `Character`s with no origin or clip area, to be drawn onto a
  surface. Methods: `Sprite::new`, `size`, `get`, `set`, `clear`, `resize` and `edit` (draw into a
  sprite using the regular `Surface` API). New method `Surface::draw_sprite(x, y, &sprite)`
  (respects the surface origin and clip area).
- new examples: `vstackedbarchart`, `vstackbarchart_editor`, `progressbar` (by
  @vishwakmandaram in #304, closes #263), 
- new example: `file_navigator` (by @emreocell in #302)
- documentation: chapters for `VStackedBarChart`, `Sprite` and for the proc-macros
  (including how to debug macro output), a refreshed `Window` guide with real screenshots
  (by @harshit3355 in #296), and a new gallery entry.

### Fixed
- The error message for `#[Window(events = WrongTrait)]` now refers to the `events` attribute
  instead of `overwrite` (by @peso in #297).

### Changed
- Internal clean-ups: Clippy fixes in the examples (`connect_four`, `game_2048`, `pacman`) and
  in `UsageChart` (the percentage computation no longer risks a division by zero), and the shared
  stacked-bar code moved into `ui/components`.

## [0.5.9] - 2026-11-14

- Termios is the default backend on Linux and on macOS. NCurses is optional (`NCURSES` feature) on Unix. CrossTerm stays optional (`CROSSTERM` feature) and is not a default feature.
- `NCURSES` feature by @peso in #291

## [0.5.8] - 2026-11-13

### Added
- **HeatMap** control: a grid of cells with values, labels, and colors. Supports scale modes,
  axes, per-cell appearance where you can edit the way each cell is drawn. Created with
  `HeatMap::new` or `heatmap!`.
- `examples/heatmap` and `examples/heatmap_editor`.
- new example: `booking_form` by @shu-chen-dot-ca in #288
- Color::to_rgb() / as_rgb() / from_rgb() apis


## [0.5.7] - 2026-10-31

### Added
- **UsageChart** control: one horizontal bar divided into segments that share a total.
  Each segment has a `u64` value, a label, a fill (`Solid`, shades, `Braille`, `Checkerboard`,
  grids, `CrossHatch`, `Dashed`, diagonals, `Notched`, or a custom character) and an optional
  color. The width of a segment is its share of `TotalUsage` (`Auto`, or a fixed total that
  leaves the rest of the bar empty). Segments stay in the order they were added, or are sorted
  by value or by label. Hover tooltips show the label, the value, the percentage, or a
  combination, with a configurable number format. Also supports `Margins`,
  `DimSegmentsOnSelection`, selection by mouse or keyboard that stays visible without focus,
  and the `on_usage_segment_selected` / `on_clear_selection` events. Created with
  `UsageChart::new` or `usagechart!`.
- `examples/usagechart` and `examples/usagechart_editor`.

## [0.5.6] - 2026-10-24

### Added
- **LineChart** control: plots one or more numeric series (any integer or floating-point type)
  over a shared X axis. Each series has its own label, color and line mode (`Braille`, a direct
  box-drawing line, or an orthogonal line with a routing direction), and every point can override
  its color and marker (small, or a 5x3 framed marker that lines attach to). Supports scale modes,
  chart-level names for the X positions (with an index, names or span X axis), per-point notes in
  tooltips, Y axis with grid, value formatting, horizontal scrolling, per-point selection by mouse
  or keyboard, `GroupHover`, `DimPointsOnSelection`, `ShowZeroLineOnYAxis`, and the
  `on_point_selected` / `on_clear_selection` events. Created with `LineChart::new` or `linechart!`.
- `BarDrawMode::FramedPoint`: a 5x3 marker (empty frames, frames with a bullet, braille circles,
  filled, or a custom character), available in `VBarChart`, `HBarChart` and `LineChart`.
- `chart.hover` theme color, used by `LineChart` with `GroupHover`.
- `examples/linechart` and `examples/linechart_editor`.

### Fixed
- `Point` without a marker kind in `vbarchart!` / `hbarchart!` (for example `dm: Point`) now uses
  `Bullet`; before, it generated code that did not compile.

## [0.5.5] - 2026-10-21

### Added
- **PieChart** control (#255): shows a series of values as parts of a whole. Supports circle,
  ellipse, square, rectangle and triangle shapes (each with an optional rounded version for the
  last three), a hole in the middle (donut) sized as a percentage or in characters, and several
  renderers (half blocks, Braille, solid, shades, grids, diagonals or a custom character). Labels
  can be drawn inside the slices or around the chart, and show the label, value, percentage, or
  label with value or percentage. Also supports a configurable start angle and direction,
  selection by mouse or keyboard, hover tooltips, `DimSlicesOnSelection`, and the `on_slice_selected`
  / `on_clear_selection` events. Created with `PieChart::new` or `piechart!`.
- **PaletteTile**: an image of palette indexes (`u8` storage, switching to `u16` automatically
  when more colors are needed) that can be drawn with small blocks, large blocks, Braille or a
  custom character, using `Surface::draw_palette_tile`.
- `examples/piechart` and `examples/piechart_editor`.
- NumericSelector example by @AnushreeKasturi in #272
- Conway's Game of Life example by @AnushreeKasturi in #276
- Charvariant by @gdt050579 in #280

### New Contributors
- @AnushreeKasturi made their first contribution in #272

## [0.5.4] - 2026-10-10

### Added
- **HBarChart** control: one horizontal bar per value of a numeric series (any integer or
  floating-point type). Supports scale modes (`FromZero`, `FromZeroMinRange`, `FitData`, `Fixed`),
  labels beside the bars (index, per-bar label or spans), per-bar height, spacing, color and draw
  mode, X axis with grid, value formatting, selection and hover events, vertical scrolling,
  `DimBarsOnSelection` and `ShowZeroLineOnXAxis`. Created with `HBarChart::new` or `hbarchart!`.
- `examples/hbarchart` and `examples/hbarchart_editor`.

### Changed
- `Bar`, `BarBuilder`, `BarScale`, `BarDrawMode` and `BarSpan` are shared by `HBarChart` and `VBarChart`.

## [0.5.3] - 2026-10-04

### Added
- **VBarChart** control: one vertical bar per value of a numeric series. Supports the same scale
  modes, X-axis labels (index, per-bar label or spans), per-bar thickness, spacing, color and draw
  mode, Y axis with grid, value formatting (decimal, hex, octal, binary), selection and hover events,
  horizontal scrolling, `DimBarsOnSelection` and `ShowZeroLineOnYAxis`. Created with
  `VBarChart::new` or `vbarchart!`.

## [0.5.2] - 2026-09-27

### Added
- **MarkdownComposer** control: editable markdown formatted as you type (bold, italic, inline and
  fenced code, bullets, quotes, links, e-mail addresses), with markers hidden unless `ShowMarkers`
  is set. Copying a selection produces valid markdown, and deleting keeps it valid.
- Suggestion lists opened by a trigger character, a built-in list of over 850 emoji, and optional
  replacement of emoticons and `:emoji_names:` (`Emoticons` flag).
- `ReadOnly` flag, `on_validate` (Ctrl+Enter) and `on_text_changed` events, and the
  `markdown_composer!` macro.

## [0.5.1] - 2026-09-07

### Added
- Polyline drawing support (#248).
- Wide-character discovery (#250).

### Changed
- Documentation for every public method, trait, enum and struct.

## [0.5.0] - 2026-09-04

### Added
- **FrameApp** mode: the whole screen as a canvas, redrawn at a fixed rate. Intended for games,
  animations and simulations.
- **InputApp** mode: the whole screen as a canvas, redrawn only in response to keyboard and mouse
  input. Intended for viewers, turn-based games and custom-drawn tools.
- New examples for both modes, including a clock and a memory card game.

## [0.4.18] - 2026-08-29

### Added
- RGB colors in proc-macros (#244).

### Fixed
- Menu navigation overflowing a fixed 128-entry array (#243).
- ListView selected-item count underflowing for pre-checked items (#234).
- Bug #222 (#233) and other fixes (#235–#242).

## [0.4.17] - 2026-08-06

### Added
- **HSlider** control: a horizontal slider with five styles (Standard, ProgressBar, Inline, Ruler,
  Blocks), the `ShowValue`, `Ticks` and `ValueAsMarker` flags, keyboard and mouse input, a tooltip
  while dragging, the `on_value_changed` event, theming (`HSliderTheme`) and the `hslider!` macro.

### Changed
- The `Number` trait moved to `ui::common`.

## [0.4.16] - 2026-08-02

### Added
- `MergeBorder` flag for ListView and TreeView.

### Changed
- Checked and fixed for Rust 1.97.1, Clippy 0.1.97 and Cargo 1.97.1.

## [0.4.15] - 2026-07-26

### Added
- **HyperLink** control: hover underline, tooltip, activation by click or Enter, the
  `HyperLinkEvents::on_open` event, the `hyperlink!` macro, theme support, documentation and tests.

## [0.4.14] - 2026-07-24

### Added
- Automatic junctions between box-drawing characters.

## [0.4.13] - 2026-07-09

### Added
- **BufferView** (hex viewer and editor) control: Hex, Dec, Oct, Float, Bin and character views;
  byte, word, dword and qword grouping; endianness; selection; labels and annotations for buffer
  regions; editing in several formats; search; code pages; fixed or automatic column count.

## [0.4.12] - 2026-06-06

### Added
- Ctrl+A (select all) in TextArea.

### Fixed
- Several TextArea bugs.

## [0.4.11] - 2026-05-06

### Added
- Undo and redo in TextField.

## [0.4.10] - 2026-05-05

### Added
- Full GraphView editing: API to add and remove nodes and edges, Ctrl+click on the surface to add a
  node, right-click and drag between nodes to add an edge, and multi-selection with Ctrl+click.

## [0.4.9] - 2026-05-02

### Added
- Editable GraphView: add and remove nodes and edges, and change their value, color and layout.

## [0.4.8] - 2026-04-23

### Added
- **RichTextField** control: a TextField with per-character styling and full undo/redo history (#199).

## [0.4.7] - 2026-03-08

### Fixed
- TAB characters are now parsed correctly in TextArea.

## [0.4.6] - 2025-12-15

### Added
- Auto-scaling fonts for the web terminal, contributed by @cristinel24 (#196).

### Changed
- Crossterm is now the default backend on Linux, contributed by @selairi (#195).

### Fixed
- Crossterm painting errors caused by wide characters, contributed by @selairi (#193).
- `get_os_roots` on the web terminal (#196).

## [0.4.5] - 2025-10-25

### Added
- The terminal is restored or cleared when the application exits (#192).

### Fixed
- Several Termios backend bugs.

## [0.4.4] - 2025-10-17

### Added
- 3D styles for Button and Panel (#187, #188).
- RadioBox types (#186).

### Changed
- ThreeStateBox methods aligned with Button, CheckBox and RadioBox (#189).

## [0.4.3] - 2025-10-11

### Added
- Window styles (#184).

### Fixed
- Crossterm performance and modifier-key handling, contributed by @selairi (#185).

## [0.4.2] - 2025-10-04

### Added
- Game examples: PacMan, Tetris, 2048, Chess and Connect Four.
- Additional `BitTile` methods.

### Fixed
- Bug #180.

### Changed
- Documentation improvements.

## [0.4.1] - 2025-09-27

### Added
- **TimePicker** control.
- `BitTile` and `Glyph` graphics types for image manipulation.
- Gallery improvements.

### Fixed
- Mouse-reporting escape sequences, contributed by @Julien-cpsn (#174).

## [0.4.0] - 2025-09-21

### Added
- **AppBar**, replacing the previous `MenuBar`. It holds `MenuButton`, `Separator`, `Label`,
  `SwitchButton`, `Button` and `ToggleButton` items, each of which can be enabled or disabled, have a
  tooltip, and sit on the left or right side of the bar. Suggested by @Julien-cpsn (#175).

### Changed
- `MenuBar` replaced by `AppBar` (see above).

The 0.4.0 release also collects the work done since the project started (pull requests #7–#175):
most stock controls, the Windows, NCurses, Termios, Crossterm and web backends, true-color support,
background tasks, dialogs, themes, the layout builder and the gallery. The
[0.4.0 release notes](https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.0) list every pull
request and its author.

[0.5.11]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.5.11
[0.5.10]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.5.10
[0.5.9]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.5.9
[0.5.8]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.5.8
[0.5.7]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.5.7
[0.5.6]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.5.6
[0.5.5]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.5.5
[0.5.4]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.5.4
[0.5.3]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.5.3
[0.5.2]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.5.2
[0.5.1]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.5.1
[0.5.0]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.5.0
[0.4.18]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.18
[0.4.17]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.17
[0.4.16]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.16
[0.4.15]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.15
[0.4.14]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.14
[0.4.13]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.13
[0.4.12]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.12
[0.4.11]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.11
[0.4.10]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.10
[0.4.9]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.9
[0.4.8]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.8
[0.4.7]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.7
[0.4.6]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.6
[0.4.5]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.5
[0.4.4]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.4
[0.4.3]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.3
[0.4.2]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.2
[0.4.1]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.1
[0.4.0]: https://github.com/gdt050579/AppCUI-rs/releases/tag/v0.4.0
