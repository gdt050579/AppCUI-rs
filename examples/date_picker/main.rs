use appcui::prelude::*;

fn main(){
    let mut a =  App::new().build().unwrap();
    let mut w = window!("Dates,d:c,w:25,h:6");
    // w.add(DatePicker::new("2024-06-13", Layout::new("d:c,w:19")));
    w.add(datepicker!("2024-06-13,x:1,y:1,w:19"));
    a.add_window(w);
    a.run();
}
