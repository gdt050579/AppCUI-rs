use appcui::prelude::*;

#[Window(events=DatePickerEvents)]
struct MyWin {
    dp: Handle<DatePicker>,
}

impl MyWin{
    fn new() -> Self{
        let mut win = MyWin{
            base: window!("Dates,a:c,w:25,h:6"),
            dp: Handle::None,
        };
        win.dp = win.add(datepicker!("2024-06-13,x:1,y:1,w:19"));
        win
    }

}

impl DatePickerEvents for MyWin{
    fn on_date_change(&mut self, _handle: Handle<DatePicker>, date: chrono::prelude::NaiveDate) -> EventProcessStatus {
        self.base.set_title(&format!("Date: {date}"));
        EventProcessStatus::Processed                                                                        
    }
}

fn main(){
    let mut a =  App::new().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}
