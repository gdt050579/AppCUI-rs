use appcui::prelude::*;

const COLUMNS: u32 = 24;
const ROWS: u32 = 14;
const VALUE_MIN: i32 = -10_000;
const VALUE_MAX: i32 = 10_000;
const DEFAULT_MARK: char = '#';
const FILL_CUSTOM: u32 = 1;

/// Centers of the two hot spots, as `(row, column)`.
const SOURCE_A: (u32, u32) = (4, 6);
const SOURCE_B: (u32, u32) = (10, 17);

const TOOLTIPS: [(&str, heatmap::TooltipFormat); 5] = [
    ("Value", heatmap::TooltipFormat::Value),
    ("Label", heatmap::TooltipFormat::Label),
    ("Percentage", heatmap::TooltipFormat::Percentage),
    ("Label and value", heatmap::TooltipFormat::LabelAndValue),
    ("Label and percentage", heatmap::TooltipFormat::LabelAndPercentage),
];

#[derive(Copy, Clone, PartialEq, Eq)]
enum Pattern {
    Peaks,
    Ramp,
    Gaps,
}

const PATTERNS: [(&str, Pattern); 3] = [("Peaks", Pattern::Peaks), ("Ramp", Pattern::Ramp), ("Gaps", Pattern::Gaps)];

fn index(row: u32, column: u32) -> usize {
    (row * COLUMNS + column) as usize
}

fn peak_value(row: u32, column: u32) -> i32 {
    let heat = |center: (u32, u32)| {
        let dx = column as i32 - center.1 as i32;
        let dy = (row as i32 - center.0 as i32) * 2;
        (180 - (dx * dx + dy * dy)).max(0)
    };
    heat(SOURCE_A) + heat(SOURCE_B)
}

fn pattern_data(pattern: Pattern) -> (Vec<Option<i32>>, Vec<String>) {
    let count = (ROWS * COLUMNS) as usize;
    let mut values = vec![None; count];
    let mut labels = vec![String::new(); count];
    match pattern {
        Pattern::Peaks => {
            for row in 0..ROWS {
                for column in 0..COLUMNS {
                    values[index(row, column)] = Some(peak_value(row, column));
                }
            }
            labels[index(SOURCE_A.0, SOURCE_A.1)] = String::from("Source A");
            labels[index(SOURCE_B.0, SOURCE_B.1)] = String::from("Source B");
        }
        Pattern::Ramp => {
            for row in 0..ROWS {
                for column in 0..COLUMNS {
                    values[index(row, column)] = Some(row as i32 * 10 + column as i32);
                }
            }
        }
        Pattern::Gaps => {
            for row in 0..ROWS {
                for column in 0..COLUMNS {
                    let source = (row, column) == SOURCE_A || (row, column) == SOURCE_B;
                    if source || (row + column) % 2 == 0 {
                        values[index(row, column)] = Some(peak_value(row, column));
                    }
                }
            }
            labels[index(SOURCE_A.0, SOURCE_A.1)] = String::from("Source A");
            labels[index(SOURCE_B.0, SOURCE_B.1)] = String::from("Source B");
        }
    }
    (values, labels)
}

fn format_values(values: &[Option<i32>]) -> String {
    let mut rows = Vec::with_capacity(ROWS as usize);
    for row in 0..ROWS {
        let mut line = String::new();
        for column in 0..COLUMNS {
            if column > 0 {
                line.push_str(", ");
            }
            if let Some(value) = values[index(row, column)] {
                line.push_str(&value.to_string());
            }
        }
        rows.push(line);
    }
    rows.join("\n")
}

fn parse_values(text: &str) -> Result<Vec<Option<i32>>, String> {
    let lines: Vec<&str> = text.lines().collect();
    if lines.len() != ROWS as usize {
        return Err(format!("Enter {ROWS} rows (this text has {}).", lines.len()));
    }
    let mut values = Vec::with_capacity((ROWS * COLUMNS) as usize);
    for (line_index, line) in lines.iter().enumerate() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != COLUMNS as usize {
            return Err(format!(
                "Row {}: enter {COLUMNS} comma-separated values (found {}).",
                line_index + 1,
                parts.len()
            ));
        }
        for (column, part) in parts.iter().enumerate() {
            let part = part.trim();
            if part.is_empty() {
                values.push(None);
                continue;
            }
            match part.parse::<i32>() {
                Ok(value) => values.push(Some(value)),
                Err(_) => {
                    return Err(format!("Row {}, column {}: enter a whole number.", line_index + 1, column + 1));
                }
            }
        }
    }
    Ok(values)
}

