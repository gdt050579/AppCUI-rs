# UsageChart

A `UsageChart` draws one horizontal bar divided into segments that share a total. The width of a segment is its share of that total. Values are `u64`, so a segment cannot be negative. A segment with the value `0`, or one that does not fit, stays in the chart (and keeps its index) but takes no space. The largest values are given space first. A segment that would otherwise be less than one cell wide gets one cell when any width remains.

<img src="img/usagechart.png" width=400/>

Create one with `UsageChart::new` (a layout and initialization flags) or with the `usagechart!` macro:

```rs
let chart = UsageChart::new(layout!("d:f"), usagechart::Flags::None);
```

```rs
let c1 = usagechart!("d:f");
let c2 = usagechart!("x:1,y:1,w:40,h:3,segments:[10,20,30]");
let c3 = usagechart!("d:f,flags:Margins+DimSegmentsOnSelection,total:256,lc:LabelAndPercentage,segments:[{48,'OS'},{36,'Apps',fill:CrossHatch}]");
```

A new chart starts empty. The segments stay in the order they were added, the total is the sum of the values (so the segments fill the bar), and the hover tooltip shows the label and the value. Values are written with thousands separators (`1,234,567`). Each segment uses a theme chart color unless it has its own attribute. With `Margins` the minimum size is 3×3 and the bar is inset by one cell on every side. Without it the minimum size is 3×1 and the bar fills the control.

A UsageChart supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name                                 | Type    | Positional parameter | Purpose                                                         |
| ---------------------------------------------- | ------- | -------------------- | --------------------------------------------------------------- |
| `flags`                                        | Flags   | **No**               | Initialization flags                                            |
| `segments` or `values` or `data`               | List    | **No**               | Segments to add. Each item is a number or a description         |
| `sort` or `sort-type` or `sorttype` or `order` | String  | **No**               | Left-to-right order of the segments                             |
| `total` or `total-usage` or `totalusage`       | String  | **No**               | What the full width of the bar represents                       |
| `content` or `label-content` or `lc`           | String  | **No**               | What the hover tooltip shows                                    |
| `numeric-format` or `nf`                       | Dict    | **No**               | Format used for the values in the tooltip                       |
| `selected` or `select` or `selected-segment`   | Integer | **No**               | The index of the segment that is selected when the chart starts |

## Flags

* `usagechart::Flags::Margins` or `Margins` (for macro initialization) — insets the bar by one cell on every side. The minimum size becomes 3×3.
* `usagechart::Flags::DimSegmentsOnSelection` or `DimSegmentsOnSelection` — while a segment is selected, every other segment is drawn with the inactive chart color.

```rs
let chart = usagechart!("d:f,flags:Margins+DimSegmentsOnSelection,segments:[10,20,30]");
```

## Segments

A segment has a value, a label (that can be empty), a fill character and an optional character attribute. Without an attribute, the segment uses the next theme chart color. A number passed to `add_segment` or written in the macro list is a solid segment with that value and an empty label.

```rs
chart.add_segment(48u64);
chart.add_segment(usagechart::SegmentBuilder::new().value(36).label("Apps").fill(usagechart::SegmentFillType::CrossHatch).build());
chart.add_segments([10u8, 20u16, 30u32]);
```

In the `segments` (or `values`, or `data`) list, an item is either a number or a dictionary. `value` is required. The other fields are optional.

| Parameter name                                 | Type    | Positional parameter | Purpose                                          |
| ---------------------------------------------- | ------- | -------------------- | ------------------------------------------------ |
| `value` or `val` or `v`                        | Integer | **Yes** (first)      | The segment value (a positive integer)           |
| `label` or `text` or `caption` or `name`       | String  | **Yes** (second)     | The segment label                                |
| `fill` or `fill-type` or `filltype` or `ft`    | String  | **No**               | The character that fills the segment             |
| `attr` or `attribute` or `charattr` or `color` | Dict    | **No**               | Character attribute (same syntax as `charattr!`) |

```rs
let chart = usagechart!("d:f,segments:[10,{20,'OS'},{30,label:'Apps',fill:Grid,attr:red},{40,Docs,color:{fore:green}}]");
```

`clear` removes every segment, the selection and the tooltip. Flags, sort, total, number format and tooltip content stay as they are. The next segment added is the first one again.

## Fill

`fill` chooses the character repeated across a segment. The values are available from code as `usagechart::SegmentFillType`. `Solid` is the default.

