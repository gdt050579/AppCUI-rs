use appcui::prelude::*;

#[Window(events=CommandBarEvents,commands:[Scale,CharSet,ColorSchema])]
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
            image::RenderOptionsBuilder::new().build(),
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
            Scale::NoScale => "Scale:100%",
            Scale::Scale50 => "Scale:50%",
            Scale::Scale33 => "Scale:33%",
            Scale::Scale25 => "Scale:25%",
            Scale::Scale20 => "Scale:20%",
            Scale::Scale10 => "Scale:10%",
            Scale::Scale5 => "Scale:5%",
        };
        commandbar.set(key!("F1"), sc_name, mywin::Commands::Scale);
        let chset = match self
            .control(self.himg)
            .map(|i| i.render_options().character_set())
            .unwrap_or(image::CharacterSet::SmallBlocks)
        {
            CharacterSet::SmallBlocks => "SmallBlocks",
            CharacterSet::LargeBlocks => "LargeBlocks",
            CharacterSet::DitheredShades => "DitheredShades",
            CharacterSet::Braille => "Braille",
            CharacterSet::AsciiArt => "AsciiArt",
        };
        commandbar.set(key!("F2"), chset, mywin::Commands::CharSet);

        let cschema = match self
            .control(self.himg)
            .map(|i| i.render_options().color_schema())
            .unwrap_or(image::ColorSchema::Auto)
        {
            ColorSchema::Auto => "Auto",
            ColorSchema::Color16 => "16 Colors",
            //#[cfg(feature = "TRUE_COLORS")]
            ColorSchema::TrueColors => "True Colors",
            ColorSchema::GrayScale4 => "Gray (4 colors)",
            //#[cfg(feature = "TRUE_COLORS")]
            ColorSchema::GrayScaleTrueColors => "Gray Scale",
            ColorSchema::BlackAndWhite => "Black and White",
        };
        commandbar.set(key!("F3"), cschema, mywin::Commands::ColorSchema);        
    }

    fn on_event(&mut self, command_id: mywin::Commands) {
        let h = self.himg;
        if let Some(img) = self.control_mut(h) {
            match command_id {
                mywin::Commands::Scale => {
                    let sc = img.render_options().scale();
                    let new_scale = match sc {
                        Scale::NoScale => image::Scale::Scale50,
                        Scale::Scale50 => image::Scale::Scale33,
                        Scale::Scale33 => image::Scale::Scale25,
                        Scale::Scale25 => image::Scale::Scale20,
                        Scale::Scale20 => image::Scale::Scale10,
                        Scale::Scale10 => image::Scale::Scale5,
                        Scale::Scale5 => image::Scale::NoScale,
                    };
                    let mut opt = img.render_options().clone();
                    opt.set_scale(new_scale);
                    img.set_render_options(opt);
                }
                mywin::Commands::CharSet => {
                    let chset = img.render_options().character_set();
                    let new_chset = match chset {
                        CharacterSet::SmallBlocks => CharacterSet::LargeBlocks,
                        CharacterSet::LargeBlocks => CharacterSet::DitheredShades,
                        CharacterSet::DitheredShades => CharacterSet::Braille,
                        CharacterSet::Braille => CharacterSet::AsciiArt,
                        CharacterSet::AsciiArt => CharacterSet::SmallBlocks,
                    };
                    let mut opt = img.render_options().clone();
                    opt.set_character_set(new_chset);
                    img.set_render_options(opt);
                }
                mywin::Commands::ColorSchema => {
                    let cs = img.render_options().color_schema();
                    let new_cs = match cs {
                        ColorSchema::Auto => ColorSchema::Color16,
                        ColorSchema::Color16 => ColorSchema::TrueColors,
                        ColorSchema::TrueColors => ColorSchema::GrayScale4,
                        ColorSchema::GrayScale4 => ColorSchema::GrayScaleTrueColors,
                        ColorSchema::GrayScaleTrueColors => ColorSchema::BlackAndWhite ,
                        ColorSchema::BlackAndWhite => ColorSchema::Auto,
                    };
                    let mut opt = img.render_options().clone();
                    opt.set_color_schema(new_cs);
                    img.set_render_options(opt);
                }
            }
        }
        self.request_update();
    }
}
