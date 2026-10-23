# LineChart

A `LineChart` plots one or more numeric series over a shared X axis. Every series is a list of points, drawn with a marker and connected by a line. The value type `T` is any numeric type that implements `Number`: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `isize`, `f32`, or `f64`.

All series share the same columns: point `i` of every series is drawn at X position `i`, so a chart with several series compares their values at each position. The X range is as long as the longest series; shorter series simply end earlier. The vertical scale is computed from the smallest and largest values across all series.

Create one with `LineChart::new` (a layout and initialization flags) or with the `linechart!` macro. With `LineChart::new`, name the numeric type on the variable or with a turbofish:

```rs
let chart: LineChart<i32> = LineChart::new(layout!("d:f"), linechart::Flags::ScrollBars);
let chart = LineChart::<f64>::new(layout!("d:f"), linechart::Flags::None);
```

The macro takes the numeric type as its first parameter:

```rs
let c1 = linechart!("i32,d:f");
let c2 = linechart!("class:i32,x:1,y:1,w:60,h:20,flags:[ScrollBars,GroupHover]");
let c3 = linechart!("f64,d:f,scale:fit,series:[{Temp,values:[21.5,23.0,19.75]}]");
```

A new chart has no series. It scales points with `BarScale::FromZero`, shows the Y axis and its grid (6 characters wide, a label every 3 rows), and draws no X axis. Points are drawn with a small bullet marker (●), connected with a single line, and separated by 2 empty cells. Series without their own color use the theme chart colors, in order. Integer values are formatted with thousands separators. Floating-point values are formatted with two decimals.

A LineChart supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name                                         | Type           | Positional parameter | Purpose                                                             |
| ------------------------------------------------------ | -------------- | -------------------- | ------------------------------------------------------------------- |
| `class` or `type`                                      | String         | **Yes** (first)      | Numeric type of the values (`i32`, `f64`, ...)                      |
| `flags`                                                | Flags          | **No**               | Initialization flags                                                |
| `series`                                               | List           | **No**               | Series to add. Each item is a series description                    |
| `labels` or `categories` or `xaxis-labels`             | List           | **No**               | Names of the X positions                                            |
| `scale` or `barscale`                                  | String         | **No**               | How values map onto the plot height                                 |
| `line` or `default-line` or `line-mode` or `lm`        | String         | **No**               | Default way to connect points                                       |
| `marker` or `default-marker` or `dmk`                  | String         | **No**               | Default point marker                                                |
| `spacing` or `default-spacing` or `space` or `s`       | Integer        | **No**               | Empty cells between two consecutive points, from 0 to 100           |
| `xlabels` or `x-labels` or `xl` or `xaxis` or `x-axis` | String or List | **No**               | What is drawn under the X axis                                      |
| `numeric-format` or `nf`                               | Dict           | **No**               | Format used for Y-axis labels and hover tooltips                    |
| `yaxis` or `y-axis` or `yaxis-visible`                 | Bool           | **No**               | Shows or hides the Y axis                                           |
| `grid` or `ygrid` or `yaxis-grid`                      | Bool           | **No**               | Shows or hides the horizontal grid lines                            |
| `yaxis-width` or `y-axis-width` or `yw`                | Integer        | **No**               | Characters reserved for Y-axis labels (0 to 32; 0 is treated as 1)  |
| `yaxis-step` or `y-axis-step` or `ystep` or `step`     | Integer        | **No**               | Rows between horizontal grid lines (1 to 255)                       |
| `lsm` or `left-scroll-margin`                          | Integer        | **No**               | Left margin of the horizontal scroll bar                            |

## Flags

* `linechart::Flags::ScrollBars` or `ScrollBars` (for macro initialization) — shows a horizontal scroll bar when the points are wider than the control. The bottom margin grows while the chart has focus so the scroll bar stays visible.
* `linechart::Flags::ShowZeroLineOnYAxis` or `ShowZeroLineOnYAxis` (for macro initialization) — draws a solid horizontal line at value zero while the Y-axis grid is visible.
* `linechart::Flags::DimPointsOnSelection` or `DimPointsOnSelection` (for macro initialization) — while a point is selected, everything except that point is drawn with the inactive chart color.
* `linechart::Flags::GroupHover` or `GroupHover` (for macro initialization) — hovering acts on a whole X position instead of a single point: the column under the mouse is highlighted with the theme hover color, and the tooltip shows the value of every series at that position.

