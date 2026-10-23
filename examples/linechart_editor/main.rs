use appcui::prelude::*;

mod group_window;

use group_window::{GroupWindow, XAxisGroup};

const DEFAULT_SPACING: u8 = 3;
const DEFAULT_MARK: char = '*';
const DEFAULT_POINT_COLOR: Color = Color::White;

const MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
const SERIES_TEXT: &str = "CPU: 35, 42, 38, 55, 61, 48, 72, 80, 66, 58, 63, 70\nRAM: 20, 22, 25, 24, 30, 33, 31, 36, 40, 38, 41, 45\nDisk: 5, 5, 10, 10, 10, 15, 15, 20, 20, 25, 25, 25";

// ---------------------------------------------------------------------------------------------
// Line mode controls (used for the chart default and for the selected series)
// ---------------------------------------------------------------------------------------------

/// Controls that describe a [`linechart::LineMode`]. With `has_default`, the first entry of the
/// mode combo is "Use chart default" (for a series).
#[derive(Copy, Clone)]
struct LineControls {
    mode: Handle<ComboBox>,
    line_type_label: Handle<Label>,
    line_type: Handle<Selector<LineType>>,
    direction_label: Handle<Label>,
    direction: Handle<ComboBox>,
    has_default: bool,
}

impl LineControls {
    fn none() -> Self {
        Self {
            mode: Handle::None,
            line_type_label: Handle::None,
            line_type: Handle::None,
            direction_label: Handle::None,
            direction: Handle::None,
            has_default: false,
        }
    }
}

/// Controls that describe a [`linechart::Marker`]. With `has_default`, the first entry of the
/// marker combo is "Use default" (for a point) and "None" is not available.
#[derive(Copy, Clone)]
struct MarkerControls {
    kind: Handle<ComboBox>,
    detail_label: Handle<Label>,
    point: Handle<ComboBox>,
    large_point: Handle<ComboBox>,
    character_label: Handle<Label>,
    character: Handle<CharPicker>,
    has_default: bool,
}

impl MarkerControls {
    fn none() -> Self {
        Self {
            kind: Handle::None,
            detail_label: Handle::None,
            point: Handle::None,
            large_point: Handle::None,
            character_label: Handle::None,
            character: Handle::None,
            has_default: false,
        }
    }
}

trait ControlHost {
    fn add_control<T>(&mut self, control: T) -> Handle<T>
    where
        T: Control + NotWindow + NotDesktop + 'static;
}

impl ControlHost for Panel {
    fn add_control<T>(&mut self, control: T) -> Handle<T>
    where
        T: Control + NotWindow + NotDesktop + 'static,
    {
        self.add(control)
    }
}

struct PageHost<'a> {
    pages: &'a mut Accordion,
    index: u32,
}

impl ControlHost for PageHost<'_> {
    fn add_control<T>(&mut self, control: T) -> Handle<T>
    where
        T: Control + NotWindow + NotDesktop + 'static,
    {
        self.pages.add(self.index, control)
    }
}

fn row(top: i32, width: Option<u16>) -> Layout {
    let mut layout = LayoutBuilder::new().left_anchor(1).top_anchor(top).height(1);
    layout = match width {
        Some(width) => layout.width(width),
        None => layout.right_anchor(1),
    };
    layout.build()
}

fn combo_with(host: &mut impl ControlHost, top: i32, items: &[&str], visible: bool) -> Handle<ComboBox> {
    let mut combo = ComboBox::new(row(top, None), combobox::Flags::None);
    for item in items {
        combo.add(item);
    }
    combo.set_index(0);
    combo.set_visible(visible);
    host.add_control(combo)
}

fn label_with(host: &mut impl ControlHost, top: i32, caption: &str, visible: bool) -> Handle<Label> {
    let mut label = Label::new(caption, row(top, Some(20)));
    label.set_visible(visible);
    host.add_control(label)
}

const DIRECTIONS: [OrthogonalDirection; 5] = [
    OrthogonalDirection::Auto,
    OrthogonalDirection::HorizontalFirst,
    OrthogonalDirection::VerticalFirst,
    OrthogonalDirection::HorizontalUntilMiddle,
    OrthogonalDirection::VerticalUntilMiddle,
];

/// Adds the line mode controls (5 rows starting at `top`).
fn add_line_controls(host: &mut impl ControlHost, top: i32, has_default: bool) -> LineControls {
    host.add_control(Label::new("Line", row(top, Some(12))));
    let modes: &[&str] = if has_default {
        &["Use chart default", "None (scatter)", "Braille", "Line", "Orthogonal (steps)"]
    } else {
        &["None (scatter)", "Braille", "Line", "Orthogonal (steps)"]
    };
    let mode = combo_with(host, top + 1, modes, true);
    let line_type_label = label_with(host, top + 2, "Line type", false);
    let mut line_type = Selector::new(Some(LineType::Single), row(top + 3, None), selector::Flags::None);
    line_type.set_visible(false);
    let line_type = host.add_control(line_type);
    let direction_label = label_with(host, top + 4, "Direction", false);
    let direction = combo_with(
        host,
        top + 5,
        &["Auto", "Horizontal first", "Vertical first", "Horizontal until middle", "Vertical until middle"],
        false,
    );
    LineControls {
        mode,
        line_type_label,
        line_type,
        direction_label,
        direction,
        has_default,
    }
}

const POINT_CUSTOM: u32 = 3;
const LARGE_POINT_CUSTOM: u32 = 6;

fn point_type(index: u32, mark: char) -> BarPointType {
    match index {
        1 => BarPointType::Diamond,
        2 => BarPointType::Square,
        POINT_CUSTOM => BarPointType::Custom(mark),
        _ => BarPointType::Bullet,
    }
}

