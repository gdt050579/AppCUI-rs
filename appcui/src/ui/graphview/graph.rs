use super::edge::Edge;
use super::node::Node;
use super::GraphNode;
use crate::graphics::*;
use crate::prelude::*;
pub struct Graph<T>
where
    T: GraphNode,
{
    pub(super) nodes: Vec<Node<T>>,
    pub(super) edges: Vec<Edge>,
    surface_size: Size,
}
impl<T> Graph<T>
where
    T: GraphNode,
{
    pub fn new(nodes: Vec<Node<T>>, edges: Vec<Edge>) -> Self {
        Self {
            nodes,
            edges,
            surface_size: Size::new(1, 1),
        }
    }
    pub fn with_slices(nodes: &[T], edges: &[(u32, u32)]) -> Self
    where
        T: GraphNode + Clone,
    {
        let v: Vec<Node<T>> = nodes.iter().map(|n| Node::new(n.clone())).collect();
        let e: Vec<Edge> = edges
            .iter()
            .map(|link| Edge {
                from_node_id: link.0,
                to_node_id: link.1,
            })
            .collect();
        Self::new(v, e)
    }
    pub(super) fn update_surface_size(&mut self) {
        if self.nodes.is_empty() {
            self.surface_size = Size::new(1, 1);
            return;
        }
        let r = self.nodes[0].rect;
        let mut tl = r.top_left();
        let mut br = r.bottom_right();
        for n in &self.nodes {
            tl.x = tl.x.min(n.rect.left());
            tl.y = tl.y.min(n.rect.top());
            br.x = br.x.max(n.rect.right());
            br.y = br.y.max(n.rect.bottom());
            log!("G","  R = {:?}",n.rect);
        }
        let dx = 2 - tl.x; // one character on X-axes
        let dy = 1 - tl.y; // two character on Y-axes
        log!("G","dx={}, dy={}, TL = {:?}, BR = {:?}",dx,dy, tl, br);
        for n in &mut self.nodes {
            n.rect += (dx, dy);
            log!("G","  R = {:?}",n.rect);
        }
        // 4 extra ccharacters on left / right (two on left, tow on right)
        // 2 extra on top-bottom (1 on top, 1 on bottom)
        self.surface_size = Size::new(((br.x - tl.x + 1 + 4) as u32).max(1), ((br.y - tl.y + 1 + 2) as u32).max(1));
        log!("G","New size = {:?}",self.surface_size);
    }
    #[inline(always)]
    pub(super) fn size(&self) -> Size {
        self.surface_size
    }
}

impl<T> Default for Graph<T>
where
    T: GraphNode,
{
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            edges: Default::default(),
            surface_size: Default::default(),
        }
    }
}
