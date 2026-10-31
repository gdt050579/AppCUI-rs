use appcui::prelude::*;
use appcui::ui::ControlBase;
use std::ops::DerefMut;
use usagechart::{LabelContent, SortType, TotalUsage};

mod segment_window;

use segment_window::{char_picker, SegmentItem, SegmentWindow, DEFAULT_COLOR, FILL_CUSTOM, FILL_NAMES};

const SORTS: [(&str, SortType); 5] = [
    ("None (insertion order)", SortType::None),
    ("Highest to lowest", SortType::HighestToLowest),
    ("Lowest to highest", SortType::LowestToHighest),
    ("Alphabetical", SortType::Alphabetical),
    ("Reverse alphabetical", SortType::ReverseAlphabetical),
];

const LABEL_CONTENTS: [(&str, LabelContent); 5] = [
    ("Label", LabelContent::Label),
    ("Value", LabelContent::Value),
    ("Percentage", LabelContent::Percentage),
    ("Label and value", LabelContent::LabelAndValue),
    ("Label and percentage", LabelContent::LabelAndPercentage),
];

fn sample_segments() -> Vec<SegmentItem> {
    vec![
        SegmentItem::new("OS", 48, usagechart::SegmentFillType::Solid, None),
        SegmentItem::new("Apps", 36, usagechart::SegmentFillType::CrossHatch, None),
        SegmentItem::new("Documents", 22, usagechart::SegmentFillType::Grid, None),
        SegmentItem::new("Media", 22, usagechart::SegmentFillType::DiagonalDown, None),
        SegmentItem::new("Cache", 8, usagechart::SegmentFillType::GridDouble, None),
    ]
}

/// The chart sorts its segments before drawing, and a click reports that sorted index.
/// These are the same sorts, so the index can be mapped back to the segment list.
fn visual_order(segments: &[SegmentItem], sort: SortType) -> Vec<usize> {
    let mut order: Vec<usize> = (0..segments.len()).collect();
    order.sort_by(|&left, &right| segments[right].value.cmp(&segments[left].value));
    match sort {
        SortType::None => order.sort(),
        SortType::HighestToLowest => {}
        SortType::LowestToHighest => order.sort_by(|&left, &right| segments[left].value.cmp(&segments[right].value)),
        SortType::Alphabetical => order.sort_by(|&left, &right| segments[left].label.cmp(&segments[right].label)),
        SortType::ReverseAlphabetical => order.sort_by(|&left, &right| segments[right].label.cmp(&segments[left].label)),
    }
    order
}

#[Window(events = [UsageChartEvents, WindowEvents, ComboBoxEvents, CheckBoxEvents, ColorPickerEvents, CharPickerEvents, NumericSelectorEvents<u64>, TextFieldEvents, ButtonEvents, ListViewEvents<SegmentItem>])]
struct UsageChartExample {
    chart: Handle<UsageChart>,
    chart_host: Handle<Panel>,
    pages: Handle<Accordion>,
    // chart settings
    sort: Handle<ComboBox>,
    height: Handle<ComboBox>,
    total_kind: Handle<ComboBox>,
    total_value_label: Handle<Label>,
    total_value: Handle<NumericSelector<u64>>,
    margins: Handle<CheckBox>,
    dim: Handle<CheckBox>,
    // tooltip
    label_content: Handle<ComboBox>,
    number_base: Handle<ComboBox>,
    number_group: Handle<ComboBox>,
    number_style: Handle<ComboBox>,
    // selected segment
    empty_panel: Handle<Panel>,
    editor_panel: Handle<Panel>,
    info: Handle<Label>,
    segment_label: Handle<TextField>,
    segment_value: Handle<NumericSelector<u64>>,
    segment_fill: Handle<ComboBox>,
    character_label: Handle<Label>,
    character: Handle<CharPicker>,
    segment_automatic: Handle<CheckBox>,
    segment_color: Handle<ColorPicker>,
    segment_delete: Handle<Button>,
    // data
    default_value: Handle<NumericSelector<u64>>,
    segments: Handle<ListView<SegmentItem>>,
    segments_add: Handle<Button>,
    segments_delete: Handle<Button>,
    segments_clear: Handle<Button>,
    // the segment shown in the "Selected segment" page. It stays edited when the chart loses
    // the focus, so that its settings can be changed. This is an index into the segment list
    // (insertion order), not the sorted index reported by the chart.
    edited: Option<usize>,
    // set while the editor fills its own controls, so that their events are ignored
    loading: bool,
}

