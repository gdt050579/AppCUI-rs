use appcui::prelude::*;

const LUMINANCE_SEQUENCE: [u8; 9] = [5, 10, 25, 33, 50, 66, 75, 90, 95];

#[Window(events=CommandBarEvents,commands:[Scale,CharSet,ColorSchema,Luminance])]
pub struct MyWin {
    himg: Handle<ImageViewer>,
}
impl MyWin {
    pub fn new(title: &str, image: Image) -> Self {
        let mut w = Self {
            base: Window::new(title, Layout::new("d:c,w:50%,h:50%"), window::Flags::Sizeable),
            himg: Handle::None,
        };

        let i = ImageViewer::new(
            image,
            Layout::new("d:c"),
            image::RenderOptionsBuilder::new().luminance_threshold(0.1).build(),
            imageviewer::Flags::ScrollBars,
        );
        w.himg = w.add(i);
        w
    }
}
impl CommandBarEvents for MyWin {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        let sc_name = match self
            .control(self.himg)
            .map(|i| i.render_options().scale())
            .unwrap_or(image::Scale::NoScale)
        {
            image::Scale::NoScale => "Scale:100%",
            image::Scale::Scale50 => "Scale:50%",
            image::Scale::Scale33 => "Scale:33%",
            image::Scale::Scale25 => "Scale:25%",
            image::Scale::Scale20 => "Scale:20%",
            image::Scale::Scale10 => "Scale:10%",
            image::Scale::Scale5 => "Scale:5%",
        };
        commandbar.set(key!("F1"), sc_name, mywin::Commands::Scale);
        let chset = match self
            .control(self.himg)
            .map(|i| i.render_options().character_set())
            .unwrap_or(image::CharacterSet::SmallBlocks)
        {
            image::CharacterSet::SmallBlocks => "SmallBlocks",
            image::CharacterSet::LargeBlocks => "LargeBlocks",
            image::CharacterSet::DitheredShades => "DitheredShades",
            image::CharacterSet::Braille => "Braille",
            image::CharacterSet::AsciiArt => "AsciiArt",
        };
        commandbar.set(key!("F2"), chset, mywin::Commands::CharSet);

        let cschema = match self
            .control(self.himg)
            .map(|i| i.render_options().color_schema())
            .unwrap_or(image::ColorSchema::Auto)
        {
            image::ColorSchema::Auto => "Auto",
            image::ColorSchema::Color16 => "16 Colors",
            image::ColorSchema::TrueColors => "True Colors",
            image::ColorSchema::GrayScale4 => "Gray (4 colors)",
            image::ColorSchema::GrayScaleTrueColors => "Gray Scale",
            image::ColorSchema::BlackAndWhite => "Black and White",
        };
        commandbar.set(key!("F3"), cschema, mywin::Commands::ColorSchema);

        let luminance = self.control(self.himg).map(|i| i.render_options().luminance_threshold()).unwrap_or(0.1);
        let current_percent = (luminance * 100.0) as u8;

        let closest_percent = LUMINANCE_SEQUENCE.iter().min_by_key(|&&x| current_percent.abs_diff(x)).unwrap_or(&5);

        commandbar.set(key!("F4"), format!("Luminance:{closest_percent}%").as_str(), mywin::Commands::Luminance);
    }

    fn on_event(&mut self, command_id: mywin::Commands) {
        let h = self.himg;
        if let Some(img) = self.control_mut(h) {
            match command_id {
                mywin::Commands::Scale => {
                    let sc = img.render_options().scale();
                    let new_scale = match sc {
                        image::Scale::NoScale => image::Scale::Scale50,
                        image::Scale::Scale50 => image::Scale::Scale33,
                        image::Scale::Scale33 => image::Scale::Scale25,
                        image::Scale::Scale25 => image::Scale::Scale20,
                        image::Scale::Scale20 => image::Scale::Scale10,
                        image::Scale::Scale10 => image::Scale::Scale5,
                        image::Scale::Scale5 => image::Scale::NoScale,
                    };
                    let mut opt = img.render_options().clone();
                    opt.set_scale(new_scale);
                    img.set_render_options(opt);
                }
                mywin::Commands::CharSet => {
                    let chset = img.render_options().character_set();
                    let new_chset = match chset {
                        image::CharacterSet::SmallBlocks => image::CharacterSet::LargeBlocks,
                        image::CharacterSet::LargeBlocks => image::CharacterSet::DitheredShades,
                        image::CharacterSet::DitheredShades => image::CharacterSet::Braille,
                        image::CharacterSet::Braille => image::CharacterSet::AsciiArt,
                        image::CharacterSet::AsciiArt => image::CharacterSet::SmallBlocks,
                    };
                    let mut opt = img.render_options().clone();
                    opt.set_character_set(new_chset);
                    img.set_render_options(opt);
                }
                mywin::Commands::ColorSchema => {
                    let cs = img.render_options().color_schema();
                    let new_cs = match cs {
                        image::ColorSchema::Auto => image::ColorSchema::Color16,
                        image::ColorSchema::Color16 => image::ColorSchema::TrueColors,
                        image::ColorSchema::TrueColors => image::ColorSchema::GrayScale4,
                        image::ColorSchema::GrayScale4 => image::ColorSchema::GrayScaleTrueColors,
                        image::ColorSchema::GrayScaleTrueColors => image::ColorSchema::BlackAndWhite,
                        image::ColorSchema::BlackAndWhite => image::ColorSchema::Auto,
                    };
                    let mut opt = img.render_options().clone();
                    opt.set_color_schema(new_cs);
                    img.set_render_options(opt);
                }
                mywin::Commands::Luminance => {
                    let current_luminance = img.render_options().luminance_threshold();
                    let current_percent = (current_luminance * 100.0) as u8;

                    let closest_percent = LUMINANCE_SEQUENCE.iter().min_by_key(|&&x| current_percent.abs_diff(x)).unwrap_or(&5);

                    let next_percent = if let Some(current_index) = LUMINANCE_SEQUENCE.iter().position(|&x| x == *closest_percent) {
                        let next_index = (current_index + 1) % LUMINANCE_SEQUENCE.len();
                        LUMINANCE_SEQUENCE[next_index]
                    } else {
                        5
                    };

                    let mut opt = img.render_options().clone();
                    opt.set_luminance_threshold(next_percent as f64 / 100.0);
                    img.set_render_options(opt);
                }
            }
        }
        self.request_update();
    }
}