| Fill           | Macro form                                      | Character      |
| -------------- | ----------------------------------------------- | -------------- |
| `Solid`        | `Solid` or `solid`                              | `█`            |
| `Shade75`      | `Shade75`                                       | `▓`            |
| `Shade50`      | `Shade50`                                       | `▒`            |
| `Shade25`      | `Shade25`                                       | `░`            |
| `Braille`      | `Braille`                                       | `⣿`            |
| `Checkerboard` | `Checkerboard` or `checkerboard`                | `▚`            |
| `Grid`         | `Grid` or `grid`                                | `┼`            |
| `GridDouble`   | `GridDouble` or `grid-double`                   | `╬`            |
| `CrossHatch`   | `CrossHatch` or `cross-hatch`                   | `╳`            |
| `Dashed`       | `Dashed`                                        | `┇`            |
| `DiagonalUp`   | `DiagonalUp` or `diagonal-up`                   | `╱`            |
| `DiagonalDown` | `DiagonalDown` or `diagonal-down`               | `╲`            |
| `Notched`      | `Notched`                                       | `■`            |
| `Custom(char)` | `Custom('#')`, `Custom(code: 0x2588)`, or `'#'` | that character |

## Order

`sort` (or `set_sort_type`) sets the left-to-right order. The values are available from code as `usagechart::SortType`. Lengths are still given to the largest values first, so a segment that does not fit is omitted whatever this order says. A segment's selection index is its position after this sort.

| Order                 | Macro form                                      | Description                                                     |
| --------------------- | ----------------------------------------------- | --------------------------------------------------------------- |
| `None`                | `None` or `none`                                | The order in which the segments were added. This is the default |
| `HighestToLowest`     | `HighestToLowest` or `highest-to-lowest`        | Largest value first                                             |
| `LowestToHighest`     | `LowestToHighest` or `lowest-to-highest`        | Smallest value first                                            |
| `Alphabetical`        | `Alphabetical` or `alpha`                       | Usual string order. An empty label comes first                  |
| `ReverseAlphabetical` | `ReverseAlphabetical` or `reverse-alphabetical` | The reverse of `Alphabetical`                                   |

## Total

`total` (or `set_total_usage`) sets what the full width of the bar represents. The values are available from code as `usagechart::TotalUsage`.

| Value      | Macro form                               | Description                                                                                |
| ---------- | ---------------------------------------- | ------------------------------------------------------------------------------------------ |
| `Auto`     | `Auto` or `auto`                         | The total is the sum of the segment values, so they fill the bar. This is the default      |
| `Fixed(n)` | `Fixed(n)` or `fix(n)` or the number `n` | The total is `n`. The unused part of the bar stays empty. A total of `0` draws no segments |

```rs
let drive = usagechart!("d:f,total:256,segments:[{48,'OS'},{36,'Apps'}]");
chart.set_total_usage(usagechart::TotalUsage::Fixed(256));
```

## Tooltip

`content` (or `set_label_content`) chooses what the hover tooltip shows. The values are available from code as `usagechart::LabelContent`. Read the current choice with `label_content`.

| Value                | Macro form                                                           | Shows                               |
| -------------------- | -------------------------------------------------------------------- | ----------------------------------- |
| `LabelAndValue`      | `LabelAndValue` or `label-and-value` or `label+value`                | `label: value`. This is the default |
| `Value`              | `Value`                                                              | The value of the segment            |
| `Label`              | `Label`                                                              | The label of the segment            |
| `Percentage`         | `Percentage` or `percent`                                            | The share of the total (`18.75%`)   |
| `LabelAndPercentage` | `LabelAndPercentage` or `label-and-percentage` or `label+percentage` | `label: percentage`                 |

With `LabelAndValue` and `LabelAndPercentage`, a segment with an empty label shows only its value or percentage. With `Label`, a segment with an empty label shows no tooltip. A percentage always has two decimals and a `%` suffix. It does not use the number format.

### Number format

`numeric-format` (or `set_number_format`) controls how values are written in the tooltip. The macro uses the same syntax as `numericformat!`. The default groups digits by thousands.

```rs
let chart = usagechart!("d:f,lc:LabelAndValue,nf:{dec,group:3,suffix:' GB'}");
chart.set_number_format(FormatNumber::new(10).group(3, b',').suffix(" GB"));
```

## Selection

A segment is selected by clicking it, with the keyboard, or with `set_selected_segment`. The selected segment is outlined with the theme selection border and, with `DimSegmentsOnSelection`, the other segments are dimmed. The outline stays when the chart loses the focus. Segments that take no space are skipped. A disabled chart is drawn with the inactive color and shows no selection.

The selection is cleared when the user clicks the empty part of the bar or presses `Escape`. `clear` also clears it. Losing the focus does not.