impl UsageChartExample {
    fn new() -> Self {
        let mut win = Self {
            base: window!("'Usage chart example',d:f"),
            chart: Handle::None,
            chart_host: Handle::None,
            pages: Handle::None,
            sort: Handle::None,
            height: Handle::None,
            total_kind: Handle::None,
            total_value_label: Handle::None,
            total_value: Handle::None,
            margins: Handle::None,
            dim: Handle::None,
            label_content: Handle::None,
            number_base: Handle::None,
            number_group: Handle::None,
            number_style: Handle::None,
            empty_panel: Handle::None,
            editor_panel: Handle::None,
            info: Handle::None,
            segment_label: Handle::None,
            segment_value: Handle::None,
            segment_fill: Handle::None,
            character_label: Handle::None,
            character: Handle::None,
            segment_automatic: Handle::None,
            segment_color: Handle::None,
            segment_delete: Handle::None,
            default_value: Handle::None,
            segments: Handle::None,
            segments_add: Handle::None,
            segments_delete: Handle::None,
            segments_clear: Handle::None,
            edited: None,
            loading: false,
        };

        let mut splitter = vsplitter!("pos:65%,d:f,resize:PreserveRightPanelSize,min-left-width:30,min-right-width:36");

        let samples = sample_segments();
        let mut chart = UsageChart::new(layout!("d:f"), usagechart::Flags::Margins | usagechart::Flags::DimSegmentsOnSelection);
        chart.set_total_usage(TotalUsage::Fixed(256));
        chart.set_label_content(LabelContent::LabelAndPercentage);
        chart.set_number_format(FormatNumber::new(10).group(3, b',').suffix(" GB"));
        chart.add_segments(samples.iter().map(SegmentItem::to_segment));
        // the page is the chart's parent, so a fixed height leaves the rest of the left side empty
        let mut host = panel!("'',d:f,type:Page");
        win.chart = host.add(chart);
        win.chart_host = splitter.add(vsplitter::Panel::Left, host);

        let mut pages = accordion!("d:f,panels:['&Chart Settings','&Tooltip','Selected Se&gment','Da&ta']");

        // chart settings
        pages.add(0, label!("'Sort',l:1,t:1,r:1,h:1"));
        let mut sort = ComboBox::new(layout!("l:1,t:2,r:1"), combobox::Flags::None);
        for (name, _) in SORTS {
            sort.add(name);
        }
        sort.set_index(0);
        win.sort = pages.add(0, sort);
        pages.add(0, label!("'Height',l:1,t:4,r:1,h:1"));
        win.height = pages.add(0, combobox!("l:1,t:5,r:1,items:[Fill],index:0"));
        pages.add(0, label!("'Total',l:1,t:7,r:1,h:1"));
        win.total_kind = pages.add(0, combobox!("l:1,t:8,r:1,items:[Auto,Fixed],index:1"));
        win.total_value_label = pages.add(0, label!("'Value',l:1,t:10,r:1,h:1"));
        win.total_value = pages.add(0, numericselector!("u64,256,0,1000000,1,l:1,t:11,r:1"));
        win.margins = pages.add(0, checkbox!("'Margins',l:1,t:13,r:1,checked:true"));
        win.dim = pages.add(0, checkbox!("'Dim unselected segments',l:1,t:15,r:1,checked:true"));

        // tooltip
        pages.add(1, label!("'Content',l:1,t:1,r:1,h:1"));
        let mut content = ComboBox::new(layout!("l:1,t:2,r:1"), combobox::Flags::None);
        for (name, _) in LABEL_CONTENTS {
            content.add(name);
        }
        content.set_index(4);
        win.label_content = pages.add(1, content);
        let mut format = panel!("'Number format (values)',l:1,t:4,r:1,h:11");
        format.add(label!("'Base',l:1,t:0,w:12"));
        win.number_base = format.add(combobox!("l:1,t:1,r:1,items:[Decimal,Hex,Octal,Binary],index:0"));
        format.add(label!("'Grouping',l:1,t:3,w:12"));
        win.number_group = format.add(combobox!("l:1,t:4,r:1,items:[None,'3 digits','4 digits'],index:1"));
        format.add(label!("'Style',l:1,t:6,w:12"));
        win.number_style = format.add(combobox!("l:1,t:7,r:1,items:[None,USD,Euro,GBP,Yen,' MB',' kg',' GB'],index:7"));
        pages.add(1, format);

        // selected segment
        let mut empty = panel!("'',d:f,type:Page");
        empty.add(label!(
            "'Click a segment (or select one with the left and right keys) to configure it.',l:1,t:1,r:1,b:1"
        ));
        win.empty_panel = pages.add(2, empty);
        let mut editor = panel!("'',d:f,type:Page");
        editor.set_visible(false);
        win.info = editor.add(label!("'',l:1,t:1,r:1,h:2"));
        editor.add(label!("'Label',l:1,t:4,w:12"));
        win.segment_label = editor.add(textfield!("'',l:1,t:5,r:1"));
        editor.add(label!("'Value',l:1,t:7,w:12"));
        win.segment_value = editor.add(numericselector!("u64,1,0,1000000,1,l:1,t:8,r:1"));
        editor.add(label!("'Fill',l:1,t:10,w:12"));
        let mut fill = ComboBox::new(layout!("l:1,t:11,r:1"), combobox::Flags::None);
        for name in FILL_NAMES {
            fill.add(name);
        }
        win.segment_fill = editor.add(fill);
        win.character_label = editor.add(label!("'Character',l:1,t:13,w:12"));
        win.character = editor.add(char_picker(layout!("l:1,t:14,r:1")));
        win.segment_automatic = editor.add(checkbox!("'Automatic color',l:1,t:16,r:1,checked:true"));
        let mut color = ColorPicker::new(DEFAULT_COLOR, layout!("l:1,t:17,r:1"));
        color.set_enabled(false);
        win.segment_color = editor.add(color);
        win.segment_delete = editor.add(button!("'&Delete segment',l:1,b:0,w:18"));
        win.editor_panel = pages.add(2, editor);

        // data
        pages.add(3, label!("'Default value (for new segments)',l:1,t:1,r:1,h:1"));
        win.default_value = pages.add(3, numericselector!("u64,10,0,1000000,1,l:1,t:2,r:1"));
        let mut segments_panel = panel!("'Segments',l:1,t:4,r:1,b:3");
        let mut segments = listview!("class: SegmentItem,d:f,flags:ScrollBars");
        segments.add_items(samples);
        win.segments = segments_panel.add(segments);
        pages.add(3, segments_panel);
        win.segments_add = pages.add(3, button!("'&Add',l:1,b:0,w:8"));
        win.segments_delete = pages.add(3, button!("'&Del',l:10,b:0,w:8"));
        win.segments_clear = pages.add(3, button!("'&Clear',l:19,b:0,w:8"));

        win.pages = splitter.add(vsplitter::Panel::Right, pages);
        win.add(splitter);
        win
    }

