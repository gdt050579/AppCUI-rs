# Changelog

All notable changes to AppCUI-rs are listed here, newest first.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/). Each version links to its GitHub release, which has
screenshots and the full notes.

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

[Unreleased]: https://github.com/gdt050579/AppCUI-rs/compare/v0.5.4...HEAD
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