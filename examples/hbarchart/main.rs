use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'Horizontal bars',a:c,w:70,h:16");
            win.add(hbarchart!(
                "i32,d:f,dbh:1,xw:4,ylabels:BarLabels,flags:ShowZeroLineOnXAxis,values:[
                    {12,label:Jan},
                    {25,label:Feb},
                    {8,label:Mar},
                    {30,label:Apr},
                    {18,label:May}
                ]"
            ));
            win
        })
        .run()
}
