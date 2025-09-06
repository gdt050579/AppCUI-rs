use crate::prelude::*;

fn build_custom_graph_1() -> graphview::Graph<String> {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    nodes.push(
        graphview::NodeBuilder::new("School".to_string())
            .border(LineType::Double)
            .size(Size::new(20, 1))
            .build(),
    );
    nodes.push(
        graphview::NodeBuilder::new("Math".to_string())
            .border(LineType::Single)
            .size(Size::new(10, 1))
            .text_alignment(TextAlignment::Left)
            .build(),
    );
    nodes.push(
        graphview::NodeBuilder::new("English".to_string())
            .border(LineType::Ascii)
            .size(Size::new(15, 1))
            .text_alignment(TextAlignment::Right)
            .build(),
    );
    nodes.push(graphview::NodeBuilder::new("Science".to_string()).size(Size::new(12, 1)).build());
    nodes.push(graphview::NodeBuilder::new("Glonal\nLiterature".to_string()).size(Size::new(12, 2)).build());    
    edges.push(graphview::EdgeBuilder::new(0, 1).build());
    edges.push(graphview::EdgeBuilder::new(0, 2).build());
    edges.push(graphview::EdgeBuilder::new(1, 3).build());
    edges.push(graphview::EdgeBuilder::new(2, 4).build());
    graphview::Graph::new(nodes, edges)
}