    // ---- helpers ------------------------------------------------------------------------

    fn combo_index(&self, handle: Handle<ComboBox>) -> u32 {
        self.control(handle).and_then(|combo| combo.index()).unwrap_or(0)
    }

    fn update_chart<F>(&mut self, f: F)
    where
        F: FnOnce(&mut UsageChart),
    {
        let chart = self.chart;
        if let Some(ctrl) = self.control_mut(chart) {
            f(ctrl);
        }
    }

    fn set_visible<T: Control + DerefMut<Target = ControlBase> + 'static>(&mut self, handle: Handle<T>, visible: bool) {
        if let Some(control) = self.control_mut(handle) {
            control.set_visible(visible);
        }
    }

    fn set_enabled<T: Control + DerefMut<Target = ControlBase> + 'static>(&mut self, handle: Handle<T>, enabled: bool) {
        if let Some(control) = self.control_mut(handle) {
            control.set_enabled(enabled);
        }
    }

    fn current_sort(&self) -> SortType {
        SORTS[(self.combo_index(self.sort) as usize).min(SORTS.len() - 1)].1
    }

    fn current_total(&self) -> TotalUsage {
        if self.combo_index(self.total_kind) == 1 {
            let value = self.control(self.total_value).map(|selector| selector.value()).unwrap_or(256);
            TotalUsage::Fixed(value)
        } else {
            TotalUsage::Auto
        }
    }

    fn visual_positions(&self) -> Vec<usize> {
        visual_order(&self.current_segments(), self.current_sort())
    }

    // ---- chart settings -----------------------------------------------------------------

    fn margins_enabled(&self) -> bool {
        self.control(self.margins).is_some_and(|check| check.is_checked())
    }

    /// Smallest height the chart can take: margins need two extra rows around the bar.
    fn min_chart_height(&self) -> u16 {
        if self.margins_enabled() { 3 } else { 1 }
    }

    fn apply_height(&mut self) {
        if self.loading {
            return;
        }
        let count = self.control(self.height).map(|combo| combo.count()).unwrap_or(0);
        let index = self.combo_index(self.height);
        let layout = if count == 0 || index + 1 >= count {
            layout!("d:f")
        } else {
            let height = self.min_chart_height() + index as u16;
            LayoutBuilder::new().dock(Dock::Top).height(height).build()
        };
        let margins = self.margins_enabled();
        // update_layout clears the size bounds, so they are put back the way the chart sets them
        self.update_chart(|chart| {
            chart.update_layout(layout);
            if margins {
                chart.set_size_bounds(3, 3, u16::MAX, u16::MAX);
            } else {
                chart.set_size_bounds(3, 1, u16::MAX, u16::MAX);
            }
        });
    }

    /// Lists every height from the minimum (1, or 3 when margins are on) up to the parent. Fill is last and keeps tracking the parent.
    fn refresh_height_choices(&mut self, parent_height: u16) {
        if parent_height == 0 {
            return;
        }
        let min_height = self.min_chart_height();
        let numeric = if parent_height >= min_height { (parent_height - min_height + 1) as u32 } else { 0 };
        let desired = numeric + 1;
        let count = self.control(self.height).map(|combo| combo.count()).unwrap_or(0);
        if count == desired {
            return;
        }
        let selected = self.control(self.height).map(|combo| combo.value().to_string()).unwrap_or_default();
        let selected_height = selected.parse::<u16>().ok();
        let handle = self.height;
        self.loading = true;
        if let Some(combo) = self.control_mut(handle) {
            combo.clear();
            if parent_height >= min_height {
                for value in min_height..=parent_height {
                    combo.add(&value.to_string());
                }
            }
            combo.add("Fill");
            let index = match selected_height {
                Some(height) if parent_height >= min_height => (height.clamp(min_height, parent_height) - min_height) as u32,
                _ => numeric,
            };
            combo.set_index(index);
        }
        self.loading = false;
        if selected_height.is_some_and(|height| height < min_height || height > parent_height) {
            self.apply_height();
        }
    }

    fn apply_sort(&mut self) {
        let sort = self.current_sort();
        self.update_chart(|chart| chart.set_sort_type(sort));
        self.reselect_edited();
    }

    fn apply_total(&mut self) {
        let fixed = self.combo_index(self.total_kind) == 1;
        self.set_enabled(self.total_value_label, fixed);
        self.set_enabled(self.total_value, fixed);
        let total = self.current_total();
        self.update_chart(|chart| chart.set_total_usage(total));
        if let Some(index) = self.edited {
            self.load_segment(index);
        }
    }

    fn apply_flags(&mut self) {
        let mut flags = usagechart::Flags::None;
        if self.control(self.margins).is_some_and(|check| check.is_checked()) {
            flags |= usagechart::Flags::Margins;
        }
        if self.control(self.dim).is_some_and(|check| check.is_checked()) {
            flags |= usagechart::Flags::DimSegmentsOnSelection;
        }
        self.update_chart(|chart| chart.set_flags(flags));
        let height = u16::try_from(self.client_size().height).unwrap_or(u16::MAX);
        self.refresh_height_choices(height);
    }

    // ---- tooltip --------------------------------------------------------------------------

    fn apply_label_content(&mut self) {
        let content = LABEL_CONTENTS[(self.combo_index(self.label_content) as usize).min(LABEL_CONTENTS.len() - 1)].1;
        self.update_chart(|chart| chart.set_label_content(content));
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
        format = match self.combo_index(self.number_style) {
            1 => format.prefix("$"),
            2 => format.prefix("€"),
            3 => format.prefix("£"),
            4 => format.prefix("¥"),
            5 => format.suffix(" MB"),
            6 => format.suffix(" kg"),
            7 => format.suffix(" GB"),
            _ => format,
        };
        self.update_chart(|chart| chart.set_number_format(format));
    }

    // ---- data -----------------------------------------------------------------------------

    fn current_segments(&self) -> Vec<SegmentItem> {
        let Some(list) = self.control(self.segments) else {
            return Vec::new();
        };
        (0..list.items_count()).filter_map(|index| list.item(index).cloned()).collect()
    }

    fn set_segments(&mut self, segments: Vec<SegmentItem>) {
        let handle = self.segments;
        if let Some(list) = self.control_mut(handle) {
            list.clear();
            list.add_items(segments);
        }
        self.rebuild_chart();
    }

    fn reselect_edited(&mut self) {
        let visual = self
            .edited
            .and_then(|index| self.visual_positions().into_iter().position(|item| item == index));
        self.update_chart(|chart| chart.set_selected_segment(visual));
    }

    // the chart is rebuilt from the list of segments; the edited segment is selected again
    fn rebuild_chart(&mut self) {
        let segments = self.current_segments();
        if self.edited.is_some_and(|index| index >= segments.len()) {
            self.edited = None;
        }
        let built: Vec<_> = segments.iter().map(SegmentItem::to_segment).collect();
        self.update_chart(|chart| {
            chart.clear();
            chart.add_segments(built);
        });
        self.reselect_edited();
        match self.edited {
            Some(index) => self.load_segment(index),
            None => self.show_segment_editor(false),
        }
    }

    fn add_segment(&mut self) {
        let value = self.control(self.default_value).map(|selector| selector.value()).unwrap_or(10);
        let draft = SegmentItem::new(
            &format!("Segment {}", self.current_segments().len() + 1),
            value,
            usagechart::SegmentFillType::Solid,
            None,
        );
        let Some(segment) = SegmentWindow::new("Add segment", &draft).show() else {
            return;
        };
        let mut segments = self.current_segments();
        segments.push(segment);
        self.set_segments(segments);
    }

    fn delete_segment(&mut self, index: usize) {
        let mut segments = self.current_segments();
        if index >= segments.len() {
            return;
        }
        segments.remove(index);
        self.edited = match self.edited {
            Some(edited) if edited == index => None,
            Some(edited) if edited > index => Some(edited - 1),
            other => other,
        };
        self.set_segments(segments);
    }

    fn delete_current_list_segment(&mut self) {
        let Some(index) = self.control(self.segments).and_then(|list| list.current_item_index()) else {
            dialogs::message("Segments", "Select a segment to delete.");
            return;
        };
        self.delete_segment(index);
    }

    fn clear_segments(&mut self) {
        if self.current_segments().is_empty() || !dialogs::validate("Segments", "Remove all the segments?") {
            return;
        }
        self.edited = None;
        self.set_segments(Vec::new());
    }

    fn edit_list_segment(&mut self, index: usize) {
        let Some(current) = self.current_segments().get(index).cloned() else {
            return;
        };
        let Some(updated) = SegmentWindow::new("Edit segment", &current).show() else {
            return;
        };
        self.update_segment(index, |segment| *segment = updated);
    }

    fn update_segment<F>(&mut self, index: usize, f: F)
    where
        F: FnOnce(&mut SegmentItem),
    {
        let handle = self.segments;
        if let Some(item) = self.control_mut(handle).and_then(|list| list.item_mut(index)) {
            f(item);
        }
        self.rebuild_chart();
    }

    // ---- selected segment -----------------------------------------------------------------

    fn show_segment_editor(&mut self, show: bool) {
        self.set_visible(self.empty_panel, !show);
        self.set_visible(self.editor_panel, show);
    }

    fn load_segment(&mut self, index: usize) {
        let segments = self.current_segments();
        let Some(segment) = segments.get(index).cloned() else {
            self.edited = None;
            self.show_segment_editor(false);
            return;
        };
        let total = match self.current_total() {
            TotalUsage::Fixed(value) => value as u128,
            TotalUsage::Auto => segments.iter().map(|item| item.value as u128).sum(),
        };
        let percent = if total > 0 { segment.value as f64 * 100.0 / total as f64 } else { 0.0 };
        let caption = if segment.label.is_empty() {
            format!("Segment {}: {} ({percent:.2}% of {total})", index + 1, segment.value)
        } else {
            format!("Segment {} ({}): {} ({percent:.2}% of {total})", index + 1, segment.label, segment.value)
        };
        let (info, segment_label, segment_value, segment_fill, character, segment_automatic, segment_color) = (
            self.info,
            self.segment_label,
            self.segment_value,
            self.segment_fill,
            self.character,
            self.segment_automatic,
            self.segment_color,
        );
        self.loading = true;
        if let Some(label) = self.control_mut(info) {
            label.set_caption(&caption);
        }
        if let Some(field) = self.control_mut(segment_label) {
            if field.text() != segment.label {
                field.set_text(&segment.label);
            }
        }
        if let Some(selector) = self.control_mut(segment_value) {
            if selector.value() != segment.value {
                selector.set_value(segment.value);
            }
        }
        if let Some(combo) = self.control_mut(segment_fill) {
            if combo.index() != Some(segment.fill_index) {
                combo.set_index(segment.fill_index);
            }
        }
        let custom = segment.fill_index == FILL_CUSTOM;
        self.set_enabled(self.character_label, custom);
        if let Some(picker) = self.control_mut(character) {
            picker.set_enabled(custom);
            if picker.char() != Some(segment.mark) {
                picker.select_char(segment.mark);
            }
        }
        if let Some(check) = self.control_mut(segment_automatic) {
            check.set_checked(segment.color.is_none());
        }
        if let Some(picker) = self.control_mut(segment_color) {
            picker.set_color(segment.color.unwrap_or(DEFAULT_COLOR));
            picker.set_enabled(segment.color.is_some());
        }
        self.loading = false;
        self.show_segment_editor(true);
    }

    fn edit_segment(&mut self, visual: usize) {
        let Some(index) = self.visual_positions().get(visual).copied() else {
            return;
        };
        self.edited = Some(index);
        self.load_segment(index);
        let pages = self.pages;
        if let Some(accordion) = self.control_mut(pages) {
            accordion.set_current_panel(2);
        }
    }

    fn update_edited_segment<F>(&mut self, f: F)
    where
        F: FnOnce(&mut SegmentItem),
    {
        if self.loading {
            return;
        }
        if let Some(index) = self.edited {
            self.update_segment(index, f);
        }
    }

    fn edited_mark(&self) -> char {
        self.control(self.character)
            .and_then(|picker| picker.char())
            .unwrap_or(segment_window::DEFAULT_MARK)
    }

    fn edited_color(&self) -> Option<Color> {
        let automatic = self.control(self.segment_automatic).map(|check| check.is_checked()).unwrap_or(true);
        if automatic {
            None
        } else {
            self.control(self.segment_color).map(|picker| picker.color())
        }
    }
}

