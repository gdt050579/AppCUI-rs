use super::super::Graph;
use super::super::GraphNode;
use crate::graphics::*;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

pub(in super::super) fn rearange<T: GraphNode>(graph: &mut Graph<T>, spacing: u32) {
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

    // Find connected components and their roots
    let components = find_connected_components_with_roots(graph);

    if components.is_empty() {
        return;
    }

    // Calculate spacing
    // Ensure at least 2 characters between children horizontally and 1 character between parent-child vertically
    let horizontal_spacing = max_node_size.width + spacing * 2; // At least 2 characters between children
    let vertical_spacing = (max_node_size.height + spacing) * 2; // At least 1 character between parent and children (doubled for aspect ratio correction)

    // Position each component's tree
    let mut current_x_offset = 0;
    for (root, nodes) in components {
        let tree_width = arrange_tree_component(
            graph,
            root,
            &nodes,
            current_x_offset,
            horizontal_spacing,
            vertical_spacing,
            &max_node_size,
        );
        current_x_offset += tree_width + (horizontal_spacing * 2) as i32;
    }
}

fn find_connected_components_with_roots<T: GraphNode>(graph: &Graph<T>) -> Vec<(usize, BTreeSet<usize>)> {
    let mut visited = vec![false; graph.nodes.len()];
    let mut components = Vec::new();

    for start_node in 0..graph.nodes.len() {
        if !visited[start_node] {
            let mut component = BTreeSet::new();
            let mut queue = VecDeque::new();

            queue.push_back(start_node);
            visited[start_node] = true;

            // Find all nodes in this component
            while let Some(node_idx) = queue.pop_front() {
                component.insert(node_idx);

                // Check all connected nodes (both directions for undirected graphs)
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
                let root = find_best_root(graph, &component);
                components.push((root, component));
            }
        }
    }

    components
}

fn find_best_root<T: GraphNode>(graph: &Graph<T>, component: &BTreeSet<usize>) -> usize {
    // First, check if this is a directed graph by looking for directed edges
    let has_directed_edges = component.iter().any(|&node_idx| {
        graph.nodes[node_idx]
            .edges_out
            .iter()
            .any(|&edge_idx| graph.edges[edge_idx as usize].directed)
    });

    if has_directed_edges {
        // For directed graphs: find nodes with no incoming edges within the component
        let mut candidates = Vec::new();
        for &node_idx in component {
            let has_incoming_in_component = graph.nodes[node_idx].edges_in.iter().any(|&edge_idx| {
                let edge = &graph.edges[edge_idx as usize];
                if edge.directed {
                    let from_node = edge.from_node_id as usize;
                    component.contains(&from_node) && from_node != node_idx
                } else {
                    false
                }
            });

            if !has_incoming_in_component {
                candidates.push(node_idx);
            }
        }

        if !candidates.is_empty() {
            // Return the first valid root candidate
            return candidates[0];
        }
    }

    // For undirected graphs or when no clear root is found:
    // Use the node with highest degree (most connections) as an approximation
    let mut best_node = *component.iter().next().unwrap();
    let mut best_degree = 0;

    for &node_idx in component {
        let degree = graph.nodes[node_idx].edges_in.len() + graph.nodes[node_idx].edges_out.len();
        if degree > best_degree {
            best_degree = degree;
            best_node = node_idx;
        }
    }

    // Alternative: use centrality-based approach for better root selection
    find_most_central_node(graph, component).unwrap_or(best_node)
}

fn find_most_central_node<T: GraphNode>(graph: &Graph<T>, component: &BTreeSet<usize>) -> Option<usize> {
    if component.len() <= 2 {
        return component.iter().next().copied();
    }

    // Calculate distance from each node to all other nodes in the component
    let mut best_node = None;
    let mut min_max_distance = usize::MAX;

    for &candidate in component {
        let distances = bfs_distances(graph, candidate, component);
        let max_distance = distances.values().max().copied().unwrap_or(0);

        if max_distance < min_max_distance {
            min_max_distance = max_distance;
            best_node = Some(candidate);
        }
    }

    best_node
}

fn bfs_distances<T: GraphNode>(graph: &Graph<T>, start: usize, component: &BTreeSet<usize>) -> HashMap<usize, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    distances.insert(start, 0);
    queue.push_back(start);

    while let Some(node_idx) = queue.pop_front() {
        let current_distance = distances[&node_idx];

        // Check all connected nodes in the component
        for &edge_idx in &graph.nodes[node_idx].edges_out {
            let edge = &graph.edges[edge_idx as usize];
            let connected_node = if edge.from_node_id as usize == node_idx {
                edge.to_node_id as usize
            } else {
                edge.from_node_id as usize
            };

            if component.contains(&connected_node) && !distances.contains_key(&connected_node) {
                distances.insert(connected_node, current_distance + 1);
                queue.push_back(connected_node);
            }
        }

        for &edge_idx in &graph.nodes[node_idx].edges_in {
            let edge = &graph.edges[edge_idx as usize];
            let connected_node = if edge.to_node_id as usize == node_idx {
                edge.from_node_id as usize
            } else {
                edge.to_node_id as usize
            };

            if component.contains(&connected_node) && !distances.contains_key(&connected_node) {
                distances.insert(connected_node, current_distance + 1);
                queue.push_back(connected_node);
            }
        }
    }

    distances
}

