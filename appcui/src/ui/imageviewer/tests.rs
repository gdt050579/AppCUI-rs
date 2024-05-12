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
        image::RenderMethod::SmallBlocks,
        image::Scale::NoScale,
        imageviewer::Flags::None,
    ));
    a.add_window(w);
    a.run();    
}

#[test]
fn check_smallbloacks_scaling() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0x2F7D4E8955E1C3D8)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Title,d:c");
    // let s = r#"
    //     |RRRRRRRR|
    //     |RGGGGGGR|
    //     |RGBBBBGR|
    //     |RGBWWBGR|
    //     |RGBWWBGR|
    //     |RGBBBBGR|
    //     |RGGGGGGR|
    //     |RRRRRRRR|
    // "#;
    let s = r#"
        |RRRRGGGG|
        |RRRRGGGG|
        |GGWWGGWW|
        |GGWWGGWW|
        |RRRRGGGG|
        |RRRRGGGG|
        |YYWWGGWW|
        |YYWWGGWW|
    "#;
    w.add(ImageViewer::new(
        Image::with_str(s).unwrap(),
        Layout::new("x:0,y:0,w:8,h:4"),
        image::RenderMethod::SmallBlocks,
        image::Scale::NoScale,
        imageviewer::Flags::None,
    ));
    w.add(ImageViewer::new(
        Image::with_str(s).unwrap(),
        Layout::new("x:10,y:0,w:16,h:8"),
        image::RenderMethod::SmallBlocks,
        image::Scale::Scale50,
        imageviewer::Flags::None,
    ));
    a.add_window(w);
    a.run();
}


#[test]
fn check_macro_creation() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0x336411586F530FA4)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Title,d:c");
    w.add(imageviewer!("image:'|RRRR|,|R..R|,|R..R|,|RRRR|',d:c,w:100%,h:100%"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_macro_creation_2() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0xD4B545A31AC55161)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Title,d:c");
    w.add(imageviewer!("image:'|RRRR|,|R..R|,|R..R|,|RRRR|',d:c,w:100%,h:100%, flags: Scrollbars, back: {char: *, fore: Yellow, back: Green}"));
    a.add_window(w);
    a.run();
}