impl WindowEvents for UsageChartExample {
    fn on_layout_changed(&mut self, _old_layout: Rect, _new_layout: Rect) {
        let height = u16::try_from(self.client_size().height).unwrap_or(u16::MAX);
        self.refresh_height_choices(height);
    }
}

impl UsageChartEvents for UsageChartExample {
    fn on_usage_segment_selected(&mut self, _handle: Handle<UsageChart>, index: usize) -> EventProcessStatus {
        self.edit_segment(index);
        EventProcessStatus::Processed
    }

    fn on_clear_selection(&mut self, handle: Handle<UsageChart>) -> EventProcessStatus {
        if self.control(handle).is_some_and(|chart| chart.has_focus()) {
            self.edited = None;
            self.show_segment_editor(false);
        } else if let Some(index) = self.edited {
            // keep the segment selected when the chart loses the focus to one of the editor's controls
            let visual = self.visual_positions().into_iter().position(|item| item == index);
            self.update_chart(|chart| chart.set_selected_segment(visual));
        }
        EventProcessStatus::Processed
    }
}

impl ComboBoxEvents for UsageChartExample {
    fn on_selection_changed(&mut self, handle: Handle<ComboBox>) -> EventProcessStatus {
        if self.loading {
            return EventProcessStatus::Processed;
        }
        match () {
            _ if handle == self.sort => self.apply_sort(),
            _ if handle == self.height => self.apply_height(),
            _ if handle == self.total_kind => self.apply_total(),
            _ if handle == self.label_content => self.apply_label_content(),
            _ if handle == self.number_base || handle == self.number_group || handle == self.number_style => self.apply_number_format(),
            _ if handle == self.segment_fill => {
                let index = self.combo_index(self.segment_fill).min(FILL_CUSTOM);
                let mark = self.edited_mark();
                self.update_edited_segment(|segment| segment.set_fill(index, mark));
            }
            _ => return EventProcessStatus::Ignored,
        }
        EventProcessStatus::Processed
    }
}

