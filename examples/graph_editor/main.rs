//! Demonstrates editing graph node data through [`GraphView::modify_graph`].
//!
//! Focus the graph, move the selection with arrow keys, then use **Edit → Rename node**
//! or **Edit → Cycle text color** to apply changes inside a `modify_graph` closure.
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

#[Window(
    events = [MenuEvents, GraphViewEvents<String>, AppBarEvents],
    commands = [RenameNode, CycleColor, ArrangeGrid, About, Exit]
)]
struct GraphEditor {
    graph_view: Handle<GraphView<String>>,
    menu_edit: Handle<MenuButton>,
    menu_graph: Handle<MenuButton>,
    menu_help: Handle<MenuButton>,
    color_idx: usize,
}

impl GraphEditor {
    fn new() -> Self {
        let mut win = Self {
            base: window!("'Graph editor (modify_graph)',a:c,w:72,h:22,Flags: Sizeable"),
            graph_view: Handle::None,
            menu_edit: Handle::None,
            menu_graph: Handle::None,
            menu_help: Handle::None,
            color_idx: 0,
        };

        let mut gv = graphview!("d:f,flags:[ScrollBars,SearchBar]");
        gv.set_graph(initial_graph());
        gv.arrange_nodes(graphview::ArrangeMethod::Hierarchical);
        win.graph_view = win.add(gv);

        let m_edit = menu!(
            "class: GraphEditor, items=[
                {'&Rename node…', cmd: RenameNode},
                {'Cycle &text color', cmd: CycleColor},
            ]"
        );
        win.menu_edit = win
            .appbar()
            .add(MenuButton::new("&Edit", m_edit, 1, Side::Left));

        let m_graph = menu!(
            "class: GraphEditor, items=[
                {'&Grid layout', cmd: ArrangeGrid},
            ]"
        );
        win.menu_graph = win
            .appbar()
            .add(MenuButton::new("&Graph", m_graph, 1, Side::Left));

        let m_help = menu!(
            "class: GraphEditor, items=[
                {&About, cmd: About},
                {E&xit, cmd: Exit},
            ]"
        );
        win.menu_help = win
            .appbar()
            .add(MenuButton::new("&Help", m_help, 1, Side::Left));

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
        if let Some(new_label) =
            dialogs::input::<String>("Rename node", "New label:", Some(old), None)
        {
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

    fn cycle_color_selected(&mut self) {
        let id = {
            let Some(gv) = self.control(self.graph_view) else {
                return;
            };
            let Some(id) = current_node_index(gv.graph()) else {
                dialogs::message("Color", "No node is selected.");
                return;
            };
            id
        };

        const PALETTE: &[Color] = &[
            Color::Green,
            Color::Aqua,
            Color::Yellow,
            Color::Magenta,
            Color::Red,
            Color::Olive,
        ];
        let fg = PALETTE[self.color_idx % PALETTE.len()];
        self.color_idx += 1;

        let gv_h = self.graph_view;
        if let Some(gv) = self.control_mut(gv_h) {
            gv.modify_graph(|g| {
                if let Some(mut node) = g.node(id) {
                    node.set_text_attribute(CharAttribute::with_color(fg, Color::Black));
                }
            });
        }
    }
}

impl GraphViewEvents<String> for GraphEditor {
    fn on_current_node_changed(
        &mut self,
        _handle: Handle<GraphView<String>>,
    ) -> EventProcessStatus {
        self.update_title_bar();
        EventProcessStatus::Processed
    }

    fn on_node_action(
        &mut self,
        _handle: Handle<GraphView<String>>,
        node_index: usize,
    ) -> EventProcessStatus {
        // Same pattern as menu actions: run edits inside `modify_graph`.
        const PALETTE: &[Color] = &[
            Color::Blue,
            Color::Silver,
            Color::Pink,
            Color::White,
        ];
        let fg = PALETTE[self.color_idx % PALETTE.len()];
        self.color_idx += 1;
        let gv_h = self.graph_view;
        if let Some(gv) = self.control_mut(gv_h) {
            gv.modify_graph(|g| {
                if let Some(mut node) = g.node(node_index) {
                    let label = node.value().clone();
                    node.set_value(format!("*{label}"));
                    node.set_text_attribute(CharAttribute::with_color(fg, Color::Black));
                }
            });
            gv.arrange_nodes(graphview::ArrangeMethod::GridPacked);
        }
        self.update_title_bar();
        EventProcessStatus::Processed
    }
}

impl MenuEvents for GraphEditor {
    fn on_command(
        &mut self,
        _menu: Handle<Menu>,
        _item: Handle<menu::Command>,
        command: grapheditor::Commands,
    ) {
        match command {
            grapheditor::Commands::RenameNode => self.rename_selected(),
            grapheditor::Commands::CycleColor => self.cycle_color_selected(),
            grapheditor::Commands::ArrangeGrid => {
                let gv_h = self.graph_view;
                if let Some(gv) = self.control_mut(gv_h) {
                    gv.arrange_nodes(graphview::ArrangeMethod::GridPacked);
                }
            }
            grapheditor::Commands::About => {
                dialogs::message(
                    "About",
                    "This example edits live graph nodes through GraphView::modify_graph.\n\
The closure receives an EditableGraph: use node(index) to change labels, colors, alignment, or position.\n\n\
Focus the graph: arrow keys move selection; Ctrl+arrows move the current node; Enter prefixes the node label and cycles color.",
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