```rs
let chart = linechart!("i32,d:f,flags:[ScrollBars,DimPointsOnSelection,GroupHover]");
```

## Series

A series has a label, an optional line mode, an optional color, and a list of points. A series that does not set its own line mode uses the chart default, and a series without its own color uses the next theme chart color.

Data is always added to a series by its index, so a series must be added before its values. `add_series` returns the index of the new series:

```rs
let mut chart = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
let cpu = chart.add_series(linechart::Series::new("CPU"));
let ram = chart.add_series(
    linechart::SeriesBuilder::new("RAM")
        .line(linechart::LineMode::Braille)
        .attr(charattr!("yellow"))
        .values([20, 25, 30]),
);
chart.add_values(cpu, [10, 40, 25]);
chart.add_value(ram, 28);
```

In the `series` list of the macro, a dictionary describes a series. All fields are optional.

| Parameter name                                   | Type   | Positional parameter | Purpose                                                   |
| ------------------------------------------------ | ------ | -------------------- | --------------------------------------------------------- |
| `label` or `name` or `caption` or `text`         | String | **Yes** (first)      | Name of the series, shown in tooltips                     |
| `values` or `data` or `points`                   | List   | **No**               | Points of the series. Each item is a number or a point    |
| `line` or `line-mode` or `linemode` or `lm`      | String | **No**               | How the points of this series are connected               |
| `attr` or `attribute` or `charattr` or `color`   | Dict   | **No**               | Color of the series (line and markers)                    |

```rs
let chart = linechart!(
    "i32,d:f,
     series:[
        {CPU, values:[35,42,38,55], attr:green},
        {RAM, values:[20,22,25,24], line:Braille, attr:yellow}
     ]"
);
```

## Points

A point is a `Bar<T>`, the same type used by [VBarChart](vbarchart.md) and [HBarChart](hbarchart.md). A value of type `T` converts into a point, so `chart.add_value(0, 10)` and a bare number in `values` are enough for a default point. A point can also override:

* its **label** — a note about that value only (for example `"peak"`), shown in the point's tooltip. It is not drawn under the X axis; the names of the X positions belong to the chart (see [X-axis labels](#x-axis-labels)).
* its **color** (`attr`) — the color of its marker.
* its **marker**, through its draw mode: `BarDrawMode::Point(...)` draws a small marker and `BarDrawMode::FramedPoint(...)` a framed one. Any other draw mode is ignored and the chart default marker is used.

The thickness and spacing of a point are ignored: all series share the same columns.

```rs
chart.add_value(0, BarBuilder::new(80).label("peak").draw_mode(BarDrawMode::FramedPoint(BarFramedPointType::RoundSquareBullet)).build());
```