fn arrange_tree_component<T: GraphNode>(
    graph: &mut Graph<T>,
    root: usize,
    component: &BTreeSet<usize>,
    x_offset: i32,
    horizontal_spacing: u32,
    vertical_spacing: u32,
    _max_node_size: &Size,
) -> i32 {
    // Build the tree structure from the root
    let tree = build_tree_from_root(graph, root, component);

    // Calculate positions for each level
    let level_positions = calculate_tree_positions(&tree, horizontal_spacing, vertical_spacing);

    // Apply positions to nodes with aspect ratio correction
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;

    for (node_idx, (x, y)) in level_positions {
        let adjusted_x = x_offset + x;
        let adjusted_y = y / 2; // Apply 2:1 aspect ratio correction

        let node = &mut graph.nodes[node_idx];
        node.rect.set_left(adjusted_x - (node.rect.width() as i32 / 2), true);
        node.rect.set_top(adjusted_y - (node.rect.height() as i32 / 2), true);

        min_x = min_x.min(adjusted_x - (node.rect.width() as i32 / 2));
        max_x = max_x.max(adjusted_x + (node.rect.width() as i32 / 2));
    }

    // Return the width of this tree component
    if min_x == i32::MAX {
        horizontal_spacing as i32
    } else {
        max_x - min_x
    }
}

fn build_tree_from_root<T: GraphNode>(graph: &Graph<T>, root: usize, component: &BTreeSet<usize>) -> HashMap<usize, Vec<usize>> {
    let mut tree = HashMap::new();
    let mut visited = BTreeSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(root);
    visited.insert(root);
    tree.insert(root, Vec::new());

    while let Some(node_idx) = queue.pop_front() {
        let mut children = Vec::new();

        // Find children (nodes connected to this node that haven't been visited)
        for &edge_idx in &graph.nodes[node_idx].edges_out {
            let edge = &graph.edges[edge_idx as usize];
            let connected_node = if edge.from_node_id as usize == node_idx {
                edge.to_node_id as usize
            } else if !edge.directed {
                edge.from_node_id as usize
            } else {
                continue; // Skip if this is a directed edge going the wrong way
            };

            if component.contains(&connected_node) && !visited.contains(&connected_node) {
                visited.insert(connected_node);
                children.push(connected_node);
                queue.push_back(connected_node);
            }
        }

        // For undirected graphs, also check incoming edges
        for &edge_idx in &graph.nodes[node_idx].edges_in {
            let edge = &graph.edges[edge_idx as usize];
            if edge.directed {
                continue; // Skip directed edges in the wrong direction
            }

            let connected_node = if edge.to_node_id as usize == node_idx {
                edge.from_node_id as usize
            } else {
                edge.to_node_id as usize
            };

            if component.contains(&connected_node) && !visited.contains(&connected_node) {
                visited.insert(connected_node);
                children.push(connected_node);
                queue.push_back(connected_node);
            }
        }

        tree.insert(node_idx, children);
    }

    tree
}

fn calculate_tree_positions(tree: &HashMap<usize, Vec<usize>>, horizontal_spacing: u32, vertical_spacing: u32) -> HashMap<usize, (i32, i32)> {
    let mut positions = HashMap::new();
    let mut node_widths = HashMap::new();

    // Find the root (node that appears as key but not as child)
    let all_children: HashSet<usize> = tree.values().flatten().copied().collect();
    let root = tree.keys().find(|&&node| !all_children.contains(&node)).copied().unwrap_or(0);

    // Calculate subtree widths bottom-up
    calculate_subtree_widths(tree, root, &mut node_widths, horizontal_spacing);

    // Position nodes top-down
    position_tree_nodes(tree, root, 0, 0, &node_widths, &mut positions, horizontal_spacing, vertical_spacing);

    positions
}

fn calculate_subtree_widths(tree: &HashMap<usize, Vec<usize>>, node: usize, widths: &mut HashMap<usize, i32>, horizontal_spacing: u32) -> i32 {
    let empty_vec = Vec::new();
    let children = tree.get(&node).unwrap_or(&empty_vec);

    if children.is_empty() {
        // Leaf node
        let width = horizontal_spacing as i32;
        widths.insert(node, width);
        width
    } else {
        // Internal node - width is sum of children widths
        let total_width: i32 = children
            .iter()
            .map(|&child| calculate_subtree_widths(tree, child, widths, horizontal_spacing))
            .sum();
        widths.insert(node, total_width);
        total_width
    }
}

fn position_tree_nodes(
    tree: &HashMap<usize, Vec<usize>>,
    node: usize,
    x: i32,
    y: i32,
    widths: &HashMap<usize, i32>,
    positions: &mut HashMap<usize, (i32, i32)>,
    horizontal_spacing: u32,
    vertical_spacing: u32,
) {
    positions.insert(node, (x, y));

    let empty_vec = Vec::new();
    let children = tree.get(&node).unwrap_or(&empty_vec);
    if children.is_empty() {
        return;
    }

    // Calculate starting position for children
    let total_width = widths.get(&node).unwrap_or(&0);
    let mut current_x = x - total_width / 2;
    let child_y = y + vertical_spacing as i32;

    for &child in children {
        let default_width = horizontal_spacing as i32;
        let child_width = widths.get(&child).unwrap_or(&default_width);
        let child_center_x = current_x + child_width / 2;

        position_tree_nodes(
            tree,
            child,
            child_center_x,
            child_y,
            widths,
            positions,
            horizontal_spacing,
            vertical_spacing,
        );

        current_x += child_width;
    }
}
