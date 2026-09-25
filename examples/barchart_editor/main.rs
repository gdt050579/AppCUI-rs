use appcui::prelude::*;

const DEFAULT_THICKNESS: u8 = 3;
const DEFAULT_SPACING: u8 = 2;
const DEFAULT_COLOR: Color = Color::Aqua;
const DEFAULT_MARK: char = '#';

#[derive(Copy, Clone)]
struct BarDrawControls {
    mode: Handle<ComboBox>,
    fill: Handle<ComboBox>,
    line: Handle<Selector<LineType>>,
    point: Handle<ComboBox>,
    character: Handle<CharPicker>,
    detail: Handle<Label>,
    character_label: Handle<Label>,
}

impl BarDrawControls {
    fn none() -> Self {
        Self {
            mode: Handle::None,
            fill: Handle::None,
            line: Handle::None,
            point: Handle::None,
            character: Handle::None,
            detail: Handle::None,
            character_label: Handle::None,
        }
    }
}

trait DrawHost {
    fn add_control<T>(&mut self, control: T) -> Handle<T>
    where
        T: Control + NotWindow + NotDesktop + 'static;
}

impl DrawHost for Panel {
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

impl DrawHost for PageHost<'_> {
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

fn fill_type(index: u32, mark: char) -> BarFillType {
    match index {
        0 => BarFillType::Solid,
        1 => BarFillType::Shade75,
        2 => BarFillType::Shade50,
        3 => BarFillType::Shade25,
        4 => BarFillType::Braille,
        _ => BarFillType::Custom(mark),
    }
}

fn point_type(index: u32, mark: char) -> BarPointType {
    match index {
        1 => BarPointType::Diamond,
        2 => BarPointType::Square,
        3 => BarPointType::Custom(mark),
        _ => BarPointType::Circle,
    }
}

fn combo_with(host: &mut impl DrawHost, top: i32, items: &[&str], visible: bool) -> Handle<ComboBox> {
    let mut combo = ComboBox::new(row(top, None), combobox::Flags::None);
    for item in items {
        combo.add(item);
    }
    combo.set_index(0);
    combo.set_visible(visible);
    host.add_control(combo)
}

fn add_draw_controls(host: &mut impl DrawHost, top: i32) -> BarDrawControls {
    host.add_control(Label::new("Draw mode", row(top, Some(12))));
    let mode = combo_with(host, top + 1, &["Fill", "Line", "Rectangle", "Filled rectangle", "Smooth", "Point"], true);
    let detail = host.add_control(Label::new("Fill type", row(top + 3, Some(12))));
    let fill = combo_with(
        host,
        top + 4,
        &["Solid", "Shade 75%", "Shade 50%", "Shade 25%", "Braille", "Custom"],
        true,
    );
    let mut line = Selector::new(Some(LineType::Single), row(top + 4, None), selector::Flags::None);
    line.set_visible(false);
    let line = host.add_control(line);
    let point = combo_with(host, top + 4, &["Circle", "Diamond", "Square", "Custom"], false);
    let mut character_label = Label::new("Character", row(top + 6, Some(12)));
    character_label.set_visible(false);
    let character_label = host.add_control(character_label);
    let mut character = CharPicker::new(Some(DEFAULT_MARK), row(top + 7, None));
    character.clear_sets();
    for (name, symbols) in [
        ("Animals", charpicker::UnicodeSymbols::Animals),
        ("Arabic", charpicker::UnicodeSymbols::Arabic),
        ("Arrows", charpicker::UnicodeSymbols::Arrows),
        ("Ascii", charpicker::UnicodeSymbols::Ascii),
        ("Blocks", charpicker::UnicodeSymbols::Blocks),
        ("BoxDrawing", charpicker::UnicodeSymbols::BoxDrawing),
        ("Braille", charpicker::UnicodeSymbols::Braille),
        ("Chinese", charpicker::UnicodeSymbols::Chinese),
        ("Currency", charpicker::UnicodeSymbols::Currency),
        ("Cyrillic", charpicker::UnicodeSymbols::Cyrillic),
        ("Emoticons", charpicker::UnicodeSymbols::Emoticons),
        ("Games", charpicker::UnicodeSymbols::Games),
        ("Greek", charpicker::UnicodeSymbols::Greek),
        ("Latin", charpicker::UnicodeSymbols::Latin),
        ("Math", charpicker::UnicodeSymbols::Math),
        ("Numbers", charpicker::UnicodeSymbols::Numbers),
        ("Pictographs", charpicker::UnicodeSymbols::Pictographs),
        ("Punctuation", charpicker::UnicodeSymbols::Punctuation),
        ("Shapes", charpicker::UnicodeSymbols::Shapes),
        ("Subscripts", charpicker::UnicodeSymbols::Subscripts),
        ("Superscripts", charpicker::UnicodeSymbols::Superscripts),
        ("Transport", charpicker::UnicodeSymbols::Transport),
        ("Unicode", charpicker::UnicodeSymbols::Unicode),
    ] {
        character.add_set(charpicker::Set::from_unicode_symbols(name, symbols));
    }
    character.select_char(DEFAULT_MARK);
    character.set_visible(false);
    let character = host.add_control(character);
    BarDrawControls {
        mode,
        fill,
        line,
        point,
        character,
        detail,
        character_label,
    }
}

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

#[Window(events = [VBarChartEvents<i32>, HSliderEvents<u8>, ColorPickerEvents, ComboBoxEvents, CharPickerEvents, CheckBoxEvents, SelectorEvents<LineType>, NumericSelectorEvents<i32>])]
struct BarChartEditor {
    chart: Handle<VBarChart<i32>>,
    pages: Handle<Accordion>,
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
    number_format_panel: Handle<Panel>,
    number_base: Handle<ComboBox>,
    number_group: Handle<ComboBox>,
    number_decimals: Handle<HSlider<u8>>,
    number_style: Handle<ComboBox>,
    empty_panel: Handle<Panel>,
    editor_panel: Handle<Panel>,
    info: Handle<Label>,
    color: Handle<ColorPicker>,
    thickness: Handle<HSlider<u8>>,
    spacing: Handle<HSlider<u8>>,
    default_color: Handle<ColorPicker>,
    default_thickness: Handle<HSlider<u8>>,
    default_spacing: Handle<HSlider<u8>>,
    default_draw: BarDrawControls,
    bar_draw: BarDrawControls,
    default_draw_mode: BarDrawMode,
}

impl BarChartEditor {
    fn new() -> Self {
        let mut win = Self {
            base: window!("'Bar chart editor',d:f"),
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
            number_format_panel: Handle::None,
            number_base: Handle::None,
            number_group: Handle::None,
            number_decimals: Handle::None,
            number_style: Handle::None,
            empty_panel: Handle::None,
            editor_panel: Handle::None,
            info: Handle::None,
            color: Handle::None,
            thickness: Handle::None,
            spacing: Handle::None,
            default_color: Handle::None,
            default_thickness: Handle::None,
            default_spacing: Handle::None,
            default_draw: BarDrawControls::none(),
            bar_draw: BarDrawControls::none(),
            default_draw_mode: BarDrawMode::Fill(BarFillType::Solid),
        };

        let mut splitter = vsplitter!("pos:70%,d:f,resize:PreserveRightPanelSize,min-left-width:30,min-right-width:26");

        let mut chart = VBarChart::new(layout!("d:f"), vbarchart::Flags::ScrollBars | vbarchart::Flags::DimBarsOnSelection);
        chart.set_default_bar_width(DEFAULT_THICKNESS);
        chart.set_default_bar_spacing(DEFAULT_SPACING);
        chart.set_default_bar_attr(CharAttribute::with_fore_color(DEFAULT_COLOR));
        chart.add_bars(sample_bars());
        chart.set_xaxis_label_mode(vbarchart::XAxisLabelMode::Custom(&quarter_spans()));
        win.chart = splitter.add(vsplitter::Panel::Left, chart);

        let mut pages = accordion!("d:f,panels:['&Chart Settings','&Default Bar Settings','C&ustom Bar Settings']");

        pages.add(0, label!("'Scale type',l:1,t:1,r:1,h:1"));
        win.scale_type = pages.add(
            0,
            combobox!("l:1,t:2,r:1,items:['From zero','Fit data','Fixed','From zero min range'],index:0"),
        );
        let mut min_line = panel!("'',l:1,t:4,r:50%,h:1,type:Page");
        let mut scale_min_label = label!("'Min',l:0,t:0,w:4");
        scale_min_label.set_enabled(false);
        win.scale_min_label = min_line.add(scale_min_label);
        let mut scale_min = numericselector!("i32,0,-1000,1000,1,l:4,t:0,r:0,flags:HideButtons");
        scale_min.set_enabled(false);
        win.scale_min = min_line.add(scale_min);
        pages.add(0, min_line);

        let mut max_line = panel!("'',l:51%,t:4,r:1,h:1,type:Page");
        let mut scale_max_label = label!("'Max',l:0,t:0,w:4");
        scale_max_label.set_enabled(false);
        win.scale_max_label = max_line.add(scale_max_label);
        let mut scale_max = numericselector!("i32,100,-1000,1000,1,l:4,t:0,r:0,flags:HideButtons");
        scale_max.set_enabled(false);
        win.scale_max = max_line.add(scale_max);
        pages.add(0, max_line);

        pages.add(0, label!("'X-axis labels',l:1,t:6,r:1,h:1"));
        win.xaxis_mode = pages.add(
            0,
            combobox!("l:1,t:7,r:1,items:['None','Index','Bar labels','Groups'],index:3"),
        );
        win.yaxis_width_enabled = pages.add(0, checkbox!("'Y-axis width',l:1,t:9,r:1,checked:true"));
        let mut yaxis_width = hslider!("u8,1,50,1,l:1,t:10,r:1,flags:ShowValue,type:Ruler");
        yaxis_width.set_value(6);
        win.yaxis_width = pages.add(0, yaxis_width);
        win.yaxis_step_enabled = pages.add(0, checkbox!("'Y-axis step',l:1,t:12,r:1,checked:true"));
        let mut yaxis_step = hslider!("u8,1,50,1,l:1,t:13,r:1,flags:ShowValue,type:Ruler");
        yaxis_step.set_value(3);
        win.yaxis_step = pages.add(0, yaxis_step);

        let mut format = panel!("'',l:1,t:15,r:1,b:1,type:Page");
        format.add(label!("'Number format',l:0,t:0,r:1,h:1"));
        format.add(label!("'Base',l:0,t:2,w:12"));
        win.number_base = format.add(combobox!("l:0,t:3,r:0,items:[Decimal,Hex,Octal,Binary],index:0"));
        format.add(label!("'Grouping',l:0,t:5,w:12"));
        win.number_group = format.add(combobox!("l:0,t:6,r:0,items:[None,'3 digits','4 digits'],index:1"));
        format.add(label!("'Decimals',l:0,t:8,w:12"));
        let mut decimals = hslider!("u8,0,8,1,l:0,t:9,r:0,flags:ShowValue,type:Ruler");
        decimals.set_value(0);
        win.number_decimals = format.add(decimals);
        format.add(label!("'Style',l:0,t:11,w:12"));
        win.number_style = format.add(combobox!(
            "l:0,t:12,r:0,items:[None,Percentage,USD,Euro,GBP,Yen],index:0"
        ));
        win.number_format_panel = pages.add(0, format);

        pages.add(1, label!("'Color',l:1,t:1,w:12"));
        win.default_color = pages.add(1, ColorPicker::new(DEFAULT_COLOR, layout!("l:1,t:2,r:1")));
        pages.add(1, label!("'Thickness',l:1,t:4,w:12"));
        let mut default_thickness = hslider!("u8,1,12,1,l:1,t:5,r:1,flags:ShowValue,type:Ruler");
        default_thickness.set_value(DEFAULT_THICKNESS);
        win.default_thickness = pages.add(1, default_thickness);
        pages.add(1, label!("'Spacing',l:1,t:7,w:12"));
        let mut default_spacing = hslider!("u8,0,12,1,l:1,t:8,r:1,flags:ShowValue,type:Ruler");
        default_spacing.set_value(DEFAULT_SPACING);
        win.default_spacing = pages.add(1, default_spacing);
        win.default_draw = add_draw_controls(&mut PageHost { pages: &mut pages, index: 1 }, 10);

        let mut empty = panel!("'',d:f,type:Page");
        empty.add(label!("'Click on a bar to configure.',l:1,t:1,r:1,b:1"));
        win.empty_panel = pages.add(2, empty);

        let mut editor = panel!("'',d:f,type:Page");
        editor.set_visible(false);
        win.info = editor.add(label!("'',l:1,t:1,r:1,h:1"));
        editor.add(label!("'Color',l:1,t:3,w:12"));
        win.color = editor.add(ColorPicker::new(DEFAULT_COLOR, layout!("l:1,t:4,r:1")));
        editor.add(label!("'Thickness',l:1,t:6,w:12"));
        let mut thickness = hslider!("u8,1,12,1,l:1,t:7,r:1,flags:ShowValue,type:Ruler");
        thickness.set_value(DEFAULT_THICKNESS);
        win.thickness = editor.add(thickness);
        editor.add(label!("'Spacing',l:1,t:9,w:12"));
        let mut spacing = hslider!("u8,0,12,1,l:1,t:10,r:1,flags:ShowValue,type:Ruler");
        spacing.set_value(DEFAULT_SPACING);
        win.spacing = editor.add(spacing);
        win.bar_draw = add_draw_controls(&mut editor, 12);
        win.editor_panel = pages.add(2, editor);

        win.pages = splitter.add(vsplitter::Panel::Right, pages);
        win.add(splitter);
        win
    }

