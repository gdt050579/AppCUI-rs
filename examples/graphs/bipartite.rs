use appcui::prelude::*;
use crate::settings::Settings;

pub fn create() -> (graphview::Graph<String>, Settings) {
    // Create a bipartite graph representing students and courses
    let nodes = vec![
        // Students (first partition)
        "Alice".to_string(),
        "Bob".to_string(),
        "Charlie".to_string(),
        "Diana".to_string(),
        "Eve".to_string(),
        "Frank".to_string(),
        
        // Courses (second partition)
        "Math 101".to_string(),
        "Physics 201".to_string(),
        "Chemistry 150".to_string(),
        "Biology 120".to_string(),
        "Computer Science 202".to_string(),
        "Statistics 180".to_string(),
        "Literature 110".to_string(),
        "History 140".to_string(),
    ];

    // Define edges connecting students to courses they're enrolled in
    // In a bipartite graph, edges only exist between the two partitions
    let edges = vec![
        // Alice's courses
        (0, 6),  // Alice -> Math 101
        (0, 7),  // Alice -> Physics 201
        (0, 10), // Alice -> Computer Science 202
        
        // Bob's courses
        (1, 6),  // Bob -> Math 101
        (1, 8),  // Bob -> Chemistry 150
        (1, 11), // Bob -> Statistics 180
        (1, 12), // Bob -> Literature 110
        
        // Charlie's courses
        (2, 7),  // Charlie -> Physics 201
        (2, 8),  // Charlie -> Chemistry 150
        (2, 9),  // Charlie -> Biology 120
        (2, 10), // Charlie -> Computer Science 202
        
        // Diana's courses
        (3, 6),  // Diana -> Math 101
        (3, 9),  // Diana -> Biology 120
        (3, 11), // Diana -> Statistics 180
        (3, 13), // Diana -> History 140
        
        // Eve's courses
        (4, 7),  // Eve -> Physics 201
        (4, 10), // Eve -> Computer Science 202
        (4, 12), // Eve -> Literature 110
        (4, 13), // Eve -> History 140
        
        // Frank's courses
        (5, 8),  // Frank -> Chemistry 150
        (5, 9),  // Frank -> Biology 120
        (5, 11), // Frank -> Statistics 180
        (5, 12), // Frank -> Literature 110
    ];

    // Create the graph with undirected edges (enrollment is bidirectional)
    let graph = graphview::Graph::with_slices(&nodes, &edges, false);
    
    // Configure settings for bipartite visualization
    let settings = Settings::new("Bipartite Graph - Students & Courses")
        .with_arrange_method(graphview::ArrangeMethod::HierarchicalPacked)
        .with_arrow_heads(false) // No arrows for undirected graph
        .with_edge_highlighting(true, true) // Highlight connections
        .with_edge_line_type(LineType::Double) // Use double lines for emphasis
        .with_edge_routing(graphview::EdgeRouting::Direct);
    
    (graph, settings)
}