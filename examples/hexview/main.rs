use appcui::prelude::*;

struct MyBuffer {
    data: Vec<u8>,
    start: u64,
}
impl MyBuffer {
    fn new() -> Self {
        let mut data: Vec<u8> = Vec::new();

        data.extend_from_slice(&[
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        ]);
        data.extend_from_slice(b"Hello, AppCUI! This is a hex view example.\n");
        data.extend_from_slice(&[
            0, 0, 0, 0, 0, b'H', 0, b'e', 0, b'l', 0, b'l', 0, b'o', 0, b',', 0, b' ', 0, b'A', 0, b'p', 0, b'p', 0, b'C', 0, b'U', 0, b'I', 0, b'!',
            0, b' ', 0, b'T', 0, b'h', 0, b'i', 0, b's', 0, b' ', 0, b'i', 0, b's', 0, b' ', 0, b'a', 0, b' ', 0, b'h', 0, b'e', 0, b'x', 0, b' ', 0,
            b'v', 0, b'i', 0, b'e', 0, b'w', 0, b' ', 0, b'e', 0, b'x', 0, b'a', 0, b'm', 0, b'p', 0, b'l', 0, b'e', 0, b'.', 0, b'\n',
        ]);
        data.extend_from_slice("Unicode: ăîșțâ Ω 你好 こんにちは €$ 😀\n".as_bytes());
        data.extend_from_slice(&[0x00, 0x01, 0x02, 0x07, 0x08, 0x09, 0x0A, 0x0D, 0x1B, 0x7F]);
        data.extend((0u16..=255u16).map(|b| b as u8));
        // add 100 byte that are not accesible
        let start = data.len() as u64;
        for _ in 0..100 {
            data.push(0xFF);
        }
        data.extend_from_slice(b"End of the buffer example !\n");
        Self { data, start }
    }
}
impl BufferAccess for MyBuffer {
    fn len(&self) -> u64 {
        self.data.len() as u64
    }
    fn get(&mut self, pos: u64) -> Option<u8> {
        if (pos < self.data.len() as u64) && ((pos < self.start) || (pos >= (self.start + 100))) {
            Some(self.data[pos as usize])
        } else {
            None
        }
    }
    fn can_write(&self) -> bool {
        true
    }
    fn set(&mut self, pos: u64, value: u8) -> bool {
        if (pos < self.data.len() as u64) && ((pos < self.start) || (pos >= (self.start + 100))) {
            self.data[pos as usize] = value;
            true
        } else {
            false
        }
    }
    fn can_resize(&self) -> bool {
        true
    }
    fn resize(&mut self, new_size: u64, value: u8) -> bool {
        self.data.resize(new_size as usize, value);
        true
    }
}

fn hex_format_from_index(index: u32) -> bufferview::HexFormat {
    match index {
        1 => bufferview::HexFormat::Word,
        2 => bufferview::HexFormat::DWord,
        3 => bufferview::HexFormat::QWord,
        _ => bufferview::HexFormat::Byte,
    }
}

fn uint_format_from_index(index: u32) -> bufferview::UIntFormat {
    match index {
        1 => bufferview::UIntFormat::U16,
        2 => bufferview::UIntFormat::U32,
        3 => bufferview::UIntFormat::U64,
        _ => bufferview::UIntFormat::U8,
    }
}

fn int_format_from_index(index: u32) -> bufferview::IntFormat {
    match index {
        1 => bufferview::IntFormat::I16,
        2 => bufferview::IntFormat::I32,
        3 => bufferview::IntFormat::I64,
        _ => bufferview::IntFormat::I8,
    }
}

