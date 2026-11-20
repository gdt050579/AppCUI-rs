use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'Stacked bars',a:c,w:74,h:20");
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
                    {May, bars: [{14, attr: red}, {2, attr: green}, {9, attr: aqua}, {4, attr: yellow}]},
                    {Jun, bars: [{16, attr: red}, {3, attr: green}, {14, attr: aqua}, {3, attr: yellow}]},
                    {Jul, bars: [{10, attr: red}, {2, attr: green}, {12, attr: aqua}, {2, attr: yellow}]},
                    {Aug, bars: [{19, attr: red}, {1, attr: green}, {8, attr: aqua}, {4, attr: yellow}]},
                    {Sep, bars: [{15, attr: red}, {5, attr: green}, {17, attr: aqua}, {5, attr: yellow}]},
                    {Oct, bars: [{11, attr: red}, {2, attr: green}, {16, attr: aqua}, {6, attr: yellow}]},
                    {Nov, bars: [{18, attr: red}, {1, attr: green}, {10, attr: aqua}, {4, attr: yellow}]},
                    {Dec, bars: [{20, attr: red}, {2, attr: green}, {9, attr: aqua}, {2, attr: yellow}]}
                ]"
            ));
            win
        })
        .run()
}
