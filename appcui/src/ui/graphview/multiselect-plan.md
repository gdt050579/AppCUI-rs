# GraphView multi-select — implementation plan

This document is the working spec for multi-selection on `GraphView`. Edit this file as the source of truth; implementation can be requested against this path (Agent mode).

## Scope (v1)

- **`Flags::MultiSelect`** — opt-in; when unset, behavior matches today.
- **`Node.selected: bool`** — per-node selection; default `false` in `NodeBuilder`.
- Keep **`Graph::current_node`** as the **primary** (keyboard anchor, `Enter` / double-click, scroll-into-view).
- **Indicators:** **☑** / **☐** only (no style enum in v1). Shown **only when `MultiSelect` is set**.
  - **Borderless:** glyph to the **left** of the label; reserve horizontal gutter so centered text does not overlap.
  - **Bordered:** glyph on the **first row** of the frame (inside border strip / top-left corner rule — pick one and keep it consistent).
- **Mouse:** **Ctrl+click** toggles `selected` on the hit node (only if `MultiSelect`); **plain click** clears all `selected`, sets hit node selected, sets `current_node`, drag semantics below. *(Implemented: Ctrl toggle runs on **button release** if movement ≤ ~3 px; larger movement starts a multi-node drag without toggling.)*
- **Move:** Any operation that moves the **current** node by a delta (mouse drag, **`Ctrl+Arrow`** nudge) applies the **same delta** to **every** node with `selected == true`.
- **Drag:** If the user starts a drag on a node that is **`selected`**, drag **all selected** nodes; if on an unselected node with plain click, **replace selection** and drag **only that** node.
- **Out of v1:** style `Type` enum (Ascii / color-only variants), Shift+range select, Ctrl+A, optional `SelectionChanged` event, batch repaint after multi-move.

## Invariants

1. After **plain click** on a node: that node is `current_node`, `selected == true`, all others `selected == false`.
2. After **Ctrl+click** with `MultiSelect`: toggle `selected` on the hit node; set `current_node` to the hit node (anchor).
3. Prefer **no drag** on a pure Ctrl+click press-release without motion (toggle only), unless you explicitly choose otherwise.
4. Avoid `current_node` pointing at a node with `selected == false` while other nodes stay selected — keep primary in sync with your click rules.

## Files to touch (expected)

| Area                        | File(s)                                                                                                                              |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Flags                       | `initialization_flags.rs` — add `MultiSelect` bit                                                                                    |
| Node state + paint + layout | `node.rs` — `selected`, gutter, ☑/☐, `resize` / `paint`                                                                              |
| Graph logic                 | `graph.rs` — `paint_node` / `repaint` attrs for selected, multi-nudge, edge-highlight policy, `EditableGraph` / remove / `set_graph` |
| Control                     | `graphview.rs` — mouse modifiers, multi-drag payload, pass “show multiselect UI” into graph render path                              |
| Docs                        | `docs/chapter-3/stock-controls/graphview.md`                                                                                         |
| Tests                       | `tests.rs`                                                                                                                           |

## Implementation order

1. `Flags::MultiSelect` and wire into `GraphView::new` / any flag checks.
2. `Node.selected` + builder default; clear on graph replace as decided.
3. Plumb “multiselect UI active” into paint/resize (e.g. via `RenderingOptions` or graph field) so **gutter width** and ☑/☐ draw only when flag is on.
4. `Node::paint` / `resize`: gutter + ☑/☐ (border vs borderless placement).
5. `Graph` painting: distinct attr for `selected`; define **edge highlight**: primary (`current_node`) only **or** union over all selected — pick one and document.
6. `graphview.rs` mouse: `mouse_data.modifier` + `KeyModifier::Ctrl`; selection + `Drag` carrying multiple ids + shared delta.
7. `graph.rs` keyboard: `Ctrl+Arrow` applies to all `selected` ids.
8. `EditableGraph` + node removal: `current_node` adjustment (existing); ensure `selected` on removed row disappears with the node.
9. Documentation + tests (modifiers available in event recorder tests).

## Design decisions (step 5)

- **Node text color (focused):** non-primary selected nodes use `Theme::button::regular::text::pressed_or_selected` unless the node has an explicit `text_attr`.
- **Primary** (`current_node`) keeps **`focused`** text attr (drawn last on top).
- **Edge highlight** (`enable_edge_highlighting`): when **`RenderingOptions::multiselect_ui`** is **off**, incident-edge overlays follow **`current_node`** only (unchanged). When **on**, overlays use the **union** of edges incident to every **`Node::selected`** node (same hovered-line attr). Duplicate edges between two selected nodes may be drawn twice with identical styling.

## Edge cases

- **Empty graph / `current_node` out of range:** same as today; clear selections when appropriate.
- **`move_node_to` in a loop:** may repaint each call; optional later optimization: move many, repaint once.
- **Glyph width:** confirm ☑/☐ width in your terminal/text path; reserve **1 or 2** columns accordingly.

## Optional follow-ups (not in v1)

- `SelectionChanged` (or richer) event for apps.
- Shared indicator style enum with checkbox, or `None` (color-only) variant.
- Shift+click range, Select all.

---

*Last aligned with design discussion: multi-select flag, per-node `selected`, ☑/☐ placement, Ctrl+click, group move/drag.*