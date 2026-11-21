use appcui::prelude::*;
use appcui::ui::ControlBase;
use std::ops::DerefMut;
use piechart::{Direction, DrawModeRenderer, DrawModeShape, HollowThickness, LabelContent, LabelLayout};

mod slice_window;

use slice_window::{SliceItem, SliceWindow};

const DEFAULT_MARK: char = '*';
const DEFAULT_COLOR: Color = Color::Aqua;

const SHAPES: [(&str, DrawModeShape); 8] = [
    ("Circle", DrawModeShape::Circle),
    ("Ellipse", DrawModeShape::Ellipse),
    ("Square", DrawModeShape::Square),
    ("Rectangle", DrawModeShape::Rectangle),
    ("Round square", DrawModeShape::RoundSquare),
    ("Round rectangle", DrawModeShape::RoundRectangle),
    ("Triangle", DrawModeShape::Triangle),
    ("Round triangle", DrawModeShape::RoundTriangle),
];

const RENDERERS: [&str; 12] = [
    "Half blocks",
    "Braille",
    "Solid",
    "Shade 75%",
    "Shade 50%",
    "Shade 25%",
    "Grid",
    "Grid double",
    "Cross-hatch",
    "Diagonal up",
    "Diagonal down",
    "Custom",
];
const RENDERER_CUSTOM: u32 = 11;

fn renderer(index: u32, mark: char) -> DrawModeRenderer {
    match index {
        1 => DrawModeRenderer::Braille,
        2 => DrawModeRenderer::Solid,
        3 => DrawModeRenderer::Shade75,
        4 => DrawModeRenderer::Shade50,
        5 => DrawModeRenderer::Shade25,
        6 => DrawModeRenderer::Grid,
        7 => DrawModeRenderer::GridDouble,
        8 => DrawModeRenderer::CrossHatch,
        9 => DrawModeRenderer::DiagonalUp,
        10 => DrawModeRenderer::DiagonalDown,
        RENDERER_CUSTOM => DrawModeRenderer::Custom(mark),
        _ => DrawModeRenderer::HalfBlocks,
    }
}

const LABEL_CONTENTS: [(&str, LabelContent); 5] = [
    ("Label", LabelContent::Label),
    ("Value", LabelContent::Value),
    ("Percentage", LabelContent::Percentage),
    ("Label and value", LabelContent::LabelAndValue),
    ("Label and percentage", LabelContent::LabelAndPercentage),
];

fn sample_slices() -> Vec<SliceItem> {
    vec![
        SliceItem::new("Rust", 45, None),
        SliceItem::new("C++", 25, None),
        SliceItem::new("Go", 15, None),
        SliceItem::new("Zig", 10, None),
        SliceItem::new("Other", 5, Some(Color::Gray)),
    ]
}

fn char_picker(layout: Layout) -> CharPicker {
    let mut picker = CharPicker::new(Some(DEFAULT_MARK), layout);
    picker.clear_sets();
    for (name, symbols) in [
        ("Ascii", charpicker::UnicodeSymbols::Ascii),
        ("Blocks", charpicker::UnicodeSymbols::Blocks),
        ("BoxDrawing", charpicker::UnicodeSymbols::BoxDrawing),
        ("Braille", charpicker::UnicodeSymbols::Braille),
        ("Shapes", charpicker::UnicodeSymbols::Shapes),
        ("Arrows", charpicker::UnicodeSymbols::Arrows),
        ("Math", charpicker::UnicodeSymbols::Math),
        ("Games", charpicker::UnicodeSymbols::Games),
        ("Emoticons", charpicker::UnicodeSymbols::Emoticons),
    ] {
        picker.add_set(charpicker::Set::from_unicode_symbols(name, symbols));
    }
    picker.select_char(DEFAULT_MARK);
    picker
}

