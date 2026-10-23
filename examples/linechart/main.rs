use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'Line chart',a:c,w:70,h:20,flags:Sizeable");
            win.add(linechart!(
                "type: i32, 
                d:f, 
                flags: [ScrollBars, GroupHover], 
                spacing: 3,
                labels: [Jan, Feb, Mar, Apr, May, Jun, Jul, Aug, Sep, Oct, Nov, Dec],
                series: [
                    {CPU, line: Braille, attr: green, values: [35, 42, 38, 55, 61, 48, 72, {80, label: peak}, 66, 58, 63, 70]},
                    {RAM, line: Line(Single), attr: yellow, values: [20, 22, 25, 24, 30, 33, 31, 36, 40, 38, 41, 45]},
                    {Disk, line: Orthogonal(Single, HorizontalFirst), attr: aqua, values: [5, 5, 10, 10, 10, 15, 15, 20, 20, 25, 25, 25]}
                ]"
            ));
            win
        })
        .run()
}
