use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'Line chart',a:c,w:70,h:20,flags:Sizeable");
            let mut chart = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::ScrollBars | linechart::Flags::GroupHover);
            chart.set_default_spacing(3);

            // the names of the X positions belong to the chart
            chart.set_xaxis_labels(["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]);
            chart.set_xaxis_label_mode(linechart::XAxisLabelMode::Labels);

            chart.add_series(
                linechart::SeriesBuilder::new("CPU")
                    .line(linechart::LineMode::Braille)
                    .attr(charattr!("green"))
                    .values([35, 42, 38, 55, 61, 48, 72, 80, 66, 58, 63, 70]),
            );
            // a point label is a note about that value, shown in its tooltip
            chart.update_series(0, |s| {
                s.get_mut(7).unwrap().set_label("peak");
            });
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
