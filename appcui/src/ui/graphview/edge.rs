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

pub struct EditableEdge<'a> {
    edge: &'a mut Edge,
    changed: &'a mut bool,
}
impl<'a> EditableEdge<'a> {
    pub(super) fn new(edge: &'a mut Edge, changed: &'a mut bool) -> Self {
        Self { edge, changed }
    }
    #[inline(always)]
    pub fn from_node_id(&self) -> u32 {
        self.edge.from_node_id
    }
    #[inline(always)]
    pub fn to_node_id(&self) -> u32 {
        self.edge.to_node_id
    }
    #[inline(always)]
    pub fn directed(&self) -> bool {
        self.edge.directed
    }
    #[inline(always)]
    pub fn attribute(&self) -> Option<CharAttribute> {
        self.edge.attribute
    }
    #[inline(always)]
    pub fn set_attribute(&mut self, attr: CharAttribute) {
        if self.edge.attribute != Some(attr) {
            self.edge.attribute = Some(attr);
            *self.changed = true;
        }
    }
    #[inline(always)]
    pub fn line_type(&self) -> Option<LineType> {
        self.edge.line_type
    }
    #[inline(always)]
    pub fn set_line_type(&mut self, lt: LineType) {
        if self.edge.line_type != Some(lt) {
            self.edge.line_type = Some(lt);
            *self.changed = true;
        }
    }
}   