fn blank_fills() -> Vec<Option<heatmap::CellFill>> {
    vec![None; (ROWS * COLUMNS) as usize]
}

fn char_picker(layout: Layout) -> CharPicker {
    let mut picker = CharPicker::new(Some(DEFAULT_MARK), layout);
    picker.clear_sets();
    for (name, symbols) in [
        ("Ascii", charpicker::UnicodeSymbols::Ascii),
        ("Blocks", charpicker::UnicodeSymbols::Blocks),
        ("Shapes", charpicker::UnicodeSymbols::Shapes),
    ] {
        picker.add_set(charpicker::Set::from_unicode_symbols(name, symbols));
    }
    picker.select_char(DEFAULT_MARK);
    picker
}

fn style_cell(cell: &mut heatmap::Cell<i32>, label: &str, fill: Option<heatmap::CellFill>) {
    cell.set_label(label);
    match fill {
        Some(fill) => cell.set_fill(fill),
        None => cell.clear_fill(),
    }
}

fn percentage(value: i32, min: i32, max: i32) -> u8 {
    let span = max as f64 - min as f64;
    if span == 0.0 {
        100
    } else {
        (((value as f64 - min as f64) / span) * 100.0).round().clamp(0.0, 100.0) as u8
    }
}

// ---------------------------------------------------------------------------------------------
// Editor window
// ---------------------------------------------------------------------------------------------

#[Window(events = [HeatMapEvents<i32>, ComboBoxEvents, CheckBoxEvents, CharPickerEvents, NumericSelectorEvents<i32>, TextFieldEvents, ButtonEvents])]
struct HeatMapEditor {
    chart: Handle<HeatMap<i32>>,
    pages: Handle<Accordion>,
    // chart settings
    tooltip: Handle<ComboBox>,
    default_fill: Handle<ComboBox>,
    default_char_label: Handle<Label>,
    default_char: Handle<CharPicker>,
    summary: Handle<Label>,
    range_label: Handle<Label>,
    // selected cell
    empty_panel: Handle<Panel>,
    editor_panel: Handle<Panel>,
    info: Handle<Label>,
    detail: Handle<Label>,
    cell_value: Handle<NumericSelector<i32>>,
    cell_label: Handle<TextField>,
    cell_use_default: Handle<CheckBox>,
    cell_fill_label: Handle<Label>,
    cell_fill: Handle<ComboBox>,
    cell_char_label: Handle<Label>,
    cell_char: Handle<CharPicker>,
    cell_clear: Handle<Button>,
    // data
    pattern: Handle<ComboBox>,
    data_text: Handle<TextArea>,
    data_apply: Handle<Button>,
    data_reload: Handle<Button>,
    // values, labels and fills edited here, then written into the heat map
    values: Vec<Option<i32>>,
    labels: Vec<String>,
    fills: Vec<Option<heatmap::CellFill>>,
    // set while the editor fills its own controls, so that their events are ignored
    loading: bool,
}

