use chrono::NaiveDate;

use appcui::prelude::*;

fn main(){
    let date_str = "2024-06-13";
    let parsed_date = date_str.parse::<NaiveDate>().unwrap();
    
    let mut a =  App::new().build().unwrap();
    let mut w = window!("Dates,d:c,w:25,h:6");
    w.add(DatePicker::new(parsed_date, Layout::new("d:c,w:19")));
    a.add_window(w);
    a.run();
}
