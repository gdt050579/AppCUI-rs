use super::super::Edge;
use super::super::Graph;
use super::super::GraphNode;
use super::super::Node;
use crate::graphics::*;

pub(in super::super) fn rearange<T: GraphNode>(graph: &mut Graph<T>) {
    if graph.nodes.is_empty() {
        return;
    }
    let len = graph.nodes.len() as u32;
    let columns = (((len as f64).sqrt()) as u32).max(1);
    let mut cell_size = Size::new(1, 1);
    for node in &graph.nodes {
        let sz = node.rect.size();
        cell_size.width = cell_size.width.max(sz.width);
        cell_size.height = cell_size.height.max(sz.height);
    }
    let mut column = 0;
    let mut x = 0;
    let mut y = 0;
    for node in &mut graph.nodes {
        node.rect.set_left(x);
        node.rect.set_top(y);
        x += (cell_size.width as i32) + 2;
        column += 1;
        if column == columns {
            x = 0;
            column = 0;
            y += (cell_size.height as i32) + 1;
        }
    }
}