impl HeatMapEditor {
    fn new() -> Self {
        let (values, labels) = pattern_data(Pattern::Peaks);
        let data_text = format_values(&values);
        let mut win = Self {
            base: window!("'Heat map editor',d:f"),
            chart: Handle::None,
            pages: Handle::None,
            tooltip: Handle::None,
            default_fill: Handle::None,
            default_char_label: Handle::None,
            default_char: Handle::None,
            summary: Handle::None,
            range_label: Handle::None,
            empty_panel: Handle::None,
            editor_panel: Handle::None,
            info: Handle::None,
            detail: Handle::None,
            cell_value: Handle::None,
            cell_label: Handle::None,
            cell_use_default: Handle::None,
            cell_fill_label: Handle::None,
            cell_fill: Handle::None,
            cell_char_label: Handle::None,
            cell_char: Handle::None,
            cell_clear: Handle::None,
            pattern: Handle::None,
            data_text: Handle::None,
            data_apply: Handle::None,
            data_reload: Handle::None,
            values,
            labels,
            fills: blank_fills(),
            loading: false,
        };

        let mut splitter = vsplitter!("pos:70%,d:f,resize:PreserveRightPanelSize,min-left-width:30,min-right-width:36");

        let mut chart = HeatMap::<i32>::new(
            layout!("d:f"),
            Size::new(COLUMNS, ROWS),
            heatmap::Flags::ScrollBars | heatmap::Flags::Margins | heatmap::Flags::DimCellsOnSelection,
        );
        chart.set_tooltip_format(heatmap::TooltipFormat::LabelAndValue);
        win.chart = splitter.add(vsplitter::Panel::Left, chart);

        let mut pages = accordion!("d:f,panels:['&Chart Settings','Selected C&ell','Da&ta']");

        // ---------------------------------------------------------------- page 0: chart settings
        pages.add(0, label!("'Tooltip',l:1,t:1,r:1,h:1"));
        let mut tooltip = ComboBox::new(layout!("l:1,t:2,r:1"), combobox::Flags::None);
        for (name, _) in TOOLTIPS {
            tooltip.add(name);
        }
        tooltip.set_index(
            TOOLTIPS
                .iter()
                .position(|(_, format)| *format == heatmap::TooltipFormat::LabelAndValue)
                .unwrap_or(0) as u32,
        );
        win.tooltip = pages.add(0, tooltip);
        pages.add(0, label!("'Default fill',l:1,t:4,r:1,h:1"));
        let mut default_fill = ComboBox::new(layout!("l:1,t:5,r:1"), combobox::Flags::None);
        default_fill.add("Shades");
        default_fill.add("Custom");
        default_fill.set_index(0);
        win.default_fill = pages.add(0, default_fill);
        let mut default_char_label = label!("'Character',l:1,t:7,w:12");
        default_char_label.set_visible(false);
        win.default_char_label = pages.add(0, default_char_label);
        let mut default_char = char_picker(layout!("l:1,t:8,r:1"));
        default_char.set_visible(false);
        win.default_char = pages.add(0, default_char);
        pages.add(0, label!("'Cells',l:1,t:10,r:1,h:1"));
        win.summary = pages.add(0, label!("'',l:1,t:11,r:1,h:1"));
        win.range_label = pages.add(0, label!("'',l:1,t:12,r:1,h:1"));
        pages.add(0, label!("'Hover a cell to see its tooltip.',l:1,t:14,r:1,h:1"));
        pages.add(0, label!("'Arrow keys move the selection.',l:1,t:15,r:1,h:1"));
        pages.add(0, label!("'Ctrl+arrows scroll the map.',l:1,t:16,r:1,h:1"));
        pages.add(0, label!("'Escape clears the selection.',l:1,t:17,r:1,h:1"));

        // ---------------------------------------------------------------- page 1: selected cell
        let mut empty = panel!("'',d:f,type:Page");
        empty.add(label!("'Click a cell to edit it.',l:1,t:1,r:1,h:1"));
        empty.add(label!("'Arrow keys move the selection.',l:1,t:2,r:1,h:1"));
        win.empty_panel = pages.add(1, empty);

        let mut editor = panel!("'',d:f,type:Page");
        editor.set_visible(false);
        win.info = editor.add(label!("'',l:1,t:1,r:1,h:1"));
        win.detail = editor.add(label!("'',l:1,t:2,r:1,h:1"));
        editor.add(label!("'Value',l:1,t:4,w:12"));
        win.cell_value = editor.add(NumericSelector::new(
            0,
            VALUE_MIN,
            VALUE_MAX,
            1,
            layout!("l:1,t:5,r:1"),
            numericselector::Flags::None,
        ));
        editor.add(label!("'Label',l:1,t:7,w:12"));
        win.cell_label = editor.add(textfield!("'',l:1,t:8,r:1"));
        win.cell_use_default = editor.add(checkbox!("'Use default fill',l:1,t:10,r:1,checked:true"));
        let mut cell_fill_label = label!("'Fill',l:1,t:12,w:12");
        cell_fill_label.set_visible(false);
        win.cell_fill_label = editor.add(cell_fill_label);
        let mut cell_fill = ComboBox::new(layout!("l:1,t:13,r:1"), combobox::Flags::None);
        cell_fill.add("Shades");
        cell_fill.add("Custom");
        cell_fill.set_index(0);
        cell_fill.set_visible(false);
        win.cell_fill = editor.add(cell_fill);
        let mut cell_char_label = label!("'Character',l:1,t:15,w:12");
        cell_char_label.set_visible(false);
        win.cell_char_label = editor.add(cell_char_label);
        let mut cell_char = char_picker(layout!("l:1,t:16,r:1"));
        cell_char.set_visible(false);
        win.cell_char = editor.add(cell_char);
        win.cell_clear = editor.add(button!("'&Clear cell',l:1,b:0,w:14"));
        win.editor_panel = pages.add(1, editor);

        // ---------------------------------------------------------------- page 2: data
        pages.add(2, label!("'Pattern',l:1,t:1,r:1,h:1"));
        let mut pattern = ComboBox::new(layout!("l:1,t:2,r:1"), combobox::Flags::None);
        for (name, _) in PATTERNS {
            pattern.add(name);
        }
        pattern.set_index(0);
        win.pattern = pages.add(2, pattern);
        pages.add(
            2,
            Label::new(&format!("{COLUMNS} x {ROWS}, comma separated."), layout!("l:1,t:4,r:1,h:1")),
        );
        pages.add(2, label!("'A blank entry is an empty cell.',l:1,t:5,r:1,h:1"));
        win.data_text = pages.add(2, TextArea::new(&data_text, layout!("l:1,t:7,r:1,b:3"), textarea::Flags::ScrollBars));
        win.data_reload = pages.add(2, button!("'&Reload',l:1,b:0,w:12"));
        win.data_apply = pages.add(2, button!("'&Apply',r:1,b:0,w:10"));

        win.pages = splitter.add(vsplitter::Panel::Right, pages);
        win.add(splitter);
        win.sync_chart();
        win.refresh_summary();
        win
    }

