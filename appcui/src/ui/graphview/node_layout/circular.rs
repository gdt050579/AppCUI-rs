use super::super::Graph;
use super::super::GraphNode;
use crate::graphics::*;
use std::collections::VecDeque;

pub(in super::super) fn rearange<T: GraphNode>(graph: &mut Graph<T>) {
    if graph.nodes.is_empty() {
        return;
    }

    // Find the maximum node size for spacing calculations
    let mut max_node_size = Size::new(1, 1);
    for node in &graph.nodes {
        let sz = node.rect.size();
        max_node_size.width = max_node_size.width.max(sz.width);
        max_node_size.height = max_node_size.height.max(sz.height);
    }

    // Find connected components using DFS
    let components = find_connected_components(graph);
    
    if components.is_empty() {
        return;
    }

    // Calculate the center and radius for the main circle
    // Account for 2:1 character aspect ratio (height = 2 * width)
    let num_components = components.len();
    let component_spacing = (max_node_size.width + max_node_size.height) as f64 * 2.0;
    let main_radius = if num_components == 1 {
        component_spacing
    } else {
        // Calculate radius based on component spacing and circumference
        (num_components as f64 * component_spacing) / (2.0 * std::f64::consts::PI)
    };
    
    // Center coordinates - use radius for both, but apply aspect ratio during positioning
    let center_x = main_radius as i32;
    let center_y = (main_radius / 2.0) as i32;  // Half radius for Y due to 2:1 aspect ratio

    // Arrange each component around the main circle
    for (i, component) in components.iter().enumerate() {
        if component.len() == 1 {
            // Single node - place it directly on the circle
            let angle = if num_components == 1 {
                0.0
            } else {
                2.0 * std::f64::consts::PI * i as f64 / num_components as f64
            };
            let x = center_x + (main_radius * angle.cos()) as i32;
            let y = center_y + (main_radius * angle.sin() / 2.0) as i32;  // Divide Y by 2 for aspect ratio
            
            let node = &mut graph.nodes[component[0]];
            node.rect.set_left(x - (node.rect.width() as i32 / 2), true);
            node.rect.set_top(y - (node.rect.height() as i32 / 2), true);
        } else {
            // Multiple nodes - arrange them in a smaller circle or line
            arrange_component_nodes(graph, component, i, num_components, center_x, center_y, main_radius, &max_node_size);
        }
    }
}

fn find_connected_components<T: GraphNode>(graph: &Graph<T>) -> Vec<Vec<usize>> {
    let mut visited = vec![false; graph.nodes.len()];
    let mut components = Vec::new();

    for start_node in 0..graph.nodes.len() {
        if !visited[start_node] {
            let mut component = Vec::new();
            let mut queue = VecDeque::new();
            
            queue.push_back(start_node);
            visited[start_node] = true;

            while let Some(node_idx) = queue.pop_front() {
                component.push(node_idx);

                // Check all edges from this node
                for &edge_idx in &graph.nodes[node_idx].edges_out {
                    let edge = &graph.edges[edge_idx as usize];
                    let connected_node = if edge.from_node_id as usize == node_idx {
                        edge.to_node_id as usize
                    } else {
                        edge.from_node_id as usize
                    };

                    if !visited[connected_node] {
                        visited[connected_node] = true;
                        queue.push_back(connected_node);
                    }
                }

                // Check all edges to this node (for undirected graphs)
                for &edge_idx in &graph.nodes[node_idx].edges_in {
                    let edge = &graph.edges[edge_idx as usize];
                    let connected_node = if edge.to_node_id as usize == node_idx {
                        edge.from_node_id as usize
                    } else {
                        edge.to_node_id as usize
                    };

                    if !visited[connected_node] {
                        visited[connected_node] = true;
                        queue.push_back(connected_node);
                    }
                }
            }

            if !component.is_empty() {
                components.push(component);
            }
        }
    }

    components
}

fn arrange_component_nodes<T: GraphNode>(
    graph: &mut Graph<T>,
    component: &[usize],
    component_idx: usize,
    total_components: usize,
    center_x: i32,
    center_y: i32,
    main_radius: f64,
    max_node_size: &Size,
) {
    let component_size = component.len();
    
    // Calculate the position for this component on the main circle
    let main_angle = if total_components == 1 {
        0.0
    } else {
        2.0 * std::f64::consts::PI * component_idx as f64 / total_components as f64
    };
    
    let component_center_x = center_x + (main_radius * main_angle.cos()) as i32;
    let component_center_y = center_y + (main_radius * main_angle.sin() / 2.0) as i32;  // Aspect ratio correction

    if component_size <= 3 {
        // For small components, arrange in a line perpendicular to the radius
        let perpendicular_angle = main_angle + std::f64::consts::PI / 2.0;
        
        for (i, &node_idx) in component.iter().enumerate() {
            let offset = (i as f64 - (component_size - 1) as f64 / 2.0) * (max_node_size.width as f64 + 2.0);
            let x = component_center_x + (offset * perpendicular_angle.cos()) as i32;
            let y = component_center_y + (offset * perpendicular_angle.sin() / 2.0) as i32;  // Aspect ratio correction
            
            let node = &mut graph.nodes[node_idx];
            node.rect.set_left(x - (node.rect.width() as i32 / 2), true);
            node.rect.set_top(y - (node.rect.height() as i32 / 2), true);
        }
    } else {
        // For larger components, arrange in a small circle
        let component_radius = (component_size as f64 * (max_node_size.width as f64 + 2.0)) / (2.0 * std::f64::consts::PI);
        let component_radius = component_radius.max(max_node_size.width as f64);
        
        for (i, &node_idx) in component.iter().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / component_size as f64;
            let x = component_center_x + (component_radius * angle.cos()) as i32;
            let y = component_center_y + (component_radius * angle.sin() / 2.0) as i32;  // Aspect ratio correction
            
            let node = &mut graph.nodes[node_idx];
            node.rect.set_left(x - (node.rect.width() as i32 / 2), true);
            node.rect.set_top(y - (node.rect.height() as i32 / 2), true);
        }
    }
}