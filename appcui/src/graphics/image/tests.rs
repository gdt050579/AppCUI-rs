use crate::prelude::Size;

use super::super::SurfaceTester;
use super::Image;
use super::Pixel;
use super::RendererType;
use super::Scale;

const HEART: &str = r#"
        |BB..........BB|
        |B..rr....rr..B|
        |..rrrr..rrrr..|
        |.rrrrrrrrrrrr.|
        |.raaaaaaaaaar.|
        |..ryyyyyyyyr..|
        |   rwwwwwwr   |
        |....rwwwwr....|
        |G....rwwr....G|
        |GG....rr....GG|
"#;

const HEART_RED: &str = r#"
        |...rr....rr...|
        |..rrrr..rrrr..|
        |.rrrrrrrrrrrr.|
        |.rrrrrrrrrrrr.|
        |..rrrrrrrrrr..|
        |   rrrrrrrr   |
        |....rrrrrr....|
        |.....rrrr.....|
        |......rr......|
"#;

const ALL_COLORS: &str = r#"
        |0123456789gArPYW|
        | BGTRMOSsbgarpyw|
        |.BGTRmoSsbgtrpyw|
"#;

#[test]
fn check_draw_smallblocks() {
    let mut s = SurfaceTester::new(40, 10);
    let i = Image::with_str(HEART).unwrap();
    s.draw_image(1, 1, &i, RendererType::SmallBlocks, Scale::NoScale);
    //s.print();
    assert_eq!(s.compute_hash(), 0x939D21530F9EB6A5);
}

#[test]
fn check_draw_smallblocks_all_colors() {
    let mut s = SurfaceTester::new(40, 10);
    let i = Image::with_str(ALL_COLORS).unwrap();
    s.draw_image(1, 1, &i, RendererType::SmallBlocks, Scale::NoScale);
    //s.print();
    assert_eq!(s.compute_hash(), 0xEBAE7CC6AE75A08D);
}

#[test]
fn check_draw_smallblocks_scale() {
    let mut s = SurfaceTester::new(40, 10);
    let i = Image::with_str(HEART_RED).unwrap();
    s.draw_image(1, 1, &i, RendererType::SmallBlocks, Scale::NoScale);
    s.draw_image(20, 1, &i, RendererType::SmallBlocks, Scale::Scale50);
    //s.print();
    assert_eq!(s.compute_hash(), 0x438928E166B239E1);
}

#[test]
fn check_draw_gray() {
    let mut s = SurfaceTester::new(40, 15);
    let i = Image::with_str(HEART).unwrap();
    s.draw_image(1, 1, &i, RendererType::GrayScale, Scale::NoScale);
    //s.print();
    assert_eq!(s.compute_hash(), 0x31B5363F572C0EA5);
}
#[test]
fn check_draw_gray_scale() {
    let mut s = SurfaceTester::new(50, 15);
    let i = Image::with_str(HEART_RED).unwrap();
    s.draw_image(1, 1, &i, RendererType::GrayScale, Scale::NoScale);
    s.draw_image(30, 1, &i, RendererType::GrayScale, Scale::Scale50);
    //s.print();
    assert_eq!(s.compute_hash(), 0x3A46E9F1E2A046BD);
}

#[test]
fn check_draw_large_chars_64() {
    let mut s = SurfaceTester::new(40, 15);
    let i = Image::with_str(HEART).unwrap();
    s.draw_image(1, 1, &i, RendererType::LargeBlocks64Colors, Scale::NoScale);
    //s.print();
    assert_eq!(s.compute_hash(), 0x69DCA4337155535D);
}
#[test]
fn check_draw_large_chars_64_scale() {
    let mut s = SurfaceTester::new(50, 15);
    let i = Image::with_str(HEART_RED).unwrap();
    s.draw_image(1, 1, &i, RendererType::LargeBlocks64Colors, Scale::NoScale);
    s.draw_image(30, 1, &i, RendererType::LargeBlocks64Colors, Scale::Scale50);
    //s.print();
    assert_eq!(s.compute_hash(), 0x1648702FD7AD361);
}

#[test]
fn check_draw_ascii_art() {
    let mut s = SurfaceTester::new(40, 15);
    let i = Image::with_str(HEART).unwrap();
    s.draw_image(1, 1, &i, RendererType::AsciiArt, Scale::NoScale);
    //s.print();
    assert_eq!(s.compute_hash(), 0xB493CD321C84CBA5);
}
#[test]
fn check_draw_ascii_art_scale() {
    let mut s = SurfaceTester::new(50, 15);
    let i = Image::with_str(HEART_RED).unwrap();
    s.draw_image(1, 1, &i, RendererType::AsciiArt, Scale::NoScale);
    s.draw_image(30, 1, &i, RendererType::AsciiArt, Scale::Scale50);
    //s.print();
    assert_eq!(s.compute_hash(), 0xFCF9279F2D7E525);
}

#[test]
fn check_image_with_invalid_size() {
    assert_eq!(Image::new(0,0).is_none(), true);
    assert_eq!(Image::new(100,0).is_none(), true);
    assert_eq!(Image::new(0xFFFF,0xFFFF).is_none(), true);
    assert_eq!(Image::with_str("0000").is_none(), true);
    assert_eq!(Image::with_str("||").is_none(), true);
}

#[test]
fn check_size_of_image() {
    let i = Image::with_str(HEART).unwrap();
    assert_eq!(i.size(), Size::new(14,10));
}

#[test]
fn check_clear() {
    let mut i = Image::new(5,5).unwrap();
    i.clear(Pixel::new(1,2,3,4));
    for x in 0..5 {
        for y in 0..5 {
            assert_eq!(i.pixel(x,y), Some(Pixel::new(1,2,3,4)));
        }
    }
}

