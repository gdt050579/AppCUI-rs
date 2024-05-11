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
            image::RenderMethod::SmallBlocks,
            image::Scale::NoScale,
            image_viewer::Flags::ScrollBars,
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
        let rd_name = match self.control(self.himg).map(|i| i.render_method()).unwrap_or(image::RenderMethod::SmallBlocks) {
            RenderMethod::SmallBlocks => "Method:SmallBlocks",
            RenderMethod::LargeBlocks64Colors => "Method:LargeBlocks (64 colors)",
            RenderMethod::GrayScale => "Method:GrayScale",
            RenderMethod::AsciiArt => "Method:AsciiArt",
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
                        RenderMethod::SmallBlocks => img.set_render_method(image::RenderMethod::LargeBlocks64Colors),
                        RenderMethod::LargeBlocks64Colors => img.set_render_method(image::RenderMethod::GrayScale),
                        RenderMethod::GrayScale => img.set_render_method(image::RenderMethod::AsciiArt),
                        RenderMethod::AsciiArt => img.set_render_method(image::RenderMethod::SmallBlocks),
                    }
                },
            }
        }
        self.request_update();
    }
}
