use appcui::prelude::*;

#[derive(Clone, ListItem)]
pub struct SliceItem {
    #[Column(name: "&Label", width: 16)]
    pub label: String,
    #[Column(name: "&Value", width: 10, align: right)]
    pub value: u64,
    #[Column(name: "&Color", width: 10)]
    pub color_name: String,
    pub color: Option<Color>,
}

impl SliceItem {
    pub fn new(label: &str, value: u64, color: Option<Color>) -> Self {
        Self {
            label: label.to_string(),
            value,
            color_name: color_name(color),
            color,
        }
    }

    pub fn set_color(&mut self, color: Option<Color>) {
        self.color = color;
        self.color_name = color_name(color);
    }
}

pub fn color_name(color: Option<Color>) -> String {
    match color {
        Some(color) => color.name().to_string(),
        None => String::from("Automatic"),
    }
}

#[ModalWindow(events = [ButtonEvents, WindowEvents, CheckBoxEvents], response = SliceItem)]
pub struct SliceWindow {
    label: Handle<TextField>,
    value: Handle<NumericSelector<u64>>,
    automatic: Handle<CheckBox>,
    color: Handle<ColorPicker>,
    ok: Handle<Button>,
    cancel: Handle<Button>,
}

impl SliceWindow {
    pub fn new(title: &str, slice: &SliceItem) -> Self {
        let mut win = Self {
            base: ModalWindow::new(title, layout!("a:c,w:46,h:13"), window::Flags::None),
            label: Handle::None,
            value: Handle::None,
            automatic: Handle::None,
            color: Handle::None,
            ok: Handle::None,
            cancel: Handle::None,
        };
        win.add(label!("'Label',l:1,t:1,w:12"));
        win.label = win.add(TextField::new(&slice.label, layout!("l:14,t:1,r:1"), textfield::Flags::None));
        win.add(label!("'Value',l:1,t:3,w:12"));
        win.value = win.add(NumericSelector::new(
            slice.value,
            0,
            1_000_000,
            1,
            layout!("l:14,t:3,r:1"),
            numericselector::Flags::None,
        ));
        win.automatic = win.add(CheckBox::new("Automatic color", layout!("l:1,t:5,r:1"), slice.color.is_none()));
        win.add(label!("'Color',l:1,t:7,w:12"));
        let mut color = ColorPicker::new(slice.color.unwrap_or(Color::Aqua), layout!("l:14,t:7,r:1"));
        color.set_enabled(slice.color.is_some());
        win.color = win.add(color);
        win.ok = win.add(button!("'&Ok',l:1,b:0,w:14"));
        win.cancel = win.add(button!("'&Cancel',r:1,b:0,w:14"));
        // start in the label field: Enter accepts the dialog (Escape cancels it)
        let label = win.label;
        win.request_focus_for_control(label);
        win
    }

    fn read_slice(&self) -> SliceItem {
        let label = self.control(self.label).map(|field| field.text().trim().to_string()).unwrap_or_default();
        let value = self.control(self.value).map(|selector| selector.value()).unwrap_or(0);
        let automatic = self.control(self.automatic).map(|check| check.is_checked()).unwrap_or(true);
        let color = if automatic { None } else { self.control(self.color).map(|picker| picker.color()) };
        SliceItem::new(&label, value, color)
    }

    fn accept(&mut self) {
        let slice = self.read_slice();
        self.exit_with(slice);
    }
}

impl WindowEvents for SliceWindow {
    fn on_accept(&mut self) {
        self.accept();
    }
}

impl CheckBoxEvents for SliceWindow {
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

impl ButtonEvents for SliceWindow {
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
