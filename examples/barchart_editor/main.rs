use appcui::prelude::*;

const DEFAULT_THICKNESS: u8 = 3;
const DEFAULT_SPACING: u8 = 2;

fn sample_bars() -> Vec<vbarchart::Bar<i32>> {
    [
        ("Jan", 12, Color::Aqua),
        ("Feb", 28, Color::Green),
        ("Mar", 19, Color::Yellow),
        ("Apr", 35, Color::Red),
        ("May", 22, Color::Magenta),
        ("Jun", 41, Color::Blue),
        ("Jul", 33, Color::Pink),
        ("Aug", 18, Color::Olive),
        ("Sep", 27, Color::Teal),
        ("Oct", 31, Color::Silver),
        ("Nov", 15, Color::White),
        ("Dec", 45, Color::DarkRed),
    ]
    .into_iter()
    .map(|(label, value, color)| {
        vbarchart::BarBuilder::new(value)
            .label(label)
            .attr(CharAttribute::with_fore_color(color))
            .thickness(DEFAULT_THICKNESS)
            .spacing(DEFAULT_SPACING)
            .build()
    })
    .collect()
}

fn quarter_spans() -> [vbarchart::BarSpan; 4] {
    [
        vbarchart::BarSpan::new(0, 3, "First quarter"),
        vbarchart::BarSpan::new(3, 3, "2nd quarter"),
        vbarchart::BarSpan::new(6, 3, "3rd quarter"),
        vbarchart::BarSpan::new(9, 3, "4th quarter"),
    ]
}

#[Window(events = [VBarChartEvents<i32>, HSliderEvents<u8>, ColorPickerEvents, ComboBoxEvents])]
struct BarChartEditor {
    chart: Handle<VBarChart<i32>>,
    xaxis_mode: Handle<ComboBox>,
    empty_panel: Handle<Panel>,
    editor_panel: Handle<Panel>,
    info: Handle<Label>,
    color: Handle<ColorPicker>,
    thickness: Handle<HSlider<u8>>,
    spacing: Handle<HSlider<u8>>,
}

impl BarChartEditor {
    fn new() -> Self {
        let mut win = Self {
            base: window!("'Bar chart editor',d:f"),
            chart: Handle::None,
            xaxis_mode: Handle::None,
            empty_panel: Handle::None,
            editor_panel: Handle::None,
            info: Handle::None,
            color: Handle::None,
            thickness: Handle::None,
            spacing: Handle::None,
        };

        let mut splitter = vsplitter!("pos:70%,d:f,resize:PreserveRightPanelSize,min-left-width:30,min-right-width:26");

        let mut chart = VBarChart::new(layout!("d:f"), vbarchart::Flags::ScrollBars | vbarchart::Flags::DimBarsOnSelection );
        chart.set_default_bar_width(DEFAULT_THICKNESS);
        chart.set_default_bar_spacing(DEFAULT_SPACING);
        chart.add_bars(sample_bars());
        chart.set_xaxis_label_mode(vbarchart::XAxisLabelMode::Custom(&quarter_spans()));
        win.chart = splitter.add(vsplitter::Panel::Left, chart);

        let mut general = panel!("'Chart properties',l:1,t:1,r:1,h:6");
        general.add(label!("'X-axis labels',l:1,t:1,w:16"));
        win.xaxis_mode = general.add(combobox!(
            "l:1,t:2,r:1,items:['None','Index','Bar labels','Groups'],index:3"
        ));
        splitter.add(vsplitter::Panel::Right, general);

        let mut empty = panel!("'Bar properties',l:1,t:8,r:1,b:1");
        empty.add(label!("'Click on a bar to configure.',l:1,t:1,r:1,b:1"));
        win.empty_panel = splitter.add(vsplitter::Panel::Right, empty);

        let mut editor = panel!("'Bar properties',l:1,t:8,r:1,b:1");
        editor.set_visible(false);
        win.info = editor.add(label!("'',l:1,t:1,r:1,h:1"));
        editor.add(label!("'Color',l:1,t:3,w:12"));
        win.color = editor.add(ColorPicker::new(Color::Aqua, layout!("l:1,t:4,r:1")));
        editor.add(label!("'Thickness',l:1,t:6,w:12"));
        let mut thickness = hslider!("u8,1,12,1,l:1,t:7,r:1,flags:ShowValue,type:Ruler");
        thickness.set_value(DEFAULT_THICKNESS);
        win.thickness = editor.add(thickness);
        editor.add(label!("'Spacing',l:1,t:9,w:12"));
        let mut spacing = hslider!("u8,0,12,1,l:1,t:10,r:1,flags:ShowValue,type:Ruler");
        spacing.set_value(DEFAULT_SPACING);
        win.spacing = editor.add(spacing);
        win.editor_panel = splitter.add(vsplitter::Panel::Right, editor);

        win.add(splitter);
        win
    }