#[Window(events = [PieChartEvents, ComboBoxEvents, HSliderEvents<u16>, CheckBoxEvents, ColorPickerEvents, CharPickerEvents, NumericSelectorEvents<u64>, TextFieldEvents, ButtonEvents, ListViewEvents<SliceItem>])]
struct PieChartEditor {
    chart: Handle<PieChart>,
    pages: Handle<Accordion>,
    // chart settings
    shape: Handle<ComboBox>,
    renderer: Handle<ComboBox>,
    character_label: Handle<Label>,
    character: Handle<CharPicker>,
    hole_type: Handle<ComboBox>,
    hole_size_label: Handle<Label>,
    hole_size: Handle<HSlider<u16>>,
    start_angle: Handle<HSlider<u16>>,
    direction: Handle<ComboBox>,
    // labels
    label_layout: Handle<ComboBox>,
    label_content: Handle<ComboBox>,
    number_base: Handle<ComboBox>,
    number_group: Handle<ComboBox>,
    number_decimals: Handle<HSlider<u16>>,
    number_style: Handle<ComboBox>,
    // selected slice
    empty_panel: Handle<Panel>,
    editor_panel: Handle<Panel>,
    info: Handle<Label>,
    slice_label: Handle<TextField>,
    slice_value: Handle<NumericSelector<u64>>,
    slice_automatic: Handle<CheckBox>,
    slice_color: Handle<ColorPicker>,
    slice_delete: Handle<Button>,
    // data
    default_value: Handle<NumericSelector<u64>>,
    slices: Handle<ListView<SliceItem>>,
    slices_add: Handle<Button>,
    slices_delete: Handle<Button>,
    slices_clear: Handle<Button>,
    // the slice shown in the "Selected slice" page; it stays edited when the chart loses
    // the focus (which clears its selection), so that its settings can be changed
    edited: Option<usize>,
    // set while the editor fills its own controls, so that their events are ignored
    loading: bool,
}

