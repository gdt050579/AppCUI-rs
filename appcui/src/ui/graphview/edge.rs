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
    pub fn directed(mut self, b: bool) -> Self {
        self.edge.directed = b;
        self
    }
    pub fn line_type(mut self, lt: LineType) -> Self {
        self.edge.line_type = Some(lt);
        self
    }
    pub fn attribute(mut self, attr: CharAttribute) -> Self {
        self.edge.attribute = Some(attr);
        self
    }
    #[inline(always)]
    pub fn build(self) -> Edge {
        self.edge
    }
}