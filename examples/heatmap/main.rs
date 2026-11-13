use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'Heat map',a:c,w:70,h:20,flags:Sizeable");
            // Cells are two characters wide, so vertical distances are doubled
            // to keep the hot spots round on screen.
            let mut heatmap: HeatMap<i32> = HeatMap::new(layout!("d:f"), Size::new(40, 20), heatmap::Flags::ScrollBars);
            heatmap.fill_with(|row, column| {
                let heat = |cx: i32, cy: i32| {
                    let dx = column as i32 - cx;
                    let dy = (row as i32 - cy) * 2;
                    (180 - (dx * dx + dy * dy)).max(0)
                };
                Some(heat(10, 6) + heat(28, 14))
            });
            win.add(heatmap);
            win
        })
        .run()
}