fn float_format_from_index(index: u32) -> bufferview::FloatFormat {
    match index {
        1 => bufferview::FloatFormat::Scientific64,
        2 => bufferview::FloatFormat::E4M3,
        3 => bufferview::FloatFormat::E5M2,
        _ => bufferview::FloatFormat::Scientific32,
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

#[Window(events = ComboBoxEvents+CheckBoxEvents+BufferViewEvents<MyBuffer>+CommandBarEvents, commands = [ReadSelection, OverwriteText, FillSelection, DeleteSelection, InsertText, ResizeBuffer])]
struct HexViewWindow {
    buffer: Handle<BufferView<MyBuffer>>,
    lb_pos: Handle<toolbar::Label>,
    cb_format: Handle<ComboBox>,
    cb_columns: Handle<ComboBox>,
    cb_offset: Handle<ComboBox>,
    cb_data_format: Handle<ComboBox>,
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
    fn new(buffer: MyBuffer) -> Self {
        let mut w = Self {
            base: window!("'HexView',a:c,w:100,h:24,flags:Sizeable"),
            buffer: Handle::None,
            lb_pos: Handle::None,
            cb_format: Handle::None,
            cb_columns: Handle::None,
            cb_offset: Handle::None,
            cb_data_format: Handle::None,
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
        bv.set_data_representation_format(bufferview::DataRepresentationFormat::Hex(bufferview::HexFormat::Byte));
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
        w.cb_format = panel.add(combobox!("l:18,t:1,r:1,items=[Hex,Oct,Bin,UInt,Int,Float,Char],index:0"));
        panel.add(label!("'Format:',l:1,t:3,w:16,h:1"));
        w.cb_data_format = panel.add(combobox!("l:18,t:3,r:1,items=[Byte,Word,DWord,QWord],index:0"));
        panel.add(label!("'Columns:',l:1,t:5,w:16,h:1"));
        w.cb_columns = panel.add(combobox!("l:18,t:5,r:1,items=[4,8,12,16,Auto],index:1"));
        panel.add(label!("'Offset format:',l:1,t:7,w:16,h:1"));
        w.cb_offset = panel.add(combobox!("l:18,t:7,r:1,items=[Hex,Dec],index:0"));
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

    fn read_selection(&mut self) {
        let h_buffer = self.buffer;
        if let Some(bv) = self.control_mut(h_buffer) {
            let Some((start, end)) = bv.selection() else {
                dialogs::message("Read Selection", "No selection.");
                return;
            };
            if end <= start {
                dialogs::message("Read Selection", "Selection is empty.");
                return;
            }
            let mut s = String::new();
            let mut v: Vec<u8> = Vec::new();
            bv.read_bytes_into_vec(pos, count, output);
            if values.is_empty() {
                dialogs::message("Read Selection", "Unable to read selection.");
                return;
            }
            let message = format!(
                "Selection 0x{start:X}..0x{end:X} ({} byte(s)):\n{}",
                end - start,
                values.join("\n")
            );
            dialogs::message("Read Selection", message.as_str());
        }
    }

    fn overwrite_text(&mut self) {
        let h_buffer = self.buffer;
        if let Some(bv) = self.control_mut(h_buffer) {
            let pos = bv.current_pos();
            let caption = format!("Enter value to overwrite at offset 0x{pos:X}:");
            let Some(text) = dialogs::input::<String>("Overwrite", caption.as_str(), None, None) else {
                return;
            };
            if text.is_empty() {
                return;
            }
            if bv.overwrite_bytes(pos, text.as_bytes()) {
                self.update_position_label();
            } else {
                dialogs::message("Overwrite", format!("Failed to overwrite at offset 0x{pos:X}.").as_str());
            }
        }
    }

    fn fill_selection(&mut self) {
        let h_buffer = self.buffer;
        if let Some(bv) = self.control_mut(h_buffer) {
            let Some((start, end)) = bv.selection() else {
                return;
            };
            let count = end - start;
            if count == 0 {
                return;
            }
            let caption = format!(
                "Fill {count} byte(s) from offset 0x{start:X} to 0x{}.\nEnter fill value (0-255):",
                end - 1
            );
            let Some(value) = dialogs::input::<u8>("Fill Selection", caption.as_str(), Some(0), None) else {
                return;
            };
            if bv.fill_buffer(start, count, value) {
                self.update_position_label();
            } else {
                dialogs::message("Fill Selection", format!("Failed to fill selection with value 0x{value:02X}.").as_str());
            }
        }
    }

    fn delete_selection(&mut self) {
        let h_buffer = self.buffer;
        if let Some(bv) = self.control_mut(h_buffer) {
            let Some((start, end)) = bv.selection() else {
                return;
            };
            let count = end - start;
            if count == 0 {
                return;
            }
            let message = format!(
                "Are you sure that you want to delete {count} byte(s) from offset 0x{start:X} to 0x{}?",
                end - 1
            );
            if dialogs::validate("Delete Selection", message.as_str()) && bv.delete_bytes(start, count) {
                bv.clear_selection();
                self.update_position_label();
            }
        }
    }

    fn insert_text(&mut self) {
        let h_buffer = self.buffer;
        if let Some(bv) = self.control_mut(h_buffer) {
            let pos = bv.current_pos();
            let caption = format!("Enter text to insert at offset 0x{pos:X}:");
            let Some(text) = dialogs::input::<String>("Insert Text", caption.as_str(), None, None) else {
                return;
            };
            if text.is_empty() {
                return;
            }
            if bv.insert_bytes(pos, text.as_bytes()) {
                self.update_position_label();
            } else {
                dialogs::message("Insert Text", format!("Failed to insert text '{text}' at offset 0x{pos:X}").as_str());
            }
        }
    }

    fn resize_buffer(&mut self) {
        let h_buffer = self.buffer;
        if let Some(bv) = self.control_mut(h_buffer) {
            let current_size = bv.bytes_count();
            let caption = format!("Current buffer size is {current_size} byte(s).\nEnter the new size in bytes:");
            let Some(new_size) = dialogs::input::<u64>("Resize Buffer", caption.as_str(), Some(current_size), None) else {
                return;
            };
            if bv.resize_buffer(new_size, 0u8) {
                bv.clear_selection();
                self.update_position_label();
            } else {
                dialogs::message("Resize Buffer", format!("Failed to resize buffer to {new_size} byte(s).").as_str());
            }
        }
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

    fn format_index(&self) -> u32 {
        self.combobox_index(self.cb_format)
    }

    fn data_format_index(&self) -> u32 {
        self.combobox_index(self.cb_data_format)
    }

    fn representation_has_format(&self) -> bool {
        matches!(self.format_index(), 0 | 3 | 4 | 5)
    }

    fn endian_enabled(&self) -> bool {
        self.representation_has_format() && self.data_format_index() >= 1
    }

    fn update_format_combobox(&mut self) {
        let repr_idx = self.format_index();
        let has_format = self.representation_has_format();
        let h = self.cb_data_format;
        if let Some(cb) = self.control_mut(h) {
            cb.clear();
            if has_format {
                let items: &[&str] = match repr_idx {
                    0 => &["Byte", "Word", "DWord", "QWord"],
                    3 => &["U8", "U16", "U32", "U64"],
                    4 => &["I8", "I16", "I32", "I64"],
                    5 => &["Scientific32", "Scientific64", "E4M3", "E5M2"],
                    _ => &[],
                };
                for item in items {
                    cb.add(item);
                }
                cb.set_index(0);
                cb.set_enabled(true);
            } else {
                cb.set_enabled(false);
            }
        }
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
        let endian_enabled = self.endian_enabled();
        let h_endian = self.cb_endian;
        if let Some(cb) = self.control_mut(h_endian) {
            cb.set_enabled(endian_enabled);
        }
    }

    fn apply_to_buffer(&mut self) {
        let format_idx = self.format_index();
        let columns_idx = self.combobox_index(self.cb_columns);
        let offset_idx = self.combobox_index(self.cb_offset);
        let data_format_idx = self.data_format_index();
        let endian_idx = self.combobox_index(self.cb_endian);
        let codepage_idx = self.combobox_index(self.cb_codepage);
        let endian_enabled = self.endian_enabled();
        let show_address = self.checkbox_checked(self.cb_show_address, true);
        let show_interval_names = self.checkbox_checked(self.cb_show_interval_names, false);
        let show_ascii_strings = self.checkbox_checked(self.cb_show_ascii_strings, true);
        let show_unicode_strings = self.checkbox_checked(self.cb_show_unicode_strings, false);
        let decode_utf8 = self.checkbox_checked(self.cb_decode_utf8, true);

        let repr = match format_idx {
            0 => bufferview::DataRepresentationFormat::Hex(hex_format_from_index(data_format_idx)),
            1 => bufferview::DataRepresentationFormat::Oct,
            2 => bufferview::DataRepresentationFormat::Bin,
            3 => bufferview::DataRepresentationFormat::UInt(uint_format_from_index(data_format_idx)),
            4 => bufferview::DataRepresentationFormat::Int(int_format_from_index(data_format_idx)),
            5 => bufferview::DataRepresentationFormat::Float(float_format_from_index(data_format_idx)),
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
    fn on_selection_changed(&mut self, handle: Handle<ComboBox>) -> EventProcessStatus {
        if handle == self.cb_format {
            self.update_format_combobox();
        }
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

impl BufferViewEvents<MyBuffer> for HexViewWindow {
    fn on_current_pos_changed(&mut self, _handle: Handle<BufferView<MyBuffer>>) -> EventProcessStatus {
        self.update_position_label();
        EventProcessStatus::Processed
    }

    fn on_selection_changed(&mut self, _handle: Handle<BufferView<MyBuffer>>) -> EventProcessStatus {
        self.request_update();
        EventProcessStatus::Processed
    }
}

impl CommandBarEvents for HexViewWindow {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        commandbar.set(key!("F3"), "Overwrite", hexviewwindow::Commands::OverwriteText);
        commandbar.set(key!("F4"), "Read Selection", hexviewwindow::Commands::ReadSelection);
        commandbar.set(key!("F6"), "Fill Selection", hexviewwindow::Commands::FillSelection);
        commandbar.set(key!("F8"), "Delete Selection", hexviewwindow::Commands::DeleteSelection);
        commandbar.set(key!("F9"), "Insert", hexviewwindow::Commands::InsertText);
        commandbar.set(key!("F10"), "Resize", hexviewwindow::Commands::ResizeBuffer);
    }

    fn on_event(&mut self, command_id: hexviewwindow::Commands) {
        match command_id {
            hexviewwindow::Commands::ReadSelection => self.read_selection(),
            hexviewwindow::Commands::OverwriteText => self.overwrite_text(),
            hexviewwindow::Commands::FillSelection => self.fill_selection(),
            hexviewwindow::Commands::DeleteSelection => self.delete_selection(),
            hexviewwindow::Commands::InsertText => self.insert_text(),
            hexviewwindow::Commands::ResizeBuffer => self.resize_buffer(),
        }
        self.request_update();
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().color_schema(false).command_bar().build()?;
    app.add_window(HexViewWindow::new(MyBuffer::new()));
    app.run();
    Ok(())
}
