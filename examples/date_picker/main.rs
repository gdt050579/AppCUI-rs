use chrono::{DateTime, Utc};

use appcui::prelude::*;

fn main(){
    let date_str = "2024-06-13T12:00:00Z";
    let parsed_date = date_str.parse::<DateTime<Utc>>().unwrap();
    
    let mut a = App::new().build().unwrap();
    let mut w = window!("Dates,d:c,w:50,h:11");
    w.add(DatePicker::new(parsed_date, Layout::new("x:1,y:1,w:19")));
    w.add(DatePicker::new(parsed_date, Layout::new("x:1,y:3,w:16")));
    w.add(DatePicker::new(parsed_date, Layout::new("x:1,y:5,w:14")));
    w.add(DatePicker::new(parsed_date, Layout::new("x:1,y:7,w:10")));
    w.add(DatePicker::new(parsed_date, Layout::new("x:23,y:1,w:23")));
    w.add(DatePicker::new(parsed_date, Layout::new("x:23,y:3,w:7")));
    w.add(DatePicker::new(parsed_date, Layout::new("x:23,y:5,w:6")));
    a.add_window(w);
    a.run();
}