    // ------------------------------------------------------------------ helpers

    fn combo_index(&self, handle: Handle<ComboBox>) -> u32 {
        self.control(handle).and_then(|combo| combo.index()).unwrap_or(0)
    }

    fn with_chart<F>(&mut self, f: F)
    where
        F: FnOnce(&mut HeatMap<i32>),
    {
        let chart = self.chart;
        if let Some(map) = self.control_mut(chart) {
            f(map);
        }
    }

    fn set_visible<T>(&mut self, handle: Handle<T>, visible: bool)
    where
        T: Control + std::ops::DerefMut<Target = ControlBase> + 'static,
    {
        if let Some(control) = self.control_mut(handle) {
            control.set_visible(visible);
        }
    }

    fn is_checked(&self, handle: Handle<CheckBox>) -> bool {
        self.control(handle).map(|check| check.is_checked()).unwrap_or(false)
    }

    fn set_checked(&mut self, handle: Handle<CheckBox>, checked: bool) {
        if let Some(check) = self.control_mut(handle) {
            check.set_checked(checked);
        }
    }

    fn mark_char(&self, handle: Handle<CharPicker>) -> char {
        self.control(handle).and_then(|picker| picker.char()).unwrap_or(DEFAULT_MARK)
    }

    fn selected_cell(&self) -> Option<(u32, u32)> {
        self.control(self.chart).and_then(|map| map.selected_cell())
    }

    fn range(&self) -> Option<(i32, i32)> {
        let mut values = self.values.iter().flatten().copied();
        let first = values.next()?;
        let (mut min, mut max) = (first, first);
        for value in values {
            min = min.min(value);
            max = max.max(value);
        }
        Some((min, max))
    }

    // ------------------------------------------------------------------ chart

    fn sync_chart(&mut self) {
        let values = self.values.clone();
        let labels = self.labels.clone();
        let fills = self.fills.clone();
        self.with_chart(|map| {
            map.fill_with(|row, column| values[index(row, column)]);
            for row in 0..ROWS {
                for column in 0..COLUMNS {
                    let idx = index(row, column);
                    let label = &labels[idx];
                    let fill = fills[idx];
                    if label.is_empty() && fill.is_none() {
                        continue;
                    }
                    map.update_cell(row, column, |cell| style_cell(cell, label, fill));
                }
            }
        });
    }

