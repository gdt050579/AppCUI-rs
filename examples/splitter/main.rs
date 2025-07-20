use appcui::prelude::*;

mod ferris;
use crate::ferris::FERRIS_PIXELS;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = window!("'Splitter Example', a:c,w:60,h:20, flags: Sizeable");
    let mut vs = vsplitter!("pos:50%,a:c,w:100%,h:100%");
    let mut iv_1 = ImageViewer::new(
        Image::from_buffer(FERRIS_PIXELS, Size::new(172, 122), false).unwrap(),
        Layout::new("a:c"),
        image::RenderOptionsBuilder::new()
            .character_set(CharacterSet::SmallBlocks)
            .color_schema(ColorSchema::Color16)
            .scale(Scale::Scale33)
            .build(),
        imageviewer::Flags::ScrollBars,
    );
    iv_1.set_components_toolbar_margins(3, 4);
    let iv_2 = ImageViewer::new(
        Image::from_buffer(FERRIS_PIXELS, Size::new(172, 122), false).unwrap(),
        Layout::new("a:c"),
        image::RenderOptionsBuilder::new()
            .character_set(CharacterSet::AsciiArt)
            .scale(Scale::Scale25)
            .color_schema(ColorSchema::BlackAndWhite)
            .build(),
        imageviewer::Flags::ScrollBars,
    );

    vs.add(vsplitter::Panel::Left, iv_1);
    vs.add(vsplitter::Panel::Right, iv_2);
    w.add(vs);
    app.add_window(w);
    app.run();
    Ok(())
}
