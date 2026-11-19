use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'Stacked bars',a:c,w:70,h:16");
            win.add(vstackedbarchart!(
                "i32, 
                 d:f, 
                 flags: ScrollBars+ShowZeroLineOnYAxis+DimBarsOnSelection, 
                 dbw: 3, 
                 dbs: 2, 
                 xlabels: StackLabels,
                 stacks: [
                    {Jan, bars: [{12, attr: red}, {5, attr: green}, {3, attr: aqua}, {4, attr: yellow}]},
                    {Feb, bars: [{25, attr: red}, {8, attr: green}, {6, attr: aqua}, {3, attr: yellow}]},
                    {Mar, bars: [{8, attr: red}, {4, attr: green}, {6, attr: aqua}, {2, attr: yellow}]},
                    {Apr, bars: [{10, attr: red}, {4, attr: green}, {30, attr: aqua}, {5, attr: yellow}]},
                    {May, bars: [{18, attr: red}, {2, attr: green}, {9, attr: aqua}, {4, attr: yellow}]}
                ]"
            ));
            win
        })
        .run()
}
