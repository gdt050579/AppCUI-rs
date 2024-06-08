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


#[test]
fn check_expand(){
    let script = "
        // Paint.Enable(false)
        Paint('Initial State')
        // CheckHash(0xBE767D638014E39A)
        Mouse.Drag(22,9,22,2)
        Mouse.Click(26,5,left)
        Paint('Calendar expanded')
        CheckHash(0xA961AB3468AE0A49)
    ";
    let date_str = "2024-06-13T12:00:00Z";
    let parsed_date = date_str.parse::<DateTime<Utc>>().unwrap();
    
    let mut a = App::debug(60, 25, script).build().unwrap();
    let mut w = window!("Dates,d:c,w:25,h:6");
    w.add(DatePicker::new(parsed_date, Layout::new("d:c,w:19")));
    a.add_window(w);
    a.run();
}

#[test]
fn check_hovers(){
    let script = "
        // Paint.Enable(false)
        Paint('Initial State')

        Mouse.Drag(22,9,22,19)
        Mouse.Click(26,22,left)
        Paint('Calendar expanded top')
        CheckHash(0x398D1A8637915FBC)
        
        Mouse.Move(24,17)
        Paint('Hover on date')
        CheckHash(0x6538AEDEA96A491C)

        Mouse.Move(24,17)
        Paint('Hover on date')
        CheckHash(0x6538AEDEA96A491C)

        Mouse.Move(24,12)
        Paint('Hover on double left arrows')
        CheckHash(0xF83DF94071A3F72C)

        Mouse.Move(26,12)
        Paint('Hover on year arrow left')
        CheckHash(0xC317DFBBC0569F14)
        
        Mouse.Move(33,12)
        Paint('Hover on year arrow right')
        CheckHash(0xAC549929AEF94374)

        Mouse.Move(36,12)
        Paint('Hover on double right arrows')
        CheckHash(0x44FC96678705B5AC)
    ";
    let date_str = "2024-06-13T12:00:00Z";
    let parsed_date = date_str.parse::<DateTime<Utc>>().unwrap();
    
    let mut a = App::debug(60, 25, script).build().unwrap();
    let mut w = window!("Dates,d:c,w:25,h:6");
    w.add(DatePicker::new(parsed_date, Layout::new("d:c,w:19")));
    a.add_window(w);
    a.run();
}