`index` is the segment's position after sorting. With `SortType::None` that is the order in which the segments were added.

## Events

To intercept events from a UsageChart, implement `UsageChartEvents` on the window that runs the event loop. The attribute `events = UsageChartEvents` generates the trait. Both methods have a default implementation that returns `EventProcessStatus::Ignored`.

```rs
pub trait UsageChartEvents {
    fn on_usage_segment_selected(&mut self, handle: Handle<UsageChart>, index: usize) -> EventProcessStatus {...}
    fn on_clear_selection(&mut self, handle: Handle<UsageChart>) -> EventProcessStatus {...}
}
```

`on_usage_segment_selected` runs when the user selects a segment (by clicking it, or with `Left` / `Right`). `index` is the position after sorting. `on_clear_selection` runs when the user clears the selection (by clicking the empty part of the bar, or with `Escape`). Selecting the segment that is already selected does not raise the event again.

The events are raised only for user actions. Changing the selection from code with `set_selected_segment`, or removing the segments with `clear`, does not raise them.

## Methods

Besides the [Common methods for all Controls](../common_methods.md), a UsageChart also has the following additional methods:

| Method                        | Purpose                                                                                                 |
| ----------------------------- | ------------------------------------------------------------------------------------------------------- |
| `add_segment(...)`            | Appends one segment and repaints. A number is a solid segment with an empty label                       |
| `add_segments(...)`           | Appends several segments and repaints once                                                              |
| `clear()`                     | Removes every segment, the selection and the tooltip, then repaints                                     |
| `set_flags(...)`              | Replaces the flag set and repaints. The same flags again do nothing                                     |
| `set_sort_type(...)`          | Sets the `SortType` and repaints                                                                        |
| `set_total_usage(...)`        | Sets the `TotalUsage` and repaints                                                                      |
| `set_label_content(...)`      | Sets the `LabelContent` of the tooltip                                                                  |
| `label_content()`             | Returns the `LabelContent` of the tooltip                                                               |
| `set_number_format(...)`      | Sets the `FormatNumber` used for values in the tooltip                                                  |
| `set_selected_segment(index)` | Selects the segment at `index`, or clears the selection when `index` is `None`. Does not raise an event |

## Key association

The following keys are processed by a UsageChart when it has focus. Segments that take no space are skipped, and the selection wraps around.

| Key      | Purpose                                                         |
| -------- | --------------------------------------------------------------- |
| `Right`  | Selects the next segment (the first one if none is selected)    |
| `Left`   | Selects the previous segment (the last one if none is selected) |
| `Escape` | Clears the selection                                            |

## Mouse association

| Action                             | Purpose                                                                                 |
| ---------------------------------- | --------------------------------------------------------------------------------------- |
| Hover                              | Shows the tooltip of the segment under the cursor. The text is chosen by `LabelContent` |
| Click on a segment                 | Selects that segment and raises `on_usage_segment_selected` when the selection changes  |
| Click on the empty part of the bar | Clears the selection and raises `on_clear_selection` when a segment was selected        |

## Example

The following window shows a 256 GB drive. The unused part of the bar stays empty. Hovering a segment shows its label and its share of 256 GB. Selecting a segment writes its index into the window title.

A small demo is in `examples/usagechart`. A larger editor (flags, sort, total, tooltip, number format and the segments themselves) is in `examples/usagechart_editor`.

```rs
use appcui::prelude::*;

#[Window(events = UsageChartEvents)]
struct MyWin {
    chart: Handle<UsageChart>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'Usage chart',a:c,w:70,h:10,flags:Sizeable"),
            chart: Handle::None,
        };
        win.chart = win.add(usagechart!(
            "d:f,
             flags: Margins+DimSegmentsOnSelection,
             total: Fixed(256),
             nf: {dec, suffix: ' GB'},
             lc: LabelAndPercentage,
             segments: [
                {48, label: 'OS', fill: Solid},
                {36, label: 'Apps', fill: CrossHatch},
                {22, label: 'Documents', fill: Grid},
                {22, label: 'Media', fill: DiagonalDown},
                {8, label: 'Cache', fill: GridDouble}
             ]"
        ));
        win
    }
}

impl UsageChartEvents for MyWin {
    fn on_usage_segment_selected(&mut self, _: Handle<UsageChart>, index: usize) -> EventProcessStatus {
        self.set_title(&format!("Usage chart - segment {index}"));
        EventProcessStatus::Processed
    }

    fn on_clear_selection(&mut self, _: Handle<UsageChart>) -> EventProcessStatus {
        self.set_title("Usage chart");
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().window(MyWin::new).run()
}
```
