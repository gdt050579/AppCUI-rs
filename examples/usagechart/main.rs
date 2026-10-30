use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'Usage chart',a:c,w:70,h:10,flags:Sizeable");
            let mut chart = UsageChart::new(layout!("d:f"), usagechart::Flags::Margins | usagechart::Flags::DimSegmentsOnSelection);
            // 256 GB drive: OS, apps, documents, media, cache. The rest stays free.
            chart.add_segment(usagechart::SegmentBuilder::new().value(48).fill(usagechart::SegmentFillType::Solid).build());
            chart.add_segment(usagechart::SegmentBuilder::new().value(36).fill(usagechart::SegmentFillType::CrossHatch).build());
            chart.add_segment(usagechart::SegmentBuilder::new().value(22).fill(usagechart::SegmentFillType::Grid).build());
            chart.add_segment(usagechart::SegmentBuilder::new().value(22).fill(usagechart::SegmentFillType::DiagonalDown).build());
            chart.add_segment(usagechart::SegmentBuilder::new().value(8).fill(usagechart::SegmentFillType::GridDouble).build());
            chart.set_total_usage(usagechart::TotalUsage::Fixed(256));
            win.add(chart);
            win
        })
        .run()
}
