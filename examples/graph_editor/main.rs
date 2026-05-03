//! Demonstrates editing graph node data through [`GraphView::modify_graph`].
//!
//! Focus the graph, move the selection with arrow keys, then use **Edit → Rename node**,
//! **Edit → Resize node**, **Edit → Node border**, or **Edit → Text color** to apply changes inside a `modify_graph` closure.
//! **Graph → Add node** / **Remove node** change the node set (`EditableGraph::add_node` / `delete_node`).
//! Press **Enter** on a node for `on_node_action` (same edits use the focused node).

use appcui::prelude::*;
use appcui::ui::appbar::{AppBar, MenuButton, Side};

fn initial_graph() -> graphview::Graph<String> {
    let nodes = vec![
        graphview::NodeBuilder::new("Input".to_string()).build(),
        graphview::NodeBuilder::new("Process".to_string()).build(),
        graphview::NodeBuilder::new("Output".to_string()).build(),
    ];
    let edges = vec![
        graphview::EdgeBuilder::new(0, 1).directed(true).build(),
        graphview::EdgeBuilder::new(1, 2).directed(true).build(),
    ];
    graphview::Graph::new(nodes, edges)
}

/// Resolves the index of the focused node using only the public [`graphview::Graph`] API.
fn current_node_index(g: &graphview::Graph<String>) -> Option<usize> {
    let cur = g.current_node()?;
    (0..g.nodes_count()).find(|&i| g.node(i).is_some_and(|n| std::ptr::eq(cur, n)))
}

fn validate_node_size(value: &Size) -> Result<(), String> {
    if value.width < 5 {
        Err("Use width at least 5 columns.".into())
    } else if value.width > 160 {
        Err("Width must be at most 160.".into())
    } else if value.height < 3 {
        Err("Use height at least 3 rows.".into())
    } else if value.height > 50 {
        Err("Height must be at most 50.".into())
    } else {
        Ok(())
    }
}

#[Window(
    events = [MenuEvents, GraphViewEvents<String>, AppBarEvents],
    commands = [
        RenameNode,
        ResizeNode,
        TextColorDefault,
        TextColorBlack,
        TextColorDarkBlue,
        TextColorDarkGreen,
        TextColorTeal,
        TextColorDarkRed,
        TextColorMagenta,
        TextColorOlive,
        TextColorSilver,
        TextColorGray,
        TextColorBlue,
        TextColorGreen,
        TextColorAqua,
        TextColorRed,
        TextColorPink,
        TextColorYellow,
        TextColorWhite,
        NodeBorderNone,
        NodeBorderSingle,
        NodeBorderDouble,
        NodeBorderThick,
        NodeBorderHeavy,
        NodeBorderAscii,
        NodeBorderAsciiRound,
        NodeBorderSingleRound,
        NodeBorderBraille,
        ArrangeGrid,
        AddNode,
        RemoveNode,
        About,
        Exit,
    ]
)]
struct GraphEditor {
    graph_view: Handle<GraphView<String>>,
    menu_edit: Handle<MenuButton>,
    menu_graph: Handle<MenuButton>,
    menu_help: Handle<MenuButton>,
}

impl GraphEditor {
    fn new() -> Self {
        let mut win = Self {
            base: window!("'Graph editor (modify_graph)',a:c,w:72,h:22,Flags: Sizeable"),
            graph_view: Handle::None,
            menu_edit: Handle::None,
            menu_graph: Handle::None,
            menu_help: Handle::None,
        };

        let mut gv = graphview!("d:f,flags:[ScrollBars,SearchBar,MultiSelect]");
        gv.set_graph(initial_graph());
        gv.arrange_nodes(graphview::ArrangeMethod::Hierarchical);
        win.graph_view = win.add(gv);

        let m_edit = menu!(
            "class: GraphEditor, items=[
                {'&Rename node…', cmd: RenameNode},
                {'&Resize node…', cmd: ResizeNode},
                { '&Text color', items = [
                    {'&Default (theme)', cmd: TextColorDefault},
                    { --- },
                    {'&Black', cmd: TextColorBlack},
                    {'Dar&k blue', cmd: TextColorDarkBlue},
                    {'D&ark green', cmd: TextColorDarkGreen},
                    {'&Teal', cmd: TextColorTeal},
                    {'Dar&k red', cmd: TextColorDarkRed},
                    {'&Magenta', cmd: TextColorMagenta},
                    {'&Olive', cmd: TextColorOlive},
                    {'&Silver', cmd: TextColorSilver},
                    {'Gr&ay', cmd: TextColorGray},
                    {'&Blue', cmd: TextColorBlue},
                    {'&Green', cmd: TextColorGreen},
                    {'A&qua', cmd: TextColorAqua},
                    {'&Red', cmd: TextColorRed},
                    {'&Pink', cmd: TextColorPink},
                    {'&Yellow', cmd: TextColorYellow},
                    {'&White', cmd: TextColorWhite},
                ]},
                { 'Node &border', items = [
                    {'N&one (default)', cmd: NodeBorderNone},
                    { --- },
                    {'&Single', cmd: NodeBorderSingle},
                    {'&Double', cmd: NodeBorderDouble},
                    {'&Thick', cmd: NodeBorderThick},
                    {'&Border heavy', cmd: NodeBorderHeavy},
                    {'&ASCII', cmd: NodeBorderAscii},
                    {'ASCII &rounded', cmd: NodeBorderAsciiRound},
                    {'&Unicode rounded', cmd: NodeBorderSingleRound},
                    {'&Braille', cmd: NodeBorderBraille},
                ]},
            ]"
        );
        win.menu_edit = win.appbar().add(MenuButton::new("&Edit", m_edit, 1, Side::Left));