impl PieChartEditor {
    fn new() -> Self {
        let mut win = Self {
            base: window!("'Pie chart editor',d:f"),
            chart: Handle::None,
            pages: Handle::None,
            shape: Handle::None,
            renderer: Handle::None,
            character_label: Handle::None,
            character: Handle::None,
            hole_type: Handle::None,
            hole_size_label: Handle::None,
            hole_size: Handle::None,
            start_angle: Handle::None,
            direction: Handle::None,
            label_layout: Handle::None,
            label_content: Handle::None,
            number_base: Handle::None,
            number_group: Handle::None,
            number_decimals: Handle::None,
            number_style: Handle::None,
            empty_panel: Handle::None,
            editor_panel: Handle::None,
            info: Handle::None,
            slice_label: Handle::None,
            slice_value: Handle::None,
            slice_automatic: Handle::None,
            slice_color: Handle::None,
            slice_delete: Handle::None,
            default_value: Handle::None,
            slices: Handle::None,
            slices_add: Handle::None,
            slices_delete: Handle::None,
            slices_clear: Handle::None,
            edited: None,
            loading: false,
        };

        let mut splitter = vsplitter!("pos:65%,d:f,resize:PreserveRightPanelSize,min-left-width:30,min-right-width:36");

        let mut chart = PieChart::new(layout!("d:f"), piechart::Flags::DimSlicesOnSelection);
        for slice in sample_slices() {
            chart.add_value(slice.value, &slice.label, slice.color.map(CharAttribute::with_fore_color));
        }
        chart.set_label_layout(LabelLayout::Outside);
        chart.set_label_content(LabelContent::LabelAndPercentage);
        win.chart = splitter.add(vsplitter::Panel::Left, chart);

        let mut pages = accordion!("d:f,panels:['&Chart Settings','&Labels','Selected Sl&ice','Da&ta']");

        // chart settings
        pages.add(0, label!("'Shape',l:1,t:1,r:1,h:1"));
        let mut shape = ComboBox::new(layout!("l:1,t:2,r:1"), combobox::Flags::None);
        for (name, _) in SHAPES {
            shape.add(name);
        }
        shape.set_index(0);
        win.shape = pages.add(0, shape);
        pages.add(0, label!("'Renderer',l:1,t:4,r:1,h:1"));
        let mut renderer = ComboBox::new(layout!("l:1,t:5,r:1"), combobox::Flags::None);
        for name in RENDERERS {
            renderer.add(name);
        }
        renderer.set_index(0);
        win.renderer = pages.add(0, renderer);
        let mut character_label = label!("'Character',l:1,t:7,r:1,h:1");
        character_label.set_visible(false);
        win.character_label = pages.add(0, character_label);
        let mut character = char_picker(layout!("l:1,t:8,r:1"));
        character.set_visible(false);
        win.character = pages.add(0, character);
        pages.add(0, label!("'Hole',l:1,t:10,r:1,h:1"));
        win.hole_type = pages.add(
            0,
            combobox!("l:1,t:11,r:1,items:['None','Percentage','Absolute (characters)'],index:0"),
        );
        let mut hole_size_label = label!("'Size',l:1,t:13,r:1,h:1");
        hole_size_label.set_enabled(false);
        win.hole_size_label = pages.add(0, hole_size_label);
        let mut hole_size = hslider!("u16,0,100,1,l:1,t:14,r:1,flags:ShowValue,type:Ruler");
        hole_size.set_value(50);
        hole_size.set_enabled(false);
        win.hole_size = pages.add(0, hole_size);
        pages.add(0, label!("'Start angle (degrees)',l:1,t:16,r:1,h:1"));
        win.start_angle = pages.add(0, hslider!("u16,0,360,15,l:1,t:17,r:1,flags:ShowValue,type:Ruler"));
        pages.add(0, label!("'Direction',l:1,t:19,r:1,h:1"));
        win.direction = pages.add(0, combobox!("l:1,t:20,r:1,items:['Clockwise','Counter-clockwise'],index:0"));

        // labels
        pages.add(1, label!("'Layout',l:1,t:1,r:1,h:1"));
        win.label_layout = pages.add(1, combobox!("l:1,t:2,r:1,items:['None','Inside','Outside'],index:2"));
        pages.add(1, label!("'Content',l:1,t:4,r:1,h:1"));
        let mut content = ComboBox::new(layout!("l:1,t:5,r:1"), combobox::Flags::None);
        for (name, _) in LABEL_CONTENTS {
            content.add(name);
        }
        content.set_index(4);
        win.label_content = pages.add(1, content);
        let mut format = panel!("'Number format (values)',l:1,t:7,r:1,h:14");
        format.add(label!("'Base',l:1,t:0,w:12"));
        win.number_base = format.add(combobox!("l:1,t:1,r:1,items:[Decimal,Hex,Octal,Binary],index:0"));
        format.add(label!("'Grouping',l:1,t:3,w:12"));
        win.number_group = format.add(combobox!("l:1,t:4,r:1,items:[None,'3 digits','4 digits'],index:1"));
        format.add(label!("'Decimals',l:1,t:6,w:12"));
        win.number_decimals = format.add(hslider!("u16,0,8,1,l:1,t:7,r:1,flags:ShowValue,type:Ruler"));
        format.add(label!("'Style',l:1,t:9,w:12"));
        win.number_style = format.add(combobox!("l:1,t:10,r:1,items:[None,USD,Euro,GBP,Yen,' MB',' kg'],index:0"));
        pages.add(1, format);

        // selected slice
        let mut empty = panel!("'',d:f,type:Page");
        empty.add(label!("'Click on a slice (or select one with the arrow keys) to configure it.',l:1,t:1,r:1,b:1"));
        win.empty_panel = pages.add(2, empty);
        let mut editor = panel!("'',d:f,type:Page");
        editor.set_visible(false);
        win.info = editor.add(label!("'',l:1,t:1,r:1,h:2"));
        editor.add(label!("'Label',l:1,t:4,w:12"));
        win.slice_label = editor.add(textfield!("'',l:1,t:5,r:1"));
        editor.add(label!("'Value',l:1,t:7,w:12"));
        win.slice_value = editor.add(numericselector!("u64,1,0,1000000,1,l:1,t:8,r:1"));
        win.slice_automatic = editor.add(checkbox!("'Automatic color',l:1,t:10,r:1,checked:true"));
        let mut color = ColorPicker::new(DEFAULT_COLOR, layout!("l:1,t:11,r:1"));
        color.set_enabled(false);
        win.slice_color = editor.add(color);
        win.slice_delete = editor.add(button!("'&Delete slice',l:1,b:0,w:16"));
        win.editor_panel = pages.add(2, editor);

        // data
        pages.add(3, label!("'Default value (for new slices)',l:1,t:1,r:1,h:1"));
        win.default_value = pages.add(3, numericselector!("u64,10,0,1000000,1,l:1,t:2,r:1"));
        let mut slices_panel = panel!("'Slices',l:1,t:4,r:1,b:3");
        let mut slices = listview!("class: SliceItem,d:f,flags:ScrollBars");
        slices.add_items(sample_slices());
        win.slices = slices_panel.add(slices);
        pages.add(3, slices_panel);
        win.slices_add = pages.add(3, button!("'&Add',l:1,b:0,w:8"));
        win.slices_delete = pages.add(3, button!("'&Del',l:10,b:0,w:8"));
        win.slices_clear = pages.add(3, button!("'&Clear',l:19,b:0,w:8"));

        win.pages = splitter.add(vsplitter::Panel::Right, pages);
        win.add(splitter);
        win
    }