fn large_point_type(index: u32, mark: char) -> BarLargePointType {
    match index {
        1 => BarLargePointType::Square,
        2 => BarLargePointType::DoubleLineSquare,
        3 => BarLargePointType::ThickSquare,
        4 => BarLargePointType::Circle,
        5 => BarLargePointType::Diamond,
        LARGE_POINT_CUSTOM => BarLargePointType::Custom(mark),
        _ => BarLargePointType::RoundSquare,
    }
}

/// Adds the marker controls (6 rows starting at `top`).
fn add_marker_controls(host: &mut impl ControlHost, top: i32, has_default: bool) -> MarkerControls {
    host.add_control(Label::new("Marker", row(top, Some(12))));
    let kinds: &[&str] = if has_default {
        &["Use chart default", "Small", "Large"]
    } else {
        &["None (line only)", "Small", "Large"]
    };
    let kind = combo_with(host, top + 1, kinds, true);
    let detail_label = label_with(host, top + 2, "Marker type", false);
    let point = combo_with(host, top + 3, &["Bullet", "Diamond", "Square", "Custom"], false);
    let large_point = combo_with(
        host,
        top + 3,
        &["Round square", "Square", "Double line square", "Thick square", "Circle", "Diamond", "Custom"],
        false,
    );
    let character_label = label_with(host, top + 4, "Character", false);
    let mut character = CharPicker::new(Some(DEFAULT_MARK), row(top + 5, None));
    character.select_char(DEFAULT_MARK);
    character.set_visible(false);
    let character = host.add_control(character);
    MarkerControls {
        kind,
        detail_label,
        point,
        large_point,
        character_label,
        character,
        has_default,
    }
}

// ---------------------------------------------------------------------------------------------
// Data helpers
// ---------------------------------------------------------------------------------------------

fn parse_values(text: &str) -> Option<Vec<i32>> {
    let mut values = Vec::new();
    for part in text.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        values.push(part.parse::<i32>().ok()?);
    }
    Some(values)
}

/// Parses one series per line: `Name: v1, v2, ...` (the name is optional).
fn parse_series(text: &str) -> Result<Vec<(String, Vec<i32>)>, usize> {
    let mut result = Vec::new();
    for (line_no, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (name, values) = match line.split_once(':') {
            Some((name, values)) => (name.trim().to_string(), values),
            None => (String::new(), line),
        };
        let values = parse_values(values).ok_or(line_no + 1)?;
        result.push((name, values));
    }
    Ok(result)
}

fn parse_labels(text: &str) -> Vec<String> {
    text.split(',').map(|l| l.trim().to_string()).collect()
}

fn default_groups() -> Vec<XAxisGroup> {
    [("Q1", 0, 3), ("Q2", 3, 3), ("Q3", 6, 3), ("Q4", 9, 3)]
        .into_iter()
        .map(|(label, start, count)| XAxisGroup {
            label: label.to_string(),
            start,
            count,
        })
        .collect()
}

fn spans(groups: &[XAxisGroup]) -> Vec<BarSpan> {
    groups.iter().map(|g| BarSpan::new(g.start, g.count.max(1), &g.label)).collect()
}

// ---------------------------------------------------------------------------------------------
// Editor window
// ---------------------------------------------------------------------------------------------

#[Window(events = [LineChartEvents<i32>, HSliderEvents<u8>, ColorPickerEvents, ComboBoxEvents, CharPickerEvents, CheckBoxEvents, SelectorEvents<LineType>, NumericSelectorEvents<i32>, ButtonEvents, ListViewEvents<XAxisGroup>])]
struct LineChartEditor {
    chart: Handle<LineChart<i32>>,
    pages: Handle<Accordion>,
    // chart settings
    scale_type: Handle<ComboBox>,
    scale_min_label: Handle<Label>,
    scale_max_label: Handle<Label>,
    scale_min: Handle<NumericSelector<i32>>,
    scale_max: Handle<NumericSelector<i32>>,
    xaxis_mode: Handle<ComboBox>,
    yaxis_width_enabled: Handle<CheckBox>,
    yaxis_width: Handle<HSlider<u8>>,
    yaxis_step_enabled: Handle<CheckBox>,
    yaxis_step: Handle<HSlider<u8>>,
    number_base: Handle<ComboBox>,
    number_group: Handle<ComboBox>,
    number_decimals: Handle<HSlider<u8>>,
    number_style: Handle<ComboBox>,
    // defaults
    default_spacing: Handle<HSlider<u8>>,
    default_line: LineControls,
    default_marker: MarkerControls,
    // series
    series_list: Handle<ComboBox>,
    series_custom_color: Handle<CheckBox>,
    series_color: Handle<ColorPicker>,
    series_line: LineControls,
    // selected point
    point_empty: Handle<Panel>,
    point_editor: Handle<Panel>,
    point_info: Handle<Label>,
    point_value: Handle<NumericSelector<i32>>,
    point_custom_color: Handle<CheckBox>,
    point_color: Handle<ColorPicker>,
    point_marker: MarkerControls,
    point_note: Handle<TextField>,
    point_note_set: Handle<Button>,
    // data
    data_labels: Handle<TextField>,
    data_series: Handle<TextArea>,
    groups: Handle<ListView<XAxisGroup>>,
    groups_add: Handle<Button>,
    groups_delete: Handle<Button>,
    groups_clear: Handle<Button>,
    data_apply: Handle<Button>,
}

