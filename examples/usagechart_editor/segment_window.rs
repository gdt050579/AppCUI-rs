use appcui::prelude::*;

pub const DEFAULT_MARK: char = '*';
pub const DEFAULT_COLOR: Color = Color::Aqua;
pub const FILL_CUSTOM: u32 = 13;

pub const FILL_NAMES: [&str; 14] = [
    "Solid",
    "Shade 75%",
    "Shade 50%",
    "Shade 25%",
    "Braille",
    "Checkerboard",
    "Grid",
    "Grid double",
    "Cross-hatch",
    "Dashed",
    "Diagonal up",
    "Diagonal down",
    "Notched",
    "Custom",
];

pub fn fill_type(index: u32, mark: char) -> usagechart::SegmentFillType {
    match index {
        1 => usagechart::SegmentFillType::Shade75,
        2 => usagechart::SegmentFillType::Shade50,
        3 => usagechart::SegmentFillType::Shade25,
        4 => usagechart::SegmentFillType::Braille,
        5 => usagechart::SegmentFillType::Checkerboard,
        6 => usagechart::SegmentFillType::Grid,
        7 => usagechart::SegmentFillType::GridDouble,
        8 => usagechart::SegmentFillType::CrossHatch,
        9 => usagechart::SegmentFillType::Dashed,
        10 => usagechart::SegmentFillType::DiagonalUp,
        11 => usagechart::SegmentFillType::DiagonalDown,
        12 => usagechart::SegmentFillType::Notched,
        FILL_CUSTOM => usagechart::SegmentFillType::Custom(mark),
        _ => usagechart::SegmentFillType::Solid,
    }
}

fn fill_index_of(fill: usagechart::SegmentFillType) -> u32 {
    match fill {
        usagechart::SegmentFillType::Solid => 0,
        usagechart::SegmentFillType::Shade75 => 1,
        usagechart::SegmentFillType::Shade50 => 2,
        usagechart::SegmentFillType::Shade25 => 3,
        usagechart::SegmentFillType::Braille => 4,
        usagechart::SegmentFillType::Checkerboard => 5,
        usagechart::SegmentFillType::Grid => 6,
        usagechart::SegmentFillType::GridDouble => 7,
        usagechart::SegmentFillType::CrossHatch => 8,
        usagechart::SegmentFillType::Dashed => 9,
        usagechart::SegmentFillType::DiagonalUp => 10,
        usagechart::SegmentFillType::DiagonalDown => 11,
        usagechart::SegmentFillType::Notched => 12,
        usagechart::SegmentFillType::Custom(_) => FILL_CUSTOM,
    }
}

pub fn fill_caption(index: u32, mark: char) -> String {
    if index == FILL_CUSTOM {
        format!("Custom ({mark})")
    } else {
        FILL_NAMES.get(index as usize).copied().unwrap_or(FILL_NAMES[0]).to_string()
    }
}

pub fn color_name(color: Option<Color>) -> String {
    match color {
        Some(color) => color.name().to_string(),
        None => String::from("Automatic"),
    }
}

