use crate::graphics::*;

pub struct Edge {
    pub(super) from_node_id: u32,
    pub(super) to_node_id: u32,
    pub(super) directed: bool,
    pub(super) attribute: Option<CharAttribute>,
    pub(super) line_type: Option<LineType>,
}

pub struct EdgeBuilder {
    edge: Edge,
}   
impl EdgeBuilder {
    /// Create a new EdgeBuilder
    pub fn new(from_node_id: u32, to_node_id: u32) -> Self {
        Self {
            edge: Edge {
                from_node_id,
                to_node_id,
                directed: false,
                attribute: None,
                line_type: None,
            },
        }
    }
    /// Set if the edge is directed (with an arrow) or not
    /// Default is false (not directed)
    pub fn directed(mut self, b: bool) -> Self {
        self.edge.directed = b;
        self
    }
    /// Set the line type for the edge (if not set, the default line type will be used)
    /// The default line type can be set using the GraphView::set_edge_line_type method
    pub fn line_type(mut self, lt: LineType) -> Self {
        self.edge.line_type = Some(lt);
        self
    }

    /// Set the attribute for the edge (if not set, the default attribute will be used)
    pub fn attribute(mut self, attr: CharAttribute) -> Self {
        self.edge.attribute = Some(attr);
        self
    }

    /// Builds the actual edge
    #[inline(always)]
    pub fn build(self) -> Edge {
        self.edge
    }
}