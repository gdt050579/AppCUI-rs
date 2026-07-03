use appcui::prelude::*;

fn build_buffer() -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();

    data.extend_from_slice(&[
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    ]);
    data.extend_from_slice(b"Hello, AppCUI! This is a hex view example.\n");
    data.extend_from_slice(&[0,0,0,0,0,b'H', 0, b'e', 0, b'l', 0, b'l', 0, b'o', 0, b',', 0, b' ', 0, b'A', 0, b'p', 0, b'p', 0, b'C', 0, b'U', 0, b'I', 0, b'!', 0, b' ', 0, b'T', 0, b'h', 0, b'i', 0, b's', 0, b' ', 0, b'i', 0, b's', 0, b' ', 0, b'a', 0, b' ', 0, b'h', 0, b'e', 0, b'x', 0, b' ', 0, b'v', 0, b'i', 0, b'e', 0, b'w', 0, b' ', 0, b'e', 0, b'x', 0, b'a', 0, b'm', 0, b'p', 0, b'l', 0, b'e', 0, b'.', 0, b'\n']);
    data.extend_from_slice("Unicode: ăîșțâ Ω 你好 こんにちは €$ 😀\n".as_bytes());
    data.extend_from_slice(&[0x00, 0x01, 0x02, 0x07, 0x08, 0x09, 0x0A, 0x0D, 0x1B, 0x7F]);
    data.extend((0u16..=255u16).map(|b| b as u8));

    data
}

fn bytes_count_from_index(index: u32) -> bufferview::BytesCount {
    match index {
        1 => bufferview::BytesCount::Two,
        2 => bufferview::BytesCount::Four,
        3 => bufferview::BytesCount::Eight,
        _ => bufferview::BytesCount::One,
    }
}

fn codepage_from_index(index: u32) -> bufferview::Codepage {
    match index {
        1 => bufferview::Codepage::ASCII,
        2 => bufferview::Codepage::CP437,
        3 => bufferview::Codepage::WINDOWS_1252,
        _ => bufferview::Codepage::new("Default"),
    }
}

#[Window(events = ComboBoxEvents+CheckBoxEvents+BufferViewEvents<Vec<u8>>)]
struct HexViewWindow {
    buffer: Handle<BufferView<Vec<u8>>>,
    lb_pos: Handle<toolbar::Label>,
    cb_format: Handle<ComboBox>,
    cb_columns: Handle<ComboBox>,
    cb_offset: Handle<ComboBox>,
    cb_grouping: Handle<ComboBox>,
    cb_endian: Handle<ComboBox>,
    cb_codepage: Handle<ComboBox>,
    cb_enabled: Handle<CheckBox>,
    cb_show_address: Handle<CheckBox>,
    cb_show_interval_names: Handle<CheckBox>,
    cb_show_ascii_strings: Handle<CheckBox>,
    cb_show_unicode_strings: Handle<CheckBox>,
    cb_decode_utf8: Handle<CheckBox>,
}

