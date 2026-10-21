use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'Pie chart',a:c,w:60,h:20,flags:Sizeable");
            win.add(piechart!(
                "d:f,
                renderer: Braille,
                angle: 90,
                dir: ccw,
                shape: RoundTriangle,
                content: LabelAndPercentage,
                labels: Outside,
                hollow: 50%,
                values: [{45, Rust}, {25, 'C++'}, {15, Go}, {10, Zig}, {5, Other, attr: gray}]"
            ));
            win
        })
        .run()
}
