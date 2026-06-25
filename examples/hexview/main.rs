use appcui::prelude::*;

fn build_buffer() -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();

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

    let mut app = App::new().build()?;
    let mut win = Window::new(
        "HexView",
        LayoutBuilder::new().alignment(Alignment::Center).width(80).height(24).build(),
        window::Flags::Sizeable,
    );
    let mut b = BufferView::new(buffer, layout!("d:f"), bufferview::Flags::ScrollBars);
    b.set_columns_count(bufferview::ColumnsCount::Auto);
    win.add(b);
    app.add_window(win);
    app.run();
    Ok(())
}
