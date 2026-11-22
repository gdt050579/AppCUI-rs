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
- [x] `Regular edit`
- [x] `Enter` → insert newline
- [x] `Tab insert` → insert tab
- [x] `Shift+Tab` → sterge tabul
- [x] `Backspace` → delete character before cursor (or delete selection)
- [x] `Delete` → delete character after cursor (or delete selection)
- [x] `Ctrl+Delete` → delete word after cursor
- [x] `Ctrl+Backspace` → delete word before cursor
- [x] `Tab selectie` → indent selected lines
- [x] `Shift+Tab` → outline 
- [x] `` (metoda vizuala de vazut taburile) (flag) → `Flags::ShowTabs`, `Flags::ShowSpaces`
- [x] `Insert` → toggle insert/overwrite mode

## Clipboard & History
- [x] `Ctrl+C` → copy selection (or current line if no selection)
- [x] `Ctrl+X` → cut selection (or current line if no selection)
- [x] `Ctrl+V` → paste, de gandit logica (paste formatat sau nu) → momentan la fel ca `Ctrl+Shift+V` (text simplu)
- [ ] `Tipuri de copiere` (UTF8, UNICODE, HTML, cu tot cu culoarea textului,)
- [x] `Ctrl+Shift+V` paste text only

## Undo/Redo
- [ ] `Ctrl+Z` → undo
- [ ] `Ctrl+Y` → redo
- [ ] `Ctrl+Shift+Z` → redo (alternative)

## Mouse
- [x] `Click` → place cursor
- [x] `Click + Drag` → select range
- [x] `Double-click` → select word
- [x] `Triple-click` → select line
- [x] `Shift+Click` → extend selection to click position
- [x] `Scroll wheel` → scroll vertically
- [x] `Alt+Click` → add cursor (multi-cursor)

<!--## File Operations
- [ ] `Ctrl+S` → save file

## Search & Navigation
- [ ] `Ctrl+F` → open find
- [ ] `Ctrl+H` → open find and replace
- [ ] `F3` → find next
- [ ] `Shift+F3` → find previous
- [ ] `Ctrl+G` → go to line
- [ ] `Ctrl+/` → toggle line comment-->

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

## Visual
- [ ] Add vertical scrollbar
- [ ] Add horizontal scrollbar - horizontal scroll is only computed for the cache (based on the largest line in the view port)
- [ ] Show selection(s)
- [ ] Sybtax highlighting