use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'Usage chart',a:c,w:70,h:10,flags:Sizeable");
            let mut chart = UsageChart::new(layout!("d:f"), usagechart::Flags::None);
            // 256 GB drive: OS, apps, documents, media, cache. The rest stays free.
            chart.add_segment(48u64);
            chart.add_segment(36u64);
            chart.add_segment(22u64);
            chart.add_segment(14u64);
            chart.add_segment(8u64);
            chart.set_total_usage(usagechart::TotalUsage::Fixed(256));
            win.add(chart);
            win
        })
        .run()
}