impl LineChartEditor {
    fn new() -> Self {
        let mut win = Self {
            base: window!("'Line chart editor',d:f"),
            chart: Handle::None,
            pages: Handle::None,
            scale_type: Handle::None,
            scale_min_label: Handle::None,
            scale_max_label: Handle::None,
            scale_min: Handle::None,
            scale_max: Handle::None,
            xaxis_mode: Handle::None,
            yaxis_width_enabled: Handle::None,
            yaxis_width: Handle::None,
            yaxis_step_enabled: Handle::None,
            yaxis_step: Handle::None,
            number_base: Handle::None,
            number_group: Handle::None,
            number_decimals: Handle::None,
            number_style: Handle::None,
            default_spacing: Handle::None,
            default_line: LineControls::none(),
            default_marker: MarkerControls::none(),
            series_list: Handle::None,
            series_custom_color: Handle::None,
            series_color: Handle::None,
            series_line: LineControls::none(),
            point_empty: Handle::None,
            point_editor: Handle::None,
            point_info: Handle::None,
            point_value: Handle::None,
            point_custom_color: Handle::None,
            point_color: Handle::None,
            point_marker: MarkerControls::none(),
            point_note: Handle::None,
            point_note_set: Handle::None,
            data_labels: Handle::None,
            data_series: Handle::None,
            groups: Handle::None,
            groups_add: Handle::None,
            groups_delete: Handle::None,
            groups_clear: Handle::None,
            data_apply: Handle::None,
        };

        let mut splitter = vsplitter!("pos:70%,d:f,resize:PreserveRightPanelSize,min-left-width:30,min-right-width:36");

        // ---------------------------------------------------------------- the chart
        let mut chart = LineChart::<i32>::new(
            layout!("d:f"),
            linechart::Flags::ScrollBars | linechart::Flags::DimPointsOnSelection | linechart::Flags::ShowZeroLineOnYAxis,
        );
        chart.set_default_spacing(DEFAULT_SPACING);
        chart.set_default_line(linechart::LineMode::Braille);
        chart.set_xaxis_labels(MONTHS);
        chart.set_xaxis_label_mode(linechart::XAxisLabelMode::Labels);
        for (name, values) in parse_series(SERIES_TEXT).unwrap_or_default() {
            chart.add_series(linechart::Series::with_values(&name, values));
        }
        win.chart = splitter.add(vsplitter::Panel::Left, chart);

        let mut pages = accordion!("d:f,panels:['&Chart Settings','&Default Settings','&Series','Selected &Point','Da&ta']");

        // ---------------------------------------------------------------- page 0: chart settings
        pages.add(0, label!("'Scale type',l:1,t:1,r:1,h:1"));
        win.scale_type = pages.add(0, combobox!("l:1,t:2,r:1,items:['From zero','Fit data','Fixed','From zero min range'],index:0"));
        let mut min_line = panel!("'',l:1,t:4,r:50%,h:1,type:Page");
        let mut scale_min_label = label!("'Min',l:0,t:0,w:4");
        scale_min_label.set_enabled(false);
        win.scale_min_label = min_line.add(scale_min_label);
        let mut scale_min = numericselector!("i32,0,-1000,1000,1,l:4,t:0,r:0");
        scale_min.set_enabled(false);
        win.scale_min = min_line.add(scale_min);
        pages.add(0, min_line);
        let mut max_line = panel!("'',l:55%,t:4,r:1,h:1,type:Page");
        let mut scale_max_label = label!("'Max',l:0,t:0,w:4");
        scale_max_label.set_enabled(false);
        win.scale_max_label = max_line.add(scale_max_label);
        let mut scale_max = numericselector!("i32,100,-1000,1000,1,l:4,t:0,r:0");
        scale_max.set_enabled(false);
        win.scale_max = max_line.add(scale_max);
        pages.add(0, max_line);

        pages.add(0, label!("'X-axis labels',l:1,t:6,r:1,h:1"));
        win.xaxis_mode = pages.add(0, combobox!("l:1,t:7,r:1,items:['None','Index','Labels','Groups'],index:2"));
        win.yaxis_width_enabled = pages.add(0, checkbox!("'Y-axis width',l:1,t:9,r:1,checked:true"));
        let mut yaxis_width = hslider!("u8,1,50,1,l:1,t:10,r:1,flags:ShowValue,type:Ruler");
        yaxis_width.set_value(6);
        win.yaxis_width = pages.add(0, yaxis_width);
        win.yaxis_step_enabled = pages.add(0, checkbox!("'Y-axis step',l:1,t:12,r:1,checked:true"));
        let mut yaxis_step = hslider!("u8,1,50,1,l:1,t:13,r:1,flags:ShowValue,type:Ruler");
        yaxis_step.set_value(3);
        win.yaxis_step = pages.add(0, yaxis_step);

        let mut format = panel!("'',l:1,t:15,r:1,b:1,type:Page");
        format.add(label!("'Base',l:0,t:0,w:12"));
        win.number_base = format.add(combobox!("l:0,t:1,r:0,items:[Decimal,Hex,Octal,Binary],index:0"));
        format.add(label!("'Grouping',l:0,t:3,w:12"));
        win.number_group = format.add(combobox!("l:0,t:4,r:0,items:[None,'3 digits','4 digits'],index:1"));
        format.add(label!("'Decimals',l:0,t:6,w:12"));
        let mut decimals = hslider!("u8,0,8,1,l:0,t:7,r:0,flags:ShowValue,type:Ruler");
        decimals.set_value(0);
        win.number_decimals = format.add(decimals);
        format.add(label!("'Style',l:0,t:9,w:12"));
        win.number_style = format.add(combobox!("l:0,t:10,r:0,items:[None,Percentage,USD,Euro,GBP,Yen],index:0"));
        pages.add(0, format);

        // ---------------------------------------------------------------- page 1: defaults
        pages.add(1, label!("'Spacing',l:1,t:1,w:12"));
        let mut default_spacing = hslider!("u8,0,12,1,l:1,t:2,r:1,flags:ShowValue,type:Ruler");
        default_spacing.set_value(DEFAULT_SPACING);
        win.default_spacing = pages.add(1, default_spacing);
        win.default_line = add_line_controls(&mut PageHost { pages: &mut pages, index: 1 }, 4, false);
        win.default_marker = add_marker_controls(&mut PageHost { pages: &mut pages, index: 1 }, 11, false);

        // ---------------------------------------------------------------- page 2: series
        pages.add(2, label!("'Series',l:1,t:1,w:12"));
        win.series_list = pages.add(2, combobox!("l:1,t:2,r:1"));
        win.series_custom_color = pages.add(2, checkbox!("'Custom color',l:1,t:4,r:1,checked:false"));
        let mut series_color = ColorPicker::new(Color::Green, layout!("l:1,t:5,r:1"));
        series_color.set_enabled(false);
        win.series_color = pages.add(2, series_color);
        win.series_line = add_line_controls(&mut PageHost { pages: &mut pages, index: 2 }, 7, true);

        // ---------------------------------------------------------------- page 3: selected point
        let mut empty = panel!("'',d:f,type:Page");
        empty.add(label!("'Click on a point to configure it.',l:1,t:1,r:1,b:1"));
        win.point_empty = pages.add(3, empty);

        let mut editor = panel!("'',d:f,type:Page");
        editor.set_visible(false);
        win.point_info = editor.add(label!("'',l:1,t:1,r:1,h:1"));
        editor.add(label!("'Value',l:1,t:3,w:12"));
        win.point_value = editor.add(numericselector!("i32,0,-1000,1000,1,l:1,t:4,r:1"));
        win.point_custom_color = editor.add(checkbox!("'Custom color',l:1,t:6,r:1,checked:false"));
        let mut point_color = ColorPicker::new(DEFAULT_POINT_COLOR, layout!("l:1,t:7,r:1"));
        point_color.set_enabled(false);
        win.point_color = editor.add(point_color);
        win.point_marker = add_marker_controls(&mut editor, 9, true);
        editor.add(label!("'Note (shown in the tooltip)',l:1,t:16,r:1,h:1"));
        win.point_note = editor.add(textfield!("'',l:1,t:17,r:11,h:1"));
        win.point_note_set = editor.add(button!("'&Set',r:1,t:17,w:9,type:Flat"));
        win.point_editor = pages.add(3, editor);

        // ---------------------------------------------------------------- page 4: data
        pages.add(4, label!("'X-axis labels (comma separated)',l:1,t:1,r:1,h:1"));
        win.data_labels = pages.add(4, TextField::new(&MONTHS.join(", "), layout!("l:1,t:2,r:1,h:2"), textfield::Flags::None));
        pages.add(4, label!("'Series (one per line: Name: v1, v2, ...)',l:1,t:5,r:1,h:1"));
        win.data_series = pages.add(4, TextArea::new(SERIES_TEXT, layout!("l:1,t:6,r:1,h:6"), textarea::Flags::ScrollBars));
        let mut groups_panel = panel!("'X-axis groups',l:1,t:13,r:1,b:3");
        let mut groups = listview!("class: XAxisGroup,d:f,flags:ScrollBars");
        groups.add_items(default_groups());
        win.groups = groups_panel.add(groups);
        pages.add(4, groups_panel);
        win.groups_add = pages.add(4, button!("'&Add',l:1,b:0,w:8"));
        win.groups_delete = pages.add(4, button!("'&Del',l:10,b:0,w:8"));
        win.groups_clear = pages.add(4, button!("'&Clear',l:19,b:0,w:8"));
        win.data_apply = pages.add(4, button!("'&Apply',r:1,b:0,w:8"));

        win.pages = splitter.add(vsplitter::Panel::Right, pages);
        win.add(splitter);

        // initial state of the controls
        win.show_line_mode(win.default_line, Some(linechart::LineMode::Braille));
        win.show_marker(win.default_marker, Some(linechart::Marker::default()));
        win.refresh_series_list(0);
        win
    }

