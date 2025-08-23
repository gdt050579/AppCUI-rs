use super::edge::Edge;
use super::graph::Graph;
use super::initialization_flags::Flags;
use super::node::Node;
use crate::{prelude::*, ui::graphview::GraphNode};

use self::components::ScrollBars;

struct StateAttr {
    text: CharAttribute,
    border: CharAttribute,
    use_custom: bool,
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct GraphView<T>
where
    T: GraphNode,
{
    graph: Graph<T>,
    origin_point: Point,
    surface: Surface,
    background: Option<Character>,
    flags: Flags,
    drag_point: Option<Point>,
    scrollbars: ScrollBars,
}
impl<T> GraphView<T>
where
    T: GraphNode,
{
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(
                layout,
                (StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput)
                    | if flags == Flags::ScrollBars {
                        StatusFlags::IncreaseBottomMarginOnFocus | StatusFlags::IncreaseRightMarginOnFocus
                    } else {
                        StatusFlags::None
                    },
            ),
            flags,
            origin_point: Point::ORIGIN,
            background: None,
            drag_point: None,
            graph: Graph::default(),
            surface: Surface::new(200, 200),
            scrollbars: ScrollBars::new(flags == Flags::ScrollBars),
        }
    }

    /// Sets the background of the GraphView to the specified character.
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// let mut GraphView = GraphView::new(Size::new(100, 100), layout!("x:1,y:1,w:30,h:10"), GraphView::Flags::ScrollBars);
    /// GraphView.set_background(Character::new('*', Color::White, Color::Black, CharFlags::None));
    /// ```
    pub fn set_background(&mut self, backgroud_char: Character) {
        self.background = Some(backgroud_char);
    }

    /// Clears the background character of the GraphView. It esentially resets it to transparent foreground and backgroud colors
    pub fn clear_background(&mut self) {
        self.background = None;
    }

    pub fn set_graph(&mut self, graph: Graph<T>) {
        self.graph = graph;
        super::layout::hierarchical_bfs::rearange(&mut self.graph);
        self.graph.update_surface_size();
        self.surface.resize(self.graph.size());
        self.repaint_graph();
    }

    // fn compute_state_attr(&self) -> StateAttr {
    //     let theme = self.theme();
    //     if !self.is_enabled() {
    //         StateAttr {
    //             text: theme.text.inactive,
    //             border: theme.border.inactive,
    //             use_custom: false,
    //         }
    //     } else if self.has_focus() {
    //         StateAttr {
    //             text: theme.text.focused,
    //             border: theme.border.focused,
    //             use_custom: true,
    //         }
    //     } else {
    //         StateAttr {
    //             text: theme.text.normal,
    //             border: theme.border.normal,
    //             use_custom: false,
    //         }
    //     }
    // }

    fn compute_state_attr(&self) -> StateAttr {
        let theme = self.theme();
        if !self.is_enabled() {
            StateAttr {
                text: theme.button.text.inactive,
                border: theme.button.hotkey.inactive,
                use_custom: false,
            }
        } else if self.has_focus() {
            StateAttr {
                text: theme.button.text.focused,
                border: theme.button.hotkey.focused,
                use_custom: true,
            }
        } else {
            StateAttr {
                text: theme.button.text.normal,
                border: theme.button.hotkey.normal,
                use_custom: false,
            }
        }
    }

    fn repaint_graph(&mut self) {
        let ch = self
            .background
            .unwrap_or(Character::new(' ', Color::Transparent, Color::Transparent, CharFlags::None));
        let state = self.compute_state_attr();
        self.surface.reset();
        for c in &mut self.surface.chars {
            *c = ch;
        }
        // first draw the lines
        for e in &self.graph.edges {
            let p1 = self.graph.nodes[e.from_node_id as usize].rect.center();
            let p2 = self.graph.nodes[e.to_node_id as usize].rect.center();
            self.surface.draw_orthogonal_line(
                p1.x,
                p1.y,
                p2.x,
                p2.y,
                LineType::Single,
                OrthogonalDirection::Auto,
                charattr!("white"),
            );
        }
        let mut out = String::with_capacity(128);
        for node in &self.graph.nodes {
            out.clear();
            let (t, b) = if state.use_custom {
                (node.text_attr.unwrap_or(state.text), node.border_attr.unwrap_or(state.border))
            } else {
                (state.text, state.border)
            };
            node.paint(&mut self.surface, t, b, &mut out);
        }
    }
    fn move_scroll_to(&mut self, x: i32, y: i32) {
        let sz = self.size();
        let surface_size = self.surface.size();
        self.origin_point.x = if surface_size.width <= sz.width {
            0
        } else {
            x.max((sz.width as i32) - (surface_size.width as i32))
        };
        self.origin_point.y = if surface_size.height <= sz.height {
            0
        } else {
            y.max((sz.height as i32) - (surface_size.height as i32))
        };
        self.origin_point.x = self.origin_point.x.min(0);
        self.origin_point.y = self.origin_point.y.min(0);
        self.scrollbars.set_indexes((-self.origin_point.x) as u64, (-self.origin_point.y) as u64);
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        let h = -(self.scrollbars.horizontal_index() as i32);
        let v = -(self.scrollbars.vertical_index() as i32);
        self.move_scroll_to(h, v);
    }
}
impl<T> OnResize for GraphView<T>
where
    T: GraphNode,
{
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        let paint_sz = self.surface.size();
        self.scrollbars.resize(paint_sz.width as u64, paint_sz.height as u64, &self.base);
        self.move_scroll_to(self.origin_point.x, self.origin_point.y);
    }
}
impl<T> OnPaint for GraphView<T>
where
    T: GraphNode,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if (self.has_focus()) && (self.flags == Flags::ScrollBars) {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        if let Some(back) = self.background {
            surface.clear(back);
        }
        surface.draw_surface(self.origin_point.x, self.origin_point.y, &self.surface);
    }
}
impl<T> OnKeyPressed for GraphView<T>
where
    T: GraphNode,
{
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl<T> OnMouseEvent for GraphView<T>
where
    T: GraphNode,
{
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.scrollbars.process_mouse_event(event) {
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}
