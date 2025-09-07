use crate::settings::Settings;
use appcui::prelude::*;

pub fn create() -> (graphview::Graph<String>, Settings) {
    // Create nodes with fancy Unicode characters and different content lengths
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
    let mut nodes = Vec::new();

    // Root node - centered, bright yellow
    nodes.push(
        graphview::NodeBuilder::new(names[0].to_string())
            .text_attribute(CharAttribute::with_color(Color::Yellow, Color::Black))
            .text_alignment(TextAlignment::Center)
            .build(),
    );

    // Lightning - left aligned, bright blue
    nodes.push(
        graphview::NodeBuilder::new(names[1].to_string())
            .text_attribute(CharAttribute::with_color(Color::Olive, Color::Black))
            .text_alignment(TextAlignment::Left)
            .build(),
    );

    // Fire - right aligned, bright red
    nodes.push(
        graphview::NodeBuilder::new(names[2].to_string())
            .text_attribute(CharAttribute::with_color(Color::Red, Color::Black))
            .text_alignment(TextAlignment::Right)
            .build(),
    );

    // Diamond - centered, bright magenta
    nodes.push(
        graphview::NodeBuilder::new(names[3].to_string())
            .text_attribute(CharAttribute::with_color(Color::Magenta, Color::Black))
            .text_alignment(TextAlignment::Center)
            .build(),
    );

    // Ocean - left aligned, blue
    nodes.push(
        graphview::NodeBuilder::new(names[4].to_string())
            .text_attribute(CharAttribute::with_color(Color::Blue, Color::Black))
            .text_alignment(TextAlignment::Left)
            .build(),
    );

    // Blossom - right aligned, pink
    nodes.push(
        graphview::NodeBuilder::new(names[5].to_string())
            .text_attribute(CharAttribute::with_color(Color::Pink, Color::Black))
            .text_alignment(TextAlignment::Right)
            .build(),
    );

    // Rocket - centered, white
    nodes.push(
        graphview::NodeBuilder::new(names[6].to_string())
            .text_attribute(CharAttribute::with_color(Color::White, Color::Black))
            .text_alignment(TextAlignment::Center)
            .build(),
    );

    // Target - left aligned, green
    nodes.push(
        graphview::NodeBuilder::new(names[7].to_string())
            .text_attribute(CharAttribute::with_color(Color::Green, Color::Black))
            .text_alignment(TextAlignment::Left)
            .build(),
    );

    // Crystal - right aligned, light blue
    nodes.push(
        graphview::NodeBuilder::new(names[8].to_string())
            .text_attribute(CharAttribute::with_color(Color::Blue, Color::Black))
            .text_alignment(TextAlignment::Right)
            .build(),
    );

    // Star - centered, bright yellow
    nodes.push(
        graphview::NodeBuilder::new(names[9].to_string())
            .text_attribute(CharAttribute::with_color(Color::Yellow, Color::Black))
            .text_alignment(TextAlignment::Center)
            .build(),
    );

    // Moon - left aligned, light gray
    nodes.push(
        graphview::NodeBuilder::new(names[10].to_string())
            .text_attribute(CharAttribute::with_color(Color::Silver, Color::Black))
            .text_alignment(TextAlignment::Left)
            .build(),
    );

    // Sun - right aligned, orange
    nodes.push(
        graphview::NodeBuilder::new(names[11].to_string())
            .text_attribute(CharAttribute::with_color(Color::Olive, Color::Black))
            .text_alignment(TextAlignment::Right)
            .build(),
    );

    // Butterfly - centered, purple
    nodes.push(
        graphview::NodeBuilder::new(names[12].to_string())
            .text_attribute(CharAttribute::with_color(Color::Magenta, Color::Black))
            .text_alignment(TextAlignment::Center)
            .build(),
    );

    // Rainbow - left aligned, bright green
    nodes.push(
        graphview::NodeBuilder::new(names[13].to_string())
            .text_attribute(CharAttribute::with_color(Color::Green, Color::Black))
            .text_alignment(TextAlignment::Left)
            .build(),
    );

    // Palette - right aligned, bright red
    nodes.push(
        graphview::NodeBuilder::new(names[14].to_string())
            .text_attribute(CharAttribute::with_color(Color::Red, Color::Black))
            .text_alignment(TextAlignment::Right)
            .build(),
    );

    // Create edges with different line types and colors
    let mut edges = Vec::new();

    // Root connections - thick lines
    edges.push(
        graphview::EdgeBuilder::new(0, 1)
            .directed(true)
            .line_type(LineType::SingleThick)
            .attribute(CharAttribute::with_color(Color::Yellow, Color::Transparent))
            .build(),
    );

    edges.push(
        graphview::EdgeBuilder::new(0, 2)
            .directed(true)
            .line_type(LineType::Double)
            .attribute(CharAttribute::with_color(Color::Red, Color::Transparent))
            .build(),
    );

    edges.push(
        graphview::EdgeBuilder::new(0, 3)
            .directed(true)
            .line_type(LineType::Single)
            .attribute(CharAttribute::with_color(Color::Magenta, Color::Transparent))
            .build(),
    );

    // Secondary connections - various line types
    edges.push(
        graphview::EdgeBuilder::new(1, 4)
            .directed(false)
            .line_type(LineType::Ascii)
            .attribute(CharAttribute::with_color(Color::Olive, Color::Transparent))
            .build(),
    );

    edges.push(
        graphview::EdgeBuilder::new(1, 5)
            .directed(false)
            .line_type(LineType::AsciiRound)
            .attribute(CharAttribute::with_color(Color::Pink, Color::Transparent))
            .build(),
    );

    edges.push(
        graphview::EdgeBuilder::new(2, 6)
            .directed(true)
            .line_type(LineType::SingleRound)
            .attribute(CharAttribute::with_color(Color::White, Color::Transparent))
            .build(),
    );

    edges.push(
        graphview::EdgeBuilder::new(2, 7)
            .directed(true)
            .line_type(LineType::Double)
            .attribute(CharAttribute::with_color(Color::Green, Color::Transparent))
            .build(),
    );

    edges.push(
        graphview::EdgeBuilder::new(3, 8)
            .directed(false)
            .line_type(LineType::Double)
            .attribute(CharAttribute::with_color(Color::Blue, Color::Transparent))
            .build(),
    );

    edges.push(
        graphview::EdgeBuilder::new(3, 9)
            .directed(false)
            .line_type(LineType::Single)
            .attribute(CharAttribute::with_color(Color::Yellow, Color::Transparent))
            .build(),
    );

    // Tertiary connections - creating a complex network
    edges.push(
        graphview::EdgeBuilder::new(4, 10)
            .directed(true)
            .line_type(LineType::Double)
            .attribute(CharAttribute::with_color(Color::Silver, Color::Transparent))
            .build(),
    );

    edges.push(
        graphview::EdgeBuilder::new(5, 11)
            .directed(true)
            .line_type(LineType::SingleThick)
            .attribute(CharAttribute::with_color(Color::Olive, Color::Transparent))
            .build(),
    );

    edges.push(
        graphview::EdgeBuilder::new(6, 12)
            .directed(false)
            .line_type(LineType::Ascii)
            .attribute(CharAttribute::with_color(Color::Magenta, Color::Transparent))
            .build(),
    );

    edges.push(
        graphview::EdgeBuilder::new(7, 13)
            .directed(false)
            .line_type(LineType::AsciiRound)
            .attribute(CharAttribute::with_color(Color::Green, Color::Transparent))
            .build(),
    );

    edges.push(
        graphview::EdgeBuilder::new(8, 14)
            .directed(true)
            .line_type(LineType::SingleRound)
            .attribute(CharAttribute::with_color(Color::Red, Color::Transparent))
            .build(),
    );

    // Cross connections for visual interest
    edges.push(
        graphview::EdgeBuilder::new(9, 12)
            .directed(false)
            .line_type(LineType::Single)
            .attribute(CharAttribute::with_color(Color::Gray, Color::Transparent))
            .build(),
    );

    edges.push(
        graphview::EdgeBuilder::new(10, 13)
            .directed(false)
            .line_type(LineType::Single)
            .attribute(CharAttribute::with_color(Color::Gray, Color::Transparent))
            .build(),
    );

    edges.push(
        graphview::EdgeBuilder::new(11, 14)
            .directed(false)
            .line_type(LineType::Single)
            .attribute(CharAttribute::with_color(Color::Olive, Color::Transparent))
            .build(),
    );

    // Build the graph
    let graph = graphview::Graph::new(nodes, edges);

    // Configure settings for showcase visualization
    let settings = Settings::new("🎨 Showcase Graph - Advanced Features")
        .with_arrange_method(graphview::ArrangeMethod::Hierarchical)
        .with_arrow_heads(true)
        .with_edge_highlighting(true, true)
        .with_edge_line_type(LineType::Single) // Will be overridden by individual edge settings
        .with_edge_routing(graphview::EdgeRouting::Orthogonal);

    (graph, settings)
}