    // ------------------------------------------------------------------ small helpers

    fn combo_index(&self, handle: Handle<ComboBox>) -> u32 {
        self.control(handle).and_then(|combo| combo.index()).unwrap_or(0)
    }
    fn set_combo(&mut self, handle: Handle<ComboBox>, index: u32) {
        if let Some(combo) = self.control_mut(handle) {
            combo.set_index(index);
        }
    }
    fn slider_value(&self, handle: Handle<HSlider<u8>>) -> u8 {
        self.control(handle).map(|slider| slider.value()).unwrap_or(1)
    }
    fn is_checked(&self, handle: Handle<CheckBox>) -> bool {
        self.control(handle).map(|c| c.is_checked()).unwrap_or(false)
    }
    fn set_checked(&mut self, handle: Handle<CheckBox>, checked: bool) {
        if let Some(c) = self.control_mut(handle) {
            c.set_checked(checked);
        }
    }
    fn color_of(&self, handle: Handle<ColorPicker>) -> Color {
        self.control(handle).map(|p| p.color()).unwrap_or(Color::White)
    }
    fn set_color(&mut self, handle: Handle<ColorPicker>, color: Color, enabled: bool) {
        if let Some(p) = self.control_mut(handle) {
            p.set_color(color);
            p.set_enabled(enabled);
        }
    }
    fn mark_char(&self, handle: Handle<CharPicker>) -> char {
        self.control(handle).and_then(|picker| picker.char()).unwrap_or(DEFAULT_MARK)
    }
    fn set_visible<T: Control + std::ops::DerefMut<Target = ControlBase> + 'static>(&mut self, handle: Handle<T>, visible: bool) {
        if let Some(c) = self.control_mut(handle) {
            c.set_visible(visible);
        }
    }
    fn with_chart<F: FnOnce(&mut LineChart<i32>)>(&mut self, f: F) {
        let chart = self.chart;
        if let Some(c) = self.control_mut(chart) {
            f(c);
        }
    }
    fn selected_series(&self) -> Option<usize> {
        let index = self.control(self.series_list).and_then(|c| c.index())? as usize;
        let count = self.control(self.chart).map(|c| c.series_count()).unwrap_or(0);
        (index < count).then_some(index)
    }
    fn selected_point(&self) -> Option<(u32, u32)> {
        self.control(self.chart).and_then(|c| c.selected_point())
    }

    // ------------------------------------------------------------------ line mode controls

    fn line_mode_from(&self, controls: LineControls) -> Option<linechart::LineMode> {
        let mut index = self.combo_index(controls.mode);
        if controls.has_default {
            if index == 0 {
                return None;
            }
            index -= 1;
        }
        let line_type = self.control(controls.line_type).map(|s| s.value()).unwrap_or(LineType::Single);
        let direction = DIRECTIONS[(self.combo_index(controls.direction) as usize).min(DIRECTIONS.len() - 1)];
        Some(match index {
            0 => linechart::LineMode::None,
            1 => linechart::LineMode::Braille,
            2 => linechart::LineMode::Line(line_type),
            _ => linechart::LineMode::Orthogonal(line_type, direction),
        })
    }
    fn sync_line_controls(&mut self, controls: LineControls) {
        let mut index = self.combo_index(controls.mode) as i32;
        if controls.has_default {
            index -= 1;
        }
        let has_type = index == 2 || index == 3;
        let has_direction = index == 3;
        self.set_visible(controls.line_type_label, has_type);
        self.set_visible(controls.line_type, has_type);
        self.set_visible(controls.direction_label, has_direction);
        self.set_visible(controls.direction, has_direction);
    }
    fn show_line_mode(&mut self, controls: LineControls, mode: Option<linechart::LineMode>) {
        let offset = if controls.has_default { 1 } else { 0 };
        let (index, line_type, direction) = match mode {
            None => (0, None, None),
            Some(linechart::LineMode::None) => (offset, None, None),
            Some(linechart::LineMode::Braille) => (offset + 1, None, None),
            Some(linechart::LineMode::Line(t)) => (offset + 2, Some(t), None),
            Some(linechart::LineMode::Orthogonal(t, d)) => (offset + 3, Some(t), Some(d)),
        };
        self.set_combo(controls.mode, index);
        if let Some(t) = line_type {
            let h = controls.line_type;
            if let Some(s) = self.control_mut(h) {
                s.set_value(t);
            }
        }
        if let Some(d) = direction {
            let i = DIRECTIONS.iter().position(|x| *x == d).unwrap_or(0) as u32;
            self.set_combo(controls.direction, i);
        }
        self.sync_line_controls(controls);
    }

    // ------------------------------------------------------------------ marker controls

    fn marker_from(&self, controls: MarkerControls) -> Option<linechart::Marker> {
        let mark = self.mark_char(controls.character);
        match self.combo_index(controls.kind) {
            0 if controls.has_default => None,
            0 => Some(linechart::Marker::None),
            1 => Some(linechart::Marker::Small(point_type(self.combo_index(controls.point), mark))),
            _ => Some(linechart::Marker::Large(large_point_type(self.combo_index(controls.large_point), mark))),
        }
    }
    fn sync_marker_controls(&mut self, controls: MarkerControls) {
        let kind = self.combo_index(controls.kind);
        let small = kind == 1;
        let large = kind == 2;
        let custom = (small && self.combo_index(controls.point) == POINT_CUSTOM)
            || (large && self.combo_index(controls.large_point) == LARGE_POINT_CUSTOM);
        self.set_visible(controls.detail_label, small || large);
        self.set_visible(controls.point, small);
        self.set_visible(controls.large_point, large);
        self.set_visible(controls.character_label, custom);
        self.set_visible(controls.character, custom);
    }
    fn show_marker(&mut self, controls: MarkerControls, marker: Option<linechart::Marker>) {
        let (kind, point, large, mark) = match marker {
            None | Some(linechart::Marker::None) => (0, None, None, None),
            Some(linechart::Marker::Small(p)) => {
                let (i, m) = match p {
                    BarPointType::Bullet => (0, None),
                    BarPointType::Diamond => (1, None),
                    BarPointType::Square => (2, None),
                    BarPointType::Custom(ch) => (POINT_CUSTOM, Some(ch)),
                };
                (1, Some(i), None, m)
            }
            Some(linechart::Marker::Large(p)) => {
                let (i, m) = match p {
                    BarLargePointType::RoundSquare => (0, None),
                    BarLargePointType::Square => (1, None),
                    BarLargePointType::DoubleLineSquare => (2, None),
                    BarLargePointType::ThickSquare => (3, None),
                    BarLargePointType::Circle => (4, None),
                    BarLargePointType::Diamond => (5, None),
                    BarLargePointType::Custom(ch) => (LARGE_POINT_CUSTOM, Some(ch)),
                };
                (2, None, Some(i), m)
            }
        };
        self.set_combo(controls.kind, kind);
        if let Some(i) = point {
            self.set_combo(controls.point, i);
        }
        if let Some(i) = large {
            self.set_combo(controls.large_point, i);
        }
        if let Some(ch) = mark {
            let h = controls.character;
            if let Some(p) = self.control_mut(h) {
                p.select_char(ch);
            }
        }
        self.sync_marker_controls(controls);
    }
    fn is_marker_control(controls: MarkerControls, handle: Handle<ComboBox>) -> bool {
        handle == controls.kind || handle == controls.point || handle == controls.large_point
    }

    // ------------------------------------------------------------------ chart settings

    fn apply_scale(&mut self) {
        let ranged = matches!(self.combo_index(self.scale_type), 2 | 3);
        for h in [self.scale_min_label, self.scale_max_label] {
            if let Some(l) = self.control_mut(h) {
                l.set_enabled(ranged);
            }
        }
        for h in [self.scale_min, self.scale_max] {
            if let Some(s) = self.control_mut(h) {
                s.set_enabled(ranged);
            }
        }
        let mut min = self.control(self.scale_min).map(|s| s.value()).unwrap_or(0);
        let mut max = self.control(self.scale_max).map(|s| s.value()).unwrap_or(100);
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }
        let scale = match self.combo_index(self.scale_type) {
            1 => BarScale::FitData,
            2 => BarScale::Fixed { min, max },
            3 => BarScale::FromZeroMinRange { min, max },
            _ => BarScale::FromZero,
        };
        self.with_chart(|c| c.set_scale(scale));
    }
    fn current_groups(&self) -> Vec<XAxisGroup> {
        let Some(list) = self.control(self.groups) else {
            return Vec::new();
        };
        (0..list.items_count()).filter_map(|i| list.item(i).cloned()).collect()
    }
    fn apply_xaxis_mode(&mut self) {
        let groups = spans(&self.current_groups());
        let mode = self.combo_index(self.xaxis_mode);
        self.with_chart(|c| match mode {
            0 => c.set_xaxis_label_mode(linechart::XAxisLabelMode::None),
            1 => c.set_xaxis_label_mode(linechart::XAxisLabelMode::Index(1)),
            2 => c.set_xaxis_label_mode(linechart::XAxisLabelMode::Labels),
            _ => c.set_xaxis_label_mode(linechart::XAxisLabelMode::Custom(&groups)),
        });
    }
    fn apply_number_format(&mut self) {
        let base = match self.combo_index(self.number_base) {
            1 => 16,
            2 => 8,
            3 => 2,
            _ => 10,
        };
        let mut format = FormatNumber::new(base);
        match self.combo_index(self.number_group) {
            1 => format = format.group(3, b','),
            2 => format = format.group(4, b' '),
            _ => {}
        }
        let decimals = self.slider_value(self.number_decimals);
        if decimals > 0 {
            format = format.decimals(decimals);
        }
        format = match self.combo_index(self.number_style) {
            1 => format.suffix("%"),
            2 => format.prefix("$"),
            3 => format.prefix("€"),
            4 => format.prefix("£"),
            5 => format.prefix("¥"),
            _ => format,
        };
        self.with_chart(|c| c.set_number_format(format));
    }
    fn set_yaxis_width_active(&mut self, active: bool) {
        let h = self.yaxis_width;
        if let Some(s) = self.control_mut(h) {
            s.set_enabled(active);
        }
        let width = self.slider_value(self.yaxis_width);
        self.with_chart(|c| {
            c.set_yaxis_visible(active);
            if active {
                c.set_yaxis_width(width);
            }
        });
    }
    fn set_yaxis_step_active(&mut self, active: bool) {
        let h = self.yaxis_step;
        if let Some(s) = self.control_mut(h) {
            s.set_enabled(active);
        }
        let step = self.slider_value(self.yaxis_step);
        self.with_chart(|c| {
            c.set_yaxis_show_grid(active);
            if active {
                c.set_yaxis_step(step);
            }
        });
    }

    // ------------------------------------------------------------------ defaults

    fn apply_default_line(&mut self) {
        if let Some(mode) = self.line_mode_from(self.default_line) {
            self.with_chart(|c| c.set_default_line(mode));
        }
    }
    fn apply_default_marker(&mut self) {
        if let Some(marker) = self.marker_from(self.default_marker) {
            self.with_chart(|c| c.set_default_marker(marker));
        }
    }

    // ------------------------------------------------------------------ series

    /// Rebuilds the list of series and selects `index` (when it exists).
    fn refresh_series_list(&mut self, index: usize) {
        let names: Vec<String> = self
            .control(self.chart)
            .map(|c| {
                (0..c.series_count())
                    .filter_map(|i| c.get_series(i))
                    .enumerate()
                    .map(|(i, s)| if s.label().is_empty() { format!("Series {}", i + 1) } else { s.label().to_string() })
                    .collect()
            })
            .unwrap_or_default();
        let h = self.series_list;
        if let Some(combo) = self.control_mut(h) {
            combo.clear();
            for name in names.iter() {
                combo.add(name);
            }
            if !names.is_empty() {
                combo.set_index(index.min(names.len() - 1) as u32);
            }
        }
        self.load_series();
    }
    fn load_series(&mut self) {
        let Some(index) = self.selected_series() else {
            return;
        };
        let Some((attr, line)) = self.control(self.chart).and_then(|c| c.get_series(index)).map(|s| (s.attr(), s.line())) else {
            return;
        };
        self.set_checked(self.series_custom_color, attr.is_some());
        let color = attr.map(|a| a.foreground).unwrap_or(Color::Green);
        self.set_color(self.series_color, color, attr.is_some());
        self.show_line_mode(self.series_line, line);
    }
    fn apply_series_style(&mut self) {
        let Some(index) = self.selected_series() else {
            return;
        };
        let attr = self
            .is_checked(self.series_custom_color)
            .then(|| CharAttribute::with_fore_color(self.color_of(self.series_color)));
        let enabled = attr.is_some();
        let h = self.series_color;
        if let Some(p) = self.control_mut(h) {
            p.set_enabled(enabled);
        }
        let line = self.line_mode_from(self.series_line);
        self.with_chart(|c| {
            c.update_series(index, |s| {
                s.set_attr(attr).set_line(line);
            })
        });
    }

    // ------------------------------------------------------------------ selected point

    fn show_point_editor(&mut self, show: bool) {
        self.set_visible(self.point_empty, !show);
        self.set_visible(self.point_editor, show);
    }
    fn load_selected_point(&mut self, open_page: bool) {
        let Some((series_index, point_index)) = self.selected_point() else {
            self.show_point_editor(false);
            return;
        };
        let Some((caption, value, attr, marker, note)) = self.control(self.chart).and_then(|c| {
            let series = c.get_series(series_index as usize)?;
            let bar = series.get(point_index as usize)?;
            let name = if series.label().is_empty() { format!("Series {}", series_index + 1) } else { series.label().to_string() };
            let x = c.xaxis_label(point_index as usize).map(|l| l.to_string()).unwrap_or_else(|| format!("#{}", point_index + 1));
            let marker = bar.draw_mode().and_then(|m| match m {
                BarDrawMode::Point(p) => Some(linechart::Marker::Small(p)),
                BarDrawMode::LargePoint(p) => Some(linechart::Marker::Large(p)),
                _ => None,
            });
            Some((format!("{name} - {x}"), bar.value(), bar.attr(), marker, bar.label().to_string()))
        }) else {
            self.show_point_editor(false);
            return;
        };
        let info = self.point_info;
        if let Some(l) = self.control_mut(info) {
            l.set_caption(&caption);
        }
        let value_h = self.point_value;
        if let Some(s) = self.control_mut(value_h) {
            s.set_value(value);
        }
        self.set_checked(self.point_custom_color, attr.is_some());
        self.set_color(self.point_color, attr.map(|a| a.foreground).unwrap_or(DEFAULT_POINT_COLOR), attr.is_some());
        self.show_marker(self.point_marker, marker);
        let note_h = self.point_note;
        if let Some(t) = self.control_mut(note_h) {
            t.set_text(&note);
        }
        self.show_point_editor(true);
        if open_page {
            // switching the page moves the focus to it; do it only when the page is not shown yet,
            // so that while it is open the arrow keys keep moving the selection in the chart
            let pages = self.pages;
            if let Some(a) = self.control_mut(pages) {
                if a.current_panel() != Some(3) {
                    a.set_current_panel(3);
                }
            }
        }
    }
    fn update_selected_point<F: FnOnce(&mut Bar<i32>)>(&mut self, f: F) {
        let Some((series_index, point_index)) = self.selected_point() else {
            return;
        };
        self.with_chart(|c| {
            c.update_series(series_index as usize, |s| {
                if let Some(bar) = s.get_mut(point_index as usize) {
                    f(bar);
                }
            })
        });
    }
    fn apply_point_color(&mut self) {
        let custom = self.is_checked(self.point_custom_color);
        let color = self.color_of(self.point_color);
        let h = self.point_color;
        if let Some(p) = self.control_mut(h) {
            p.set_enabled(custom);
        }
        self.update_selected_point(|bar| {
            if custom {
                bar.set_attr(CharAttribute::with_fore_color(color));
            } else {
                bar.clear_attr();
            }
        });
    }
    fn apply_point_marker(&mut self) {
        let marker = self.marker_from(self.point_marker);
        self.update_selected_point(|bar| {
            match marker {
                Some(linechart::Marker::Small(p)) => bar.set_draw_mode(BarDrawMode::Point(p)),
                Some(linechart::Marker::Large(p)) => bar.set_draw_mode(BarDrawMode::LargePoint(p)),
                _ => bar.clear_draw_mode(),
            };
        });
    }
    fn apply_point_note(&mut self) {
        let note = self.control(self.point_note).map(|t| t.text().trim().to_string()).unwrap_or_default();
        self.update_selected_point(|bar| {
            bar.set_label(&note);
        });
    }

    // ------------------------------------------------------------------ data

    fn apply_data(&mut self) {
        let text = self.control(self.data_series).map(|t| t.text().to_string()).unwrap_or_default();
        let series = match parse_series(&text) {
            Ok(series) if !series.is_empty() => series,
            Ok(_) => {
                dialogs::message("Data", "Add at least one series (Name: v1, v2, ...).");
                return;
            }
            Err(line) => {
                dialogs::message("Data", &format!("Line {line}: enter whole numbers separated by commas."));
                return;
            }
        };
        let labels = parse_labels(&self.control(self.data_labels).map(|t| t.text().to_string()).unwrap_or_default());
        self.with_chart(|c| {
            // keep the style of the existing series (by position), replace their name and values
            while c.series_count() > series.len() {
                c.remove_series(c.series_count() - 1);
            }
            for (index, (name, values)) in series.into_iter().enumerate() {
                if index < c.series_count() {
                    c.update_series(index, |s| {
                        s.set_label(&name);
                        s.clear();
                        s.add_values(values);
                    });
                } else {
                    c.add_series(linechart::Series::with_values(&name, values));
                }
            }
            c.set_xaxis_labels(labels);
        });
        self.apply_xaxis_mode();
        let current = self.selected_series().unwrap_or(0);
        self.refresh_series_list(current);
        self.load_selected_point(false);
    }
    fn add_group(&mut self) {
        let groups = self.current_groups();
        let start = groups.last().map(|g| g.start.saturating_add(g.count)).unwrap_or(0);
        let draft = XAxisGroup {
            label: format!("Group {}", groups.len() + 1),
            start,
            count: 3,
        };
        let Some(group) = GroupWindow::new("Add group", &draft).show() else {
            return;
        };
        let h = self.groups;
        if let Some(list) = self.control_mut(h) {
            list.add(group);
        }
        self.apply_xaxis_mode();
    }
    fn delete_selected_group(&mut self) {
        let h = self.groups;
        let Some(index) = self.control(h).and_then(|list| list.current_item_index()) else {
            dialogs::message("Groups", "Select a group to delete.");
            return;
        };
        let mut groups = self.current_groups();
        if index < groups.len() {
            groups.remove(index);
        }
        if let Some(list) = self.control_mut(h) {
            list.clear();
            list.add_items(groups);
        }
        self.apply_xaxis_mode();
    }
    fn clear_groups(&mut self) {
        let h = self.groups;
        if let Some(list) = self.control_mut(h) {
            list.clear();
        }
        self.apply_xaxis_mode();
    }
    fn edit_group(&mut self, index: usize) {
        let h = self.groups;
        let Some(current) = self.control(h).and_then(|list| list.item(index).cloned()) else {
            return;
        };
        let Some(updated) = GroupWindow::new("Edit group", &current).show() else {
            return;
        };
        if let Some(item) = self.control_mut(h).and_then(|list| list.item_mut(index)) {
            *item = updated;
        }
        self.apply_xaxis_mode();
    }
}

