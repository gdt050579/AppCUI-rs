use appcui::prelude::*;
mod ferris;

fn create_image(width: u32, height: u32, buf: &[u32]) -> Image {
    let mut img = Image::new(width, height).unwrap();
    for y in 0..height {
        for x in 0..width {
            img.set_pixel(x, y, Pixel::from(buf[(y * width + x) as usize]));
        }
    }
    img
}
fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = window!("'Splitter Example', d:c,w:60,h:20, flags: Sizeable");
    let mut vs = vsplitter!("pos:50%,d:c,w:100%,h:100%");
    let mut iv_1 = ImageViewer::new(
        create_image(172, 122, ferris::FERRIS_PIXELS),
        Layout::new("d:c"),
        image::RendererType::SmallBlocks,
        image::Scale::Scale33,
        imageviewer::Flags::ScrollBars,
    );
    iv_1.set_components_toolbar_margins(3, 4);
    let iv_2 = ImageViewer::new(
        create_image(172, 122, ferris::FERRIS_PIXELS),
        Layout::new("d:c"),
        image::RendererType::AsciiArt,
        image::Scale::Scale25,
        imageviewer::Flags::ScrollBars,
    );


    vs.add(vsplitter::Panel::Left, iv_1);
    vs.add(vsplitter::Panel::Right, iv_2);
    w.add(vs);
    app.add_window(w);
    app.run();
    Ok(())
}