    // ---- helpers ------------------------------------------------------------------------

    fn combo_index(&self, handle: Handle<ComboBox>) -> u32 {
        self.control(handle).and_then(|combo| combo.index()).unwrap_or(0)
    }

    fn slider_value(&self, handle: Handle<HSlider<u16>>) -> u16 {
        self.control(handle).map(|slider| slider.value()).unwrap_or(0)
    }

    fn update_chart<F>(&mut self, f: F)
    where
        F: FnOnce(&mut PieChart),
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

    // ---- chart settings -----------------------------------------------------------------

    fn apply_shape(&mut self) {
        let shape = SHAPES[(self.combo_index(self.shape) as usize).min(SHAPES.len() - 1)].1;
        self.update_chart(|chart| chart.set_shape(shape));
    }

    fn apply_renderer(&mut self) {
        let index = self.combo_index(self.renderer);
        let custom = index == RENDERER_CUSTOM;
        self.set_visible(self.character_label, custom);
        self.set_visible(self.character, custom);
        let mark = self.control(self.character).and_then(|picker| picker.char()).unwrap_or(DEFAULT_MARK);
        self.update_chart(|chart| chart.set_renderer(renderer(index, mark)));
    }

    fn apply_hole(&mut self) {
        let kind = self.combo_index(self.hole_type);
        let size = self.slider_value(self.hole_size).min(255) as u8;
        self.set_enabled(self.hole_size_label, kind != 0);
        self.set_enabled(self.hole_size, kind != 0);
        let hole_size_label = self.hole_size_label;
        if let Some(label) = self.control_mut(hole_size_label) {
            label.set_caption(match kind {
                1 => "Size (percentage)",
                2 => "Width (characters)",
                _ => "Size",
            });
        }
        let hole = match kind {
            1 => HollowThickness::Percentage(size.min(100)),
            2 => HollowThickness::Absolute(size),
            _ => HollowThickness::None,
        };
        self.update_chart(|chart| chart.set_hollow_thickness(hole));
    }

    fn apply_direction(&mut self) {
        let direction = if self.combo_index(self.direction) == 1 { Direction::CounterClockwise } else { Direction::Clockwise };
        self.update_chart(|chart| chart.set_direction(direction));
    }

    // ---- labels ---------------------------------------------------------------------------