// ---------------------------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------------------------

impl LineChartEvents<i32> for LineChartEditor {
    fn on_point_selected(&mut self, _handle: Handle<LineChart<i32>>, series_index: u32, _point_index: u32) -> EventProcessStatus {
        // keep the series page in sync with the series of the selected point
        if self.selected_series() != Some(series_index as usize) {
            self.set_combo(self.series_list, series_index);
            self.load_series();
        }
        self.load_selected_point(true);
        EventProcessStatus::Processed
    }
    fn on_clear_selection(&mut self, _handle: Handle<LineChart<i32>>) -> EventProcessStatus {
        self.show_point_editor(false);
        EventProcessStatus::Processed
    }
}

impl HSliderEvents<u8> for LineChartEditor {
    fn on_value_changed(&mut self, handle: Handle<HSlider<u8>>, value: u8) -> EventProcessStatus {
        match () {
            _ if handle == self.default_spacing => self.with_chart(|c| c.set_default_spacing(value)),
            _ if handle == self.yaxis_width => self.with_chart(|c| c.set_yaxis_width(value)),
            _ if handle == self.yaxis_step => self.with_chart(|c| c.set_yaxis_step(value)),
            _ if handle == self.number_decimals => self.apply_number_format(),
            _ => return EventProcessStatus::Ignored,
        }
        EventProcessStatus::Processed
    }
}

