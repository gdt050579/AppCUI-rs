use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'Pie chart',a:c,w:60,h:20,flags:Sizeable");
            win.add(piechart!(
                "d:f,
                renderer: HalfBlocks,
                angle: 30,
                dir: ccw,
                shape: Circle,
                content: LabelAndPercentage,
                labels: Outside,
                hollow: 50%,
                values: [{45, Rust, color: Green}, 
                         {25, 'C++', color: Red}, 
                         {15, Go, color: Aqua}, 
                         {10, Zig, color: yellow}, 
                         {5, Other, attr: magenta}]"
            ));
            win
        })
        .run()
}
