use crate::prelude::*;

#[test]
fn sinple_display() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xC976C47E78769FFC) 
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:f");
    let mut gv = GraphView::new(Layout::fill(), graphview::Flags::ScrollBars | graphview::Flags::SearchBar);
    let g = graphview::Graph::with_slices(&["A","B","C","D"], &[(0,1),(0,2),(1,3)], true);
    gv.arrange_nodes(graphview::ArrangeMethod::Hierarchical);
    gv.set_edge_routing(graphview::EdgeRouting::Orthogonal);
    gv.set_edge_line_type(LineType::SingleThick);
    gv.enable_arrow_heads(false);
    gv.enable_edge_highlighting(true, true);
    gv.set_background(char!(".,gray,black"));
    gv.set_components_toolbar_margins(2, 1);
    gv.set_graph(g);
    w.add(gv);
    a.add_window(w);
    a.run();
}

#[test]
fn sinple_display_proc_macro() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xC976C47E78769FFC) 
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:f");
    let mut gv = graphview!("line-type: SingleThick, routing: Orthogonal, hie: true, hoe: true, arrows: false, arrange: Hierarchical, back:{.,gray,black}, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
    let g = graphview::Graph::with_slices(&["A","B","C","D"], &[(0,1),(0,2),(1,3)], true);
    gv.set_graph(g);
    w.add(gv);
    a.add_window(w);
    a.run();
}