use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'Usage chart',a:c,w:70,h:10,flags:Sizeable");
            // 256 GB drive: OS, apps, documents, media, cache. The rest stays free.
            win.add(usagechart!(
                "d:f, 
                flags: Margins+DimSegmentsOnSelection, 
                total: Fixed(256), 
                nf: {dec, suffix: ' GB'}, 
                lc: LabelAndPercentage,
                segments: [
                    {48, label: 'OS', fill: Solid},
                    {36, label: 'Apps', fill: CrossHatch},
                    {22, label: 'Documents', fill: Grid},
                    {22, label: 'Media', fill: DiagonalDown},
                    {8, label: 'Cache', fill: GridDouble}
                ]"
            ));
            win
        })
        .run()
}
