use appcui::prelude::*;
use crate::settings::Settings;

pub fn create() -> (graphview::Graph<String>, Settings) {
    // Create a social network graph with circular connections
    let nodes = vec![
        "Alice".to_string(),
        "Bob".to_string(),
        "Charlie".to_string(),
        "Diana".to_string(),
        "Eve".to_string(),
        "Frank".to_string(),
        "Grace".to_string(),
        "Henry".to_string(),
        "Ivy".to_string(),
        "Jack".to_string(),
    ];

    // Define edges to create a circular network with some cross-connections
    let edges = vec![
        // Main circular connections
        (0, 1),  // Alice -> Bob
        (1, 2),  // Bob -> Charlie
        (2, 3),  // Charlie -> Diana
        (3, 4),  // Diana -> Eve
        (4, 5),  // Eve -> Frank
        (5, 6),  // Frank -> Grace
        (6, 7),  // Grace -> Henry
        (7, 8),  // Henry -> Ivy
        (8, 9),  // Ivy -> Jack
        (9, 0),  // Jack -> Alice (completes the circle)
        
        // Additional cross-connections for a more interesting graph
        (0, 3),  // Alice -> Diana
        (1, 4),  // Bob -> Eve
        (2, 5),  // Charlie -> Frank
        (3, 6),  // Diana -> Grace
        (4, 7),  // Eve -> Henry
        (5, 8),  // Frank -> Ivy
        (6, 9),  // Grace -> Jack
        
        // Some bidirectional friendships
        (0, 5),  // Alice <-> Frank
        (5, 0),  
        (2, 7),  // Charlie <-> Henry
        (7, 2),
        (4, 9),  // Eve <-> Jack
        (9, 4),
    ];

    // Create the graph with undirected edges for social connections
    let graph = graphview::Graph::with_slices_and_border(&nodes, &edges, LineType::Single, false);
    
    // Configure settings for circular visualization
    let settings = Settings::new("Circular Graph - Social Network")
        .with_arrange_method(graphview::ArrangeMethod::Circular)
        .with_arrow_heads(false) // No arrows for undirected graph
        .with_edge_highlighting(true, true) // Highlight connections
        .with_edge_line_type(LineType::Single)
        .with_edge_routing(graphview::EdgeRouting::Direct);
    
    (graph, settings)
}