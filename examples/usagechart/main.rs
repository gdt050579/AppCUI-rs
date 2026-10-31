use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'Usage chart',a:c,w:70,h:10,flags:Sizeable");
            // 256 GB drive: OS, apps, documents, media, cache. The rest stays free.
            win.add(usagechart!(
                "d:f, flags: Margins+DimSegmentsOnSelection, total: Fixed(256), segments: [
                    {48, fill: Solid},
                    {36, fill: CrossHatch},
                    {22, fill: Grid},
                    {22, fill: DiagonalDown},
                    {8, fill: GridDouble}
                ]"
            ));
            win
        })
        .run()
}
