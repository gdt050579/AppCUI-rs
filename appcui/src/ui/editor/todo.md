# Editor Shortcuts — Implementation Checklist

## Cursor Movement
- [x] `Left` → move one character to the left
- [x] `Right` → move one character to the right
- [x] `Up` → move one line up
- [x] `Down` → move one line down
- [x] `Home` → move to start of line
- [x] `End` → move to end of line
- [x] `Ctrl+Left` → move one word to the left
- [x] `Ctrl+Right` → move one word to the right
- [x] `Ctrl+Home` → move to start of document
- [x] `Ctrl+End` → move to end of document
- [x] `Ctrl+Up` → move viewport up
- [x] `Ctrl+Down` → move viewport down
- [x] `PageUp` → move one screen up
- [x] `PageDown` → move one screen down

## Selection
- [x] `Shift+Left` → extend selection one character left
- [x] `Shift+Right` → extend selection one character right
- [x] `Shift+Up` → extend selection one line up
- [x] `Shift+Down` → extend selection one line down
- [x] `Shift+Home` → extend selection to start of line
- [x] `Shift+End` → extend selection to end of line
- [x] `Shift+Ctrl+Left` → extend selection one word left
- [x] `Shift+Ctrl+Right` → extend selection one word right
- [x] `Shift+Ctrl+Home` → extend selection to start of document
- [x] `Shift+Ctrl+End` → extend selection to end of document
- [x] `Shift+PageUp` → extend selection one screen up
- [x] `Shift+PageDown` → extend selection one screen down
- [x] `Ctrl+A` → select all

## Text Editing
- [ ] `Backspace` → delete character before cursor (or delete selection)
- [ ] `Delete` → delete character after cursor (or delete selection)
- [ ] `Ctrl+Backspace` → delete word before cursor
- [ ] `Ctrl+Delete` → delete word after cursor
- [ ] `Enter` → insert newline
- [ ] `Tab` → insert tab (or indent selected lines)
- [ ] `Shift+Tab` → outdent line (or selected lines)
- [ ] `Insert` → toggle insert/overwrite mode

## Clipboard & History
- [ ] `Ctrl+C` → copy selection (or current line if no selection)
- [ ] `Ctrl+X` → cut selection (or current line if no selection)
- [ ] `Ctrl+V` → paste
- [ ] `Ctrl+Z` → undo
- [ ] `Ctrl+Y` → redo
- [ ] `Ctrl+Shift+Z` → redo (alternative)

## Mouse
- [ ] `Click` → place cursor
- [ ] `Click + Drag` → select range
- [ ] `Double-click` → select word
- [ ] `Triple-click` → select line
- [ ] `Shift+Click` → extend selection to click position
- [ ] `Scroll wheel` → scroll vertically
- [ ] `Ctrl+Scroll` → zoom in/out
- [ ] `Alt+Click` → add cursor (multi-cursor)

## File Operations
- [ ] `Ctrl+S` → save file

## Search & Navigation
- [ ] `Ctrl+F` → open find
- [ ] `Ctrl+H` → open find and replace
- [ ] `F3` → find next
- [ ] `Shift+F3` → find previous
- [ ] `Ctrl+G` → go to line
- [ ] `Ctrl+/` → toggle line comment

## Line Manipulation
- [ ] `Alt+Up` → move line up
- [ ] `Alt+Down` → move line down
- [ ] `Shift+Alt+Up` → duplicate line up
- [ ] `Shift+Alt+Down` → duplicate line down
- [ ] `Ctrl+D` → select next occurrence of word
- [ ] `Ctrl+L` → select current line
- [ ] `Ctrl+Shift+K` → delete line
- [ ] `Ctrl+]` → indent line
- [ ] `Ctrl+[` → outdent line

## Performance
- [x] Replace `document.char_at` loops in `move_to_next_word` / `move_to_previous_word` with ropey `Chars` iterator
- [ ] Cache cursor virtual column; update incrementally on horizontal moves instead of recomputing via `position_to_virtual_column`
- [ ] Decide between dirty-region painting vs. cheap full-repaint before adding highlighting
- [ ] Refactor `view: Option<Surface>` to `surface: Surface` + lazy init in `on_resize` + `needs_repaint` flag
- [x] Verify `Selection::contains` is O(1); if not, precompute per-line selection overlap before the char loop in `paint_line`
- [ ] Fix off-by-one in `coordinates_to_position` for the final line of a document with no trailing newline
- [ ] Make `update_view` panic-safe (don't leave `view` as `None` if anything between `take()` and reassign panics)
- [ ] Skip horizontally-scrolled-off chars in `paint_line` without iterating them one by one
- [ ] Design syntax highlighting: per-line `LineCache` with `state_in`/`state_out`, `Highlighter` trait, style palette indexed by `u8`
- [ ] Implement lazy tokenization: only tokenize visible lines + buffer, track highest-tokenized-line watermark
- [ ] Define style precedence: selection > match-highlight > syntax > default
- [ ] Design "highlight similar selection": viewport-bounded search, debounced, cached by `(needle_hash, viewport_range)`
- [ ] Implement viewport-bounded search using ropey `Chunks` + `memmem`, handling chunk boundaries
- [ ] Exclude the active selection itself from the "similar matches" list
- [ ] Add unit tests for `coordinates_to_position`, `position_to_virtual_column`, and word-motion edge cases