    fn selected_index(&self) -> Option<u32> {
        self.control(self.chart).and_then(|chart| chart.selected_bar())
    }

    fn scale_limit(&self, handle: Handle<NumericSelector<i32>>) -> i32 {
        self.control(handle).map(|selector| selector.value()).unwrap_or(0)
    }

    fn set_scale_range_enabled(&mut self, enabled: bool) {
        for handle in [self.scale_min_label, self.scale_max_label] {
            if let Some(label) = self.control_mut(handle) {
                label.set_enabled(enabled);
            }
        }
        for handle in [self.scale_min, self.scale_max] {
            if let Some(selector) = self.control_mut(handle) {
                selector.set_enabled(enabled);
            }
        }
    }

    fn apply_scale(&mut self) {
        let ranged = matches!(self.combo_index(self.scale_type), 2 | 3);
        self.set_scale_range_enabled(ranged);
        let mut min = self.scale_limit(self.scale_min);
        let mut max = self.scale_limit(self.scale_max);
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }
        let scale = match self.combo_index(self.scale_type) {
            1 => vbarchart::BarScale::FitData,
            2 => vbarchart::BarScale::Fixed { min, max },
            3 => vbarchart::BarScale::FromZeroMinRange { min, max },
            _ => vbarchart::BarScale::FromZero,
        };
        let chart = self.chart;
        if let Some(ctrl) = self.control_mut(chart) {
            ctrl.set_bars_scale(scale);
        }
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
        let Some((color, thickness, spacing, caption, draw_mode)) = self.control(chart).and_then(|c| {
            let bar = c.get_bar(index as usize)?;
            let caption = if bar.label().is_empty() {
                format!("Bar {}  value: {}", index + 1, bar.value())
            } else {
                format!("Bar {} ({})  value: {}", index + 1, bar.label(), bar.value())
            };
            Some((
                bar.attr().map(|attr| attr.foreground).unwrap_or(DEFAULT_COLOR),
                bar.thickness().unwrap_or(DEFAULT_THICKNESS),
                bar.spacing().unwrap_or(DEFAULT_SPACING),
                caption,
                bar.draw_mode(),
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
        let mode = draw_mode.unwrap_or(self.default_draw_mode);
        self.show_draw_mode(self.bar_draw, mode);
        self.show_bar_editors(true);
        self.show_custom_panel();
    }

    fn show_custom_panel(&mut self) {
        let pages = self.pages;
        if let Some(accordion) = self.control_mut(pages) {
            accordion.set_current_panel(2);
        }
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

    fn set_chart_default_attr(&mut self, color: Color) {
        let chart = self.chart;
        if let Some(ctrl) = self.control_mut(chart) {
            ctrl.set_default_bar_attr(CharAttribute::with_fore_color(color));
        }
    }

    fn set_chart_default_width(&mut self, width: u8) {
        let chart = self.chart;
        if let Some(ctrl) = self.control_mut(chart) {
            ctrl.set_default_bar_width(width);
        }
    }

    fn set_chart_default_spacing(&mut self, spacing: u8) {
        let chart = self.chart;
        if let Some(ctrl) = self.control_mut(chart) {
            ctrl.set_default_bar_spacing(spacing);
        }
    }

    fn slider_value(&self, handle: Handle<HSlider<u8>>) -> u8 {
        self.control(handle).map(|slider| slider.value()).unwrap_or(1)
    }

    fn set_slider_enabled(&mut self, handle: Handle<HSlider<u8>>, enabled: bool) {
        if let Some(slider) = self.control_mut(handle) {
            slider.set_enabled(enabled);
        }
    }

    fn set_chart_yaxis_width(&mut self, width: u8) {
        let chart = self.chart;
        if let Some(ctrl) = self.control_mut(chart) {
            ctrl.set_yaxis_width(width);
        }
    }

    fn set_chart_yaxis_step(&mut self, step: u8) {
        let chart = self.chart;
        if let Some(ctrl) = self.control_mut(chart) {
            ctrl.set_yaxis_step(step);
        }
    }

    fn set_yaxis_width_active(&mut self, active: bool) {
        self.set_slider_enabled(self.yaxis_width, active);
        let chart = self.chart;
        if let Some(ctrl) = self.control_mut(chart) {
            ctrl.set_yaxis_visible(active);
        }
        if active {
            let width = self.slider_value(self.yaxis_width);
            self.set_chart_yaxis_width(width);
        }
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
        let chart = self.chart;
        if let Some(ctrl) = self.control_mut(chart) {
            ctrl.set_number_format(format);
        }
    }

    fn set_yaxis_step_active(&mut self, active: bool) {
        self.set_slider_enabled(self.yaxis_step, active);
        let chart = self.chart;
        if let Some(ctrl) = self.control_mut(chart) {
            ctrl.set_yaxis_show_grid(active);
        }
        if active {
            let step = self.slider_value(self.yaxis_step);
            self.set_chart_yaxis_step(step);
        }
    }

    fn combo_index(&self, handle: Handle<ComboBox>) -> u32 {
        self.control(handle).and_then(|combo| combo.index()).unwrap_or(0)
    }

    fn mark_char(&self, handle: Handle<CharPicker>) -> char {
        self.control(handle).and_then(|picker| picker.char()).unwrap_or(DEFAULT_MARK)
    }

    fn selected_line(&self, handle: Handle<Selector<LineType>>) -> LineType {
        self.control(handle).map(|selector| selector.value()).unwrap_or(LineType::Single)
    }

    fn set_combo(&mut self, handle: Handle<ComboBox>, index: u32) {
        if let Some(combo) = self.control_mut(handle) {
            combo.set_index(index);
        }
    }

    fn draw_mode_from(&self, controls: BarDrawControls) -> BarDrawMode {
        let mark = self.mark_char(controls.character);
        match self.combo_index(controls.mode) {
            0 => BarDrawMode::Fill(fill_type(self.combo_index(controls.fill), mark)),
            1 => BarDrawMode::Line(self.selected_line(controls.line)),
            2 => BarDrawMode::Rectangle(self.selected_line(controls.line)),
            3 => BarDrawMode::FilledRectangle(self.selected_line(controls.line)),
            5 => BarDrawMode::Point(point_type(self.combo_index(controls.point), mark)),
            _ => BarDrawMode::Smooth,
        }
    }

    fn sync_draw_controls(&mut self, controls: BarDrawControls) {
        let mode = self.combo_index(controls.mode);
        let (caption, fill, line, point, character) = match mode {
            0 => ("Fill type", true, false, false, self.combo_index(controls.fill) == 5),
            1 | 2 | 3 => ("Line type", false, true, false, false),
            5 => ("Point type", false, false, true, self.combo_index(controls.point) == 3),
            _ => ("", false, false, false, false),
        };
        let detail = controls.detail;
        if let Some(label) = self.control_mut(detail) {
            if !caption.is_empty() {
                label.set_caption(caption);
            }
            label.set_visible(!caption.is_empty());
        }
        let fill_h = controls.fill;
        if let Some(combo) = self.control_mut(fill_h) {
            combo.set_visible(fill);
        }
        let line_h = controls.line;
        if let Some(combo) = self.control_mut(line_h) {
            combo.set_visible(line);
        }
        let point_h = controls.point;
        if let Some(combo) = self.control_mut(point_h) {
            combo.set_visible(point);
        }
        let character_label = controls.character_label;
        if let Some(label) = self.control_mut(character_label) {
            label.set_visible(character);
        }
        let character_h = controls.character;
        if let Some(picker) = self.control_mut(character_h) {
            picker.set_visible(character);
        }
    }

    fn show_draw_mode(&mut self, controls: BarDrawControls, mode: BarDrawMode) {
        let (mode_index, fill_index, line, point_index, mark) = match mode {
            BarDrawMode::Fill(fill) => {
                let (index, mark) = match fill {
                    BarFillType::Solid => (0, None),
                    BarFillType::Shade75 => (1, None),
                    BarFillType::Shade50 => (2, None),
                    BarFillType::Shade25 => (3, None),
                    BarFillType::Braille => (4, None),
                    BarFillType::Custom(ch) => (5, Some(ch)),
                };
                (0, Some(index), None, None, mark)
            }
            BarDrawMode::Line(line) => (1, None, Some(line), None, None),
            BarDrawMode::Rectangle(line) => (2, None, Some(line), None, None),
            BarDrawMode::FilledRectangle(line) => (3, None, Some(line), None, None),
            BarDrawMode::Smooth => (4, None, None, None, None),
            BarDrawMode::Point(point) => {
                let (index, mark) = match point {
                    BarPointType::Circle => (0, None),
                    BarPointType::Diamond => (1, None),
                    BarPointType::Square => (2, None),
                    BarPointType::Custom(ch) => (3, Some(ch)),
                };
                (5, None, None, Some(index), mark)
            }
        };
        self.set_combo(controls.mode, mode_index);
        if let Some(index) = fill_index {
            self.set_combo(controls.fill, index);
        }
        if let Some(line) = line {
            let line_h = controls.line;
            if let Some(selector) = self.control_mut(line_h) {
                selector.set_value(line);
            }
        }
        if let Some(index) = point_index {
            self.set_combo(controls.point, index);
        }
        if let Some(ch) = mark {
            let picker = controls.character;
            if let Some(picker) = self.control_mut(picker) {
                picker.select_char(ch);
            }
        }
        self.sync_draw_controls(controls);
    }

    fn apply_default_draw(&mut self) {
        let mode = self.draw_mode_from(self.default_draw);
        self.default_draw_mode = mode;
        let chart = self.chart;
        if let Some(ctrl) = self.control_mut(chart) {
            ctrl.set_default_bar_drawmode(mode);
        }
    }

    fn apply_bar_draw(&mut self) {
        let mode = self.draw_mode_from(self.bar_draw);
        self.update_selected_bar(|bar| {
            bar.set_draw_mode(mode);
        });
    }

    fn draw_combo_changed(&mut self, handle: Handle<ComboBox>, controls: BarDrawControls, defaults: bool) -> bool {
        let relevant = handle == controls.mode || handle == controls.fill || handle == controls.point;
        if !relevant {
            return false;
        }
        self.sync_draw_controls(controls);
        if defaults {
            self.apply_default_draw();
        } else {
            self.apply_bar_draw();
        }
        true
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
            _ if handle == self.default_thickness => {
                self.set_chart_default_width(value);
                EventProcessStatus::Processed
            }
            _ if handle == self.default_spacing => {
                self.set_chart_default_spacing(value);
                EventProcessStatus::Processed
            }
            _ if handle == self.yaxis_width => {
                self.set_chart_yaxis_width(value);
                EventProcessStatus::Processed
            }
            _ if handle == self.yaxis_step => {
                self.set_chart_yaxis_step(value);
                EventProcessStatus::Processed
            }
            _ if handle == self.number_decimals => {
                self.apply_number_format();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl ColorPickerEvents for BarChartEditor {
    fn on_color_changed(&mut self, handle: Handle<ColorPicker>, color: Color) -> EventProcessStatus {
        if handle == self.color {
            self.update_selected_bar(|bar| {
                bar.set_attr(CharAttribute::with_fore_color(color));
            });
            EventProcessStatus::Processed
        } else if handle == self.default_color {
            self.set_chart_default_attr(color);
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

impl SelectorEvents<LineType> for BarChartEditor {
    fn on_selection_changed(&mut self, handle: Handle<Selector<LineType>>, _value: Option<LineType>) -> EventProcessStatus {
        if handle == self.default_draw.line {
            self.apply_default_draw();
            EventProcessStatus::Processed
        } else if handle == self.bar_draw.line {
            self.apply_bar_draw();
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

impl ComboBoxEvents for BarChartEditor {
    fn on_selection_changed(&mut self, handle: Handle<ComboBox>) -> EventProcessStatus {
        if handle == self.number_base || handle == self.number_group || handle == self.number_style {
            self.apply_number_format();
            return EventProcessStatus::Processed;
        }
        if handle == self.scale_type {
            self.apply_scale();
            return EventProcessStatus::Processed;
        }
        if handle == self.xaxis_mode {
            let Some(index) = self.control(handle).and_then(|cb| cb.index()) else {
                return EventProcessStatus::Ignored;
            };
            self.apply_xaxis_mode(index);
            return EventProcessStatus::Processed;
        }
        if self.draw_combo_changed(handle, self.default_draw, true) || self.draw_combo_changed(handle, self.bar_draw, false) {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

impl CheckBoxEvents for BarChartEditor {
    fn on_status_changed(&mut self, handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
        if handle == self.yaxis_width_enabled {
            self.set_yaxis_width_active(checked);
            EventProcessStatus::Processed
        } else if handle == self.yaxis_step_enabled {
            self.set_yaxis_step_active(checked);
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

impl NumericSelectorEvents<i32> for BarChartEditor {
    fn on_value_changed(&mut self, handle: Handle<NumericSelector<i32>>, _value: i32) -> EventProcessStatus {
        if handle == self.scale_min || handle == self.scale_max {
            self.apply_scale();
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

impl CharPickerEvents for BarChartEditor {
    fn on_char_changed(&mut self, handle: Handle<CharPicker>, _code: Option<char>) -> EventProcessStatus {
        if handle == self.default_draw.character {
            self.apply_default_draw();
            EventProcessStatus::Processed
        } else if handle == self.bar_draw.character {
            self.apply_bar_draw();
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::single_window(BarChartEditor::new).title("Bar chart editor").run()
}
