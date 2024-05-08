use crate::prelude::*;

#[test]
fn check_create() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0x84E9E28762625B3)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:8");
    let img = Image::with_str(
        r#"
        |RR.........RR|
        |B..rr...rr..B|
        |..rrrr.rrrr..|
        |.rrrrrrrrrrr.|
        |.raaaaaaaaar.|
        |..ryyyyyyyr..|
        |   rwwwwwr   |
        |....rwwwr....|
        |G....rwr....G|
        |GG....r....GG|
    "#,
    )
    .unwrap();
    w.add(ImageViewer::new(
        img,
        Layout::new("d:c"),
        ImageRenderingMethod::SmallBlocks,
        ImageScaleMethod::NoScale,
        image_viewer::Flags::None,
    ));
    a.add_window(w);
    a.run();
}
