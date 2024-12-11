use appcui::prelude::*;

#[Window(events = ButtonEvents)]
struct MyWin {}
impl MyWin {
    fn new() -> Self {
        let mut w = Self { base: window!("Test, d:c") };
        w.add(button!("Test,d:c,w:10"));
        w
    }
}
impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        dialogs::save("abc.exe", ".", "Images = [jpg,png,bmp], Documents = [txt,docx], Executable and scripts = [exe,dll,js,py,ps1,sh,bat,cmd]");
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