pub fn char_picker(layout: Layout) -> CharPicker {
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

#[derive(Clone, ListItem)]
pub struct SegmentItem {
    #[Column(name: "&Label", width: 16)]
    pub label: String,
    #[Column(name: "&Value", width: 10, align: right)]
    pub value: u64,
    #[Column(name: "&Fill", width: 14)]
    pub fill_name: String,
    #[Column(name: "&Color", width: 12)]
    pub color_name: String,
    pub fill_index: u32,
    pub mark: char,
    pub color: Option<Color>,
}

impl SegmentItem {
    pub fn new(label: &str, value: u64, fill: usagechart::SegmentFillType, color: Option<Color>) -> Self {
        let mark = match fill {
            usagechart::SegmentFillType::Custom(mark) => mark,
            _ => DEFAULT_MARK,
        };
        let fill_index = fill_index_of(fill);
        Self {
            label: label.to_string(),
            value,
            fill_name: fill_caption(fill_index, mark),
            color_name: color_name(color),
            fill_index,
            mark,
            color,
        }
    }

    pub fn set_fill(&mut self, index: u32, mark: char) {
        self.fill_index = index.min(FILL_CUSTOM);
        self.mark = mark;
        self.fill_name = fill_caption(self.fill_index, self.mark);
    }

    pub fn set_color(&mut self, color: Option<Color>) {
        self.color = color;
        self.color_name = color_name(color);
    }

    pub fn fill(&self) -> usagechart::SegmentFillType {
        fill_type(self.fill_index, self.mark)
    }

    pub fn to_segment(&self) -> usagechart::Segment {
        let mut builder = usagechart::SegmentBuilder::new().value(self.value).label(&self.label).fill(self.fill());
        if let Some(color) = self.color {
            builder = builder.attr(CharAttribute::with_fore_color(color));
        }
        builder.build()
    }
}

#[ModalWindow(events = [ButtonEvents, WindowEvents, CheckBoxEvents, ComboBoxEvents], response = SegmentItem)]
pub struct SegmentWindow {
    label: Handle<TextField>,
    value: Handle<NumericSelector<u64>>,
    fill: Handle<ComboBox>,
    character: Handle<CharPicker>,
    automatic: Handle<CheckBox>,
    color: Handle<ColorPicker>,
    ok: Handle<Button>,
    cancel: Handle<Button>,
}

impl SegmentWindow {
    pub fn new(title: &str, segment: &SegmentItem) -> Self {
        let mut win = Self {
            base: ModalWindow::new(title, layout!("a:c,w:52,h:17"), window::Flags::None),
            label: Handle::None,
            value: Handle::None,
            fill: Handle::None,
            character: Handle::None,
            automatic: Handle::None,
            color: Handle::None,
            ok: Handle::None,
            cancel: Handle::None,
        };
        win.add(label!("'Label',l:1,t:1,w:12"));
        win.label = win.add(TextField::new(&segment.label, layout!("l:14,t:1,r:1"), textfield::Flags::None));
        win.add(label!("'Value',l:1,t:3,w:12"));
        win.value = win.add(NumericSelector::new(
            segment.value,
            0,
            1_000_000,
            1,
            layout!("l:14,t:3,r:1"),
            numericselector::Flags::None,
        ));
        win.add(label!("'Fill',l:1,t:5,w:12"));
        let mut fill = ComboBox::new(layout!("l:14,t:5,r:1"), combobox::Flags::None);
        for name in FILL_NAMES {
            fill.add(name);
        }
        fill.set_index(segment.fill_index.min(FILL_CUSTOM));
        win.fill = win.add(fill);
        win.add(label!("'Character',l:1,t:7,w:12"));
        let mut character = char_picker(layout!("l:14,t:7,r:1"));
        character.select_char(segment.mark);
        character.set_enabled(segment.fill_index == FILL_CUSTOM);
        win.character = win.add(character);
        win.automatic = win.add(CheckBox::new("Automatic color", layout!("l:1,t:9,r:1"), segment.color.is_none()));
        win.add(label!("'Color',l:1,t:11,w:12"));
        let mut color = ColorPicker::new(segment.color.unwrap_or(DEFAULT_COLOR), layout!("l:14,t:11,r:1"));
        color.set_enabled(segment.color.is_some());
        win.color = win.add(color);
        win.ok = win.add(button!("'&Ok',l:1,b:0,w:14"));
        win.cancel = win.add(button!("'&Cancel',r:1,b:0,w:14"));
        let label = win.label;
        win.request_focus_for_control(label);
        win
    }

    fn read_segment(&self) -> SegmentItem {
        let label = self.control(self.label).map(|field| field.text().trim().to_string()).unwrap_or_default();
        let value = self.control(self.value).map(|selector| selector.value()).unwrap_or(0);
        let fill_index = self.control(self.fill).and_then(|combo| combo.index()).unwrap_or(0).min(FILL_CUSTOM);
        let mark = self.control(self.character).and_then(|picker| picker.char()).unwrap_or(DEFAULT_MARK);
        let automatic = self.control(self.automatic).map(|check| check.is_checked()).unwrap_or(true);
        let color = if automatic {
            None
        } else {
            self.control(self.color).map(|picker| picker.color())
        };
        let mut segment = SegmentItem::new(&label, value, fill_type(fill_index, mark), color);
        segment.mark = mark;
        segment.set_fill(fill_index, mark);
        segment
    }

    fn accept(&mut self) {
        let segment = self.read_segment();
        self.exit_with(segment);
    }
}

impl WindowEvents for SegmentWindow {
    fn on_accept(&mut self) {
        self.accept();
    }
}

impl CheckBoxEvents for SegmentWindow {
    fn on_status_changed(&mut self, handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
        if handle != self.automatic {
            return EventProcessStatus::Ignored;
        }
        let color = self.color;
        if let Some(picker) = self.control_mut(color) {
            picker.set_enabled(!checked);
        }
        EventProcessStatus::Processed
    }
}

impl ComboBoxEvents for SegmentWindow {
    fn on_selection_changed(&mut self, handle: Handle<ComboBox>) -> EventProcessStatus {
        if handle != self.fill {
            return EventProcessStatus::Ignored;
        }
        let custom = self.control(self.fill).and_then(|combo| combo.index()) == Some(FILL_CUSTOM);
        let character = self.character;
        if let Some(picker) = self.control_mut(character) {
            picker.set_enabled(custom);
        }
        EventProcessStatus::Processed
    }
}

impl ButtonEvents for SegmentWindow {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.ok {
            self.accept();
            EventProcessStatus::Processed
        } else if handle == self.cancel {
            self.exit();
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}