    fn write_cell(&mut self, row: u32, column: u32) {
        let idx = index(row, column);
        let value = self.values[idx];
        let label = self.labels[idx].clone();
        let fill = self.fills[idx];
        self.with_chart(|map| match value {
            Some(value) => {
                map.set(row, column, value);
                map.update_cell(row, column, |cell| style_cell(cell, &label, fill));
            }
            None => map.clear_cell(row, column),
        });
    }

    fn refresh_summary(&mut self) {
        let total = (ROWS * COLUMNS) as usize;
        let filled = self.values.iter().filter(|value| value.is_some()).count();
        let count = format!("{filled} / {total} cells");
        let range = match self.range() {
            Some((min, max)) => format!("{min} to {max}"),
            None => String::from("no values"),
        };
        let summary = self.summary;
        if let Some(label) = self.control_mut(summary) {
            label.set_caption(&count);
        }
        let range_label = self.range_label;
        if let Some(label) = self.control_mut(range_label) {
            label.set_caption(&range);
        }
    }

    fn apply_tooltip(&mut self) {
        let format = TOOLTIPS
            .get(self.combo_index(self.tooltip) as usize)
            .map(|(_, format)| *format)
            .unwrap_or(heatmap::TooltipFormat::LabelAndValue);
        self.with_chart(|map| map.set_tooltip_format(format));
    }

    fn apply_default_fill(&mut self) {
        if self.loading {
            return;
        }
        let custom = self.combo_index(self.default_fill) == FILL_CUSTOM;
        self.set_visible(self.default_char_label, custom);
        self.set_visible(self.default_char, custom);
        if custom {
            let mark = self.mark_char(self.default_char);
            self.with_chart(|map| {
                if map.default_fill() != Some(heatmap::CellFill::Custom(mark)) {
                    map.set_default_fill(heatmap::CellFill::Custom(mark));
                }
            });
        } else {
            self.with_chart(|map| {
                if map.default_fill().is_some() {
                    map.clear_default_fill();
                }
            });
        }
    }

    // ------------------------------------------------------------------ selected cell

    fn show_editor(&mut self, show: bool) {
        self.set_visible(self.empty_panel, !show);
        self.set_visible(self.editor_panel, show);
    }

    fn load_cell(&mut self, row: u32, column: u32, open_page: bool) {
        let idx = index(row, column);
        let Some(value) = self.values.get(idx).copied().flatten() else {
            self.show_editor(false);
            return;
        };
        let label = self.labels.get(idx).cloned().unwrap_or_default();
        let fill = self.fills.get(idx).copied().flatten();
        let (min, max) = self.range().unwrap_or((value, value));
        let position = format!("Row {row}, column {column}");
        let detail = format!("{}% of the range {min}..{max}", percentage(value, min, max));
        let (info, detail_label, cell_value, cell_label) = (self.info, self.detail, self.cell_value, self.cell_label);
        self.loading = true;
        if let Some(caption) = self.control_mut(info) {
            caption.set_caption(&position);
        }
        if let Some(caption) = self.control_mut(detail_label) {
            caption.set_caption(&detail);
        }
        if let Some(selector) = self.control_mut(cell_value) {
            selector.set_value(value.clamp(VALUE_MIN, VALUE_MAX));
        }
        if let Some(field) = self.control_mut(cell_label) {
            if field.text() != label {
                field.set_text(&label);
            }
        }
        self.show_cell_fill(fill);
        self.loading = false;
        self.show_editor(true);
        if open_page {
            // switching the page moves the focus to it; do it only when the page is not shown yet,
            // so that while it is open the arrow keys keep moving the selection in the chart
            let pages = self.pages;
            if let Some(accordion) = self.control_mut(pages) {
                if accordion.current_panel() != Some(1) {
                    accordion.set_current_panel(1);
                }
            }
        }
    }