    fn selected_index(&self) -> Option<u32> {
        self.control(self.chart).and_then(|chart| chart.selected_bar())
    }

    fn apply_xaxis_mode(&mut self, index: u32) {
        let chart = self.chart;
        let Some(ctrl) = self.control_mut(chart) else {
            return;
        };
        match index {
            0 => ctrl.set_xaxis_label_mode(vbarchart::XAxisLabelMode::None),
            1 => ctrl.set_xaxis_label_mode(vbarchart::XAxisLabelMode::Index { start: 1 }),
            2 => ctrl.set_xaxis_label_mode(vbarchart::XAxisLabelMode::BarLabels),
            3 => ctrl.set_xaxis_label_mode(vbarchart::XAxisLabelMode::Custom(&quarter_spans())),
            _ => {}
        }
    }

    fn show_bar_editors(&mut self, show: bool) {
        let empty = self.empty_panel;
        let editor = self.editor_panel;
        if let Some(panel) = self.control_mut(empty) {
            panel.set_visible(!show);
        }
        if let Some(panel) = self.control_mut(editor) {
            panel.set_visible(show);
        }
    }

    fn load_selected_bar(&mut self, index: u32) {
        let chart = self.chart;
        let Some((color, thickness, spacing, caption)) = self.control(chart).and_then(|c| {
            let bar = c.get_bar(index as usize)?;
            let caption = if bar.label().is_empty() {
                format!("Bar {}  value: {}", index + 1, bar.value())
            } else {
                format!("Bar {} ({})  value: {}", index + 1, bar.label(), bar.value())
            };
            Some((
                bar.attr().map(|attr| attr.foreground).unwrap_or(Color::Aqua),
                bar.thickness().unwrap_or(DEFAULT_THICKNESS),
                bar.spacing().unwrap_or(DEFAULT_SPACING),
                caption,
            ))
        }) else {
            return;
        };

        let info = self.info;
        if let Some(label) = self.control_mut(info) {
            label.set_caption(&caption);
        }
        let color_h = self.color;
        if let Some(picker) = self.control_mut(color_h) {
            picker.set_color(color);
        }
        let thickness_h = self.thickness;
        if let Some(slider) = self.control_mut(thickness_h) {
            slider.set_value(thickness);
        }
        let spacing_h = self.spacing;
        if let Some(slider) = self.control_mut(spacing_h) {
            slider.set_value(spacing);
        }
        self.show_bar_editors(true);
    }

    fn clear_selection_ui(&mut self) {
        self.show_bar_editors(false);
    }

    fn update_selected_bar<F>(&mut self, f: F)
    where
        F: FnOnce(&mut vbarchart::Bar<i32>),
    {
        let Some(index) = self.selected_index() else {
            return;
        };
        let chart = self.chart;
        if let Some(ctrl) = self.control_mut(chart) {
            ctrl.update_bar(index as usize, f);
        }
    }
}

impl VBarChartEvents<i32> for BarChartEditor {
    fn on_bar_selected(&mut self, _handle: Handle<VBarChart<i32>>, index: u32) -> EventProcessStatus {
        self.load_selected_bar(index);
        EventProcessStatus::Processed
    }

    fn on_clear_selection(&mut self, _handle: Handle<VBarChart<i32>>) -> EventProcessStatus {
        self.clear_selection_ui();
        EventProcessStatus::Processed
    }
}

impl HSliderEvents<u8> for BarChartEditor {
    fn on_value_changed(&mut self, handle: Handle<HSlider<u8>>, value: u8) -> EventProcessStatus {
        match () {
            _ if handle == self.thickness => {
                self.update_selected_bar(|bar| {
                    bar.set_thickness(value);
                });
                EventProcessStatus::Processed
            }
            _ if handle == self.spacing => {
                self.update_selected_bar(|bar| {
                    bar.set_spacing(value);
                });
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl ColorPickerEvents for BarChartEditor {
    fn on_color_changed(&mut self, _handle: Handle<ColorPicker>, color: Color) -> EventProcessStatus {
        self.update_selected_bar(|bar| {
            bar.set_attr(CharAttribute::with_fore_color(color));
        });
        EventProcessStatus::Processed
    }
}

impl ComboBoxEvents for BarChartEditor {
    fn on_selection_changed(&mut self, handle: Handle<ComboBox>) -> EventProcessStatus {
        if handle != self.xaxis_mode {
            return EventProcessStatus::Ignored;
        }
        let Some(index) = self.control(handle).and_then(|cb| cb.index()) else {
            return EventProcessStatus::Ignored;
        };
        self.apply_xaxis_mode(index);
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::single_window(BarChartEditor::new).title("Bar chart editor").run()
}