impl ColorPickerEvents for LineChartEditor {
    fn on_color_changed(&mut self, handle: Handle<ColorPicker>, _color: Color) -> EventProcessStatus {
        if handle == self.series_color {
            self.apply_series_style();
        } else if handle == self.point_color {
            self.apply_point_color();
        } else {
            return EventProcessStatus::Ignored;
        }
        EventProcessStatus::Processed
    }
}

impl SelectorEvents<LineType> for LineChartEditor {
    fn on_selection_changed(&mut self, handle: Handle<Selector<LineType>>, _value: Option<LineType>) -> EventProcessStatus {
        if handle == self.default_line.line_type {
            self.apply_default_line();
        } else if handle == self.series_line.line_type {
            self.apply_series_style();
        } else {
            return EventProcessStatus::Ignored;
        }
        EventProcessStatus::Processed
    }
}

impl ComboBoxEvents for LineChartEditor {
    fn on_selection_changed(&mut self, handle: Handle<ComboBox>) -> EventProcessStatus {
        if handle == self.number_base || handle == self.number_group || handle == self.number_style {
            self.apply_number_format();
        } else if handle == self.scale_type {
            self.apply_scale();
        } else if handle == self.xaxis_mode {
            self.apply_xaxis_mode();
        } else if handle == self.default_line.mode || handle == self.default_line.direction {
            self.sync_line_controls(self.default_line);
            self.apply_default_line();
        } else if Self::is_marker_control(self.default_marker, handle) {
            self.sync_marker_controls(self.default_marker);
            self.apply_default_marker();
        } else if handle == self.series_list {
            self.load_series();
        } else if handle == self.series_line.mode || handle == self.series_line.direction {
            self.sync_line_controls(self.series_line);
            self.apply_series_style();
        } else if Self::is_marker_control(self.point_marker, handle) {
            self.sync_marker_controls(self.point_marker);
            self.apply_point_marker();
        } else {
            return EventProcessStatus::Ignored;
        }
        EventProcessStatus::Processed
    }
}

