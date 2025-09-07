use appcui::prelude::*;

#[derive(Clone)]
pub struct Settings {
    pub title: String,
    pub arrange_method: graphview::ArrangeMethod,
    pub show_arrow_heads: bool,
    pub highlight_incoming_edges: bool,
    pub highlight_outgoing_edges: bool,
    pub edge_line_type: LineType,
    pub edge_routing: graphview::EdgeRouting,
}

impl Settings {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            arrange_method: graphview::ArrangeMethod::GridPacked,
            show_arrow_heads: true,
            highlight_incoming_edges: false,
            highlight_outgoing_edges: false,
            edge_line_type: LineType::Single,
            edge_routing: graphview::EdgeRouting::Direct,
        }
    }

    pub fn with_arrange_method(mut self, method: graphview::ArrangeMethod) -> Self {
        self.arrange_method = method;
        self
    }

    pub fn with_arrow_heads(mut self, show: bool) -> Self {
        self.show_arrow_heads = show;
        self
    }

    pub fn with_edge_highlighting(mut self, incoming: bool, outgoing: bool) -> Self {
        self.highlight_incoming_edges = incoming;
        self.highlight_outgoing_edges = outgoing;
        self
    }

    pub fn with_edge_line_type(mut self, line_type: LineType) -> Self {
        self.edge_line_type = line_type;
        self
    }

    pub fn with_edge_routing(mut self, routing: graphview::EdgeRouting) -> Self {
        self.edge_routing = routing;
        self
    }
}