    fn apply_label_layout(&mut self) {
        let layout = match self.combo_index(self.label_layout) {
            1 => LabelLayout::Inside,
            2 => LabelLayout::Outside,
            _ => LabelLayout::None,
        };
        self.update_chart(|chart| chart.set_label_layout(layout));
    }

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
        let decimals = self.slider_value(self.number_decimals).min(8) as u8;
        if decimals > 0 {
            format = format.decimals(decimals);
        }
        format = match self.combo_index(self.number_style) {
            1 => format.prefix("$"),
            2 => format.prefix("€"),
            3 => format.prefix("£"),
            4 => format.prefix("¥"),
            5 => format.suffix(" MB"),
            6 => format.suffix(" kg"),
            _ => format,
        };
        self.update_chart(|chart| chart.set_number_format(format));
    }

    // ---- data -----------------------------------------------------------------------------

    fn current_slices(&self) -> Vec<SliceItem> {
        let Some(list) = self.control(self.slices) else {
            return Vec::new();
        };
        (0..list.items_count()).filter_map(|index| list.item(index).cloned()).collect()
    }

    fn set_slices(&mut self, slices: Vec<SliceItem>) {
        let handle = self.slices;
        if let Some(list) = self.control_mut(handle) {
            list.clear();
            list.add_items(slices);
        }
        self.rebuild_chart();
    }

    // the chart is rebuilt from the list of slices; the edited slice is selected again
    fn rebuild_chart(&mut self) {
        let slices = self.current_slices();
        if self.edited.is_some_and(|index| index >= slices.len()) {
            self.edited = None;
        }
        let edited = self.edited;
        self.update_chart(|chart| {
            chart.clear();
            for slice in &slices {
                chart.add_value(slice.value, &slice.label, slice.color.map(CharAttribute::with_fore_color));
            }
            if let Some(index) = edited {
                chart.set_selected_slice(index);
            }
        });
        match self.edited {
            Some(index) => self.load_slice(index),
            None => self.show_slice_editor(false),
        }
    }

    fn add_slice(&mut self) {
        let value = self.control(self.default_value).map(|selector| selector.value()).unwrap_or(10);
        let draft = SliceItem::new(&format!("Slice {}", self.current_slices().len() + 1), value, None);
        let Some(slice) = SliceWindow::new("Add slice", &draft).show() else {
            return;
        };
        let mut slices = self.current_slices();
        slices.push(slice);
        self.set_slices(slices);
    }

    fn delete_slice(&mut self, index: usize) {
        let mut slices = self.current_slices();
        if index >= slices.len() {
            return;
        }
        slices.remove(index);
        self.edited = match self.edited {
            Some(edited) if edited == index => None,
            Some(edited) if edited > index => Some(edited - 1),
            other => other,
        };
        self.set_slices(slices);
    }

    fn delete_current_list_slice(&mut self) {
        let Some(index) = self.control(self.slices).and_then(|list| list.current_item_index()) else {
            dialogs::message("Slices", "Select a slice to delete.");
            return;
        };
        self.delete_slice(index);
    }

    fn clear_slices(&mut self) {
        if self.current_slices().is_empty() || !dialogs::validate("Slices", "Remove all the slices?") {
            return;
        }
        self.edited = None;
        self.set_slices(Vec::new());
    }

    fn edit_list_slice(&mut self, index: usize) {
        let Some(current) = self.current_slices().get(index).cloned() else {
            return;
        };
        let Some(updated) = SliceWindow::new("Edit slice", &current).show() else {
            return;
        };
        self.update_slice(index, |slice| *slice = updated);
    }

    fn update_slice<F>(&mut self, index: usize, f: F)
    where
        F: FnOnce(&mut SliceItem),
    {
        let handle = self.slices;
        if let Some(item) = self.control_mut(handle).and_then(|list| list.item_mut(index)) {
            f(item);
        }
        self.rebuild_chart();
    }

    // ---- selected slice -------------------------------------------------------------------

    fn show_slice_editor(&mut self, show: bool) {
        self.set_visible(self.empty_panel, !show);
        self.set_visible(self.editor_panel, show);
    }

    fn load_slice(&mut self, index: usize) {
        let slices = self.current_slices();
        let Some(slice) = slices.get(index).cloned() else {
            self.edited = None;
            self.show_slice_editor(false);
            return;
        };
        let total: u64 = slices.iter().map(|s| s.value).sum();
        let percent = if total > 0 { slice.value as f64 * 100.0 / total as f64 } else { 0.0 };
        let caption = if slice.label.is_empty() {
            format!("Slice {}: {} ({:.2}% of {})", index + 1, slice.value, percent, total)
        } else {
            format!("Slice {} ({}): {} ({:.2}% of {})", index + 1, slice.label, slice.value, percent, total)
        };
        let (info, slice_label, slice_value, slice_automatic, slice_color) = (self.info, self.slice_label, self.slice_value, self.slice_automatic, self.slice_color);
        self.loading = true;
        if let Some(label) = self.control_mut(info) {
            label.set_caption(&caption);
        }
        if let Some(field) = self.control_mut(slice_label) {
            if field.text() != slice.label {
                field.set_text(&slice.label);
            }
        }
        if let Some(selector) = self.control_mut(slice_value) {
            if selector.value() != slice.value {
                selector.set_value(slice.value);
            }
        }
        if let Some(check) = self.control_mut(slice_automatic) {
            check.set_checked(slice.color.is_none());
        }
        if let Some(picker) = self.control_mut(slice_color) {
            picker.set_color(slice.color.unwrap_or(DEFAULT_COLOR));
            picker.set_enabled(slice.color.is_some());
        }
        self.loading = false;
        self.show_slice_editor(true);
    }

    fn edit_slice(&mut self, index: usize) {
        self.edited = Some(index);
        self.load_slice(index);
        let pages = self.pages;
        if let Some(accordion) = self.control_mut(pages) {
            accordion.set_current_panel(2);
        }
    }

    fn update_edited_slice<F>(&mut self, f: F)
    where
        F: FnOnce(&mut SliceItem),
    {
        if self.loading {
            return;
        }
        if let Some(index) = self.edited {
            self.update_slice(index, f);
        }
    }

    fn edited_color(&self) -> Option<Color> {
        let automatic = self.control(self.slice_automatic).map(|check| check.is_checked()).unwrap_or(true);
        if automatic {
            None
        } else {
            self.control(self.slice_color).map(|picker| picker.color())
        }
    }
}

impl PieChartEvents for PieChartEditor {
    fn on_slice_selected(&mut self, _handle: Handle<PieChart>, index: usize) -> EventProcessStatus {
        self.edit_slice(index);
        EventProcessStatus::Processed
    }