impl CharPickerEvents for UsageChartExample {
    fn on_char_changed(&mut self, handle: Handle<CharPicker>, _code: Option<char>) -> EventProcessStatus {
        if handle != self.character {
            return EventProcessStatus::Ignored;
        }
        let index = self.combo_index(self.segment_fill).min(FILL_CUSTOM);
        let mark = self.edited_mark();
        self.update_edited_segment(|segment| segment.set_fill(index, mark));
        EventProcessStatus::Processed
    }
}

impl TextFieldEvents for UsageChartExample {
    fn on_text_changed(&mut self, handle: Handle<TextField>) -> EventProcessStatus {
        if handle != self.segment_label {
            return EventProcessStatus::Ignored;
        }
        let label = self.control(handle).map(|field| field.text().to_string()).unwrap_or_default();
        self.update_edited_segment(|segment| segment.label = label);
        EventProcessStatus::Processed
    }
}

impl NumericSelectorEvents<u64> for UsageChartExample {
    fn on_value_changed(&mut self, handle: Handle<NumericSelector<u64>>, value: u64) -> EventProcessStatus {
        match () {
            _ if handle == self.total_value => self.apply_total(),
            _ if handle == self.segment_value => self.update_edited_segment(|segment| segment.value = value),
            _ => return EventProcessStatus::Ignored,
        }
        EventProcessStatus::Processed
    }
}