    fn show_cell_fill(&mut self, fill: Option<heatmap::CellFill>) {
        let (index, mark) = match fill {
            Some(heatmap::CellFill::Custom(mark)) => (FILL_CUSTOM, Some(mark)),
            Some(heatmap::CellFill::Shades) => (0, None),
            None => (0, None),
        };
        self.set_checked(self.cell_use_default, fill.is_none());
        let combo = self.cell_fill;
        if let Some(combo) = self.control_mut(combo) {
            combo.set_index(index);
        }
        if let Some(mark) = mark {
            let picker = self.cell_char;
            if let Some(picker) = self.control_mut(picker) {
                picker.select_char(mark);
            }
        }
        let own_fill = fill.is_some();
        let custom = matches!(fill, Some(heatmap::CellFill::Custom(_)));
        self.set_visible(self.cell_fill_label, own_fill);
        self.set_visible(self.cell_fill, own_fill);
        self.set_visible(self.cell_char_label, custom);
        self.set_visible(self.cell_char, custom);
    }

    fn cell_fill_choice(&self) -> Option<heatmap::CellFill> {
        if self.is_checked(self.cell_use_default) {
            return None;
        }
        if self.combo_index(self.cell_fill) == FILL_CUSTOM {
            Some(heatmap::CellFill::Custom(self.mark_char(self.cell_char)))
        } else {
            Some(heatmap::CellFill::Shades)
        }
    }

    fn sync_cell_fill_controls(&mut self) {
        let own_fill = !self.is_checked(self.cell_use_default);
        let custom = own_fill && self.combo_index(self.cell_fill) == FILL_CUSTOM;
        self.set_visible(self.cell_fill_label, own_fill);
        self.set_visible(self.cell_fill, own_fill);
        self.set_visible(self.cell_char_label, custom);
        self.set_visible(self.cell_char, custom);
    }

    fn apply_cell_fill(&mut self) {
        if self.loading {
            return;
        }
        self.sync_cell_fill_controls();
        let Some((row, column)) = self.selected_cell() else {
            return;
        };
        let fill = self.cell_fill_choice();
        let idx = index(row, column);
        if idx >= self.fills.len() || self.fills[idx] == fill {
            return;
        }
        self.fills[idx] = fill;
        self.with_chart(|map| {
            map.update_cell(row, column, |cell| match fill {
                Some(fill) => cell.set_fill(fill),
                None => cell.clear_fill(),
            });
        });
    }

    fn refresh_detail(&mut self, value: i32) {
        let (min, max) = self.range().unwrap_or((value, value));
        let detail = format!("{}% of the range {min}..{max}", percentage(value, min, max));
        let handle = self.detail;
        if let Some(label) = self.control_mut(handle) {
            label.set_caption(&detail);
        }
    }

    fn refresh_selection(&mut self) {
        let Some((row, column)) = self.selected_cell() else {
            self.show_editor(false);
            return;
        };
        if self.values.get(index(row, column)).copied().flatten().is_some() {
            self.load_cell(row, column, false);
            return;
        }
        self.with_chart(|map| map.set_selected_cell(None));
        self.show_editor(false);
    }

    fn apply_cell_value(&mut self, value: i32) {
        if self.loading {
            return;
        }
        let Some((row, column)) = self.selected_cell() else {
            return;
        };
        let idx = index(row, column);
        if self.values.get(idx).copied().flatten() == Some(value) {
            return;
        }
        self.values[idx] = Some(value);
        self.write_cell(row, column);
        self.refresh_summary();
        self.refresh_detail(value);
        self.sync_text();
    }

    fn apply_cell_label(&mut self) {
        if self.loading {
            return;
        }
        let Some((row, column)) = self.selected_cell() else {
            return;
        };
        let text = self.control(self.cell_label).map(|field| field.text().to_string()).unwrap_or_default();
        let idx = index(row, column);
        if self.labels.get(idx).is_some_and(|label| label == &text) {
            return;
        }
        self.labels[idx] = text;
        let label = self.labels[idx].clone();
        self.with_chart(|map| map.update_cell(row, column, |cell| cell.set_label(&label)));
    }

    fn clear_selected(&mut self) {
        let Some((row, column)) = self.selected_cell() else {
            return;
        };
        let idx = index(row, column);
        self.values[idx] = None;
        self.labels[idx].clear();
        self.fills[idx] = None;
        self.write_cell(row, column);
        self.show_editor(false);
        self.refresh_summary();
        self.sync_text();
    }

    // ------------------------------------------------------------------ data

