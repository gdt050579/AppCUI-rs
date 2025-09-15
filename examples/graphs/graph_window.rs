use appcui::prelude::*;
use appcui::ui::appbar::*;
use crate::settings::Settings;

#[Window(events=[MenuEvents], 
         commands=[ArrangeNone, ArrangeGrid, ArrangeGridPacked, ArrangeCircular, 
                  ArrangeHierarchical, ArrangeHierarchicalPacked, ArrangeForceDirected,
                  ToggleArrowHeads, ToggleEdgeHighlightingIn, ToggleEdgeHighlightingOut,
                  SetEdgeLineSingle, SetEdgeLineDouble, SetEdgeLineThick, SetEdgeLineAscii,
                  SetEdgeRoutingDirect, SetEdgeRoutingOrthogonal, SetEdgeLineBorder,
                  SetEdgeLineAsciiRound, SetEdgeLineSingleRound, SetEdgeLineBraille])]
pub struct GraphWindow {
    graph_view: Handle<GraphView<String>>,
    menu_graph: Handle<MenuEntry>,
    settings: Settings,
    h_arrange_none: Handle<menu::SingleChoice>, 
    h_arrange_grid: Handle<menu::SingleChoice>,
    h_arrange_grid_packed: Handle<menu::SingleChoice>,
    h_arrange_circular: Handle<menu::SingleChoice>,
    m_hierarchical: Handle<menu::SingleChoice>,
    m_hierarchical_packed: Handle<menu::SingleChoice>,
    m_force_directed: Handle<menu::SingleChoice>,
    h_edge_routing_direct: Handle<menu::SingleChoice>,
    h_edge_routing_orthogonal: Handle<menu::SingleChoice>,
    h_edge_line_single: Handle<menu::SingleChoice>,
    h_edge_line_double: Handle<menu::SingleChoice>,
    h_edge_line_thick: Handle<menu::SingleChoice>,
    h_edge_line_ascii: Handle<menu::SingleChoice>,
    h_edge_line_border: Handle<menu::SingleChoice>,
    h_edge_line_ascii_round: Handle<menu::SingleChoice>,
    h_edge_line_single_round: Handle<menu::SingleChoice>,
    h_edge_line_braille: Handle<menu::SingleChoice>,
    h_arrow_heads: Handle<menu::CheckBox>,
    h_highlight_incoming_edges: Handle<menu::CheckBox>,
    h_highlight_outgoing_edges: Handle<menu::CheckBox>,
}