impl HexViewWindow {
    fn new(buffer: Vec<u8>) -> Self {
        let mut w = Self {
            base: window!("'HexView',a:c,w:100,h:24,flags:Sizeable"),
            buffer: Handle::None,
            lb_pos: Handle::None,
            cb_format: Handle::None,
            cb_columns: Handle::None,
            cb_offset: Handle::None,
            cb_grouping: Handle::None,
            cb_endian: Handle::None,
            cb_codepage: Handle::None,
            cb_enabled: Handle::None,
            cb_show_address: Handle::None,
            cb_show_interval_names: Handle::None,
            cb_show_ascii_strings: Handle::None,
            cb_show_unicode_strings: Handle::None,
            cb_decode_utf8: Handle::None,
        };

        let mut vs = vsplitter!("d:f,pos:50%,resize:PreserveLeftPanelSize,minrightwidth:28");

        let mut bv = BufferView::new(
            buffer,
            layout!("d:f"),
            bufferview::Flags::ScrollBars
                | bufferview::Flags::ShowAddress
                | bufferview::Flags::SearchBar
                | bufferview::Flags::DecodeUTF8Characters
                | bufferview::Flags::ShowAsciiStrings,
        );
        bv.set_columns_count(bufferview::ColumnsCount::Fixed(8));
        bv.set_address_width(6);
        bv.set_offset_format(bufferview::OffsetFormat::Hex);
        bv.set_data_representation_format(bufferview::DataRepresentationFormat::Hex(bufferview::BytesCount::One));
        bv.set_endian(bufferview::Endian::Little);
        bv.set_intervals(&[
            bufferview::Interval::new(0, 10, CharAttribute::with_color(Color::Red, Color::Black), "ASCII"),
            bufferview::Interval::new(10, 10, CharAttribute::with_color(Color::Green, Color::Transparent), "Unicode"),
            bufferview::Interval::new(20, 10, CharAttribute::with_color(Color::White, Color::DarkRed), "Non-printable"),
            bufferview::Interval::new(30, 10, CharAttribute::with_color(Color::Yellow, Color::DarkGreen), "Full range"),
        ]);
        bv.set_components_toolbar_margins(8, 4);
        w.buffer = vs.add(vsplitter::Panel::Left, bv);

        let mut panel = panel!("'Configuration',d:f");
        panel.add(label!("'Representation:',l:1,t:1,w:16,h:1"));
        w.cb_format = panel.add(combobox!("l:18,t:1,r:1,items=[Hex,Oct,Bin,Char],index:0"));
        panel.add(label!("'Columns:',l:1,t:3,w:16,h:1"));
        w.cb_columns = panel.add(combobox!("l:18,t:3,r:1,items=[4,8,12,16,Auto],index:1"));
        panel.add(label!("'Offset format:',l:1,t:5,w:16,h:1"));
        w.cb_offset = panel.add(combobox!("l:18,t:5,r:1,items=[Hex,Dec],index:0"));
        panel.add(label!("'Grouping:',l:1,t:7,w:16,h:1"));
        w.cb_grouping = panel.add(combobox!("l:18,t:7,r:1,items=[Byte,Word,DWord,QWord],index:0"));
        panel.add(label!("'Endian:',l:1,t:9,w:16,h:1"));
        w.cb_endian = panel.add(combobox!("l:18,t:9,r:1,items=[Little,Big],index:0"));
        panel.add(label!("'Codepage:',l:1,t:11,w:16,h:1"));
        w.cb_codepage = panel.add(combobox!("l:18,t:11,r:1,items=[Default,ASCII,CP437,WINDOWS_1252],index:0"));
        w.cb_enabled = panel.add(checkbox!("'&Enabled',l:1,t:13,r:1,checked:true"));
        w.cb_show_address = panel.add(checkbox!("'Show &address column',l:1,t:14,r:1,checked:true"));
        w.cb_show_interval_names = panel.add(checkbox!("'Show &IntervalName column',l:1,t:15,r:1"));
        w.cb_show_ascii_strings = panel.add(checkbox!("'Show &ASCII strings in Char view mode',l:1,t:16,r:1,checked:true"));
        w.cb_show_unicode_strings = panel.add(checkbox!("'Show &Unicode strings in Char view mode',l:1,t:17,r:1"));
        w.cb_decode_utf8 = panel.add(checkbox!("'Decode UTF-&8 characters in Char view mode',l:1,t:18,r:1,checked:true"));
        vs.add(vsplitter::Panel::Right, panel);

        w.add(vs);

        let pos_group = w.toolbar().create_group(toolbar::GroupPosition::BottomLeft);
        w.lb_pos = w.toolbar().add(pos_group, toolbar::Label::new("Position: 0x0"));

        w.update_dependent_controls();
        w.apply_to_buffer();
        w.update_position_label();
        w
    }

    fn update_position_label(&mut self) {
        let pos = self.control(self.buffer).map(|bv| bv.current_pos()).unwrap_or(0);
        let text = format!("{pos:04X}");
        let h = self.lb_pos;
        if let Some(label) = self.toolbar().get_mut(h) {
            label.set_content(text.as_str());
        }
    }

    fn combobox_index(&self, handle: Handle<ComboBox>) -> u32 {
        self.control(handle).and_then(|cb| cb.index()).unwrap_or(0)
    }

    fn is_hex_format(&self) -> bool {
        self.combobox_index(self.cb_format) == 0
    }

    fn format_index(&self) -> u32 {
        self.combobox_index(self.cb_format)
    }

    fn grouping_index(&self) -> u32 {
        self.combobox_index(self.cb_grouping)
    }

    fn endian_enabled(&self) -> bool {
        self.is_hex_format() && self.grouping_index() >= 1
    }

    fn grouping_enabled(&self) -> bool {
        self.is_hex_format()
    }

    fn update_buffer_enabled(&mut self) {
        let enabled = self.control(self.cb_enabled).map(|cb| cb.is_checked()).unwrap_or(true);
        let h_buffer = self.buffer;
        if let Some(bv) = self.control_mut(h_buffer) {
            bv.set_enabled(enabled);
        }
    }

