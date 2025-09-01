use super::EdgeRouting;
use crate::graphics::LineType;

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct RenderingOptions {
    pub(super) highlight_edges_in: bool,
    pub(super) highlight_edges_out: bool,
    pub(super) show_arrow_heads: bool,
    pub(super) edge_routing: EdgeRouting,
    pub(super) edge_line_type: LineType,
}

impl RenderingOptions {
    pub fn new() -> Self {
        Self {
            highlight_edges_in: false,
            highlight_edges_out: false,
            show_arrow_heads: true,
            edge_routing: EdgeRouting::Direct,
            edge_line_type: LineType::Single,
        }
    }
}