impl GraphWindow { 
    pub fn new(graph: graphview::Graph<String>, settings: Settings) -> Self {
        let mut win = Self {
            base: Window::new(&settings.title, layout!("a:c,w:75%,h:75%"), window::Flags::Sizeable),
            graph_view: Handle::None,
            menu_graph: Handle::None,
            settings: settings.clone(),
            h_arrange_none: Handle::None,
            h_arrange_grid: Handle::None,
            h_arrange_grid_packed: Handle::None,
            h_arrange_circular: Handle::None,
            m_hierarchical: Handle::None,
            m_hierarchical_packed: Handle::None,
            m_force_directed: Handle::None,
            h_edge_routing_direct: Handle::None,
            h_edge_routing_orthogonal: Handle::None,
            h_edge_line_single: Handle::None,
            h_edge_line_double: Handle::None,
            h_edge_line_thick: Handle::None,
            h_edge_line_ascii: Handle::None,
            h_edge_line_border: Handle::None,
            h_edge_line_ascii_round: Handle::None,
            h_edge_line_single_round: Handle::None,
            h_edge_line_braille: Handle::None,
            h_arrow_heads: Handle::None,
            h_highlight_incoming_edges: Handle::None,
            h_highlight_outgoing_edges: Handle::None,
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
        let mut m = Menu::new();
        let mut a = Menu::new();
        win.h_arrange_none = a.add(menu::SingleChoice::new("&None", Key::None, graphwindow::Commands::ArrangeNone, false));
        win.h_arrange_grid = a.add(menu::SingleChoice::new("&Grid", Key::None, graphwindow::Commands::ArrangeGrid, false));
        win.h_arrange_grid_packed = a.add(menu::SingleChoice::new("Grid &Packed", Key::None, graphwindow::Commands::ArrangeGridPacked, false));
        win.h_arrange_circular = a.add(menu::SingleChoice::new("&Circular", Key::None, graphwindow::Commands::ArrangeCircular, false));
        win.m_hierarchical = a.add(menu::SingleChoice::new("&Hierarchical", Key::None, graphwindow::Commands::ArrangeHierarchical, false));
        win.m_hierarchical_packed = a.add(menu::SingleChoice::new("Hierarchical P&acked", Key::None, graphwindow::Commands::ArrangeHierarchicalPacked, false));
        win.m_force_directed = a.add(menu::SingleChoice::new("&Force Directed", Key::None, graphwindow::Commands::ArrangeForceDirected, false));
        m.add(menu::SubMenu::new("&Arrangement", a));

        let mut er = Menu::new();
        win.h_edge_routing_direct = er.add(menu::SingleChoice::new("&Direct", Key::None, graphwindow::Commands::SetEdgeRoutingDirect, false));
        win.h_edge_routing_orthogonal = er.add(menu::SingleChoice::new("&Orthogonal", Key::None, graphwindow::Commands::SetEdgeRoutingOrthogonal, false));
        m.add(menu::SubMenu::new("&Edge routing", er));
        
        let mut et = Menu::new();
        win.h_edge_line_single = et.add(menu::SingleChoice::new("&Single", Key::None, graphwindow::Commands::SetEdgeLineSingle, false));
        win.h_edge_line_double = et.add(menu::SingleChoice::new("&Double", Key::None, graphwindow::Commands::SetEdgeLineDouble, false));
        win.h_edge_line_thick = et.add(menu::SingleChoice::new("&Thick", Key::None, graphwindow::Commands::SetEdgeLineThick, false));
        win.h_edge_line_ascii = et.add(menu::SingleChoice::new("&ASCII", Key::None, graphwindow::Commands::SetEdgeLineAscii, false));
        win.h_edge_line_border = et.add(menu::SingleChoice::new("&Border", Key::None, graphwindow::Commands::SetEdgeLineBorder, false));
        win.h_edge_line_ascii_round = et.add(menu::SingleChoice::new("&ASCII Round", Key::None, graphwindow::Commands::SetEdgeLineAsciiRound, false));
        win.h_edge_line_single_round = et.add(menu::SingleChoice::new("&Single Round", Key::None, graphwindow::Commands::SetEdgeLineSingleRound, false));
        win.h_edge_line_braille = et.add(menu::SingleChoice::new("&Braille", Key::None, graphwindow::Commands::SetEdgeLineBraille, false));
        m.add(menu::SubMenu::new("Edge &line type", et));

        m.add(menu::Separator::new());

        win.h_arrow_heads = m.add(menu::CheckBox::new("Show &Arrow Heads", Key::None, graphwindow::Commands::ToggleArrowHeads, false));
        win.h_highlight_incoming_edges = m.add(menu::CheckBox::new("Highlight &Incoming Edges", Key::None, graphwindow::Commands::ToggleEdgeHighlightingIn, false));
        win.h_highlight_outgoing_edges = m.add(menu::CheckBox::new("Highlight &Outgoing Edges", Key::None, graphwindow::Commands::ToggleEdgeHighlightingOut, false));

        win.menu_graph = win.menubar_mut().add(MenuEntry::new("&Graph", m,1,AppBarPosition::Left));
        
        win
    }
    fn update_graph_view(&mut self, rearange: bool) {
        let h = self.graph_view;
        let arrange_method = self.settings.arrange_method;
        let show_arrow_heads = self.settings.show_arrow_heads;
        let highlight_incoming_edges = self.settings.highlight_incoming_edges;
        let highlight_outgoing_edges = self.settings.highlight_outgoing_edges;
        let edge_line_type = self.settings.edge_line_type;
        let edge_routing = self.settings.edge_routing;
        if let Some(gv) = self.control_mut(h) {
            if rearange {
                gv.arrange_nodes(arrange_method);
            }
            gv.enable_arrow_heads(show_arrow_heads);
            gv.enable_edge_highlighting(highlight_incoming_edges, highlight_outgoing_edges);
            gv.set_edge_line_type(edge_line_type);
            gv.set_edge_routing(edge_routing);
        }
    }
}


impl MenuEvents for GraphWindow {
    fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<menu::SingleChoice>, command: graphwindow::Commands) {
        let mut rearange = false;
        match command {
            graphwindow::Commands::ArrangeNone => {
                self.settings.arrange_method = graphview::ArrangeMethod::None;
                rearange = true;
            },
            graphwindow::Commands::ArrangeGrid => {
                self.settings.arrange_method = graphview::ArrangeMethod::Grid;
                rearange = true;
            },
            graphwindow::Commands::ArrangeGridPacked => {
                self.settings.arrange_method = graphview::ArrangeMethod::GridPacked;
                rearange = true;
            },
            graphwindow::Commands::ArrangeCircular => {
                self.settings.arrange_method = graphview::ArrangeMethod::Circular;
                rearange = true;
            },
            graphwindow::Commands::ArrangeHierarchical => {
                self.settings.arrange_method = graphview::ArrangeMethod::Hierarchical;
                rearange = true;
            },
            graphwindow::Commands::ArrangeHierarchicalPacked => {
                self.settings.arrange_method = graphview::ArrangeMethod::HierarchicalPacked;
                rearange = true;
            },
            graphwindow::Commands::ArrangeForceDirected => {
                self.settings.arrange_method = graphview::ArrangeMethod::ForceDirected;
                rearange = true;
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
            graphwindow::Commands::SetEdgeLineBorder => {
                self.settings.edge_line_type = LineType::Border;
            },
            graphwindow::Commands::SetEdgeLineAsciiRound => {
                self.settings.edge_line_type = LineType::AsciiRound;
            },
            graphwindow::Commands::SetEdgeLineSingleRound => {
                self.settings.edge_line_type = LineType::SingleRound;
            },
            graphwindow::Commands::SetEdgeLineBraille => {
                self.settings.edge_line_type = LineType::Braille;
            },
            _ => {}
        }
        self.update_graph_view(rearange);
        
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
        self.update_graph_view(false);
    }

    fn on_update_menubar(&self, menubar: &mut AppBar) {
        menubar.show(self.menu_graph);
    }

    fn on_menu_open(&self,menu: &mut Menu) {
        // update the status of menu items based on the settings.
        let h = match self.settings.arrange_method {
            graphview::ArrangeMethod::None => self.h_arrange_none,
            graphview::ArrangeMethod::Grid => self.h_arrange_grid,
            graphview::ArrangeMethod::GridPacked => self.h_arrange_grid_packed,
            graphview::ArrangeMethod::Circular => self.h_arrange_circular,
            graphview::ArrangeMethod::Hierarchical => self.m_hierarchical,
            graphview::ArrangeMethod::HierarchicalPacked => self.m_hierarchical_packed,
            graphview::ArrangeMethod::ForceDirected => self.m_force_directed,
        };
        if let Some(item) = menu.get_mut(h) {
            item.set_selected();
        }
        // update the status of edge routing menu items based on the settings.
        let h = match self.settings.edge_routing {
            graphview::EdgeRouting::Direct => self.h_edge_routing_direct,
            graphview::EdgeRouting::Orthogonal => self.h_edge_routing_orthogonal,
        };
        if let Some(item) = menu.get_mut(h) {
            item.set_selected();
        }
        // update the status of edge line type menu items based on the settings.
        let h = match self.settings.edge_line_type {
            LineType::Single => self.h_edge_line_single,
            LineType::Double => self.h_edge_line_double,
            LineType::SingleThick => self.h_edge_line_thick,
            LineType::Ascii => self.h_edge_line_ascii,
            LineType::Border => self.h_edge_line_border,
            LineType::AsciiRound => self.h_edge_line_ascii_round,
            LineType::SingleRound => self.h_edge_line_single_round,
            LineType::Braille => self.h_edge_line_braille,            
        };
        if let Some(item) = menu.get_mut(h) {
            item.set_selected();
        }
        // update the status of arrow heads menu item based on the settings.
        if let Some(item) = menu.get_mut(self.h_arrow_heads) {
            item.set_checked(self.settings.show_arrow_heads);
        }
        // update the status of incoming edges menu item based on the settings.
        if let Some(item) = menu.get_mut(self.h_highlight_incoming_edges) {
            item.set_checked(self.settings.highlight_incoming_edges);
        }
        // update the status of outgoing edges menu item based on the settings.
        if let Some(item) = menu.get_mut(self.h_highlight_outgoing_edges) {
            item.set_checked(self.settings.highlight_outgoing_edges);
        }
    }
}