        let m_graph = menu!(
            "class: GraphEditor, items=[
                {'&Add node…', cmd: AddNode},
                {'&Remove node', cmd: RemoveNode},
                { --- },
                {'&Grid layout', cmd: ArrangeGrid},
            ]"
        );
        win.menu_graph = win.appbar().add(MenuButton::new("&Graph", m_graph, 1, Side::Left));

        let m_help = menu!(
            "class: GraphEditor, items=[
                {&About, cmd: About},
                {E&xit, cmd: Exit},
            ]"
        );
        win.menu_help = win.appbar().add(MenuButton::new("&Help", m_help, 1, Side::Left));

        win.update_title_bar();
        win
    }

    fn update_title_bar(&mut self) {
        let suffix = if let Some(gv) = self.control(self.graph_view) {
            if let Some(i) = current_node_index(gv.graph()) {
                if let Some(n) = gv.graph().node(i) {
                    format!(" — [{i}] {}", n.value())
                } else {
                    String::new()
                }
            } else {
                " — (empty)".to_string()
            }
        } else {
            String::new()
        };
        self.set_title(&format!("Graph editor{suffix}"));
    }

    fn rename_selected(&mut self) {
        let (id, old) = {
            let Some(gv) = self.control(self.graph_view) else {
                return;
            };
            let Some(id) = current_node_index(gv.graph()) else {
                dialogs::message("Rename", "No node is selected. Click the graph or use arrow keys.");
                return;
            };
            let old = gv.graph().node(id).unwrap().value().clone();
            (id, old)
        };
        if let Some(new_label) = dialogs::input::<String>("Rename node", "New label:", Some(old), None) {
            let gv_h = self.graph_view;
            if let Some(gv) = self.control_mut(gv_h) {
                gv.modify_graph(|g| {
                    if let Some(mut node) = g.node(id) {
                        node.set_value(new_label);
                    }
                });
            }
        }
    }

    fn resize_selected(&mut self) {
        let (id, top_left, size) = {
            let gv_h = self.graph_view;
            let Some(gv) = self.control_mut(gv_h) else {
                return;
            };
            let Some(id) = current_node_index(gv.graph()) else {
                dialogs::message("Resize", "No node is selected. Click the graph or use arrow keys.");
                return;
            };
            let mut top_left = Point::ORIGIN;
            let mut size = Size::new(1, 1);
            gv.modify_graph(|g| {
                if let Some(node) = g.node(id) {
                    let r = node.bounds();
                    top_left = r.top_left();
                    size = r.size();
                }
            });
            (id, top_left, size)
        };

        let Some(new_size) = dialogs::input::<Size>("Resize node", "Size as width x height (e.g. 16x4):", Some(size), Some(validate_node_size))
        else {
            return;
        };

        let gv_h = self.graph_view;
        if let Some(gv) = self.control_mut(gv_h) {
            gv.modify_graph(|g| {
                if let Some(mut node) = g.node(id) {
                    node.set_bounds(Rect::with_point_and_size(top_left, new_size));
                }
            });
        }
    }

    fn apply_text_color(&mut self, foreground: Option<Color>) {
        let id = {
            let Some(gv) = self.control(self.graph_view) else {
                return;
            };
            let Some(id) = current_node_index(gv.graph()) else {
                dialogs::message("Text color", "No node is selected. Click the graph or use arrow keys.");
                return;
            };
            id
        };

        let gv_h = self.graph_view;
        if let Some(gv) = self.control_mut(gv_h) {
            gv.modify_graph(|g| {
                if let Some(mut node) = g.node(id) {
                    match foreground {
                        None => node.clear_text_attribute(),
                        Some(fg) => {
                            node.set_text_attribute(CharAttribute::with_color(fg, Color::Black));
                        }
                    }
                }
            });
        }
    }

    fn add_node(&mut self) {
        let anchor = {
            let Some(gv) = self.control(self.graph_view) else {
                return;
            };
            if gv.graph().nodes_count() == 0 {
                None
            } else {
                current_node_index(gv.graph())
            }
        };

        let Some(label) = dialogs::input::<String>("Add node", "Label for the new node:", Some("New node".to_string()), None) else {
            return;
        };

        let gv_h = self.graph_view;
        if let Some(gv) = self.control_mut(gv_h) {
            gv.modify_graph(|g| {
                let node = graphview::NodeBuilder::new(label).build();
                let id = g.add_node(node);
                if let Some(from) = anchor {
                    let _ = g.add_edge(graphview::EdgeBuilder::new(from as u32, id as u32).directed(true).build());
                }
                g.set_current_node(id);
            });
        }
        self.update_title_bar();
    }

    fn remove_selected_node(&mut self) {
        let id = {
            let Some(gv) = self.control(self.graph_view) else {
                return;
            };
            if gv.graph().nodes_count() == 0 {
                dialogs::message("Remove node", "The graph has no nodes.");
                return;
            };
            let Some(id) = current_node_index(gv.graph()) else {
                dialogs::message("Remove node", "No node is selected. Focus the graph and choose a node first.");
                return;
            };
            id
        };

        let gv_h = self.graph_view;
        if let Some(gv) = self.control_mut(gv_h) {
            gv.modify_graph(|g| {
                g.delete_node(id);
            });
        }
        self.update_title_bar();
    }

    fn apply_node_border(&mut self, border: Option<LineType>) {
        let id = {
            let Some(gv) = self.control(self.graph_view) else {
                return;
            };
            let Some(id) = current_node_index(gv.graph()) else {
                dialogs::message("Border", "No node is selected. Click the graph or use arrow keys.");
                return;
            };
            id
        };

        let gv_h = self.graph_view;
        if let Some(gv) = self.control_mut(gv_h) {
            gv.modify_graph(|g| {
                if let Some(mut node) = g.node(id) {
                    match border {
                        None => node.clear_border(),
                        Some(lt) => node.set_border(lt),
                    }
                }
            });
        }
    }
}