fn build_custom_graph_2() -> graphview::Graph<&'static str> {
    let nodes = &[
        "N1","N2","N3","N4","N5",
        "N6","N7","N8","N9","N10",
        "N11","N12","N13","N14","N15",
        "N16","N17","N18","N19","N20",
    ];

    let edges: &[(u32, u32)] = &[
        (0,1), (0,2), (0,3),
        (1,4), (1,5), (1,6),
        (2,6), (2,7), (2,8),
        (3,8), (3,9), (3,10),
        (4,10), (4,11), (4,12),
        (5,12), (5,13), (5,14),
        (6,14), (6,15), (6,16),
        (7,16), (7,17), (7,18),
        (8,18), (8,19), (8,0),
        (9,1), (9,5), (9,13),
        (10,2), (10,6), (10,14),
        (11,3), (11,7), (11,15),
        (12,4), (12,8), (12,16),
        (13,5), (13,9), (13,17),
        (14,6), (14,10), (14,18),
        (15,7), (15,11), (15,19),
        (16,8), (16,12), (16,0),
        (17,9), (17,13), (17,1),
        (18,10), (18,14), (18,2),
        (19,11), (19,15), (19,3),
    ];

    graphview::Graph::with_slices(nodes, edges, false)
}


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
    let g = graphview::Graph::with_slices(&["A", "B", "C", "D"], &[(0, 1), (0, 2), (1, 3)], true);
    gv.arrange_nodes(graphview::ArrangeMethod::Grid);
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
    let mut gv = graphview!("line-type: SingleThick, routing: Orthogonal, hie: true, hoe: true, arrows: false, arrange: Grid, back:{.,gray,black}, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
    let g = graphview::Graph::with_slices(&["A", "B", "C", "D"], &[(0, 1), (0, 2), (1, 3)], true);
    gv.set_graph(g);
    w.add(gv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_events() {
    #[Window(events = GraphViewEvents<u32>, internal: true)]
    struct MyWin {}
    impl MyWin {
        fn new(g: graphview::Graph<u32>) -> Self {
            let mut w = Self { base: window!("Test,d:f") };
            let mut gv = graphview!("line-type: SingleThick, routing: Orthogonal, hie: true, hoe: true, arrows: false, arrange: Grid, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
            gv.set_graph(g);
            w.add(gv);
            w
        }
    }
    impl GraphViewEvents<u32> for MyWin {
        fn on_current_node_changed(&mut self, handle: Handle<GraphView<u32>>) -> EventProcessStatus {
            let nid = self.control(handle).unwrap().graph().current_node_id().unwrap();
            let f = format!("Node IDX: {}", nid);
            self.set_title(&f);
            EventProcessStatus::Ignored
        }

        fn on_node_action(&mut self, _: Handle<GraphView<u32>>, item_index: usize) -> EventProcessStatus {
            let f = format!("Action IDX: {}", item_index);
            self.set_title(&f);
            EventProcessStatus::Ignored
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0x369776D345DF3EB4) 
        Key.Pressed('Right')
        Paint('2. New node - title is: New IDX: 1')   
        CheckHash(0xCFF990B23499F06B) 
        Key.Pressed('Down')
        Paint('3. New node - title is: New IDX: 3')   
        CheckHash(0xA9F5CB1A947005) 
        Key.Pressed('Enter')
        Paint('4. Action - title is: Action IDX: 3')   
        CheckHash(0xDAB162CDCCC002FF) 
        Key.Pressed('Left')
        Paint('5. New node - title is: New IDX: 2')   
        CheckHash(0xFA69C3E174F6553C)         
        Key.Pressed('Up')
        Paint('6. New node - title is: New IDX: 0')   
        CheckHash(0xDD4D4629B0731CA6)         
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let g = graphview::Graph::with_slices(&[0, 1, 2, 3], &[(0, 1), (0, 2), (1, 3)], true);
    a.add_window(MyWin::new(g));
    a.run();
}

#[test]
fn check_events_with_mouse() {
    #[Window(events = GraphViewEvents<u32>, internal: true)]
    struct MyWin {}
    impl MyWin {
        fn new(g: graphview::Graph<u32>) -> Self {
            let mut w = Self { base: window!("Test,d:f") };
            let mut gv = graphview!("line-type: SingleThick, routing: Orthogonal, hie: true, hoe: true, arrows: false, arrange: Grid, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
            gv.set_graph(g);
            w.add(gv);
            w
        }
    }
    impl GraphViewEvents<u32> for MyWin {
        fn on_current_node_changed(&mut self, handle: Handle<GraphView<u32>>) -> EventProcessStatus {
            let nid = self.control(handle).unwrap().graph().current_node_id().unwrap();
            let f = format!("Node IDX: {}", nid);
            self.set_title(&f);
            EventProcessStatus::Ignored
        }

        fn on_node_action(&mut self, _: Handle<GraphView<u32>>, item_index: usize) -> EventProcessStatus {
            let f = format!("Action IDX: {}", item_index);
            self.set_title(&f);
            EventProcessStatus::Ignored
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0x369776D345DF3EB4) 
        Mouse.Click(9,2,left)
        Paint('2. New node - title is: New IDX: 1')   
        CheckHash(0xCFF990B23499F06B) 
        Mouse.Click(9,4,left)
        Paint('3. New node - title is: New IDX: 3')   
        CheckHash(0xA9F5CB1A947005) 
        Mouse.DoubleClick(9,4,left)
        Paint('4. Action - title is: Action IDX: 3')   
        CheckHash(0xDAB162CDCCC002FF) 
        Mouse.Click(4,4,left)
        Paint('5. New node - title is: New IDX: 2')   
        CheckHash(0xFA69C3E174F6553C)         
        Mouse.Click(4,2,left)
        Paint('6. New node - title is: New IDX: 0')   
        CheckHash(0xDD4D4629B0731CA6)         
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let g = graphview::Graph::with_slices(&[0, 1, 2, 3], &[(0, 1), (0, 2), (1, 3)], true);
    a.add_window(MyWin::new(g));
    a.run();
}

#[test]
fn check_graph_with_margins_for_node_single() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0x7CDDF286305BF42D) 
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,d:f");
    let mut gv = graphview!("line-type: SingleThick, routing: Orthogonal, hie: true, hoe: true, arrows: false, arrange: Grid, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
    let g = graphview::Graph::with_slices_and_border(
        &["Node-A", "Node-B", "Node-C", "Node-D"],
        &[(0, 1), (0, 2), (1, 3)],
        LineType::Single,
        true,
    );
    gv.set_graph(g);
    w.add(gv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_graph_with_margins_for_node_double() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0x91DEDEB78C848365) 
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,d:f");
    let mut gv = graphview!("line-type: SingleThick, routing: Orthogonal, hie: true, hoe: true, arrows: false, arrange: Grid, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
    let g = graphview::Graph::with_slices_and_border(
        &["Node-A", "Node-B", "Node-C", "Node-D"],
        &[(0, 1), (0, 2), (1, 3)],
        LineType::Double,
        true,
    );
    gv.set_graph(g);
    w.add(gv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_hover_node() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xA52D4BE5FE3169FD) 
        Mouse.Move(15,4)
        Paint('2. Hovered over Node-B')   
        CheckHash(0xD82A932648700D17) 
        Mouse.Move(15,7)
        Paint('3. Hovered over Node-D')   
        CheckHash(0x2C607075309AE9F3)      
        Mouse.Move(40,7)
        Paint('4. hovered outside any node')   
        CheckHash(0xA52D4BE5FE3169FD)             
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,d:f");
    let mut gv = graphview!("line-type: SingleThick, routing: Orthogonal, hie: true, hoe: true, arrows: false, arrange: Grid, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
    let g = graphview::Graph::with_slices_and_border(
        &["Node-A", "Node-B", "Node-C", "Node-D"],
        &[(0, 1), (0, 2), (1, 3)],
        LineType::SingleRound,
        true,
    );
    gv.set_graph(g);
    w.add(gv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_tree_view() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xBE6D432823B0BC2B)           
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,d:f");
    let mut gv = graphview!("line-type: SingleThick, routing: Orthogonal, hie: false, hoe: false, arrows: true, arrange: Hierarchical, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
    let g = graphview::Graph::with_slices(
        &["1", "1-1", "1-2", "1-1-1", "1-1-2", "1-2-1", "1-2-2"],
        &[(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6)],
        true,
    );
    gv.set_graph(g);
    w.add(gv);
    a.add_window(w);
    a.run();
}
#[test]
fn check_tree_view_packed() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0x29694D45CF699BF)           
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,d:f");
    let mut gv = graphview!("line-type: SingleThick, routing: Orthogonal, hie: false, hoe: false, arrows: false, arrange: HierarchicalPacked, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
    let g = graphview::Graph::with_slices(
        &["1", "1-1", "1-2", "1-1-1", "1-1-2", "1-2-1", "1-2-2"],
        &[(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6)],
        true,
    );
    gv.set_graph(g);
    w.add(gv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_custom_graph() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xD896F64DAEC2F4B0)           
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Test,d:f");
    let mut gv = graphview!("line-type: SingleThick, routing: Orthogonal, hie: false, hoe: false, arrows: false, arrange: Hierarchical, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
    gv.set_graph(build_custom_graph_1());
    w.add(gv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_arrange_hierarchical() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xC6B8B2D87813BCED)           
    ";
    let mut a = App::debug(130, 15, script).build().unwrap();
    let mut w = window!("Test,d:f");
    let mut gv = graphview!("line-type: Single, routing: Orthogonal, hie: false, hoe: false, arrows: false, arrange: Hierarchical, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
    gv.set_graph(build_custom_graph_2());
    w.add(gv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_arrange_grid() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0x1F4C38ABDB6BB845)           
    ";
    let mut a = App::debug(130, 15, script).build().unwrap();
    let mut w = window!("Test,d:f");
    let mut gv = graphview!("line-type: Single, routing: Orthogonal, hie: false, hoe: false, arrows: false, arrange: Grid, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
    gv.set_graph(build_custom_graph_2());
    w.add(gv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_arrange_circular() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0x710407C8369BB527)           
    ";
    let mut a = App::debug(60, 30, script).build().unwrap();
    let mut w = window!("Test,d:f");
    let mut gv = graphview!("line-type: Single, routing: Orthogonal, hie: false, hoe: false, arrows: false, arrange: Circular, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
    gv.set_graph(build_custom_graph_2());
    w.add(gv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_arrange_force_directed() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xCF8E04F6F85CD000)           
    ";
    let mut a = App::debug(120, 30, script).build().unwrap();
    let mut w = window!("Test,d:f");
    let mut gv = graphview!("line-type: Single, routing: Orthogonal, hie: false, hoe: false, arrows: false, arrange: ForceDirected, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
    gv.set_graph(build_custom_graph_2());
    w.add(gv);
    a.add_window(w);
    a.run();
}


#[test]
fn check_mouse_wheel() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0x2A306A3D0F8190EA)
        Mouse.Wheel(20,10,right,3)
        Paint('2. Scroll right 3 times')   
        CheckHash(0x7E3EDB940E168E0F)
        Mouse.Wheel(20,10,down,10)
        Paint('3. Scroll down 10 times')   
        CheckHash(0x29BD7AAF1237E703)
        Mouse.Wheel(20,10,left,2)
        Paint('4. Scroll left 2 times')   
        CheckHash(0x90627173D19DEAC0)
        Mouse.Wheel(20,10,up,4)
        Paint('5. Scroll left 4 times')   
        CheckHash(0xA8F8EDAEAFB28238)
    ";
    let mut a = App::debug(40, 20, script).build().unwrap();
    let mut w = window!("Test,d:f");
    let mut gv = graphview!("line-type: Single, routing: Orthogonal, hie: false, hoe: false, arrows: false, arrange: ForceDirected, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
    gv.set_graph(build_custom_graph_2());
    w.add(gv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_scroll_view_via_keyboard() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0x2A306A3D0F8190EA)
        Key.Pressed(Alt+Right,3)
        Paint('2. Scroll right 3 times')   
        CheckHash(0x7E3EDB940E168E0F)
        Key.Pressed(Alt+Down,10)
        Paint('3. Scroll down 10 times')   
        CheckHash(0x29BD7AAF1237E703)
        Key.Pressed(Alt+Left,2)
        Paint('4. Scroll left 2 times')   
        CheckHash(0x90627173D19DEAC0)
        Key.Pressed(Alt+Up,4)
        Paint('5. Scroll left 4 times')   
        CheckHash(0xA8F8EDAEAFB28238)
        Key.Pressed(PageDown)
        Paint('6. Scroll Down One Page')   
        CheckHash(0x1082AF67FBBE796A)
        Key.Pressed(PageUp)
        Paint('7. Scroll Up One Page')   
        CheckHash(0x4617C19A7708D5B3)
        Key.Pressed(Home)
        Paint('8. Goto top-left of the graph')   
        CheckHash(0x2A306A3D0F8190EA)
        Key.Pressed(End)
        Paint('9. Goto bottom-right of graph')   
        CheckHash(0xB37A0A8CC46F7CCB)
    ";
    let mut a = App::debug(40, 20, script).build().unwrap();
    let mut w = window!("Test,d:f");
    let mut gv = graphview!("line-type: Single, routing: Orthogonal, hie: false, hoe: false, arrows: false, arrange: ForceDirected, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
    gv.set_graph(build_custom_graph_2());
    w.add(gv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_search_bar() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0x710407C8369BB527)   
        Key.TypeText('N1')        
        Paint('2. Nodes that conrain N1 are selected')   
        CheckHash(0xF95D8B2BE34ED00B)   
        Key.Pressed(Enter)
        Paint('3. Move to Next node found (-> N10)')   
        CheckHash(0xF4A63DCADBB69072)   
        Key.Pressed(Ctrl+Enter,2)
        Paint('4. Move backwards twice to Previous node found (-> N19)')   
        CheckHash(0x64E4BBBE905D4056)   
        Key.TypeText('7')        
        Paint('5. Now only N17 is selected')   
        CheckHash(0xCD55F86BFF9BC314)   
        Key.Pressed(Escape)       
        Paint('6. Clear search text (N17 is still current node'))   
        CheckHash(0x836840446A2F8D16)   
        Key.Pressed(Escape)       
        Paint('7. Exit window (Escape is no longer processed)')   
        CheckHash(0x9EE74A86D600A6F5)   
    ";
    let mut a = App::debug(60, 30, script).build().unwrap();
    let mut w = window!("Test,d:f");
    let mut gv = graphview!("line-type: Single, routing: Orthogonal, hie: false, hoe: false, arrows: false, arrange: Circular, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
    gv.set_graph(build_custom_graph_2());
    w.add(gv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_moving_nodes_with_keyboard() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (on node <1>)')   
        CheckHash(0x11284079999C4D55)
        Key.Pressed(Right)           
        Paint('2. Move to node: <1-2-1>')   
        CheckHash(0x94ECE0C4BEEF149)
        Key.Pressed(Right)           
        Paint('3. Move to node: <1-2>')   
        CheckHash(0x315E4E42E45F35AD)
        Key.Pressed(Right)              
        Paint('4. Move to node: <1-2-2>')   
        CheckHash(0x46209BF2096F9E75)
        Key.Pressed(Left, 2)           
        Paint('5. Move to node: <1-2-2>')   
        CheckHash(0x94ECE0C4BEEF149)
        Key.Pressed(Up)           
        Paint('6. Move to node: <1-2>')   
        CheckHash(0x315E4E42E45F35AD)
        Key.Pressed(Ctrl+Right,10)           
        Paint('7. Move node: <1-2> to a right most position')   
        CheckHash(0x1BDE5E242E760D11)
        Key.Pressed(Ctrl+Up,3)           
        Paint('8. Move node: <1-2> to a top position (by 3)')   
        CheckHash(0x14E7FC3ACDE047D1)        
        Key.Pressed(Ctrl+Left,12)           
        Paint('9. Move node: <1-2> to a left most poition')   
        CheckHash(0x6FFBC2338F6C3597)        
        Key.Pressed(Down)           
        Paint('10. Move to node: <1-2-1>')   
        CheckHash(0xFABC5CEE2E3A23A3)
        Key.Pressed(Ctrl+Down,5)           
        Paint('11. Move node: <1-2-1> down, also so scroll moves to ensure <1-2-1> is visible')   
        CheckHash(0x60E8B63004E31F9C)        
        Key.Pressed(Ctrl+Left,40)           
        Paint('12. Move node: <1-2-1> left, also so scroll moves to ensure <1-2-1> is visible')   
        CheckHash(0x1B960839B2B9F81E)        
        Key.Pressed(Ctrl+Up,10)           
        Paint('13. Move node: <1-2-1> up, also so scroll moves to ensure <1-2-1> is visible')   
        CheckHash(0xDDD275848378D703)        
        Key.Pressed(Ctrl+Right,60)           
        Paint('14. Move node: <1-2-1> right, also so scroll moves to ensure <1-2-1> is visible')   
        CheckHash(0x97B0C6BD53B493DB)        
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:f");
    let mut gv = graphview!("line-type: SingleThick, routing: Orthogonal, hie: false, hoe: false, arrows: true, arrange: Hierarchical, d:f, flags:[ScrollBars,SearchBar],lsm:2,tsm:1");
    let g = graphview::Graph::with_slices(
        &["1", "1-1", "1-2", "1-1-1", "1-1-2", "1-2-1", "1-2-2"],
        &[(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6)],
        true,
    );
    gv.set_graph(g);
    w.add(gv);
    a.add_window(w);
    a.run();
}