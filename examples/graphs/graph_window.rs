use appcui::prelude::*;
use crate::settings::Settings;

#[Window(events=[MenuEvents], 
         commands=[ArrangeNone, ArrangeGrid, ArrangeGridPacked, ArrangeCircular, 
                  ArrangeHierarchical, ArrangeHierarchicalPacked, ArrangeForceDirected,
                  ToggleArrowHeads, ToggleEdgeHighlightingIn, ToggleEdgeHighlightingOut,
                  SetEdgeLineSingle, SetEdgeLineDouble, SetEdgeLineThick, SetEdgeLineAscii,
                  SetEdgeRoutingDirect, SetEdgeRoutingOrthogonal])]
pub struct GraphWindow {
    graph_view: Handle<GraphView<String>>,
    menu_graph: Handle<Menu>,
    settings: Settings,
}

impl GraphWindow {
    pub fn new(graph: graphview::Graph<String>, settings: Settings) -> Self {
        let mut win = Self {
            base: Window::new(&settings.title, layout!("x:5,y:5,w:80,h:30"), window::Flags::Sizeable),
            graph_view: Handle::None,
            menu_graph: Handle::None,
            settings: settings.clone(),
        };
        
        let mut gv = graphview!("d:f,flags:[ScrollBars,SearchBar]");
        gv.set_graph(graph);
        gv.arrange_nodes(settings.arrange_method);
        gv.enable_arrow_heads(settings.show_arrow_heads);
        gv.enable_edge_highlighting(settings.highlight_incoming_edges, settings.highlight_outgoing_edges);
        gv.set_edge_line_type(settings.edge_line_type);
        gv.set_edge_routing(settings.edge_routing);
        win.graph_view = win.add(gv);

        // create a menu and register it
        let m = menu!("&Graph,class:GraphWindow,items:[
            {'&Arrangement',items:[
                {'&None',cmd: ArrangeNone, select: false},
                {'&Grid',cmd: ArrangeGrid, select: false},
                {'Grid &Packed',cmd: ArrangeGridPacked, select: false},
                {'&Circular',cmd: ArrangeCircular, select: false},
            ]},
            {---},
            {'&Edge Line Type',items:[
                {'&Single',cmd: SetEdgeLineSingle, select: false},
                {'&Double',cmd: SetEdgeLineDouble, select: false},
                {'&Thick',cmd: SetEdgeLineThick, select: false},
                {'&ASCII',cmd: SetEdgeLineAscii, select: false},
            ]},
            {'&Edge Routing',items:[
                {'&Direct',cmd: SetEdgeRoutingDirect, select: false},
                {'&Orthogonal',cmd: SetEdgeRoutingOrthogonal, select: false},
            ]},
            {---},
            {'Show &Arrow Heads',cmd: ToggleArrowHeads, check: false},
            {'Highlight &Incoming Edges',cmd: ToggleEdgeHighlightingIn, check: false},
            {'Highlight &Outgoing Edges',cmd: ToggleEdgeHighlightingOut, check: false},
        ]");
        win.menu_graph = win.register_menu(m);
        
        win
    }
    fn update_graph_view(&mut self) {
        let h = self.graph_view;
        let arrange_method = self.settings.arrange_method;
        let show_arrow_heads = self.settings.show_arrow_heads;
        let highlight_incoming_edges = self.settings.highlight_incoming_edges;
        let highlight_outgoing_edges = self.settings.highlight_outgoing_edges;
        let edge_line_type = self.settings.edge_line_type;
        let edge_routing = self.settings.edge_routing;
        if let Some(gv) = self.control_mut(h) {
            gv.arrange_nodes(arrange_method);
            gv.enable_arrow_heads(show_arrow_heads);
            gv.enable_edge_highlighting(highlight_incoming_edges, highlight_outgoing_edges);
            gv.set_edge_line_type(edge_line_type);
            gv.set_edge_routing(edge_routing);
        }
    }
}


impl MenuEvents for GraphWindow {
    fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<menu::SingleChoice>, command: graphwindow::Commands) {
        match command {
            graphwindow::Commands::ArrangeNone => {
                self.settings.arrange_method = graphview::ArrangeMethod::None;
            },
            graphwindow::Commands::ArrangeGrid => {
                self.settings.arrange_method = graphview::ArrangeMethod::Grid;
            },
            graphwindow::Commands::ArrangeGridPacked => {
                self.settings.arrange_method = graphview::ArrangeMethod::GridPacked;
            },
            graphwindow::Commands::ArrangeCircular => {
                self.settings.arrange_method = graphview::ArrangeMethod::Circular;
            },
            graphwindow::Commands::ArrangeHierarchical => {
                self.settings.arrange_method = graphview::ArrangeMethod::Hierarchical;
            },
            graphwindow::Commands::ArrangeHierarchicalPacked => {
                self.settings.arrange_method = graphview::ArrangeMethod::HierarchicalPacked;
            },
            graphwindow::Commands::ArrangeForceDirected => {
                self.settings.arrange_method = graphview::ArrangeMethod::ForceDirected;
            },
            graphwindow::Commands::SetEdgeLineSingle => {
                self.settings.edge_line_type = LineType::Single;
            },
            graphwindow::Commands::SetEdgeLineDouble => {
                self.settings.edge_line_type = LineType::Double;
            },
            graphwindow::Commands::SetEdgeLineThick => {
                self.settings.edge_line_type = LineType::SingleThick;
            },
            graphwindow::Commands::SetEdgeLineAscii => {
                self.settings.edge_line_type = LineType::Ascii;
            },
            graphwindow::Commands::SetEdgeRoutingDirect => {
                self.settings.edge_routing = graphview::EdgeRouting::Direct;
            },
            graphwindow::Commands::SetEdgeRoutingOrthogonal => {
                self.settings.edge_routing = graphview::EdgeRouting::Orthogonal;
            },
            _ => {}
        }
        self.update_graph_view();
        
    }
    fn on_check(&mut self, _menu: Handle<Menu>, _: Handle<menu::CheckBox>, command: graphwindow::Commands, checked: bool) {
        match command {
            graphwindow::Commands::ToggleArrowHeads => {
                self.settings.show_arrow_heads = checked;
            },
            graphwindow::Commands::ToggleEdgeHighlightingIn => {
                self.settings.highlight_incoming_edges = checked;
            },
            graphwindow::Commands::ToggleEdgeHighlightingOut => {
                self.settings.highlight_outgoing_edges = checked;
            },
            _ => {}
        }
        self.update_graph_view();
    }

    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.add(self.menu_graph, 1);
    }
}