impl CheckBoxEvents for UsageChartExample {
    fn on_status_changed(&mut self, handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
        if handle == self.margins || handle == self.dim {
            self.apply_flags();
            return EventProcessStatus::Processed;
        }
        if handle != self.segment_automatic {
            return EventProcessStatus::Ignored;
        }
        self.set_enabled(self.segment_color, !checked);
        let color = self.edited_color();
        self.update_edited_segment(|segment| segment.set_color(color));
        EventProcessStatus::Processed
    }
}

impl ColorPickerEvents for UsageChartExample {
    fn on_color_changed(&mut self, handle: Handle<ColorPicker>, _color: Color) -> EventProcessStatus {
        if handle != self.segment_color {
            return EventProcessStatus::Ignored;
        }
        let color = self.edited_color();
        self.update_edited_segment(|segment| segment.set_color(color));
        EventProcessStatus::Processed
    }
}

impl ButtonEvents for UsageChartExample {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        match () {
            _ if handle == self.segments_add => self.add_segment(),
            _ if handle == self.segments_delete => self.delete_current_list_segment(),
            _ if handle == self.segments_clear => self.clear_segments(),
            _ if handle == self.segment_delete => {
                if let Some(index) = self.edited {
                    self.delete_segment(index);
                }
            }
            _ => return EventProcessStatus::Ignored,
        }
        EventProcessStatus::Processed
    }
}

impl ListViewEvents<SegmentItem> for UsageChartExample {
    fn on_item_action(&mut self, handle: Handle<ListView<SegmentItem>>, index: usize) -> EventProcessStatus {
        if handle == self.segments {
            self.edit_list_segment(index);
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::single_window(UsageChartExample::new).title("Usage chart example").run()
}
