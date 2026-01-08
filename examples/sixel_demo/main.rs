use appcui::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;

static LOADED_IMAGE: OnceLock<Image> = OnceLock::new();

fn load_png_image(path: &Path) -> Option<Image> {
    let img = ::image::open(path).ok()?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    let pixels: Vec<u32> = rgba
        .pixels()
        .map(|p| {
            let r = p[0] as u32;
            let g = p[1] as u32;
            let b = p[2] as u32;
            let a = p[3] as u32;
            (a << 24) | (r << 16) | (g << 8) | b
        })
        .collect();

    Image::from_buffer(&pixels, Size::new(width, height), true)
}

fn charset_to_name(cs: image::CharacterSet) -> &'static str {
    match cs {
        image::CharacterSet::SmallBlocks => "SmallBlocks",
        image::CharacterSet::LargeBlocks => "LargeBlocks",
        image::CharacterSet::DitheredShades => "Dithered",
        image::CharacterSet::Braille => "Braille",
        image::CharacterSet::AsciiArt => "AsciiArt",
        image::CharacterSet::Sixel => "Sixel",
    }
}

fn scale_to_name(sc: image::Scale) -> &'static str {
    match sc {
        image::Scale::NoScale => "100%",
        image::Scale::Scale50 => "50%",
        image::Scale::Scale33 => "33%",
        image::Scale::Scale25 => "25%",
        image::Scale::Scale20 => "20%",
        image::Scale::Scale10 => "10%",
        image::Scale::Scale5 => "5%",
    }
}

fn next_charset(cs: image::CharacterSet) -> image::CharacterSet {
    match cs {
        image::CharacterSet::Sixel => image::CharacterSet::SmallBlocks,
        image::CharacterSet::SmallBlocks => image::CharacterSet::LargeBlocks,
        image::CharacterSet::LargeBlocks => image::CharacterSet::DitheredShades,
        image::CharacterSet::DitheredShades => image::CharacterSet::Braille,
        image::CharacterSet::Braille => image::CharacterSet::AsciiArt,
        image::CharacterSet::AsciiArt => image::CharacterSet::Sixel,
    }
}

fn next_scale(sc: image::Scale) -> image::Scale {
    match sc {
        image::Scale::NoScale => image::Scale::Scale50,
        image::Scale::Scale50 => image::Scale::Scale33,
        image::Scale::Scale33 => image::Scale::Scale25,
        image::Scale::Scale25 => image::Scale::Scale20,
        image::Scale::Scale20 => image::Scale::Scale10,
        image::Scale::Scale10 => image::Scale::Scale5,
        image::Scale::Scale5 => image::Scale::NoScale,
    }
}

#[Window(events = CommandBarEvents, commands: [Scale, CharSet])]
struct MyWin {
    himg: Handle<ImageViewer>,
}

impl MyWin {
    pub fn new(img: Image) -> Self {
        let mut w = Self {
            base: Window::new("SpongeBob - HD Image Demo", layout!("d:f"), window::Flags::Sizeable),
            himg: Handle::None,
        };

        let render_options = image::RenderOptionsBuilder::new().character_set(image::CharacterSet::Sixel).build();

        let i = ImageViewer::new(img, layout!("d:f"), render_options, imageviewer::Flags::ScrollBars);
        w.himg = w.add(i);
        w
    }
}

impl CommandBarEvents for MyWin {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        if let Some(img) = self.control(self.himg) {
            let scale = img.render_options().scale();
            commandbar.set(key!("F1"), scale_to_name(scale), mywin::Commands::Scale);

            let chset = img.render_options().character_set();
            commandbar.set(key!("F2"), charset_to_name(chset), mywin::Commands::CharSet);
        }
    }

    fn on_event(&mut self, command_id: mywin::Commands) {
        let h = self.himg;
        match command_id {
            mywin::Commands::Scale => {
                if let Some(img) = self.control_mut(h) {
                    let scale = img.render_options().scale();
                    let new_scale = next_scale(scale);
                    let mut opt = img.render_options().clone();
                    opt.set_scale(new_scale);
                    img.set_render_options(opt);
                }
            }
            mywin::Commands::CharSet => {
                if let Some(img) = self.control_mut(h) {
                    let chset = img.render_options().character_set();
                    let new_chset = next_charset(chset);
                    let mut opt = img.render_options().clone();
                    opt.set_character_set(new_chset);
                    img.set_render_options(opt);
                }
            }
        }
        self.request_update();
    }
}

#[Desktop(events = DesktopEvents)]
struct MyDesktop {}

impl MyDesktop {
    fn new() -> Self {
        Self { base: Desktop::new() }
    }
}

impl DesktopEvents for MyDesktop {
    fn on_start(&mut self) {
        if let Some(image) = LOADED_IMAGE.get() {
            self.add_window(MyWin::new(image.clone()));
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let image_path = Path::new("/home/crandrei/personal/AppCUI-rs/spongbob.png");

    let image = load_png_image(image_path).unwrap_or_else(|| panic!("Failed to load image from: {}", image_path.display()));

    let render_options = image::RenderOptionsBuilder::new().build();
    let sixel_string = image.to_sixel(&render_options);
    let sixel_path = image_path.with_extension("six");
    let mut file = File::create(&sixel_path).expect("Failed to create sixel file");
    file.write_all(sixel_string.as_bytes()).expect("Failed to write sixel data");
    println!("Saved sixel to: {}", sixel_path.display());

    let _ = LOADED_IMAGE.set(image);

    App::new().desktop(MyDesktop::new()).command_bar().build()?.run();
    Ok(())
}
