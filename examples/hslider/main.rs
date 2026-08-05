use appcui::{backend, prelude::*};

#[Window(events = [HSliderEvents<i32>, HSliderEvents<f32>])]
struct MyWin {}
impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'Test',a:c,w:40,h:10,flags:Sizeable"),
        };
        let hslider = hslider!("i32,0,10,1,x:0,y:0,w:20,h:1");
        win.add(hslider);
        let hslider = hslider!("class:f32,min:0f32,max:10f32,step:1.5f32,x:0,y:1,w:20,h:1,flags:ShowValue");
        win.add(hslider);
        let hslider = HSlider::new(0, 10, 1, hslider::Type::Inline, layout!("x:0,y:2,w:20"), hslider::Flags::None);
        win.add(hslider);
        let hslider = HSlider::new(0, 100, 1, hslider::Type::Standard, layout!("x:0,y:3,w:30"), hslider::Flags::ValueAsMarker);
        win.add(hslider);
        let mut hslider = hslider!("i32,0,100,1,type:Inline,x:0,y:5,w:21,h:1,flags:Ticks");
        hslider.set_ticks(5);
        win.add(hslider);

        let mut hslider = HSlider::new(0, 100, 1, hslider::Type::ProgressBar, layout!("x:0,y:7,w:21,h:1"), hslider::Flags::None);
        hslider.set_value(50);
        win.add(hslider);
        win
    }
}

impl HSliderEvents<i32> for MyWin {
    fn on_value_changed(&mut self, _handle: Handle<HSlider<i32>>, value: i32) -> EventProcessStatus {
        self.set_title(&format!("{}", value));
        EventProcessStatus::Processed
    }
}

impl HSliderEvents<f32> for MyWin {
    fn on_value_changed(&mut self, _handle: Handle<HSlider<f32>>, value: f32) -> EventProcessStatus {
        self.set_title(&format!("{}", value));
        EventProcessStatus::Ignored
    }
}

fn main() -> Result<(), appcui::system::Error> {
    #[cfg(target_os = "windows")]
    let mut app = App::with_backend(backend::Type::WindowsVT).color_schema(false).build()?;
    #[cfg(not(target_os = "windows"))]
    let mut app = App::new().color_schema(false).build()?;

    app.add_window(MyWin::new());

    app.run();
    Ok(())
}
