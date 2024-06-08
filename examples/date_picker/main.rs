use chrono::{DateTime, Utc};

use appcui::prelude::*;

fn main(){
    let date_str = "2024-06-13T12:00:00Z";
    let parsed_date = date_str.parse::<DateTime<Utc>>().unwrap();
    
    let mut a =  App::new().build().unwrap();
    let mut w = window!("Dates,d:c,w:25,h:6");
    w.add(DatePicker::new(parsed_date, Layout::new("d:c,w:19")));
    a.add_window(w);
    a.run();
}
