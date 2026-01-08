#![cfg(target_arch = "wasm32")]

use appcui::prelude::*;
use wasm_bindgen::prelude::*;

fn generate_test_image(width: u32, height: u32) -> Image {
    let mut pixels = Vec::with_capacity((width * height) as usize);

    for y in 0..height {
        for x in 0..width {
            let r = ((x as f32 / width as f32) * 255.0) as u32;
            let g = ((y as f32 / height as f32) * 255.0) as u32;
            let b = (((x + y) as f32 / (width + height) as f32) * 255.0) as u32;
            let a = 255u32;

            // ARGB format
            let pixel = (a << 24) | (r << 16) | (g << 8) | b;
            pixels.push(pixel);
        }
    }

    Image::from_buffer(&pixels, Size::new(width, height), true).expect("Failed to create test image")
}

fn generate_checkerboard_image(width: u32, height: u32, cell_size: u32) -> Image {
    let mut pixels = Vec::with_capacity((width * height) as usize);

    for y in 0..height {
        for x in 0..width {
            let is_white = ((x / cell_size) + (y / cell_size)) % 2 == 0;
            let (r, g, b) = if is_white { (255u32, 255u32, 255u32) } else { (50u32, 50u32, 50u32) };
            let a = 255u32;
            let pixel = (a << 24) | (r << 16) | (g << 8) | b;
            pixels.push(pixel);
        }
    }

    Image::from_buffer(&pixels, Size::new(width, height), true).expect("Failed to create checkerboard image")
}

fn generate_rainbow_image(width: u32, height: u32) -> Image {
    let mut pixels = Vec::with_capacity((width * height) as usize);

    let colors: [(u32, u32, u32); 7] = [
        (255, 0, 0),   // Red
        (255, 127, 0), // Orange
        (255, 255, 0), // Yellow
        (0, 255, 0),   // Green
        (0, 0, 255),   // Blue
        (75, 0, 130),  // Indigo
        (148, 0, 211), // Violet
    ];

    for y in 0..height {
        let color_idx = (y * 7 / height) as usize;
        let (r, g, b) = colors[color_idx.min(6)];
        for _x in 0..width {
            let a = 255u32;
            let pixel = (a << 24) | (r << 16) | (g << 8) | b;
            pixels.push(pixel);
        }
    }

    Image::from_buffer(&pixels, Size::new(width, height), true).expect("Failed to create rainbow image")
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
        image::CharacterSet::SmallBlocks => image::CharacterSet::LargeBlocks,
        image::CharacterSet::LargeBlocks => image::CharacterSet::DitheredShades,
        image::CharacterSet::DitheredShades => image::CharacterSet::Braille,
        image::CharacterSet::Braille => image::CharacterSet::AsciiArt,
        image::CharacterSet::AsciiArt => image::CharacterSet::Sixel,
        image::CharacterSet::Sixel => image::CharacterSet::SmallBlocks,
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

#[derive(Clone, Copy, PartialEq)]
enum ImageType {
    Gradient,
    Checkerboard,
    Rainbow,
}

impl ImageType {
    fn name(&self) -> &'static str {
        match self {
            ImageType::Gradient => "Gradient",
            ImageType::Checkerboard => "Checkerboard",
            ImageType::Rainbow => "Rainbow",
        }
    }

    fn next(&self) -> Self {
        match self {
            ImageType::Gradient => ImageType::Checkerboard,
            ImageType::Checkerboard => ImageType::Rainbow,
            ImageType::Rainbow => ImageType::Gradient,
        }
    }

    fn generate(&self) -> Image {
        match self {
            ImageType::Gradient => generate_test_image(256, 256),
            ImageType::Checkerboard => generate_checkerboard_image(256, 256, 32),
            ImageType::Rainbow => generate_rainbow_image(256, 256),
        }
    }
}

#[Window(events = CommandBarEvents, commands: [Scale, CharSet, ImageType])]
struct ImageWindow {
    himg:       Handle<ImageViewer>,
    image_type: ImageType,
}

impl ImageWindow {
    pub fn new() -> Self {
        let image_type = ImageType::Gradient;
        let img = image_type.generate();

        let mut w = Self {
            base: Window::new("Image Viewer - Web Demo", layout!("d:f"), window::Flags::NoCloseButton),
            himg: Handle::None,
            image_type,
        };

        let render_options = image::RenderOptionsBuilder::new().character_set(image::CharacterSet::SmallBlocks).build();

        let viewer = ImageViewer::new(img, layout!("d:f"), render_options, imageviewer::Flags::ScrollBars);
        w.himg = w.add(viewer);
        w
    }
}

impl CommandBarEvents for ImageWindow {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        if let Some(img) = self.control(self.himg) {
            let scale = img.render_options().scale();
            commandbar.set(key!("F1"), scale_to_name(scale), imagewindow::Commands::Scale);

            let chset = img.render_options().character_set();
            commandbar.set(key!("F2"), charset_to_name(chset), imagewindow::Commands::CharSet);
        }
        commandbar.set(key!("F3"), self.image_type.name(), imagewindow::Commands::ImageType);
    }

    fn on_event(&mut self, command_id: imagewindow::Commands) {
        let h = self.himg;
        match command_id {
            imagewindow::Commands::Scale => {
                if let Some(img) = self.control_mut(h) {
                    let scale = img.render_options().scale();
                    let new_scale = next_scale(scale);
                    let mut opt = img.render_options().clone();
                    opt.set_scale(new_scale);
                    img.set_render_options(opt);
                }
            }
            imagewindow::Commands::CharSet => {
                if let Some(img) = self.control_mut(h) {
                    let chset = img.render_options().character_set();
                    let new_chset = next_charset(chset);
                    let mut opt = img.render_options().clone();
                    opt.set_character_set(new_chset);
                    img.set_render_options(opt);
                }
            }
            imagewindow::Commands::ImageType => {
                self.image_type = self.image_type.next();
                let new_image = self.image_type.generate();
                if let Some(img) = self.control_mut(h) {
                    img.set_image(new_image);
                }
            }
        }
        self.request_update();
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"Starting Image Viewer Web Demo...".into());
    web_sys::console::log_1(&"Press F1 to change scale, F2 to change rendering method, F3 to change image".into());

    let mut app = App::new().single_window().command_bar().build().unwrap();
    app.add_window(ImageWindow::new());
    app.run();
}