    fn checkbox_checked(&self, handle: Handle<CheckBox>, default: bool) -> bool {
        self.control(handle).map(|cb| cb.is_checked()).unwrap_or(default)
    }

    fn update_dependent_controls(&mut self) {
        let grouping_enabled = self.grouping_enabled();
        let endian_enabled = self.endian_enabled();
        let h_grouping = self.cb_grouping;
        if let Some(cb) = self.control_mut(h_grouping) {
            cb.set_enabled(grouping_enabled);
        }
        let h_endian = self.cb_endian;
        if let Some(cb) = self.control_mut(h_endian) {
            cb.set_enabled(endian_enabled);
        }
    }

    fn apply_to_buffer(&mut self) {
        let format_idx = self.format_index();
        let columns_idx = self.combobox_index(self.cb_columns);
        let offset_idx = self.combobox_index(self.cb_offset);
        let grouping_idx = self.grouping_index();
        let endian_idx = self.combobox_index(self.cb_endian);
        let codepage_idx = self.combobox_index(self.cb_codepage);
        let endian_enabled = self.endian_enabled();
        let show_address = self.checkbox_checked(self.cb_show_address, true);
        let show_interval_names = self.checkbox_checked(self.cb_show_interval_names, false);
        let show_ascii_strings = self.checkbox_checked(self.cb_show_ascii_strings, true);
        let show_unicode_strings = self.checkbox_checked(self.cb_show_unicode_strings, false);
        let decode_utf8 = self.checkbox_checked(self.cb_decode_utf8, true);

        let repr = match format_idx {
            0 => bufferview::DataRepresentationFormat::Hex(bytes_count_from_index(grouping_idx)),
            1 => bufferview::DataRepresentationFormat::Oct,
            2 => bufferview::DataRepresentationFormat::Bin,
            _ => bufferview::DataRepresentationFormat::Char,
        };

        let columns = match columns_idx {
            0 => bufferview::ColumnsCount::Fixed(4),
            1 => bufferview::ColumnsCount::Fixed(8),
            2 => bufferview::ColumnsCount::Fixed(12),
            3 => bufferview::ColumnsCount::Fixed(16),
            _ => bufferview::ColumnsCount::Auto,
        };

        let offset = if offset_idx == 0 {
            bufferview::OffsetFormat::Hex
        } else {
            bufferview::OffsetFormat::Dec
        };

        let endian = if endian_idx == 0 {
            bufferview::Endian::Little
        } else {
            bufferview::Endian::Big
        };

        let h_buffer = self.buffer;
        if let Some(bv) = self.control_mut(h_buffer) {
            bv.set_data_representation_format(repr);
            bv.set_columns_count(columns);
            bv.set_offset_format(offset);
            bv.set_address_visible(show_address);
            bv.set_interval_names_visible(show_interval_names);
            bv.set_ascii_strings_visible(show_ascii_strings);
            bv.set_unicode_strings_visible(show_unicode_strings);
            bv.set_decode_utf8(decode_utf8);
            if endian_enabled {
                bv.set_endian(endian);
            }
            bv.set_codepage(codepage_from_index(codepage_idx));
        }
    }
}

impl ComboBoxEvents for HexViewWindow {
    fn on_selection_changed(&mut self, _handle: Handle<ComboBox>) -> EventProcessStatus {
        self.update_dependent_controls();
        self.apply_to_buffer();
        EventProcessStatus::Processed
    }
}

impl CheckBoxEvents for HexViewWindow {
    fn on_status_changed(&mut self, handle: Handle<CheckBox>, _checked: bool) -> EventProcessStatus {
        if handle == self.cb_enabled {
            self.update_buffer_enabled();
        }
        if handle == self.cb_show_address
            || handle == self.cb_show_interval_names
            || handle == self.cb_show_ascii_strings
            || handle == self.cb_show_unicode_strings
            || handle == self.cb_decode_utf8
        {
            self.apply_to_buffer();
        }
        EventProcessStatus::Processed
    }
}

impl BufferViewEvents<Vec<u8>> for HexViewWindow {
    fn on_current_pos_changed(&mut self, _handle: Handle<BufferView<Vec<u8>>>) -> EventProcessStatus {
        self.update_position_label();
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let buffer = build_buffer();
    let mut app = App::new().color_schema(false).build()?;
    app.add_window(HexViewWindow::new(buffer));
    app.run();
    Ok(())
}