    fn sync_text(&mut self) {
        let text = format_values(&self.values);
        let handle = self.data_text;
        if let Some(area) = self.control_mut(handle) {
            if area.text() != text {
                area.set_text(&text);
            }
        }
    }

    fn current_pattern(&self) -> Pattern {
        PATTERNS
            .get(self.combo_index(self.pattern) as usize)
            .map(|(_, pattern)| *pattern)
            .unwrap_or(Pattern::Peaks)
    }

    fn load_pattern(&mut self) {
        let (values, labels) = pattern_data(self.current_pattern());
        self.values = values;
        self.labels = labels;
        self.fills = blank_fills();
        self.sync_text();
        self.sync_chart();
        self.refresh_summary();
        self.refresh_selection();
    }

    fn apply_data(&mut self) {
        let text = self.control(self.data_text).map(|area| area.text().to_string()).unwrap_or_default();
        let values = match parse_values(&text) {
            Ok(values) => values,
            Err(message) => {
                dialogs::message("Data", &message);
                return;
            }
        };
        for (idx, value) in values.iter().enumerate() {
            if value.is_none() {
                self.labels[idx].clear();
                self.fills[idx] = None;
            }
        }
        self.values = values;
        self.sync_text();
        self.sync_chart();
        self.refresh_summary();
        self.refresh_selection();
    }
}

impl HeatMapEvents<i32> for HeatMapEditor {
    fn on_cell_selected(&mut self, _handle: Handle<HeatMap<i32>>, row: u32, column: u32) -> EventProcessStatus {
        self.load_cell(row, column, true);
        EventProcessStatus::Processed
    }

    fn on_clear_selection(&mut self, _handle: Handle<HeatMap<i32>>) -> EventProcessStatus {
        self.show_editor(false);
        EventProcessStatus::Processed
    }
}

impl ComboBoxEvents for HeatMapEditor {
    fn on_selection_changed(&mut self, handle: Handle<ComboBox>) -> EventProcessStatus {
        if self.loading {
            return EventProcessStatus::Processed;
        }
        if handle == self.tooltip {
            self.apply_tooltip();
        } else if handle == self.default_fill {
            self.apply_default_fill();
        } else if handle == self.cell_fill {
            self.apply_cell_fill();
        } else if handle == self.pattern {
            self.load_pattern();
        } else {
            return EventProcessStatus::Ignored;
        }
        EventProcessStatus::Processed
    }
}

impl CheckBoxEvents for HeatMapEditor {
    fn on_status_changed(&mut self, handle: Handle<CheckBox>, _checked: bool) -> EventProcessStatus {
        if handle != self.cell_use_default {
            return EventProcessStatus::Ignored;
        }
        self.apply_cell_fill();
        EventProcessStatus::Processed
    }
}

impl CharPickerEvents for HeatMapEditor {
    fn on_char_changed(&mut self, handle: Handle<CharPicker>, _code: Option<char>) -> EventProcessStatus {
        if handle == self.default_char {
            self.apply_default_fill();
        } else if handle == self.cell_char {
            self.apply_cell_fill();
        } else {
            return EventProcessStatus::Ignored;
        }
        EventProcessStatus::Processed
    }
}

impl NumericSelectorEvents<i32> for HeatMapEditor {
    fn on_value_changed(&mut self, handle: Handle<NumericSelector<i32>>, value: i32) -> EventProcessStatus {
        if handle != self.cell_value {
            return EventProcessStatus::Ignored;
        }
        self.apply_cell_value(value);
        EventProcessStatus::Processed
    }
}

impl TextFieldEvents for HeatMapEditor {
    fn on_text_changed(&mut self, handle: Handle<TextField>) -> EventProcessStatus {
        if handle != self.cell_label {
            return EventProcessStatus::Ignored;
        }
        self.apply_cell_label();
        EventProcessStatus::Processed
    }
}

impl ButtonEvents for HeatMapEditor {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.data_apply {
            self.apply_data();
        } else if handle == self.data_reload {
            self.load_pattern();
        } else if handle == self.cell_clear {
            self.clear_selected();
        } else {
            return EventProcessStatus::Ignored;
        }
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::single_window(HeatMapEditor::new).title("Heat map editor").run()
}