In the macro, a point is described with the same dictionary as a [VBarChart](vbarchart.md#bars) bar:

```rs
let chart = linechart!("i32,d:f,series:[{CPU,values:[35,42,{80,label:peak,dm:FramedPoint(RoundSquareBullet),attr:red},38]}]");
```

## Line modes

`linechart::LineMode` chooses how consecutive points of a series are connected. Lines are drawn first and markers on top of them. The default is `Line(Single)`.

| Mode                           | Macro form                                                     | Description                                                     |
| ------------------------------ | -------------------------------------------------------------- | --------------------------------------------------------------- |
| `None`                         | `None`                                                         | Points only, no line                                            |
| `Braille`                      | `Braille`                                                      | A thin line drawn with braille dots                             |
| `Line(line)`                   | `Line`, `Line(Single)`, `Line(Double)`                         | A direct line drawn with box characters                         |
| `Orthogonal(line, direction)`  | `Orthogonal`, `Orthogonal(Single)`, `Orthogonal(Double, Auto)` | Only horizontal and vertical segments                           |

`Line` and `Orthogonal` take a `LineType`: `Single`, `Double`, `SingleThick` (`thick`), `Border`, `Ascii`, `AsciiRound`, `SingleRound` (`round`), or `Braille`. `Orthogonal` also takes an `OrthogonalDirection`: `Auto` (the default), `HorizontalFirst`, `VerticalFirst`, `HorizontalUntilMiddle`, or `VerticalUntilMiddle`.

```rs
chart.set_default_line(linechart::LineMode::Orthogonal(LineType::Double, OrthogonalDirection::HorizontalFirst));
chart.update_series(1, |s| {
    s.set_line(Some(linechart::LineMode::Braille));
});
```

## Markers

`linechart::Marker` chooses what is drawn at every point. The default is `Small(Bullet)`.

| Marker          | Macro form                                                       | Size  |
| --------------- | ---------------------------------------------------------------- | ----- |
| `None`          | `None`                                                           | 1 x 1 |
| `Small(kind)`   | `Small(Bullet)` or `Point(Bullet)`, `Small(Diamond)`, `Small('*')` | 1 x 1 |
| `Framed(kind)`  | `Framed` or `FramedPoint`, `Framed(RoundSquareBullet)`, `Framed('#')` | 5 x 3 |

`Small` accepts `Bullet` (●, the default), `Diamond` (◆), `Square` (■), or a custom character.

`Framed` accepts:
* `RoundSquare` (the default), `Square`, `DoubleLineSquare`, `ThickSquare` — an empty frame;
* `RoundSquareBullet`, `SquareBullet`, `DoubleLineSquareBullet`, `ThickSquareBullet` — the same frames with a bullet in the middle;
* `BrailleCircle`, `BrailleCircleBullet`, `BrailleFilledCircle` — a circle drawn with braille dots;
* `Filled`, or a custom character that fills the whole marker.

Every column is as wide as the widest marker in the chart, so points of different series stay aligned. The middle row of a framed marker is on the value, and the plot keeps one free row at the top and at the bottom so a framed marker at the minimum or maximum is not cut. Lines are attached to the middle of the left and right sides of a framed marker; where a box-drawing line touches a box-drawing frame, the frame gets a junction (`─┤ ● ├─`).

```rs
chart.set_default_marker(linechart::Marker::Framed(BarFramedPointType::BrailleCircle));
let chart = linechart!("i32,d:f,marker:Framed(DoubleLineSquareBullet),line:Line(Double)");
```

## Scale

`scale` chooses how values become rows. The same values are available from `set_scale` as `linechart::BarScale<T>`. The range covers the values of every series.

* `FromZero` or `zero` — the visible range includes zero and every value. This is the default.
* `FromZeroMinRange(min, max)` or `zero-min-range(min, max)` — same as `FromZero`, and the range is also expanded so that it covers `min` and `max`.
* `FitData` or `fit` — the smallest and largest values stretch across the full plot height.
* `Fixed(min, max)` or `fix(min, max)` — uses a fixed range. Values outside it are drawn at the corresponding edge of the plot.

```rs
let fit = linechart!("f64,d:f,scale:fit");
let fixed = linechart!("i32,d:f,scale:Fixed(0,100)");
```

## X-axis labels

The names of the X positions (for example months) belong to the chart: label `i` names the points at index `i` of every series. They are shown in the hover tooltips, and under the X axis when the label mode is `Labels`.

```rs
chart.set_xaxis_labels(["Jan", "Feb", "Mar"]);
chart.add_xaxis_label("Apr");
chart.set_xaxis_label_mode(linechart::XAxisLabelMode::Labels);
```

In the macro, `labels` sets the names. When `labels` is given without `xlabels`, the label mode is set to `Labels` as well:

```rs
let chart = linechart!("i32,d:f,labels:[Jan,Feb,Mar],series:[{CPU,values:[35,42,38]}]");
```

`xlabels` chooses what is drawn under the X axis. The same modes are available from `set_xaxis_label_mode` as `linechart::XAxisLabelMode`.

* `None` — draws no X axis. This is the default.
* `Index(start)` — labels each position with `start + index`. `Index(1)` labels the first position `1`, the next `2`, and so on.
* `Labels` — uses the names set with `labels` / `set_xaxis_labels`. Positions without a name (or with an empty one) are skipped.
* A list of spans — each span covers a run of positions. A span is `{start, count, label}`, exactly as for [VBarChart](vbarchart.md#x-axis-labels).

```rs
let by_index = linechart!("i32,d:f,xlabels:Index(2020)");
let by_span = linechart!("i32,d:f,xlabels:[{0,3,'Q1'},{3,3,'Q2'}]");
```

### Number format

`numeric-format` (or `set_number_format`) controls the text used for Y-axis labels, the zero-line label, and hover tooltips, with the same syntax as for [VBarChart](vbarchart.md#number-format).

```rs
let chart = linechart!("f64,d:f,nf:{dec,decimals:1}");
```

## Hover and selection

Hovering a point shows a tooltip with the series name, the name of the X position and the value, and the point's note when it has one. With `GroupHover`, the tooltip lists the value of every series at the hovered X position instead.

Clicking a point selects it; the selected point is outlined with the theme selection border. Clicking away from every point clears the selection. When markers overlap, the one drawn on top (the last series) is selected. From code, `select_point` and `clear_selection` change the selection without raising events.

## Events

To intercept events from a LineChart, implement `LineChartEvents<T>` on the window that runs the event loop. The attribute `events = LineChartEvents<T>` generates the trait. Both methods are required.

```rs
pub trait LineChartEvents<T> {
    fn on_point_selected(&mut self, handle: Handle<LineChart<T>>, series_index: u32, point_index: u32) -> EventProcessStatus;
    fn on_clear_selection(&mut self, handle: Handle<LineChart<T>>) -> EventProcessStatus;
}
```

`on_point_selected` runs when the user selects a point that was not already selected, with a click or with the keyboard. `series_index` and `point_index` are zero-based. `on_clear_selection` runs when the user clears the selection, with a click outside every point or with `Escape`.

## Methods

Besides the [Common methods for all Controls](../common_methods.md), a LineChart also has the following additional methods:

| Method                             | Purpose                                                                                                                       |
| ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `add_series(...)`                  | Appends a `Series` or a `SeriesBuilder` and returns its index                                                                 |
| `series_count()`                   | Returns the number of series                                                                                                  |
| `points_count()`                   | Returns the number of X positions (the length of the longest series)                                                          |
| `get_series(index)`                | Returns `Some(&Series<T>)`, or `None` if `index` is out of range                                                              |
| `update_series(index, f)`          | Mutates one series, then relayouts and repaints. Does nothing if `index` is out of range                                      |
| `update_all_series(f)`             | Calls `f(index, series)` for every series, then relayouts and repaints once                                                   |
| `remove_series(index)`             | Removes a series and returns it, or `None` if out of range                                                                    |
| `add_value(series, ...)`           | Appends one point to a series. The argument is a value of type `T` or any type that converts into a `Bar`                     |
| `add_values(series, ...)`          | Appends several points to a series and repaints once                                                                          |
| `set_xaxis_labels(...)`            | Replaces the names of the X positions                                                                                         |
| `add_xaxis_label(...)`             | Appends the name of one X position                                                                                            |
| `add_xaxis_labels(...)`            | Appends the names of several X positions                                                                                      |
| `xaxis_label(index)`               | Returns the name of an X position, or `None`                                                                                  |
| `xaxis_labels_count()`             | Returns the number of names                                                                                                   |
| `remove_first_xaxis_labels(count)` | Removes the first `count` names                                                                                               |
| `clear_xaxis_labels()`             | Removes every name                                                                                                            |
| `set_xaxis_label_mode(...)`        | Sets what is drawn under the X axis                                                                                           |
| `default_line()` / `set_default_line(...)`       | Line mode of series that do not set their own                                                                   |
| `default_marker()` / `set_default_marker(...)`   | Marker of points that do not set their own                                                                      |
| `default_spacing()` / `set_default_spacing(...)` | Empty cells between two consecutive points                                                                      |
| `set_scale(...)`                   | Sets the `BarScale` used to map values onto the plot height                                                                   |
| `set_number_format(...)`           | Sets the `FormatNumber` used for axis labels and tooltips                                                                     |
| `set_yaxis_visible(...)`           | Shows or hides the Y axis and the column reserved for its labels                                                              |
| `set_yaxis_show_grid(...)`         | Shows or hides the horizontal grid lines and the numeric labels beside them                                                   |
| `set_yaxis_width(...)`             | Sets how many characters are reserved for Y-axis labels. Values below 1 are treated as 1                                      |
| `set_yaxis_step(...)`              | Sets the distance, in rows, between horizontal grid lines. Values below 1 are treated as 1                                    |
| `ensure_visible(index)`            | Scrolls horizontally so the points at X position `index` are visible                                                          |
| `selected_point()`                 | Returns the selected point as `(series_index, point_index)`, or `None`                                                        |
| `select_point(series, index)`      | Selects a point and scrolls it into view (no event is raised). Clears the selection if the point does not exist               |
| `clear_selection()`                | Clears the selection (no event is raised)                                                                                     |

### `Series`

On a series returned by `get_series` or passed into `update_series`:

| Method                                       | Purpose                                                                  |
| -------------------------------------------- | ------------------------------------------------------------------------ |
| `label()` / `set_label(...)`                 | Name of the series                                                       |
| `line()` / `set_line(...)`                   | Line mode of the series, or `None` to use the chart default              |
| `attr()` / `set_attr(...)`                   | Color of the series, or `None` to use the theme color                    |
| `len()` / `is_empty()`                       | Number of points                                                         |
| `get(index)` / `get_mut(index)`              | Borrow one point, or `None` if out of range                              |
| `add(...)` / `add_values(...)`               | Append one point or several points                                       |
| `insert(index, point)`                       | Inserts at `index`. Returns `false` if `index` is greater than `len()`   |
| `set(index, point)`                          | Replaces the point and returns the previous one, or `None` if out of range |
| `delete(index)`                              | Removes the point and returns it, or `None` if out of range              |
| `remove_first(count)`                        | Removes the first `count` points                                         |
| `clear()`                                    | Removes every point                                                      |
| `iter()` / `iter_mut()`                      | Walk the points                                                          |

To keep a sliding window over live data, append the new values and their X names, then drop the oldest ones from every series and from the names:

```rs
chart.update_all_series(|_, s| s.remove_first(1));
chart.remove_first_xaxis_labels(1);
```

The selection is not moved when points are removed this way: clear or move it with `clear_selection` / `select_point` if needed.

## Key association

The following keys are processed by a LineChart when it has focus.

| Key                        | Purpose                                                                                                       |
| -------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `Left` / `Right`           | With a selected point, select the previous or next point of the same series. Otherwise, scroll by one cell     |
| `Up` / `Down`              | With a selected point, select the point drawn above or below it at the same X position                         |
| `Escape`                   | Clears the selection                                                                                          |
| `Home`                     | Scroll to the start of the chart                                                                              |
| `End`                      | Scroll to the end of the chart                                                                                |
| `Ctrl+Left` / `Ctrl+Right` | Make the previous or next X position the first visible one                                                    |

## Mouse association

| Action                                | Purpose                                                                                                              |
| ------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| Hover                                 | Shows a tooltip over the point under the cursor (or, with `GroupHover`, highlights the X position under the cursor)   |
| Click or double-click on a point      | Selects that point and raises `on_point_selected` when the selection changes                                         |
| Click or double-click outside a point | Clears the selection and raises `on_clear_selection` when a point was selected                                       |
| Wheel left or wheel up                | Scrolls one cell toward the start of the chart                                                                       |
| Wheel right or wheel down             | Scrolls one cell toward the end of the chart                                                                         |
| Drag the scroll bar                   | Scrolls the chart when `ScrollBars` is enabled                                                                       |

## Example

The following window shows three series over twelve months. Selecting a point writes its series, month and value into the window title. Clearing the selection restores the original title.

A larger editor (series, per-point appearance, axes, scale and markers) is in `examples/linechart_editor`.

```rs
use appcui::prelude::*;

#[Window(events = LineChartEvents<i32>)]
struct MyWin {
    chart: Handle<LineChart<i32>>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("Usage,d:f"),
            chart: Handle::None,
        };
        win.chart = win.add(linechart!(
            "type: i32, d:f, flags: ScrollBars+GroupHover, spacing: 3,
            labels: [Jan, Feb, Mar, Apr, May, Jun, Jul, Aug, Sep, Oct, Nov, Dec],
            series: [
                {CPU, line: Braille, attr: green,
                 values: [35, 42, 38, 55, 61, 48, 72, {80, label: peak}, 66, 58, 63, 70]},
                {RAM, line: Line(Single), attr: yellow,
                 values: [20, 22, 25, 24, 30, 33, 31, 36, 40, 38, 41, 45]},
                {Disk, line: Orthogonal(Single, HorizontalFirst), attr: aqua,
                 values: [5, 5, 10, 10, 10, 15, 15, 20, 20, 25, 25, 25]}
            ]"
        ));
        win
    }
}

impl LineChartEvents<i32> for MyWin {
    fn on_point_selected(&mut self, handle: Handle<LineChart<i32>>, series_index: u32, point_index: u32) -> EventProcessStatus {
        let mut title = String::from("Usage");
        if let Some(chart) = self.control(handle) {
            if let Some(series) = chart.get_series(series_index as usize) {
                if let Some(point) = series.get(point_index as usize) {
                    let month = chart.xaxis_label(point_index as usize).unwrap_or("");
                    title = format!("Usage - {} {} = {}", series.label(), month, point.value());
                }
            }
        }
        self.set_title(&title);
        EventProcessStatus::Processed
    }

    fn on_clear_selection(&mut self, _: Handle<LineChart<i32>>) -> EventProcessStatus {
        self.set_title("Usage");
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().window(MyWin::new).run()
}
```
