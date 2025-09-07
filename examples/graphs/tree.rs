use appcui::prelude::*;
use crate::settings::Settings;

pub fn create() -> (graphview::Graph<String>, Settings) {
    // Create a tree structure representing a file system hierarchy
    let nodes = vec![
        "root".to_string(),
        "bin".to_string(),
        "usr".to_string(),
        "home".to_string(),
        "var".to_string(),
        "etc".to_string(),
        "ls".to_string(),
        "cat".to_string(),
        "grep".to_string(),
        "local".to_string(),
        "share".to_string(),
        "lib".to_string(),
        "alice".to_string(),
        "bob".to_string(),
        "charlie".to_string(),
        "log".to_string(),
        "tmp".to_string(),
        "www".to_string(),
        "passwd".to_string(),
        "hosts".to_string(),
        "fstab".to_string(),
        "documents".to_string(),
        "downloads".to_string(),
        "music".to_string(),
        "pictures".to_string(),
    ];

    // Define edges to create a tree structure
    let edges = vec![
        // root level
        (0, 1),  // root -> bin
        (0, 2),  // root -> usr
        (0, 3),  // root -> home
        (0, 4),  // root -> var
        (0, 5),  // root -> etc
        
        // bin directory
        (1, 6),  // bin -> ls
        (1, 7),  // bin -> cat
        (1, 8),  // bin -> grep
        
        // usr directory
        (2, 9),  // usr -> local
        (2, 10), // usr -> share
        (2, 11), // usr -> lib
        
        // home directory
        (3, 12), // home -> alice
        (3, 13), // home -> bob
        (3, 14), // home -> charlie
        
        // var directory
        (4, 15), // var -> log
        (4, 16), // var -> tmp
        (4, 17), // var -> www
        
        // etc directory
        (5, 18), // etc -> passwd
        (5, 19), // etc -> hosts
        (5, 20), // etc -> fstab
        
        // user directories
        (12, 21), // alice -> documents
        (12, 22), // alice -> downloads
        (13, 23), // bob -> music
        (14, 24), // charlie -> pictures
    ];

    // Create the graph with directed edges (typical for tree structures)
    let graph = graphview::Graph::with_slices(&nodes, &edges, true);
    
    // Configure settings for tree visualization
    let settings = Settings::new("Tree Graph - File System")
        .with_arrange_method(graphview::ArrangeMethod::Hierarchical)
        .with_arrow_heads(true)
        .with_edge_highlighting(false, false)
        .with_edge_line_type(LineType::Single)
        .with_edge_routing(graphview::EdgeRouting::Direct);
    
    (graph, settings)
}