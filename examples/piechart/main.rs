use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'Pie chart',a:c,w:60,h:20,flags:Sizeable");
            let mut chart = PieChart::new(layout!("d:f"), piechart::Flags::None);
            chart.set_renderer(piechart::DrawModeRenderer::HalfBlocks);
            chart.set_shape(piechart::DrawModeShape::Ellipse);
            chart.set_label_content(piechart::LabelContent::Value);
            chart.set_label_layout(piechart::LabelLayout::Outside);
            chart.set_hollow_thickness(piechart::HollowThickness::Percentage(50));
            chart.add_value(45, "Rust", None);
            chart.add_value(25, "C++", None);
            chart.add_value(15, "Go", None);
            chart.add_value(10, "Zig", None);
            chart.add_value(5, "Other", Some(CharAttribute::with_fore_color(Color::Gray)));
            win.add(chart);
            win
        })
        .run()
}
