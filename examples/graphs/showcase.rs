use crate::settings::Settings;
use appcui::prelude::*;

pub fn create() -> (graphview::Graph<String>, Settings) {
    let names = vec![
        "🌟 Root\nMaster Node",
        "⚡ Lightning\nFast Processor",
        "🔥 Fire\nHot Data",
        "💎 Diamond\nPrecious Info",
        "🌊 Ocean\nDeep Learning",
        "🌸 Blossom\nBeautiful UI",
        "🚀 Rocket\nHigh Performance",
        "🎯 Target\nPrecision Mode",
        "🔮 Crystal\nMystic Powers",
        "⭐ Star\nShining Bright",
        "🌙 Moon\nNight Mode",
        "☀️ Sun\nDay Mode",
        "🦋 Butterfly\nTransformation",
        "🌈 Rainbow\nColorful Data",
        "🎨 Palette\nArt Generator",
    ];

    // Create nodes with different colors and alignments
    let nodes = vec![
        graphview::NodeBuilder::new(names[0].to_string())
            .text_attribute(CharAttribute::with_color(Color::Yellow, Color::Black))
            .text_alignment(TextAlignment::Center)
            .build(),
        graphview::NodeBuilder::new(names[1].to_string())
            .text_attribute(CharAttribute::with_color(Color::Olive, Color::Black))
            .text_alignment(TextAlignment::Left)
            .build(),
        graphview::NodeBuilder::new(names[2].to_string())
            .text_attribute(CharAttribute::with_color(Color::Red, Color::Black))
            .text_alignment(TextAlignment::Right)
            .build(),
        graphview::NodeBuilder::new(names[3].to_string())
            .text_attribute(CharAttribute::with_color(Color::Magenta, Color::Black))
            .text_alignment(TextAlignment::Center)
            .build(),
        graphview::NodeBuilder::new(names[4].to_string())
            .text_attribute(CharAttribute::with_color(Color::Blue, Color::Black))
            .text_alignment(TextAlignment::Left)
            .build(),
        graphview::NodeBuilder::new(names[5].to_string())
            .text_attribute(CharAttribute::with_color(Color::Pink, Color::Black))
            .text_alignment(TextAlignment::Right)
            .build(),
        graphview::NodeBuilder::new(names[6].to_string())
            .text_attribute(CharAttribute::with_color(Color::White, Color::Black))
            .text_alignment(TextAlignment::Center)
            .build(),
        graphview::NodeBuilder::new(names[7].to_string())
            .text_attribute(CharAttribute::with_color(Color::Green, Color::Black))
            .text_alignment(TextAlignment::Left)
            .build(),
        graphview::NodeBuilder::new(names[8].to_string())
            .text_attribute(CharAttribute::with_color(Color::Blue, Color::Black))
            .text_alignment(TextAlignment::Right)
            .build(),
        graphview::NodeBuilder::new(names[9].to_string())
            .text_attribute(CharAttribute::with_color(Color::Yellow, Color::Black))
            .text_alignment(TextAlignment::Center)
            .build(),
        graphview::NodeBuilder::new(names[10].to_string())
            .text_attribute(CharAttribute::with_color(Color::Silver, Color::Black))
            .text_alignment(TextAlignment::Left)
            .build(),
        graphview::NodeBuilder::new(names[11].to_string())
            .text_attribute(CharAttribute::with_color(Color::Olive, Color::Black))
            .text_alignment(TextAlignment::Right)
            .build(),
        graphview::NodeBuilder::new(names[12].to_string())
            .text_attribute(CharAttribute::with_color(Color::Magenta, Color::Black))
            .text_alignment(TextAlignment::Center)
            .build(),
        graphview::NodeBuilder::new(names[13].to_string())
            .text_attribute(CharAttribute::with_color(Color::Green, Color::Black))
            .text_alignment(TextAlignment::Left)
            .build(),
        graphview::NodeBuilder::new(names[14].to_string())
            .text_attribute(CharAttribute::with_color(Color::Red, Color::Black))
            .text_alignment(TextAlignment::Right)
            .build(),
    ];

    // Create edges with different line types and colors
    let edges = vec![
        graphview::EdgeBuilder::new(0, 1)
            .directed(true)
            .line_type(LineType::SingleThick)
            .attribute(CharAttribute::with_color(Color::Yellow, Color::Transparent))
            .build(),
        graphview::EdgeBuilder::new(0, 2)
            .directed(true)
            .line_type(LineType::Double)
            .attribute(CharAttribute::with_color(Color::Red, Color::Transparent))
            .build(),
        graphview::EdgeBuilder::new(0, 3)
            .directed(true)
            .line_type(LineType::Single)
            .attribute(CharAttribute::with_color(Color::Magenta, Color::Transparent))
            .build(),
        graphview::EdgeBuilder::new(1, 4)
            .directed(false)
            .line_type(LineType::Ascii)
            .attribute(CharAttribute::with_color(Color::Olive, Color::Transparent))
            .build(),
        graphview::EdgeBuilder::new(1, 5)
            .directed(false)
            .line_type(LineType::AsciiRound)
            .attribute(CharAttribute::with_color(Color::Pink, Color::Transparent))
            .build(),
        graphview::EdgeBuilder::new(2, 6)
            .directed(true)
            .line_type(LineType::SingleRound)
            .attribute(CharAttribute::with_color(Color::White, Color::Transparent))
            .build(),
        graphview::EdgeBuilder::new(2, 7)
            .directed(true)
            .line_type(LineType::Double)
            .attribute(CharAttribute::with_color(Color::Green, Color::Transparent))
            .build(),
        graphview::EdgeBuilder::new(3, 8)
            .directed(false)
            .line_type(LineType::Double)
            .attribute(CharAttribute::with_color(Color::Blue, Color::Transparent))
            .build(),
        graphview::EdgeBuilder::new(3, 9)
            .directed(false)
            .line_type(LineType::Single)
            .attribute(CharAttribute::with_color(Color::Yellow, Color::Transparent))
            .build(),
        graphview::EdgeBuilder::new(4, 10)
            .directed(true)
            .line_type(LineType::Double)
            .attribute(CharAttribute::with_color(Color::Silver, Color::Transparent))
            .build(),
        graphview::EdgeBuilder::new(5, 11)
            .directed(true)
            .line_type(LineType::SingleThick)
            .attribute(CharAttribute::with_color(Color::Olive, Color::Transparent))
            .build(),
        graphview::EdgeBuilder::new(6, 12)
            .directed(false)
            .line_type(LineType::Ascii)
            .attribute(CharAttribute::with_color(Color::Magenta, Color::Transparent))
            .build(),
        graphview::EdgeBuilder::new(7, 13)
            .directed(false)
            .line_type(LineType::AsciiRound)
            .attribute(CharAttribute::with_color(Color::Green, Color::Transparent))
            .build(),
        graphview::EdgeBuilder::new(8, 14)
            .directed(true)
            .line_type(LineType::SingleRound)
            .attribute(CharAttribute::with_color(Color::Red, Color::Transparent))
            .build(),
        graphview::EdgeBuilder::new(9, 12)
            .directed(false)
            .line_type(LineType::Single)
            .attribute(CharAttribute::with_color(Color::Gray, Color::Transparent))
            .build(),
        graphview::EdgeBuilder::new(10, 13)
            .directed(false)
            .line_type(LineType::Single)
            .attribute(CharAttribute::with_color(Color::Gray, Color::Transparent))
            .build(),
        graphview::EdgeBuilder::new(11, 14)
            .directed(false)
            .line_type(LineType::Single)
            .attribute(CharAttribute::with_color(Color::Olive, Color::Transparent))
            .build(),
    ];

    let graph = graphview::Graph::new(nodes, edges);

    let settings = Settings::new("🎨 Showcase Graph - Advanced Features")
        .with_arrange_method(graphview::ArrangeMethod::Hierarchical)
        .with_arrow_heads(true)
        .with_edge_highlighting(true, true)
        .with_edge_line_type(LineType::Single) 
        .with_edge_routing(graphview::EdgeRouting::Orthogonal);

    (graph, settings)
}
