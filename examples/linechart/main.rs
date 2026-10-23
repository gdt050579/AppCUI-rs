use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'Line chart',a:c,w:70,h:20,flags:Sizeable");
            let mut chart = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::ScrollBars | linechart::Flags::GroupHover);
            chart.set_default_spacing(3);
            chart.set_xaxis_label_mode(linechart::XAxisLabelMode::PointLabels);

            // first series also provides the labels for the X axis
            let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
            let cpu = [35, 42, 38, 55, 61, 48, 72, 80, 66, 58, 63, 70];
            chart.add_series(
                linechart::SeriesBuilder::new("CPU")
                    .line(linechart::LineMode::Braille)
                    .attr(charattr!("green"))
                    .values(months.iter().zip(cpu).map(|(m, v)| BarBuilder::new(v).label(m).build())),
            );
            chart.add_series(
                linechart::SeriesBuilder::new("RAM")
                    .line(linechart::LineMode::Line(LineType::Single))
                    .attr(charattr!("yellow"))
                    .values([20, 22, 25, 24, 30, 33, 31, 36, 40, 38, 41, 45]),
            );
            chart.add_series(
                linechart::SeriesBuilder::new("Disk")
                    .line(linechart::LineMode::Orthogonal(LineType::Single, OrthogonalDirection::HorizontalFirst))
                    .attr(charattr!("aqua"))
                    .values([5, 5, 10, 10, 10, 15, 15, 20, 20, 25, 25, 25]),
            );
            win.add(chart);
            win
        })
        .run()
}
