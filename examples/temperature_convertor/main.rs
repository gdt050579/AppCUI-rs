use appcui::prelude::*;

#[Window(events = NumericSelectorEvents<f64>)]
struct MyWin {
    celsius: Handle<NumericSelector<f64>>,
    fahrenheit: Handle<NumericSelector<f64>>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'Convert',d:c,w:40,h:7"),
            celsius: Handle::None,
            fahrenheit: Handle::None,
        };
        win.add(label!("'Celsius:',x:1,y:1,w:12,h:1"));
        win.celsius = win.add(numericselector!("f64,0.0,x:14,y:1,w:23,min:-100.0,max:100.0,step:1.0"));
        win.add(label!("'Fahrenheit:',x:1,y:3,w:12,h:1"));
        win.fahrenheit = win.add(numericselector!("f64,32.0,x:14,y:3,w:23,min:-213.0,max:213.0,step:0.1"));
        win
    }
    fn convert_celsius_to_feherenheit(&mut self) {
        let celsius = self.control(self.celsius).unwrap().value();
        let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
        let h = self.fahrenheit;
        self.control_mut(h).unwrap().set_value(fahrenheit);
    }
    fn convert_fahrenheit_to_celsius(&mut self) {
        let fahrenheit = self.control(self.fahrenheit).unwrap().value();
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        let h = self.celsius;
        self.control_mut(h).unwrap().set_value(celsius);
    }
}

impl NumericSelectorEvents<f64> for MyWin {
    fn on_value_changed(&mut self, handle: Handle<NumericSelector<f64>>, _value: f64) -> EventProcessStatus {
        match () {
            _ if handle == self.celsius => {
                self.convert_celsius_to_feherenheit();
                EventProcessStatus::Processed
            }
            _ if handle == self.fahrenheit => {
                self.convert_fahrenheit_to_celsius();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
