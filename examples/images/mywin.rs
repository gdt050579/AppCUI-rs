use appcui::prelude::*;

#[Window(events=CommandBarEvents,commands:[Scale,RenderMethod])]
pub struct MyWin {
    himg: Handle<ImageViewer>,
}
impl MyWin {
    pub fn new(title: &str, image: Image) -> Self {
        let mut w = Self {
            base: Window::new(title,Layout::new("d:c,w:50%,h:50%"),window::Flags::Sizeable),
            himg: Handle::None,
        };

        let i = ImageViewer::new(
            image,
            Layout::new("d:c"),
            image::RendererType::SmallBlocks,
            image::Scale::NoScale,
            imageviewer::Flags::ScrollBars,
        );
        w.himg = w.add(i);
        w
    }
}
impl CommandBarEvents for MyWin {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        let sc_name = match self.control(self.himg).map(|i| i.scale()).unwrap_or(image::Scale::NoScale) {
            Scale::NoScale => "Scale:100%",
            Scale::Scale50 => "Scale:50%",
            Scale::Scale33 => "Scale:33%",
            Scale::Scale25 => "Scale:25%",
            Scale::Scale20 => "Scale:20%",
            Scale::Scale10 => "Scale:10%",
            Scale::Scale5 => "Scale:5%",
        };
        commandbar.set(key!("F1"), sc_name, mywin::Commands::Scale);
        let rd_name = match self.control(self.himg).map(|i| i.render_method()).unwrap_or(image::RendererType::SmallBlocks) {
            RendererType::SmallBlocks => "Method:SmallBlocks",
            RendererType::LargeBlocks64Colors => "Method:LargeBlocks (64 colors)",
            RendererType::GrayScale => "Method:GrayScale",
            RendererType::AsciiArt => "Method:AsciiArt",
        };
        commandbar.set(key!("F2"), rd_name, mywin::Commands::RenderMethod);
    }

    fn on_event(&mut self, command_id: mywin::Commands) {
        let h = self.himg;
        if let Some(img) = self.control_mut(h) {
            match command_id {
                mywin::Commands::Scale => {
                    let sc = img.scale();
                    match sc {
                        Scale::NoScale => img.set_scale(image::Scale::Scale50),
                        Scale::Scale50 => img.set_scale(image::Scale::Scale33),
                        Scale::Scale33 => img.set_scale(image::Scale::Scale25),
                        Scale::Scale25 => img.set_scale(image::Scale::Scale20),
                        Scale::Scale20 => img.set_scale(image::Scale::Scale10),
                        Scale::Scale10 => img.set_scale(image::Scale::Scale5),
                        Scale::Scale5 => img.set_scale(image::Scale::NoScale),
                    }
                }
                mywin::Commands::RenderMethod => {
                    let m = img.render_method();
                    match m {
                        RendererType::SmallBlocks => img.set_render_method(image::RendererType::LargeBlocks64Colors),
                        RendererType::LargeBlocks64Colors => img.set_render_method(image::RendererType::GrayScale),
                        RendererType::GrayScale => img.set_render_method(image::RendererType::AsciiArt),
                        RendererType::AsciiArt => img.set_render_method(image::RendererType::SmallBlocks),
                    }
                },
            }
        }
        self.request_update();
    }
}
