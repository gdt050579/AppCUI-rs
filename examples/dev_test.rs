use appcui::prelude::*;
use appcui::ui::numericslider::*;
use appcui::ui::common::number::*;

#[Window(events = ButtonEvents)]
struct MyWin {}
impl MyWin {
    fn new() -> Self {
        let mut w = Self { base: window!("Test,d:c,w:300,h:11,flags: Sizeable") };
        let num_slider = NumericSlider::new(50, 100, 5, 75, Format::Decimal, Layout::new("x:1,y:2,w:100%"), Flags::SingleLine | Flags::HorizontalSlider);
        w.add(num_slider);
        w
    }
}
impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        //dialogs::save("abc.exe", ".", "Images = [jpg,png,bmp], Documents = [txt,docx], Executable and scripts = [exe,dll,js,py,ps1,sh,bat,cmd]");
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