    fn on_clear_selection(&mut self, handle: Handle<PieChart>) -> EventProcessStatus {
        if self.control(handle).is_some_and(|chart| chart.has_focus()) {
            // the user cleared the selection (clicked outside the slices, or pressed Escape)
            self.edited = None;
            self.show_slice_editor(false);
        } else if let Some(index) = self.edited {
            // the chart clears its selection when it loses the focus (for example when one of
            // the editor's controls is clicked): the slice stays edited and selected
            self.update_chart(|chart| chart.set_selected_slice(index));
        }
        EventProcessStatus::Processed
    }
}

impl ComboBoxEvents for PieChartEditor {
    fn on_selection_changed(&mut self, handle: Handle<ComboBox>) -> EventProcessStatus {
        match () {
            _ if handle == self.shape => self.apply_shape(),
            _ if handle == self.renderer => self.apply_renderer(),
            _ if handle == self.hole_type => self.apply_hole(),
            _ if handle == self.direction => self.apply_direction(),
            _ if handle == self.label_layout => self.apply_label_layout(),
            _ if handle == self.label_content => self.apply_label_content(),
            _ if handle == self.number_base || handle == self.number_group || handle == self.number_style => self.apply_number_format(),
            _ => return EventProcessStatus::Ignored,
        }
        EventProcessStatus::Processed
    }
}

impl HSliderEvents<u16> for PieChartEditor {
    fn on_value_changed(&mut self, handle: Handle<HSlider<u16>>, value: u16) -> EventProcessStatus {
        match () {
            _ if handle == self.hole_size => self.apply_hole(),
            _ if handle == self.start_angle => self.update_chart(|chart| chart.set_start_angle(value as u32)),
            _ if handle == self.number_decimals => self.apply_number_format(),
            _ => return EventProcessStatus::Ignored,
        }
        EventProcessStatus::Processed
    }
}

impl CharPickerEvents for PieChartEditor {
    fn on_char_changed(&mut self, handle: Handle<CharPicker>, _code: Option<char>) -> EventProcessStatus {
        if handle == self.character {
            self.apply_renderer();
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

impl TextFieldEvents for PieChartEditor {
    fn on_text_changed(&mut self, handle: Handle<TextField>) -> EventProcessStatus {
        if handle != self.slice_label {
            return EventProcessStatus::Ignored;
        }
        let label = self.control(handle).map(|field| field.text().to_string()).unwrap_or_default();
        self.update_edited_slice(|slice| slice.label = label);
        EventProcessStatus::Processed
    }
}

impl NumericSelectorEvents<u64> for PieChartEditor {
    fn on_value_changed(&mut self, handle: Handle<NumericSelector<u64>>, value: u64) -> EventProcessStatus {
        if handle == self.slice_value {
            self.update_edited_slice(|slice| slice.value = value);
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

impl CheckBoxEvents for PieChartEditor {
    fn on_status_changed(&mut self, handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
        if handle != self.slice_automatic {
            return EventProcessStatus::Ignored;
        }
        self.set_enabled(self.slice_color, !checked);
        let color = self.edited_color();
        self.update_edited_slice(|slice| slice.set_color(color));
        EventProcessStatus::Processed
    }
}

impl ColorPickerEvents for PieChartEditor {
    fn on_color_changed(&mut self, handle: Handle<ColorPicker>, _color: Color) -> EventProcessStatus {
        if handle != self.slice_color {
            return EventProcessStatus::Ignored;
        }
        let color = self.edited_color();
        self.update_edited_slice(|slice| slice.set_color(color));
        EventProcessStatus::Processed
    }
}

impl ButtonEvents for PieChartEditor {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        match () {
            _ if handle == self.slices_add => self.add_slice(),
            _ if handle == self.slices_delete => self.delete_current_list_slice(),
            _ if handle == self.slices_clear => self.clear_slices(),
            _ if handle == self.slice_delete => {
                if let Some(index) = self.edited {
                    self.delete_slice(index);
                }
            }
            _ => return EventProcessStatus::Ignored,
        }
        EventProcessStatus::Processed
    }
}

impl ListViewEvents<SliceItem> for PieChartEditor {
    fn on_item_action(&mut self, handle: Handle<ListView<SliceItem>>, index: usize) -> EventProcessStatus {
        if handle == self.slices {
            self.edit_list_slice(index);
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::single_window(PieChartEditor::new).title("Pie chart editor").run()
}