impl GraphViewEvents<String> for GraphEditor {
    fn on_current_node_changed(&mut self, _handle: Handle<GraphView<String>>) -> EventProcessStatus {
        self.update_title_bar();
        EventProcessStatus::Processed
    }

    fn on_node_action(&mut self, _handle: Handle<GraphView<String>>, node_index: usize) -> EventProcessStatus {
        // Same pattern as menu actions: run edits inside `modify_graph`.
        let gv_h = self.graph_view;
        if let Some(gv) = self.control_mut(gv_h) {
            gv.modify_graph(|g| {
                if let Some(mut node) = g.node(node_index) {
                    let label = node.value().clone();
                    node.set_value(format!("*{label}"));
                }
            });
            gv.arrange_nodes(graphview::ArrangeMethod::GridPacked);
        }
        self.update_title_bar();
        EventProcessStatus::Processed
    }

    fn on_request_new_node(&mut self, handle: Handle<GraphView<String>>, p: Point) -> EventProcessStatus {
        if let Some(gv) = self.control_mut(handle) {
            gv.modify_graph(|g| {
                let n = graphview::NodeBuilder::new(format!("Node:{}", g.nodes_count() + 1)).position(p).build();
                let id = g.add_node(n);
                g.set_current_node(id);
            });
        }
        EventProcessStatus::Processed
    }
    fn on_selection_changed(&mut self, handle: Handle<GraphView<String>>) -> EventProcessStatus {
        let cnt = if let Some(gv) = self.control(handle) {
            gv.selected_count()
        } else {
            0
        };
        self.set_title(format!("Selected count: {cnt}").as_str());
        EventProcessStatus::Processed
    }
}

