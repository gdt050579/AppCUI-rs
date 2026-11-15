use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'Stacked bars',a:c,w:70,h:16");
            let mut chart = VStackedBarChart::<i32>::new(
                layout!("d:f"),
                vstackedbarchart::Flags::ScrollBars | vstackedbarchart::Flags::ShowZeroLineOnYAxis,
            );
            chart.set_default_bar_width(3);
            chart.set_default_bar_spacing(2);
            chart.set_xaxis_label_mode(vstackedbarchart::XAxisLabelMode::StackLabels);
            chart.add_stack(vstackedbarchart::BarsStackBuilder::new().label("Jan").bar(12).bar(5).bar(-3).build());
            chart.add_stack(vstackedbarchart::BarsStackBuilder::new().label("Feb").bar(25).bar(-8).build());
            chart.add_stack(vstackedbarchart::BarsStackBuilder::new().label("Mar").bar(8).bar(4).bar(6).build());
            chart.add_stack(vstackedbarchart::BarsStackBuilder::new().label("Apr").bar(-10).bar(-4).bar(30).build());
            chart.add_stack(vstackedbarchart::BarsStackBuilder::new().label("May").bar(18).bar(2).build());
            win.add(chart);
            win
        })
        .run()
}
