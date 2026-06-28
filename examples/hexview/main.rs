use appcui::prelude::*;

fn build_buffer() -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();


    data.extend_from_slice(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F]);
    // Plain ASCII text
    data.extend_from_slice(b"Hello, AppCUI! This is a hex view example.\n");

    // Unicode text (multi-byte UTF-8 sequences)
    data.extend_from_slice("Unicode: ăîșțâ Ω 你好 こんにちは €$ 😀\n".as_bytes());

    // Non-printable / control bytes and full 0x00..=0xFF range
    data.extend_from_slice(&[0x00, 0x01, 0x02, 0x07, 0x08, 0x09, 0x0A, 0x0D, 0x1B, 0x7F]);
    data.extend((0u16..=255u16).map(|b| b as u8));

    data
}

fn main() -> Result<(), appcui::system::Error> {
    let buffer = build_buffer();

    let mut app = App::new().color_schema(false).build()?;
    let mut win = Window::new(
        "HexView",
        LayoutBuilder::new().alignment(Alignment::Center).width(80).height(24).build(),
        window::Flags::Sizeable,
    );
    let mut b = BufferView::new(buffer, layout!("d:f"), bufferview::Flags::ScrollBars | bufferview::Flags::ShowAddress | bufferview::Flags::ShowLabels );
    b.set_columns_count(bufferview::ColumnsCount::Fixed(8));
    b.set_address_width(2);
    b.set_offset_format(bufferview::OffsetFormat::Dec);
    b.set_data_representation_format(bufferview::DataRepresentationFormat::Hex(bufferview::BytesCount::Two));
    //b.set_codepage(bufferview::Codepage::ASCII);
    b.set_intervals(&[
        bufferview::Interval::new(0, 10, CharAttribute::with_color(Color::Red, Color::Black), "ASCII"),
        bufferview::Interval::new(10, 10, CharAttribute::with_color(Color::Green, Color::Transparent), "Unicode"),
        bufferview::Interval::new(20, 10, CharAttribute::with_color(Color::White, Color::DarkRed), "Non-printable"),
        bufferview::Interval::new(30, 10, CharAttribute::with_color(Color::Yellow, Color::DarkGreen), "Full range"),
    ]);
    win.add(b);
    app.add_window(win);
    app.run();
    Ok(())
}
