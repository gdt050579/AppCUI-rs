use chrono::{DateTime, Utc};

use crate::prelude::*;

#[test]
fn check_create(){
    let script = "
        Paint.Enable(false)
        Paint('Initial State')
        CheckHash(0xBE767D638014E39A)
    ";
    let date_str = "2024-06-13T12:00:00Z";
    let parsed_date = date_str.parse::<DateTime<Utc>>().unwrap();
    
    let mut a = App::debug(60, 11, script).build().unwrap();
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

#[test]
fn check_on_hover(){
    let script = "
        Paint.Enable(false)
        Paint('Initial State')
        CheckHash(0xBE767D638014E39A)
        Mouse.Move(10,8)
        Paint('Mouse hover')
        CheckHash(0x5F940267F4247C82)
        Mouse.Move(31,6)
        Paint('Mouse hover 2')
        CheckHash(0x93797768E0658943)
        
    ";
    let date_str = "2024-06-13T12:00:00Z";
    let parsed_date = date_str.parse::<DateTime<Utc>>().unwrap();
    
    let mut a = App::debug(60, 11, script).build().unwrap();
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