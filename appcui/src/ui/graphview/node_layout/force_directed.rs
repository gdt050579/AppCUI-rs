use super::super::Graph;
use super::super::GraphNode;
use crate::graphics::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalized(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self { x: self.x / len, y: self.y / len }
        } else {
            Self::zero()
        }
    }
}

impl std::ops::Add for Vector2D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

impl std::ops::AddAssign for Vector2D {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Sub for Vector2D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }
}

impl std::ops::Mul<f64> for Vector2D {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self { x: self.x * scalar, y: self.y * scalar }
    }
}

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

    // Initialize positions randomly in a bounded area
    let node_count = graph.nodes.len();
    let area_size = ((node_count as f64).sqrt() * (max_node_size.width as f64 + max_node_size.height as f64) * 3.0) as i32;
    
    let mut positions = Vec::with_capacity(node_count);
    let mut velocities = Vec::with_capacity(node_count);
    
    // Initialize with random positions around the center
    for i in 0..node_count {
        let angle = 2.0 * std::f64::consts::PI * i as f64 / node_count as f64;
        let radius = area_size as f64 * 0.3;
        positions.push(Vector2D::new(
            radius * angle.cos(),
            radius * angle.sin(),
        ));
        velocities.push(Vector2D::zero());
    }

    // Run force-directed simulation
    run_force_simulation(graph, &mut positions, &mut velocities, area_size, &max_node_size);

    // Apply final positions to nodes with aspect ratio correction
    apply_positions_to_nodes(graph, &positions, &max_node_size);
}

fn run_force_simulation<T: GraphNode>(
    graph: &Graph<T>,
    positions: &mut Vec<Vector2D>,
    velocities: &mut Vec<Vector2D>,
    area_size: i32,
    max_node_size: &Size,
) {
    let node_count = positions.len();
    if node_count <= 1 {
        return;
    }

    // Simulation parameters
    let iterations = 50.min(node_count * 2); // Adaptive iteration count
    let area = (area_size as f64).powi(2);
    let k = (area / node_count as f64).sqrt(); // Optimal distance between nodes
    
    // Force constants
    let repulsion_strength = k * k;
    let attraction_strength = k;
    let damping = 0.9; // Velocity damping
    let max_displacement = area_size as f64 * 0.1; // Maximum movement per iteration
    
    for iteration in 0..iterations {
        let mut forces = vec![Vector2D::zero(); node_count];
        
        // Calculate cooling factor (reduces over time for stability)
        let cooling = 1.0 - (iteration as f64 / iterations as f64);
        let temperature = max_displacement * cooling;
        
        // Calculate repulsive forces between all node pairs
        for i in 0..node_count {
            for j in (i + 1)..node_count {
                let delta = positions[i] - positions[j];
                let distance = delta.length().max(0.1); // Avoid division by zero
                
                // Repulsive force (Fruchterman-Reingold style)
                let repulsion_force = repulsion_strength / distance;
                let force_direction = delta.normalized();
                let force = force_direction * repulsion_force;
                
                forces[i] += force;
                forces[j] += force * -1.0;
            }
        }
        
        // Calculate attractive forces for connected nodes (springs)
        for edge in &graph.edges {
            let from_idx = edge.from_node_id as usize;
            let to_idx = edge.to_node_id as usize;
            
            if from_idx < node_count && to_idx < node_count {
                let delta = positions[to_idx] - positions[from_idx];
                let distance = delta.length().max(0.1);
                
                // Attractive force (spring force)
                let attraction_force = (distance * distance) / attraction_strength;
                let force_direction = delta.normalized();
                let force = force_direction * attraction_force;
                
                forces[from_idx] += force;
                forces[to_idx] += force * -1.0;
            }
        }
        
        // Update velocities and positions
        for i in 0..node_count {
            // Update velocity with force and damping
            velocities[i] = (velocities[i] + forces[i]) * damping;
            
            // Limit displacement by temperature
            let displacement = velocities[i] * temperature;
            let displacement_length = displacement.length();
            
            if displacement_length > temperature {
                velocities[i] = displacement.normalized() * temperature;
            }
            
            // Update position
            positions[i] += velocities[i];
            
            // Keep nodes within bounds (soft boundary)
            let boundary = area_size as f64 * 0.4;
            if positions[i].x.abs() > boundary {
                positions[i].x = positions[i].x.signum() * boundary;
                velocities[i].x *= -0.5;
            }
            if positions[i].y.abs() > boundary {
                positions[i].y = positions[i].y.signum() * boundary;
                velocities[i].y *= -0.5;
            }
        }
    }
}

fn apply_positions_to_nodes<T: GraphNode>(
    graph: &mut Graph<T>,
    positions: &[Vector2D],
    max_node_size: &Size,
) {
    if positions.is_empty() {
        return;
    }

    // Find the bounding box of all positions
    let mut min_x = positions[0].x;
    let mut max_x = positions[0].x;
    let mut min_y = positions[0].y;
    let mut max_y = positions[0].y;
    
    for pos in positions {
        min_x = min_x.min(pos.x);
        max_x = max_x.max(pos.x);
        min_y = min_y.min(pos.y);
        max_y = max_y.max(pos.y);
    }
    
    // Calculate offset to center the layout and ensure positive coordinates
    let width = max_x - min_x;
    let height = max_y - min_y;
    let padding = (max_node_size.width.max(max_node_size.height) as f64) * 2.0;
    
    let offset_x = -min_x + padding;
    let offset_y = -min_y + padding;
    
    // Apply positions to nodes with aspect ratio correction
    for (i, node) in graph.nodes.iter_mut().enumerate() {
        if i < positions.len() {
            let final_x = (positions[i].x + offset_x) as i32;
            let final_y = ((positions[i].y + offset_y) / 2.0) as i32; // Apply 2:1 aspect ratio correction
            
            node.rect.set_left(final_x - (node.rect.width() as i32 / 2), true);
            node.rect.set_top(final_y - (node.rect.height() as i32 / 2), true);
        }
    }
}
