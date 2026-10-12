use appcui::prelude::*;

#[Window(events = [NumericSelectorEvents<i32>, NumericSelectorEvents<f32>, NumericSelectorEvents<u32>, NumericSelectorEvents<u64>, NumericSelectorEvents<i64>])]
struct MyWin {
    volume: Handle<NumericSelector<i32>>,
    volume_lb: Handle<Label>,
    opacity: Handle<NumericSelector<f32>>,
    opacity_lb: Handle<Label>,
    color: Handle<NumericSelector<u32>>,
    color_lb: Handle<Label>,
    file_size: Handle<NumericSelector<u64>>,
    file_size_lb: Handle<Label>,
    population: Handle<NumericSelector<i64>>,
    population_lb: Handle<Label>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'NumericSelector',a:c,w:62,h:14"),
            volume: Handle::None,
            volume_lb: Handle::None,
            opacity: Handle::None,
            opacity_lb: Handle::None,
            color: Handle::None,
            color_lb: Handle::None,
            file_size: Handle::None,
            file_size_lb: Handle::None,
            population: Handle::None,
            population_lb: Handle::None,
        };

        // Integer, plain decimal.
        win.add(label!("'Volume:',x:1,y:1,w:14,h:1"));
        win.volume = win.add(numericselector!("i32,50,0,100,step:5,x:16,y:1,w:25,h:1"));
        win.volume_lb = win.add(label!("'50',x:43,y:1,w:17,h:1"));

        // Float, shown as a percentage.
        win.add(label!("'Opacity:',x:1,y:3,w:14,h:1"));
        win.opacity = win.add(numericselector!("f32,0.5,0.0,1.0,step:0.05,x:16,y:3,w:25,h:1,format:Percentage"));
        win.opacity_lb = win.add(label!("'0.50',x:43,y:3,w:17,h:1"));

        // Unsigned integer, shown in hexadecimal.
        win.add(label!("'Color:',x:1,y:5,w:14,h:1"));
        win.color = win.add(numericselector!("u32,16,0,255,step:1,x:16,y:5,w:25,h:1,format:Hex"));
        win.color_lb = win.add(label!("'0x10',x:43,y:5,w:17,h:1"));

        // Unsigned integer, shown as a human-readable size.
        win.add(label!("'File size:',x:1,y:7,w:14,h:1"));
        win.file_size = win.add(numericselector!("u64,5242880,0,1073741824,step:1048576,x:16,y:7,w:25,h:1,format:Size"));
        win.file_size_lb = win.add(label!("'5242880 bytes',x:43,y:7,w:17,h:1"));

        // Signed integer, with digit grouping and no +/- buttons.
        win.add(label!("'Population:',x:1,y:9,w:14,h:1"));
        win.population = win.add(numericselector!(
            "i64,1000000,0,10000000,step:1000,x:16,y:9,w:25,h:1,format:DigitGrouping,flags:HideButtons"
        ));
        win.population_lb = win.add(label!("'1000000',x:43,y:9,w:17,h:1"));

        win
    }

    fn show(&mut self, lb: Handle<Label>, text: String) {
        self.control_mut(lb).unwrap().set_caption(&text);
    }
}

impl NumericSelectorEvents<i32> for MyWin {
    fn on_value_changed(&mut self, handle: Handle<NumericSelector<i32>>, value: i32) -> EventProcessStatus {
        if handle == self.volume {
            let lb = self.volume_lb;
            self.show(lb, format!("{}", value));
        }
        EventProcessStatus::Processed
    }
}

impl NumericSelectorEvents<f32> for MyWin {
    fn on_value_changed(&mut self, handle: Handle<NumericSelector<f32>>, value: f32) -> EventProcessStatus {
        if handle == self.opacity {
            let lb = self.opacity_lb;
            self.show(lb, format!("{:.2}", value));
        }
        EventProcessStatus::Processed
    }
}

impl NumericSelectorEvents<u32> for MyWin {
    fn on_value_changed(&mut self, handle: Handle<NumericSelector<u32>>, value: u32) -> EventProcessStatus {
        if handle == self.color {
            let lb = self.color_lb;
            self.show(lb, format!("{:#X}", value));
        }
        EventProcessStatus::Processed
    }
}

impl NumericSelectorEvents<u64> for MyWin {
    fn on_value_changed(&mut self, handle: Handle<NumericSelector<u64>>, value: u64) -> EventProcessStatus {
        if handle == self.file_size {
            let lb = self.file_size_lb;
            self.show(lb, format!("{} bytes", value));
        }
        EventProcessStatus::Processed
    }
}

impl NumericSelectorEvents<i64> for MyWin {
    fn on_value_changed(&mut self, handle: Handle<NumericSelector<i64>>, value: i64) -> EventProcessStatus {
        if handle == self.population {
            let lb = self.population_lb;
            self.show(lb, format!("{}", value));
        }
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().window(MyWin::new).run()
}