impl CharPickerEvents for LineChartEditor {
    fn on_char_changed(&mut self, handle: Handle<CharPicker>, _code: Option<char>) -> EventProcessStatus {
        if handle == self.default_marker.character {
            self.apply_default_marker();
        } else if handle == self.point_marker.character {
            self.apply_point_marker();
        } else {
            return EventProcessStatus::Ignored;
        }
        EventProcessStatus::Processed
    }
}

impl CheckBoxEvents for LineChartEditor {
    fn on_status_changed(&mut self, handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
        if handle == self.yaxis_width_enabled {
            self.set_yaxis_width_active(checked);
        } else if handle == self.yaxis_step_enabled {
            self.set_yaxis_step_active(checked);
        } else if handle == self.series_custom_color {
            self.apply_series_style();
        } else if handle == self.point_custom_color {
            self.apply_point_color();
        } else {
            return EventProcessStatus::Ignored;
        }
        EventProcessStatus::Processed
    }
}

impl NumericSelectorEvents<i32> for LineChartEditor {
    fn on_value_changed(&mut self, handle: Handle<NumericSelector<i32>>, value: i32) -> EventProcessStatus {
        if handle == self.scale_min || handle == self.scale_max {
            self.apply_scale();
        } else if handle == self.point_value {
            self.update_selected_point(|bar| {
                bar.set_value(value);
            });
        } else {
            return EventProcessStatus::Ignored;
        }
        EventProcessStatus::Processed
    }
}

impl ButtonEvents for LineChartEditor {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        match () {
            _ if handle == self.data_apply => self.apply_data(),
            _ if handle == self.groups_add => self.add_group(),
            _ if handle == self.groups_delete => self.delete_selected_group(),
            _ if handle == self.groups_clear => self.clear_groups(),
            _ if handle == self.point_note_set => self.apply_point_note(),
            _ => return EventProcessStatus::Ignored,
        }
        EventProcessStatus::Processed
    }
}

impl ListViewEvents<XAxisGroup> for LineChartEditor {
    fn on_item_action(&mut self, handle: Handle<ListView<XAxisGroup>>, index: usize) -> EventProcessStatus {
        if handle == self.groups {
            self.edit_group(index);
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::single_window(LineChartEditor::new).title("Line chart editor").run()
}