impl MenuEvents for GraphEditor {
    fn on_command(&mut self, _menu: Handle<Menu>, _item: Handle<menu::Command>, command: grapheditor::Commands) {
        match command {
            grapheditor::Commands::RenameNode => self.rename_selected(),
            grapheditor::Commands::ResizeNode => self.resize_selected(),
            grapheditor::Commands::TextColorDefault => self.apply_text_color(None),
            grapheditor::Commands::TextColorBlack => {
                self.apply_text_color(Some(Color::Black));
            }
            grapheditor::Commands::TextColorDarkBlue => {
                self.apply_text_color(Some(Color::DarkBlue));
            }
            grapheditor::Commands::TextColorDarkGreen => {
                self.apply_text_color(Some(Color::DarkGreen));
            }
            grapheditor::Commands::TextColorTeal => {
                self.apply_text_color(Some(Color::Teal));
            }
            grapheditor::Commands::TextColorDarkRed => {
                self.apply_text_color(Some(Color::DarkRed));
            }
            grapheditor::Commands::TextColorMagenta => {
                self.apply_text_color(Some(Color::Magenta));
            }
            grapheditor::Commands::TextColorOlive => {
                self.apply_text_color(Some(Color::Olive));
            }
            grapheditor::Commands::TextColorSilver => {
                self.apply_text_color(Some(Color::Silver));
            }
            grapheditor::Commands::TextColorGray => {
                self.apply_text_color(Some(Color::Gray));
            }
            grapheditor::Commands::TextColorBlue => {
                self.apply_text_color(Some(Color::Blue));
            }
            grapheditor::Commands::TextColorGreen => {
                self.apply_text_color(Some(Color::Green));
            }
            grapheditor::Commands::TextColorAqua => {
                self.apply_text_color(Some(Color::Aqua));
            }
            grapheditor::Commands::TextColorRed => {
                self.apply_text_color(Some(Color::Red));
            }
            grapheditor::Commands::TextColorPink => {
                self.apply_text_color(Some(Color::Pink));
            }
            grapheditor::Commands::TextColorYellow => {
                self.apply_text_color(Some(Color::Yellow));
            }
            grapheditor::Commands::TextColorWhite => {
                self.apply_text_color(Some(Color::White));
            }
            grapheditor::Commands::NodeBorderNone => self.apply_node_border(None),
            grapheditor::Commands::NodeBorderSingle => {
                self.apply_node_border(Some(LineType::Single));
            }
            grapheditor::Commands::NodeBorderDouble => {
                self.apply_node_border(Some(LineType::Double));
            }
            grapheditor::Commands::NodeBorderThick => {
                self.apply_node_border(Some(LineType::SingleThick));
            }
            grapheditor::Commands::NodeBorderHeavy => {
                self.apply_node_border(Some(LineType::Border));
            }
            grapheditor::Commands::NodeBorderAscii => {
                self.apply_node_border(Some(LineType::Ascii));
            }
            grapheditor::Commands::NodeBorderAsciiRound => {
                self.apply_node_border(Some(LineType::AsciiRound));
            }
            grapheditor::Commands::NodeBorderSingleRound => {
                self.apply_node_border(Some(LineType::SingleRound));
            }
            grapheditor::Commands::NodeBorderBraille => {
                self.apply_node_border(Some(LineType::Braille));
            }
            grapheditor::Commands::ArrangeGrid => {
                let gv_h = self.graph_view;
                if let Some(gv) = self.control_mut(gv_h) {
                    gv.arrange_nodes(graphview::ArrangeMethod::GridPacked);
                }
            }
            grapheditor::Commands::AddNode => self.add_node(),
            grapheditor::Commands::RemoveNode => self.remove_selected_node(),
            grapheditor::Commands::About => {
                dialogs::message(
                    "About",
                    "This example edits live graph nodes through GraphView::modify_graph.\n\
The closure receives an EditableGraph: use add_node/delete_node for structure; node(index) for labels, size, borders, colors, alignment, position.\n\n\
Focus the graph: arrow keys move selection; Ctrl+arrows move the current node; Enter prefixes the node label with *. Graph→Add draws an edge from the selected node when the graph is non-empty.",
                );
            }
            grapheditor::Commands::Exit => self.close(),
        }
    }
}

impl AppBarEvents for GraphEditor {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.menu_edit);
        appbar.show(self.menu_graph);
        appbar.show(self.menu_help);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().app_bar().build()?;
    app.add_window(GraphEditor::new());
    app.run();
    Ok(()) 
}
 